//! O idioma do app. As traduções são as do Aylis: `locales/<idioma>.json` na raiz do projeto,
//! com o texto em português como chave (`{"Configurações": "Settings"}`), lidas pelo
//! `aylis::tr` de qualquer lugar e thread (a tela, a bandeja, as notificações). Aqui só a
//! escolha do idioma (a do config ou a do Windows) e os valores por nome (`{{sec}}`).

use std::sync::Mutex;

use serde_json::{json, Value};

use crate::{config, log};

/// Os idiomas do app: o código, o nome em inglês e o nome no próprio idioma. O português é
/// o do projeto (o texto original); os outros têm `locales/<código>.json`.
const LANGUAGES: [(&str, &str, &str); 3] = [("pt-BR", "Portuguese (Brazil)", "Português (Brasil)"), ("en", "English", "English"), ("es", "Spanish", "Español")];

/// O idioma de reserva quando o do Windows não está entre os do app.
const DEFAULT_LANGUAGE: &str = "en";

static CURRENT: Mutex<String> = Mutex::new(String::new());

/// O código do idioma (`pt-BR`, `en`, `es`) mais parecido com `locale`: o exato, depois pelas
/// duas letras (`pt-PT` → `pt-BR`), por fim o inglês.
fn match_locale(locale: &str) -> &'static str {
    if let Some((code, ..)) = LANGUAGES.iter().find(|(c, ..)| c.eq_ignore_ascii_case(locale)) {
        return code;
    }
    let prefix = locale.split(['-', '_']).next().unwrap_or("").to_ascii_lowercase();
    LANGUAGES.iter().find(|(c, ..)| c.split('-').next().unwrap_or("").eq_ignore_ascii_case(&prefix)).map_or(DEFAULT_LANGUAGE, |(c, ..)| c)
}

/// Locale do Windows (`GetUserDefaultLocaleName`), ex.: `pt-BR`.
fn system_locale() -> String {
    use windows::Win32::Globalization::GetUserDefaultLocaleName;
    let mut buf = [0u16; 85]; // LOCALE_NAME_MAX_LENGTH
    let n = unsafe { GetUserDefaultLocaleName(&mut buf) };
    if n > 1 { String::from_utf16_lossy(&buf[..(n as usize - 1)]) } else { DEFAULT_LANGUAGE.to_owned() }
}

/// Escolhe o idioma (`language` do config; vazio = o do Windows) e carrega as traduções do
/// Aylis. Com a janela aberta, quem chama traduz a tela também (`aylis::Application::set_language`).
pub fn init_language() {
    let configured = config::get_config_str("language", "");
    let code = if configured.is_empty() { match_locale(&system_locale()) } else { match_locale(&configured) };
    aylis::load_translations(&crate::PROJECT, code);
    log!("Idioma: {code}");
    *CURRENT.lock().unwrap_or_else(|p| p.into_inner()) = code.to_owned();
}

pub fn current_language() -> String {
    CURRENT.lock().unwrap_or_else(|p| p.into_inner()).clone()
}

/// O texto no idioma do app (o texto em português é a chave).
pub fn tr(text: &str) -> String {
    aylis::tr(text)
}

/// [`tr`] com valores por nome: `tr_with("Gravando há {{sec}} s", &[("sec", "3")])`.
pub fn tr_with(text: &str, args: &[(&str, &str)]) -> String {
    let mut out = aylis::tr(text);
    for (name, value) in args {
        out = out.replace(&format!("{{{{{name}}}}}"), value);
    }
    out
}

/// O texto de um item escolhido em tempo de execução (a aba, o modo, o mês...): a família e
/// o item (`tr_of("settings.tabs", "general")` → "Geral", traduzido). Item desconhecido:
/// vazio (o fim de uma lista numerada).
pub fn tr_of(family: &str, item: &str) -> String {
    let text = match (family, item) {
        ("about.builtWith", "0") => "Rust — target Win64 nativo",
        ("about.builtWith", "1") => "Direct3D 11 + Windows Graphics Capture + WASAPI — captura de tela, janela, webcam e áudio",
        ("about.builtWith", "2") => "FFmpeg — encoders, MKV/MP4, remux, extração de faixas e thumbnails",
        ("about.builtWith", "3") => "Aylis — interface nativa (renderer Direct2D)",
        ("about.features", "0") => "<b>Pronto em um clique</b> — telas, webcam, microfones e áudio do sistema detectados automaticamente, sem cenas nem fontes pra configurar",
        ("about.features", "1") => "<b>Multi-monitor</b> e <b>webcam</b> gravados simultaneamente",
        ("about.features", "2") => "<b>Faixas de áudio separadas</b> — o mic e cada saída viram uma track própria no MKV",
        ("about.features", "3") => "<b>Aceleração de GPU</b>: AV1, HEVC, H.264 (NVIDIA / AMD / Intel) com fallback CPU x264",
        ("about.features", "4") => "<b>Edição rápida no player</b> — dividir, unir, waveform e volume por faixa de áudio",
        ("about.features", "5") => "<b>Gravação automática em chamadas</b> — detecta o microfone em uso por Teams, WhatsApp e afins, e para sozinho ao fim",
        ("about.features", "6") => "<b>Atalhos globais</b> — gravam, salvam o buffer e capturam área mesmo com o NoOBS na bandeja",
        ("about.features", "7") => "<b>Container MKV</b> — a gravação sobrevive a queda de energia",
        ("devices.groups", "mics") => "Microfones",
        ("devices.groups", "monitors") => "Monitores",
        ("devices.groups", "speakers") => "Alto-falantes",
        ("devices.groups", "webcams") => "Webcams",
        ("export.scaleHint", "area") => "Média dos pixels descartados: o mais rápido e o que menos serrilha em reduções grandes (4K → 720p, por exemplo). Em reduções pequenas fica levemente mais macio que o bicúbico.",
        ("export.scaleHint", "bicubic") => "Mais nitidez nos detalhes finos, e o mais lento dos três. É o padrão por ser o meio-termo seguro para qualquer conteúdo.",
        ("export.scaleHint", "bilinear") => "Cerca de 1,8× mais rápido que o bicúbico, com imagem um pouco mais suave. Boa escolha quando a pressa importa mais que o detalhe.",
        ("export.scaleOpt", "area") => "Média de área",
        ("export.scaleOpt", "bicubic") => "Bicúbico (padrão)",
        ("export.scaleOpt", "bilinear") => "Bilinear",
        ("hotkeys.mode", "area") => "Gravar uma área",
        ("hotkeys.mode", "record") => "Gravar as telas",
        ("hotkeys.mode", "replay") => "Salvar trecho do buffer",
        ("hotkeys.mode", "window") => "Gravar uma janela",
        ("hotkeys.status", "busy") => "Outro programa já usa esta combinação — escolha outra.",
        ("hotkeys.status", "duplicate") => "Combinação repetida: só a primeira da lista funciona.",
        ("hotkeys.status", "empty") => "Sem combinação: escolha uma tecla para ativar.",
        ("hotkeys.status", "inactive") => "Ativo só com o buffer ligado.",
        ("hotkeys.status", "invalid") => "Combinação inválida.",
        ("hotkeys.status", "reserved") => "Combinação reservada pelo Windows.",
        ("months", "0") => "Janeiro",
        ("months", "1") => "Fevereiro",
        ("months", "10") => "Novembro",
        ("months", "11") => "Dezembro",
        ("months", "2") => "Março",
        ("months", "3") => "Abril",
        ("months", "4") => "Maio",
        ("months", "5") => "Junho",
        ("months", "6") => "Julho",
        ("months", "7") => "Agosto",
        ("months", "8") => "Setembro",
        ("months", "9") => "Outubro",
        ("settings.audioBitrate.hint", "default") => "Padrão — transparente para quase tudo.",
        ("settings.audioBitrate.hint", "good") => "Boa para voz e som do sistema.",
        ("settings.audioBitrate.hint", "high") => "Alta — só faz diferença em música com bons fones.",
        ("settings.audioBitrate.hint", "voice") => "Suficiente para voz; música e som do sistema perdem brilho.",
        ("settings.fps.hint", "good") => "{{fps}} fps — boa fluidez, ótimo para uso geral.",
        ("settings.fps.hint", "high") => "{{fps}} fps — alta fluidez, ideal para jogos e capturas rápidas.",
        ("settings.fps.hint", "low") => "{{fps}} fps — baixa taxa, movimento pode parecer irregular. Arquivo menor.",
        ("settings.fps.hint", "smooth") => "{{fps}} fps — suave, padrão de vídeo. Arquivo compacto.",
        ("settings.fps.hint", "veryHigh") => "{{fps}} fps — altíssima fluidez. Arquivo maior.",
        ("settings.fullscreenDetect", "hint") => "Janelas por cima de uma aplicação em tela cheia derrubam o FPS dela. Desmarque se o indicador sumir ou as prévias congelarem sem nada em tela cheia aberto.",
        ("settings.fullscreenDetect", "hintPreviews") => "Atualizar as prévias da tela inicial por cima de uma aplicação em tela cheia derruba o FPS dela. Desmarque se as prévias congelarem sem nada em tela cheia aberto.",
        ("settings.fullscreenDetect", "label") => "Com uma aplicação em tela cheia, esconder os indicadores e pausar as prévias",
        ("settings.fullscreenDetect", "labelBoth") => "Com uma aplicação em tela cheia, esconder os indicadores e pausar as prévias",
        ("settings.fullscreenDetect", "labelBuffer") => "Com uma aplicação em tela cheia, esconder o indicador do buffer e pausar as prévias",
        ("settings.fullscreenDetect", "labelPreviews") => "Com uma aplicação em tela cheia, pausar as prévias dos monitores",
        ("settings.fullscreenDetect", "labelRecording") => "Com uma aplicação em tela cheia, esconder o indicador de gravação e pausar as prévias",
        ("settings.keyframe.hint", "balanced") => "A cada {{sec}} segundos — bom equilíbrio entre pontos de corte e tamanho. (Recomendado)",
        ("settings.keyframe.hint", "medium") => "A cada {{sec}} segundos — arquivo um pouco menor; cortes menos precisos.",
        ("settings.keyframe.hint", "precise") => "A cada {{sec}} segundo — cortes bem precisos ao dividir no player. Arquivo levemente maior.",
        ("settings.keyframe.hint", "spaced") => "A cada {{sec}} segundos — arquivo menor; a divisão pode desviar bastante do ponto escolhido.",
        ("settings.library", "always") => "Sempre gerar",
        ("settings.library", "auto") => "Automático (recomendado)",
        ("settings.library", "cloudNote") => "Sua pasta de gravação parece estar no OneDrive.",
        ("settings.library", "hint.always") => "Gera prévia e duração de todos os vídeos, baixando da nuvem se necessário.",
        ("settings.library", "hint.auto") => "Gera prévia e duração só dos vídeos disponíveis no computador; pula os que estão só na nuvem (OneDrive) para não baixá-los.",
        ("settings.library", "hint.off") => "Não gera prévia nem duração dos vídeos já existentes — só das novas gravações feitas no NoOBS.",
        ("settings.library", "label") => "Prévias e duração da biblioteca",
        ("settings.library", "off") => "Só das gravações novas",
        ("settings.library.hint", "always") => "Gera prévia e duração de todos os vídeos, baixando da nuvem se necessário.",
        ("settings.library.hint", "auto") => "Gera prévia e duração só dos vídeos disponíveis no computador; pula os que estão só na nuvem (OneDrive) para não baixá-los.",
        ("settings.library.hint", "off") => "Não gera prévia nem duração dos vídeos já existentes — só das novas gravações feitas no NoOBS.",
        ("settings.quality.hint", "0") => "Compactação máxima — arquivos minúsculos, qualidade bem baixa.",
        ("settings.quality.hint", "1") => "Compactação altíssima — arquivos muito pequenos, perda bem visível.",
        ("settings.quality.hint", "10") => "Qualidade máxima — arquivos enormes, além do que o olho distingue.",
        ("settings.quality.hint", "2") => "Compactação muito alta — arquivos pequenos, qualidade nitidamente reduzida.",
        ("settings.quality.hint", "3") => "Compactação alta — arquivos bem menores, qualidade visivelmente abaixo do normal.",
        ("settings.quality.hint", "4") => "Compactação moderada — arquivos menores, leve perda de nitidez.",
        ("settings.quality.hint", "5") => "Padrão — balanceado entre tamanho do arquivo e qualidade visual.",
        ("settings.quality.hint", "6") => "Qualidade boa — arquivos um pouco maiores, imagem mais limpa.",
        ("settings.quality.hint", "7") => "Qualidade alta — arquivos maiores, vídeo mais nítido.",
        ("settings.quality.hint", "8") => "Qualidade muito alta — arquivos bem maiores, perda difícil de notar.",
        ("settings.quality.hint", "9") => "Qualidade altíssima — arquivos grandes, praticamente sem perda visível.",
        ("settings.recIndicator.corners", "bottomLeft") => "Inferior esquerdo",
        ("settings.recIndicator.corners", "bottomRight") => "Inferior direito",
        ("settings.recIndicator.corners", "topLeft") => "Superior esquerdo",
        ("settings.recIndicator.corners", "topRight") => "Superior direito",
        ("settings.tabs", "about") => "Sobre",
        ("settings.tabs", "audio") => "Áudio",
        ("settings.tabs", "devices") => "Dispositivos",
        ("settings.tabs", "files") => "Arquivos",
        ("settings.tabs", "general") => "Geral",
        ("settings.tabs", "hotkeys") => "Atalhos",
        ("settings.tabs", "onrecord") => "Gravação",
        ("settings.tabs", "recording") => "Vídeo",
        ("settings.tabs", "replay") => "Buffer",
        ("settings.tabs", "transcribe") => "Transcrição",
        ("settings.tabs", "updates") => "Atualizações",
        ("settings.transcribe.local", "cancel") => "Cancelar",
        ("settings.transcribe.local", "error") => "A instalação falhou: {{error}}",
        ("settings.transcribe.local", "hint") => "Roda nesta máquina, na placa de vídeo, sem internet. Não separa quem falou: gravando com faixas isoladas, o falante vem do nome do dispositivo de cada faixa.",
        ("settings.transcribe.local", "install") => "Baixar e instalar",
        ("settings.transcribe.local", "installing") => "Instalando: {{stage}}",
        ("settings.transcribe.local", "label") => "Motor de transcrição",
        ("settings.transcribe.local", "missing") => "O motor local ainda não está instalado.",
        ("settings.transcribe.local", "missingHow") => "O download tem cerca de {{size}} (o motor audio.cpp e os modelos Qwen3) e é feito uma vez só. Precisa de uma placa de vídeo com driver Vulkan — AMD, NVIDIA ou Intel. Sem Docker e sem instalar mais nada.",
        ("settings.transcribe.local", "progress") => "{{done}} de {{total}}",
        ("settings.transcribe.local", "ready") => "Pronto — transcrevendo na {{device}}.",
        ("settings.transcribe.local", "readyCpu") => "Pronto — transcrevendo no processador, bem mais devagar que na placa de vídeo: a placa não tem Vulkan ou não tem memória suficiente para os modelos.",
        ("settings.transcribe.local", "readyHow") => "Ocupa {{size}} em %LOCALAPPDATA%\\NoOBS\\asr. O motor só roda enquanto há transcrição na fila e libera a memória de vídeo alguns minutos depois.",
        ("settings.transcribe.local", "remove") => "Remover",
        ("settings.transcribe.local", "removeBusy") => "Não dá para remover o motor local durante uma instalação ou transcrição.",
        ("settings.transcribe.local", "removeConfirm") => "Confirmar remoção",
        ("settings.transcribe.local", "resumeHint") => "Se a conexão cair ou você cancelar, o que já foi baixado fica guardado e o download continua de onde parou.",
        ("settings.transcribe.local", "retry") => "Tentar de novo",
        ("settings.transcribe.local", "stages.align") => "baixando o modelo de alinhamento",
        ("settings.transcribe.local", "stages.alignMirror") => "baixando o modelo de alinhamento (Google Drive)",
        ("settings.transcribe.local", "stages.asr") => "baixando o modelo de transcrição",
        ("settings.transcribe.local", "stages.asrMirror") => "baixando o modelo de transcrição (Google Drive)",
        ("settings.transcribe.local", "stages.detecting") => "procurando a placa de vídeo",
        ("settings.transcribe.local", "stages.engine") => "baixando o motor",
        ("settings.transcribe.local", "stages.engineMirror") => "baixando o motor (Google Drive)",
        ("settings.transcribe.local", "stages.extracting") => "descompactando o motor",
        ("settings.transcribe.local", "stages.verifying") => "conferindo o arquivo baixado",
        ("settings.transcribe.stage", "aligning") => "alinhando palavras",
        ("settings.transcribe.stage", "decoding") => "decodificando",
        ("settings.transcribe.stage", "done") => "concluído",
        ("settings.transcribe.stage", "extracting") => "extraindo o áudio",
        ("settings.transcribe.stage", "paused") => "automáticas em pausa durante a captura",
        ("settings.transcribe.stage", "starting") => "iniciando o motor local",
        ("settings.transcribe.stage", "transcribing") => "transcrevendo",
        ("settings.transcribe.stage", "waitingInstall") => "aguardando a instalação do motor local",
        _ => return String::new(),
    };
    tr(text)
}

/// [`tr_of`] com valores por nome.
pub fn tr_of_with(family: &str, item: &str, args: &[(&str, &str)]) -> String {
    let mut out = tr_of(family, item);
    for (name, value) in args {
        out = out.replace(&format!("{{{{{name}}}}}"), value);
    }
    out
}

/// `[{code, name, nativeName}]` para a escolha do idioma.
pub fn available_languages() -> Value {
    Value::Array(LANGUAGES.iter().map(|(code, name, native)| json!({ "code": code, "name": name, "nativeName": native })).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_por_prefixo() {
        assert_eq!(match_locale("pt-PT"), "pt-BR");
        assert_eq!(match_locale("pt-BR"), "pt-BR");
        assert_eq!(match_locale("es-MX"), "es");
        assert_eq!(match_locale("ja-JP"), "en");
    }

    #[test]
    fn item_desconhecido_e_vazio() {
        assert_eq!(tr_of("settings.tabs", "nao-existe"), "");
        assert_eq!(tr_of("about.features", "999"), "");
    }
}
