//! O player acelerado, medido de verdade: abre o motor (sem a janela do app), toca em
//! 1×, 2×, 4×, 8× e 16× e conta os quadros que chegam à interface e o maior salto entre
//! eles, em segundos do vídeo. Pular de keyframe em keyframe aparece como um salto de
//! segundos; tocando liso, o salto fica perto de velocidade ÷ quadros por segundo.
//!
//!   cargo test --release --features app-tests --test player -- <arquivo> [<arquivo>...]
//!
//! Cada arquivo imprime uma linha por velocidade: `<arquivo> <velocidade>× <qps> qps,
//! maior salto <s> s, saltos > 0,5 s: <n>`.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use noobs::playback::{Out, Playback, ViewSpec};

fn main() {
    let runtime = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("runtime");
    let w: Vec<u16> = runtime.to_string_lossy().encode_utf16().chain([0]).collect();
    unsafe {
        let _ = windows::Win32::System::LibraryLoader::SetDllDirectoryW(windows::core::PCWSTR(w.as_ptr()));
    }
    let files: Vec<String> = std::env::args().skip(1).filter(|a| !a.starts_with('-')).collect();
    if files.is_empty() {
        println!("PULADO player: passe os arquivos depois de --");
        return;
    }
    if std::env::args().any(|a| a == "--seek") {
        for file in files {
            for playing in [false, true] {
                measure_seek(&file, playing);
            }
        }
        return;
    }
    for file in files {
        for rate in [1.0, 2.0, 4.0, 8.0, 16.0] {
            measure(&file, rate);
        }
    }
}

/// Clicar na barra: pula para frente e para trás (tocando ou parado) e lê a posição que a
/// barra mostra a cada 10 ms por 1,5 s. Uma linha por clique: o alvo, onde a barra ficou
/// logo depois, o quanto ela voltou para antes do alvo (ou foi além do que o tempo
/// explica) e em quanto tempo assentou.
///
///   cargo test --release --features app-tests --test player -- --seek <arquivo>
fn measure_seek(file: &str, playing: bool) {
    let view = ViewSpec { stage_w: 1920, stage_h: 1080, ..Default::default() };
    let out = Arc::new(|_o: Out| {});
    let p = Playback::open_paused(file, view, out);
    // Sem `Duration` no arquivo, o motor varre os pacotes antes de saber a duração.
    let opened = Instant::now();
    while p.shared.duration() <= 0.0 && opened.elapsed() < Duration::from_secs(30) {
        std::thread::sleep(Duration::from_millis(50));
    }
    std::thread::sleep(Duration::from_millis(1500));
    let dur = p.shared.duration();
    let name = std::path::Path::new(file).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    println!("{name} ({dur:.1} s) {}", if playing { "TOCANDO" } else { "PARADO" });
    if playing {
        p.play();
        std::thread::sleep(Duration::from_millis(800));
    }
    for frac in [0.30, 0.70, 0.40, 0.45, 0.20, 0.90, 0.10] {
        let target = dur * frac;
        let start = Instant::now();
        p.seek(target);
        let mut samples = Vec::new();
        let mut clocks = Vec::new();
        while start.elapsed() < Duration::from_millis(1500) {
            std::thread::sleep(Duration::from_millis(10));
            samples.push((start.elapsed().as_secs_f64(), p.shared.position()));
            clocks.push(p.shared.clock.now());
        }
        // Quanto a barra (último quadro mostrado) ficou atrás do relógio da reprodução.
        let behind_clock = samples.iter().zip(&clocks).map(|((_, pos), c)| c - pos).fold(f64::MIN, f64::max);
        // O que se vê como "a barra voltou": ficar antes do ponto clicado, ou recuar entre
        // duas leituras.
        let before_click = samples.iter().map(|(_, pos)| target - pos).fold(0.0f64, f64::max);
        let max_backstep = samples.windows(2).map(|w| w[0].1 - w[1].1).fold(0.0f64, f64::max);
        let clock_end =clocks.last().copied().unwrap_or(0.0) - samples.last().map_or(0.0, |s| if playing { target + s.0 } else { target });
        // Onde deveria estar em cada amostra: o alvo, mais o tempo corrido se tocando.
        let expected = |t: f64| if playing { target + t } else { target };
        let worst_back = samples.iter().map(|(t, pos)| expected(*t) - pos).fold(f64::MIN, f64::max);
        let worst_ahead = samples.iter().map(|(t, pos)| pos - expected(*t)).fold(f64::MIN, f64::max);
        let settled = samples.iter().rposition(|(t, pos)| (pos - expected(*t)).abs() > 0.15).map_or(0.0, |i| samples[(i + 1).min(samples.len() - 1)].0);
        let at_100ms = samples.iter().find(|(t, _)| *t >= 0.1).map_or(0.0, |s| s.1);
        let last = samples.last().map_or(0.0, |s| s.1 - expected(s.0));
        println!(
            "  alvo {target:8.2} s | aos 100 ms {at_100ms:8.2} s | atrás no pior {worst_back:6.2} s | à frente no pior {worst_ahead:6.2} s | no fim {last:+6.2} s | assentou em {:.0} ms | barra atrás do relógio no pior {behind_clock:5.2} s | relógio no fim {clock_end:+6.2} s | antes do clique {before_click:4.2} s | maior recuo {max_backstep:4.2} s",
            settled * 1000.0
        );
    }
    // Arrastar o polegar: um seek a cada 16 ms (o mouse) por 400 ms, de `from` a `to`,
    // e solta. Depois de soltar, a barra tem que ficar no ponto final, sem voltar a um
    // ponto do meio do arrasto.
    for (from, to) in [(0.30, 0.50), (0.60, 0.40)] {
        let (a, b) = (dur * from, dur * to);
        let steps = 25;
        let mut samples = Vec::new();
        for i in 0..=steps {
            let t = a + (b - a) * i as f64 / steps as f64;
            p.seek(t);
            std::thread::sleep(Duration::from_millis(16));
            samples.push(p.shared.position());
        }
        let released = Instant::now();
        let mut after = Vec::new();
        while released.elapsed() < Duration::from_millis(1500) {
            std::thread::sleep(Duration::from_millis(10));
            after.push((released.elapsed().as_secs_f64(), p.shared.position()));
        }
        let expected = |t: f64| if playing { b + t } else { b };
        // Durante o arrasto, a barra deve acompanhar o mouse no sentido dele.
        let dir = (b - a).signum();
        let against = samples.windows(2).map(|w| (w[0] - w[1]) * dir).fold(0.0f64, f64::max);
        let worst_after = after.iter().map(|(t, pos)| (pos - expected(*t)).abs()).fold(0.0f64, f64::max);
        let last = after.last().map_or(0.0, |s| s.1 - expected(s.0));
        println!(
            "  arrasto {a:8.2} → {b:8.2} s | contra o mouse no pior {against:6.2} s | depois de soltar, longe do ponto no pior {worst_after:6.2} s | no fim {last:+6.2} s"
        );
    }
    p.pause();
    drop(p);
    std::thread::sleep(Duration::from_millis(300));
}

fn measure(file: &str, rate: f64) {
    let shared: Arc<OnceLock<Arc<noobs::playback::Shared>>> = Arc::default();
    let frames: Arc<Mutex<Vec<(Instant, f64)>>> = Arc::default();
    let out = {
        let (shared, frames) = (shared.clone(), frames.clone());
        Arc::new(move |o: Out| {
            if let (Out::Frame { .. }, Some(s)) = (o, shared.get()) {
                // A interface pega só o mais recente: aqui ele é "mostrado" na hora.
                s.ui_pending.store(false, std::sync::atomic::Ordering::Relaxed);
                frames.lock().unwrap().push((Instant::now(), s.position()));
            }
        })
    };
    let view = ViewSpec { stage_w: 1920, stage_h: 1080, ..Default::default() };
    let mut p = Playback::open_paused(file, view, out);
    let _ = shared.set(p.shared.clone());
    std::thread::sleep(Duration::from_millis(1500));
    p.set_rate(rate);
    p.play();
    // Um segundo para assentar; mede os seis seguintes (ou até o fim do arquivo).
    std::thread::sleep(Duration::from_secs(1));
    frames.lock().unwrap().clear();
    let start = Instant::now();
    std::thread::sleep(Duration::from_secs(6));
    let elapsed = start.elapsed().as_secs_f64();
    p.pause();
    let list = frames.lock().unwrap().clone();
    let fps = list.len() as f64 / elapsed;
    let gaps: Vec<f64> = list.windows(2).map(|w| w[1].1 - w[0].1).collect();
    let max_gap = gaps.iter().copied().fold(0.0, f64::max);
    let big = gaps.iter().filter(|g| **g > 0.5).count();
    let stalls = list.windows(2).map(|w| w[1].0 - w[0].0).max().unwrap_or_default();
    let name = std::path::Path::new(file).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    if rate == 1.0 {
        let info = noobs::probe::probe(std::path::Path::new(file));
        let v = info.as_ref().and_then(|r| r.video_stream().cloned());
        let kf = noobs::ffmpeg_ops::list_video_keyframes(file).map(|k| k.0.len()).unwrap_or(0);
        let dur = info.as_ref().map_or(0.0, |r| r.duration);
        if let Some(v) = v {
            println!("{name}: {}x{} {} {:.1} qps, {kf} keyframes em {dur:.0} s (um a cada {:.1} s)", v.width, v.height, v.codec, v.frame_rate, dur / kf.max(1) as f64);
        }
    }
    println!(
        "{name} {rate}× {fps:.0} qps, maior salto {max_gap:.2} s (esperado ~{:.2}), saltos > 0,5 s: {big}, maior pausa {} ms",
        rate / 60.0,
        stalls.as_millis()
    );
    drop(p);
    std::thread::sleep(Duration::from_millis(300));
}
