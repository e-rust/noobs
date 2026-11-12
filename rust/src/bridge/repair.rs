//! Reparo automático de gravações interrompidas.
//!
//! Uma gravação cortada antes do fim (Windows desligado, queda de energia, o app
//! encerrado à força) é um MKV sem o trailer: sem `Duration` e sem o índice `Cues`.
//! Ele toca, mas abrir lê o arquivo inteiro para achar a duração e pular na barra
//! anda sem índice. Aqui:
//!
//! * [`check`] (worker da varredura da biblioteca): lê só o cabeçalho; sem duração,
//!   o arquivo entra na fila. Os íntegros ganham a marca `intactSig` no cache (tamanho
//!   e data) e não são reabertos enquanto não mudarem.
//! * Uma worker conserta um por vez: copia TODAS as faixas, sem recodificar, para
//!   `<arquivo>.reparo.part` ([`ffmpeg_ops::repair_mkv`], que confere o resultado);
//!   troca pelo original (renomeia o original para `.original`, o novo para o nome
//!   dele, devolve a data de modificação) e manda o original para a Lixeira.
//!
//! Nada é feito num arquivo que outro processo esteja ESCREVENDO (a abertura negando
//! escrita falha: a gravação em curso, o OBS gravando na mesma pasta). Aberto para
//! leitura (o player, a exportação) a troca falha no renomear, tudo é desfeito e ele
//! volta a ser checado na próxima varredura.

use std::collections::VecDeque;
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use serde_json::{json, Value};

use crate::{ffmpeg_ops, file_meta, lang, log, mainthread, player};

use super::is_shutting_down;
use super::library::{delete_to_recycle_bin, is_file_cloud_only, push_recordings};

const FILE_SHARE_READ: u32 = 0x1;
const FILE_SHARE_DELETE: u32 = 0x4;
/// A marca do cache: o arquivo (com este tamanho e data) foi conferido e está íntegro.
const KEY_INTACT: &str = "intactSig";
/// Modificado há menos que isto: pode estar sendo escrito ainda.
const SETTLE: Duration = Duration::from_secs(30);

static QUEUE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
static RUNNING: AtomicBool = AtomicBool::new(false);
/// O que a worker está consertando agora: a varredura pode rodar de novo no meio (e
/// sem isto o mesmo arquivo entrava na fila de novo e era consertado duas vezes).
static CURRENT: Mutex<String> = Mutex::new(String::new());

/// Tamanho e data de modificação: a assinatura do arquivo para a marca de íntegro.
fn sig(path: &str) -> String {
    let Ok(md) = std::fs::metadata(path) else { return String::new() };
    let ms = md.modified().ok().and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok()).map_or(0, |d| d.as_millis());
    format!("{}:{ms}", md.len())
}

fn marked_intact(path: &str, sig: &str) -> bool {
    let v = player::load_meta_subset(path, &[KEY_INTACT]);
    serde_json::from_str::<Value>(&v).ok().and_then(|v| v.get(KEY_INTACT).and_then(Value::as_str).map(|s| s == sig)).unwrap_or(false)
}

/// Alguém está escrevendo o arquivo? (A abertura que nega escrita aos outros falha.)
fn being_written(path: &str) -> bool {
    std::fs::OpenOptions::new().read(true).share_mode(FILE_SHARE_READ | FILE_SHARE_DELETE).open(path).is_err()
}

/// Worker. Confere `path` (só o cabeçalho) e, se for um MKV interrompido, põe na fila.
pub fn check(path: &str) {
    if !path.to_lowercase().ends_with(".mkv") || is_file_cloud_only(path) {
        return;
    }
    let s = sig(path);
    if s.is_empty() || marked_intact(path, &s) {
        return;
    }
    match ffmpeg_ops::mkv_lacks_duration(path) {
        Some(false) => player::save_meta_subset(path, &json!({ KEY_INTACT: s }).to_string()),
        Some(true) => enqueue(path),
        None => {}
    }
}

/// Põe na fila (sem repetir) e acorda a worker.
pub fn enqueue(path: &str) {
    {
        let mut q = QUEUE.lock().unwrap_or_else(|p| p.into_inner());
        let current = CURRENT.lock().unwrap_or_else(|p| p.into_inner());
        if current.eq_ignore_ascii_case(path) || q.iter().any(|p| p.eq_ignore_ascii_case(path)) {
            return;
        }
        q.push_back(path.to_owned());
    }
    log!("Reparo: \"{}\" sem duracao (gravacao interrompida) — na fila.", file_name(path));
    if RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }
    let spawned = std::thread::Builder::new().name("noobs-reparo".into()).spawn(|| {
        loop {
            let next = {
                // Tira da fila e marca como "em conserto" sob a mesma trava da fila.
                let mut q = QUEUE.lock().unwrap_or_else(|p| p.into_inner());
                let next = q.pop_front();
                *CURRENT.lock().unwrap_or_else(|p| p.into_inner()) = next.clone().unwrap_or_default();
                next
            };
            let Some(path) = next else { break };
            if is_shutting_down() {
                break;
            }
            repair_one(&path);
        }
        CURRENT.lock().unwrap_or_else(|p| p.into_inner()).clear();
        RUNNING.store(false, Ordering::SeqCst);
    });
    if spawned.is_err() {
        RUNNING.store(false, Ordering::SeqCst);
    }
}

fn file_name(p: &str) -> String {
    Path::new(p).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
}

fn with_suffix(path: &str, suffix: &str) -> PathBuf {
    PathBuf::from(format!("{path}{suffix}"))
}

/// Conserta um arquivo. Qualquer falha deixa o original como estava.
fn repair_one(path: &str) {
    let name = file_name(path);
    let Ok(md) = std::fs::metadata(path) else { return };
    let mtime = md.modified().unwrap_or(SystemTime::now());
    if SystemTime::now().duration_since(mtime).unwrap_or_default() < SETTLE || being_written(path) {
        log!("Reparo: \"{name}\" em uso (sendo escrito) — fica para depois.");
        return;
    }
    // Ainda precisa? (Entre entrar na fila e chegar a vez, ele pode ter sido consertado.)
    if ffmpeg_ops::mkv_lacks_duration(path) != Some(true) {
        return;
    }
    let part = with_suffix(path, ".reparo.part");
    let backup = with_suffix(path, ".original");
    let _ = std::fs::remove_file(&part);
    let started = std::time::Instant::now();
    let cancel = || is_shutting_down();
    let result = ffmpeg_ops::repair_mkv(path, &part.to_string_lossy(), Some(&cancel));
    let check = match result {
        Ok(c) => c,
        Err(e) => {
            let _ = std::fs::remove_file(&part);
            log!("Reparo: \"{name}\" falhou ({e}); o original fica como estava.");
            return;
        }
    };
    // A duração já medida (cache da biblioteca) bate com a do reparado? Uma cópia que
    // parou no meio sairia mais curta.
    let cached = player::get_cached_meta(path).0;
    if cached > 0 && (check.duration - f64::from(cached)).abs() > 2.0 {
        let _ = std::fs::remove_file(&part);
        log!("Reparo: \"{name}\" saiu com {:.1} s, o original tem {cached} s — descartado.", check.duration);
        return;
    }
    // A troca: original → .original; reparado → nome do original; original → Lixeira.
    if let Err(e) = std::fs::rename(path, &backup) {
        let _ = std::fs::remove_file(&part);
        log!("Reparo: \"{name}\" em uso (aberto no player ou em outro programa: {e}) — fica para depois.");
        return;
    }
    if let Err(e) = std::fs::rename(&part, path) {
        let _ = std::fs::rename(&backup, path);
        let _ = std::fs::remove_file(&part);
        log!("Reparo: \"{name}\" — nao deu para pôr o reparado no lugar ({e}); o original voltou.");
        return;
    }
    // A data de modificação do original: a biblioteca agrupa e ordena por ela.
    if let Ok(f) = std::fs::OpenOptions::new().write(true).open(path) {
        let _ = f.set_modified(mtime);
    }
    let recycled = delete_to_recycle_bin(&backup.to_string_lossy());
    if !recycled {
        log!("Reparo: o original de \"{name}\" ficou em {} (a Lixeira recusou).", backup.display());
    }
    log!(
        "Reparo: \"{name}\" corrigido em {:.0} s — {:.1} s, {} faixa(s), {} keyframes no indice; original {}.",
        started.elapsed().as_secs_f64(),
        check.duration,
        check.streams,
        check.keyframes,
        if recycled { "na Lixeira" } else { "mantido ao lado" }
    );
    // O arquivo agora termina certo: os dados do NoOBS (layout, transcrição) podem ir
    // para o fim dele, e ele não precisa ser conferido de novo.
    player::save_meta_subset(path, &json!({ KEY_INTACT: sig(path) }).to_string());
    file_meta::mark_changed(path);
    let path = path.to_owned();
    mainthread::queue(move || {
        push_recordings();
        let msg = lang::tr_with("A gravação {{file}} tinha sido interrompida e foi corrigida. O arquivo original está na Lixeira.", &[("file", &file_name(&path))]);
        crate::view::call(move |v| v.toast(&lang::tr("Gravação corrigida"), &msg, false, 8000));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Data de modificação de 1 hora atrás: passa da espera de "pode estar sendo escrito".
    fn envelhece(path: &str) -> SystemTime {
        let old = SystemTime::now() - Duration::from_secs(3600);
        std::fs::OpenOptions::new().write(true).open(path).unwrap().set_modified(old).unwrap();
        std::fs::metadata(path).unwrap().modified().unwrap()
    }

    /// A troca de verdade: o reparado fica no lugar, com a data do original; o original
    /// vai para a Lixeira (no teste, apagado); nada sobra ao lado.
    #[test]
    fn repara_no_lugar_e_manda_o_original_para_a_lixeira() {
        let s = crate::testkit::begin();
        if !crate::testkit::media() {
            return;
        }
        let path = s.sample("interrompido.mkv", "2026-11-09 08-30-15.mkv");
        let antes = envelhece(&path);
        assert_eq!(ffmpeg_ops::mkv_lacks_duration(&path), Some(true));
        repair_one(&path);
        assert_eq!(ffmpeg_ops::mkv_lacks_duration(&path), Some(false), "o arquivo no lugar tem duração");
        assert_eq!(std::fs::metadata(&path).unwrap().modified().unwrap(), antes, "a data do original");
        assert!(!with_suffix(&path, ".reparo.part").exists(), "sem o .part");
        assert!(!with_suffix(&path, ".original").exists(), "o original saiu (Lixeira)");
        assert!(crate::testkit::effects().iter().any(|e| e.starts_with("recycle:") && e.ends_with(".original")), "pela Lixeira");
        // Conferido como íntegro: a próxima varredura não abre de novo.
        assert!(marked_intact(&path, &sig(&path)));
    }

    /// Arquivo sendo escrito (a gravação em curso, o OBS): não mexe.
    #[test]
    fn nao_mexe_em_arquivo_sendo_escrito() {
        let s = crate::testkit::begin();
        if !crate::testkit::media() {
            return;
        }
        let path = s.sample("interrompido.mkv", "gravando.mkv");
        envelhece(&path);
        let escritor = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
        assert!(being_written(&path));
        repair_one(&path);
        drop(escritor);
        assert_eq!(ffmpeg_ops::mkv_lacks_duration(&path), Some(true), "intocado");
        assert!(!with_suffix(&path, ".reparo.part").exists());
    }

    /// Aberto para leitura (o player): a troca falha no renomear e tudo volta.
    #[test]
    fn aberto_no_player_fica_como_estava() {
        let s = crate::testkit::begin();
        if !crate::testkit::media() {
            return;
        }
        let path = s.sample("interrompido.mkv", "no-player.mkv");
        envelhece(&path);
        let bytes = std::fs::read(&path).unwrap();
        let leitor = crate::ffi::ffmpeg::Input::open(&path).unwrap();
        repair_one(&path);
        drop(leitor);
        assert_eq!(std::fs::read(&path).unwrap(), bytes, "o original, byte a byte");
        assert!(!with_suffix(&path, ".reparo.part").exists());
        assert!(!with_suffix(&path, ".original").exists());
    }

    /// Recente demais (pode estar sendo escrito): espera.
    #[test]
    fn recem_modificado_espera() {
        let s = crate::testkit::begin();
        if !crate::testkit::media() {
            return;
        }
        let path = s.sample("interrompido.mkv", "recente.mkv");
        // A cópia preserva a data do fixture: marca agora, como uma gravação que acabou de parar.
        std::fs::OpenOptions::new().write(true).open(&path).unwrap().set_modified(SystemTime::now()).unwrap();
        repair_one(&path);
        assert_eq!(ffmpeg_ops::mkv_lacks_duration(&path), Some(true));
    }

    /// Íntegro: marcado no cache e não entra na fila.
    #[test]
    fn integro_ganha_a_marca_e_nao_entra_na_fila() {
        let s = crate::testkit::begin();
        if !crate::testkit::media() {
            return;
        }
        let path = s.sample("h264_2faixas.mkv", "inteiro.mkv");
        check(&path);
        assert!(marked_intact(&path, &sig(&path)));
        assert!(!QUEUE.lock().unwrap().iter().any(|p| p == &path));
    }
}

#[cfg(test)]
mod tests_fila {
    use super::*;

    /// O que já foi consertado não é consertado de novo (a 2ª entrada da fila desiste).
    #[test]
    fn consertado_nao_e_consertado_de_novo() {
        let s = crate::testkit::begin();
        if !crate::testkit::media() {
            return;
        }
        let path = s.sample("interrompido.mkv", "duas-vezes.mkv");
        let old = SystemTime::now() - Duration::from_secs(3600);
        std::fs::OpenOptions::new().write(true).open(&path).unwrap().set_modified(old).unwrap();
        repair_one(&path);
        let depois = std::fs::read(&path).unwrap();
        let recicladas = crate::testkit::effects().iter().filter(|e| e.starts_with("recycle:")).count();
        repair_one(&path);
        assert_eq!(std::fs::read(&path).unwrap(), depois, "o 2º conserto não mexe");
        assert_eq!(crate::testkit::effects().iter().filter(|e| e.starts_with("recycle:")).count(), recicladas, "nada a mais na Lixeira");
    }

    /// O arquivo em conserto não volta para a fila.
    #[test]
    fn em_conserto_nao_entra_de_novo_na_fila() {
        *CURRENT.lock().unwrap() = r"C:\v\em-conserto.mkv".into();
        // Sem a worker: só a decisão da fila.
        RUNNING.store(true, Ordering::SeqCst);
        enqueue(r"C:\V\EM-CONSERTO.mkv");
        assert!(!QUEUE.lock().unwrap().iter().any(|p| p.eq_ignore_ascii_case(r"C:\v\em-conserto.mkv")));
        CURRENT.lock().unwrap().clear();
        RUNNING.store(false, Ordering::SeqCst);
    }
}
