//! O editor do print de verdade (o form do Aylis sem janela): o mouse e o teclado entram
//! pelo `Ui`, como na tela, e a imagem final sai do desenho (`render_rgba`).

use super::*;
use aylis::headless::Headless;

static DONE: std::sync::Mutex<Vec<Option<ShotDone>>> = std::sync::Mutex::new(Vec::new());

fn on_done(d: Option<ShotDone>) {
    DONE.lock().unwrap().push(d);
}

fn take_done() -> Vec<Option<ShotDone>> {
    crate::mainthread::drain();
    std::mem::take(&mut *DONE.lock().unwrap())
}

/// O cinza do print de teste.
const GRAY: u8 = 128;
const W: i32 = 1200;
const H: i32 = 900;

/// O editor aberto num print cinza de 1200×900, com a área de trabalho virtual começando em
/// (-1920, 0) (um monitor à esquerda do principal).
struct Ed {
    h: Headless,
    form: ComponentId,
}

impl Ed {
    fn open() -> Ed {
        Ed::open_at(None)
    }

    /// O editor aberto com a área `preset` (coordenadas de tela) já escolhida.
    fn open_at(preset: Option<AreaRect>) -> Ed {
        take_done();
        with(|s| *s = State::default());
        let mut app = aylis::Application::bundled(crate::PROJECT).unwrap();
        crate::generated::register(&mut app.ui);
        let mut h = app.headless();
        with(|s| {
            s.open = true;
            s.on_shot = Some(on_done);
        });
        let rgba: Vec<u8> = (0..W * H).flat_map(|_| [GRAY, GRAY, GRAY, 255]).collect();
        let form = open(&mut h.ui, rgba, W, H, (-1920, 0), 1.0, preset).expect("o editor abre");
        h.set_form(form);
        h.settle();
        Ed { h, form }
    }

    fn ev(x: f32, y: f32, button: PointerButton) -> PointerEvent {
        PointerEvent { position: PointF::new(x, y), button, click_count: 1, ..Default::default() }
    }

    fn down(&mut self, x: f32, y: f32) {
        self.h.ui.pointer_move(self.form, Self::ev(x, y, PointerButton::Primary));
        self.h.ui.pointer_down(self.form, Self::ev(x, y, PointerButton::Primary));
        self.h.settle();
    }

    fn mv(&mut self, x: f32, y: f32) {
        self.h.ui.pointer_move(self.form, Self::ev(x, y, PointerButton::Primary));
        self.h.settle();
    }

    fn up(&mut self, x: f32, y: f32) {
        self.h.ui.pointer_up(self.form, Self::ev(x, y, PointerButton::Primary));
        self.h.settle();
    }

    fn drag(&mut self, from: (f32, f32), to: (f32, f32)) {
        self.down(from.0, from.1);
        self.mv((from.0 + to.0) / 2.0, (from.1 + to.1) / 2.0);
        self.mv(to.0, to.1);
        self.up(to.0, to.1);
    }

    fn click(&mut self, x: f32, y: f32) {
        self.down(x, y);
        self.up(x, y);
    }

    fn wheel(&mut self, notches: i32) {
        let at = with(|s| s.sel.map_or(PointF::new(10.0, 10.0), |r| r.center()));
        self.h.ui.wheel(self.form, WheelEvent { position: at, delta: PointF::new(0.0, notches as f32 * WHEEL_LINE), lines: true, ..Default::default() });
        self.h.settle();
    }

    fn key(&mut self, key: Key) {
        self.h.press(key);
    }

    fn ctrl(&mut self, c: char) {
        self.h.press_with(KeyEvent { key: Key::Char(c), modifiers: Modifiers { ctrl: true, ..Default::default() }, repeat: false });
    }

    /// Clica no botão `b` da barra (as cores e a faixa de matizes: abre o seletor antes).
    fn button(&mut self, b: ShotButton) {
        if matches!(b, ShotButton::Color(_) | ShotButton::Hue) && !with(|s| s.picker) {
            self.button(ShotButton::ColorMenu);
        }
        let c = with(|s| s.bar.as_ref().and_then(|bar| bar.button(b))).expect("o botão na barra").center();
        self.click(c.x, c.y);
    }

    fn marks(&self) -> Vec<an::Mark> {
        with(|s| s.items.iter().filter_map(|i| if let Item::Mark(m) = i { Some(m.clone()) } else { None }).collect())
    }

    fn texts(&self) -> Vec<TextMark> {
        with(|s| s.items.iter().filter_map(|i| if let Item::Text(t) = i { Some(*t) } else { None }).collect())
    }
}

fn arrow(x1: f32, y1: f32, x2: f32, y2: f32, width: f32) -> an::Mark {
    an::Mark::Arrow(an::Arrow { x1, y1, x2, y2, width, color: an::COLOR })
}

/// O pixel (x, y) da imagem RGBA `w` de largura.
fn px(d: &ShotDone, x: i32, y: i32) -> (u8, u8, u8) {
    let i = ((y * d.area.w + x) * 4) as usize;
    (d.rgba[i], d.rgba[i + 1], d.rgba[i + 2])
}

/// Soltar a seleção não termina: a área fica com a barra. Arrastar dentro dela faz uma seta
/// (do ponto inicial ao final), com a espessura da roda; Salvar devolve a área em
/// coordenadas de tela e a imagem dela: o print nítido (sem o escurecido) com a seta.
#[test]
fn seleciona_desenha_seta_e_salva() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open();
    e.drag((100.0, 100.0), (500.0, 400.0));
    assert!(with(|s| s.editing), "a área ficou, esperando");
    assert_eq!(with(|s| s.sel), Some(RectF::new(100.0, 100.0, 400.0, 300.0)));
    assert!(take_done().is_empty(), "nada entregue ainda");
    assert!(e.h.ui.control(with(|s| s.bar_id).unwrap()).is_some_and(|c| c.visible()), "a barra aparece");

    // Seta: roda +3 durante o arrasto (a seta em andamento acompanha).
    e.down(150.0, 150.0);
    e.mv(300.0, 220.0);
    e.wheel(3);
    assert_eq!(with(|s| s.drawing.map(|a| a.width)), Some(an::WIDTH_DEFAULT + 3.0));
    e.mv(320.0, 230.0);
    e.up(320.0, 230.0);
    // Um clique sem arrasto não vira seta.
    e.drag((200.0, 300.0), (201.0, 301.0));
    assert_eq!(e.marks(), vec![arrow(150.0, 150.0, 320.0, 230.0, an::WIDTH_DEFAULT + 3.0)]);
    assert_eq!(with(|s| s.sel), Some(RectF::new(100.0, 100.0, 400.0, 300.0)), "desenhar não mexe na área");
    // A roda não passa dos limites.
    e.wheel(-100);
    assert_eq!(with(|s| s.width), an::WIDTH_MIN);
    e.wheel(100);
    assert_eq!(with(|s| s.width), an::WIDTH_MAX);

    e.button(ShotButton::Save);
    let done = take_done();
    assert_eq!(done.len(), 1);
    let d = done[0].clone().expect("salvo");
    assert_eq!(d.action, ShotAction::Save);
    assert_eq!(d.area, AreaRect { x: 100 - 1920, y: 100, w: 400, h: 300 });
    assert_eq!(d.rgba.len(), 400 * 300 * 4);
    // O meio do corpo da seta (235, 190) é vermelho; longe dela, o print como estava (nem
    // escurecido, nem com a borda ou as alças).
    let (r, g, b) = px(&d, 235 - 100, 190 - 100);
    assert!(r > 180 && g < 100 && b < 100, "a seta: {:?}", (r, g, b));
    assert_eq!(px(&d, 300, 250), (GRAY, GRAY, GRAY), "o print, nítido");
    assert_eq!(px(&d, 0, 0), (GRAY, GRAY, GRAY), "sem a borda nem as alças");
    assert!(!is_active() && with(|s| s.form.is_none() && s.on_shot.is_none()), "fechou");
}

/// Arrastar fora da área escolhe outra (as marcas ficam); só um clique fora devolve a de
/// antes. O fundo da barra não começa outra seleção. Copiar pelo botão, Enter salva, Esc
/// cancela.
#[test]
fn refaz_a_area_copia_e_cancela() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open();
    e.drag((100.0, 100.0), (300.0, 300.0));
    e.drag((150.0, 150.0), (250.0, 200.0)); // uma seta
    e.drag((600.0, 500.0), (601.0, 500.0)); // clique fora: a área de antes volta
    assert!(with(|s| s.editing));
    assert_eq!(with(|s| s.sel), Some(RectF::new(100.0, 100.0, 200.0, 200.0)));
    e.drag((50.0, 60.0), (700.0, 600.0)); // outra área, maior
    assert_eq!(with(|s| s.sel), Some(RectF::new(50.0, 60.0, 650.0, 540.0)));
    assert_eq!(e.marks().len(), 1, "a seta continua");

    let bar = with(|s| s.bar.as_ref().map(|b| b.rect)).expect("barra");
    e.click(bar.x + 1.0, bar.y + 1.0);
    assert_eq!(with(|s| s.sel), Some(RectF::new(50.0, 60.0, 650.0, 540.0)), "o fundo da barra não mexe na área");

    e.button(ShotButton::Copy);
    let d = take_done().pop().flatten().expect("copiado");
    assert_eq!(d.action, ShotAction::Copy);
    assert_eq!(d.area, AreaRect { x: 50 - 1920, y: 60, w: 650, h: 540 });

    // Enter salva; antes de escolher a área, Enter não faz nada.
    let mut e = Ed::open();
    e.key(Key::Enter);
    assert!(take_done().is_empty());
    e.drag((10.0, 10.0), (210.0, 110.0));
    e.key(Key::Enter);
    assert_eq!(take_done().pop().flatten().map(|d| d.action), Some(ShotAction::Save));

    // Esc: cancelado (None); o botão direito também.
    let mut e = Ed::open();
    e.drag((10.0, 10.0), (210.0, 110.0));
    e.key(Key::Escape);
    let done = take_done();
    assert!(done.len() == 1 && done[0].is_none(), "cancelado: {done:?}");
    let mut e = Ed::open();
    e.h.ui.pointer_down(e.form, Ed::ev(400.0, 400.0, PointerButton::Secondary));
    e.h.ui.pointer_up(e.form, Ed::ev(400.0, 400.0, PointerButton::Secondary));
    e.h.settle();
    let done = take_done();
    assert!(done.len() == 1 && done[0].is_none(), "o botão direito cancela: {done:?}");
}

/// A cor: a barra mostra só a atual; clicar nela abre o seletor (as cores prontas e a faixa
/// de matizes). Escolher fecha; Esc e clicar fora também fecham, sem cancelar o print nem
/// desenhar; a faixa dá a cor do ponto (o arrasto acompanha).
#[test]
fn seletor_de_cores() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open();
    e.drag((100.0, 100.0), (700.0, 500.0));
    let colors = || with(|s| s.bar.as_ref().map_or(0, |b| b.buttons.iter().filter(|(k, _)| matches!(k, ShotButton::Color(_))).count()));
    assert_eq!(colors(), 0, "fechado: só a cor atual");
    e.button(ShotButton::ColorMenu);
    assert_eq!((with(|s| s.picker), colors()), (true, an::PALETTE.len()));
    e.button(ShotButton::Color(7));
    assert_eq!(with(|s| (s.color, s.picker)), (an::PALETTE[7], false));

    // Esc com o seletor aberto: só fecha ele.
    e.button(ShotButton::ColorMenu);
    e.key(Key::Escape);
    assert!(with(|s| !s.picker && s.open), "o print continua");

    // Arrastar na área com o seletor aberto: só fecha (nada desenhado, a área fica).
    e.button(ShotButton::ColorMenu);
    e.drag((200.0, 200.0), (400.0, 300.0));
    assert!(with(|s| !s.picker) && e.marks().is_empty());
    assert_eq!(with(|s| s.sel), Some(RectF::new(100.0, 100.0, 600.0, 400.0)));

    // A faixa de matizes: o começo é vermelho; arrastado até o meio, ciano.
    e.button(ShotButton::ColorMenu);
    let hue = with(|s| s.bar.as_ref().and_then(|b| b.button(ShotButton::Hue))).expect("faixa");
    let (y, mid) = (hue.center().y, hue.x + (hue.width - 1.0) / 2.0);
    e.down(hue.x, y);
    assert_eq!(with(|s| s.color), an::hsv(0.0, 0.85, 0.95));
    e.mv(mid, y);
    e.up(mid, y);
    let c = with(|s| s.color);
    assert!((c >> 16) & 0xFF < 0x40 && (c & 0xFF) > 0xC0 && (c >> 8) & 0xFF > 0xC0, "ciano: {c:08X}");
    assert!(!with(|s| s.picker), "soltar fecha o seletor");
    e.key(Key::Escape);
    take_done();
}

/// O − e o + mudam o tamanho como a roda (a espessura; com o Texto, a fonte), e a barra não
/// muda de largura com o número.
#[test]
fn menos_e_mais() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open();
    e.drag((100.0, 100.0), (700.0, 500.0));
    let bar = || with(|s| s.bar.as_ref().map(|b| b.rect)).expect("barra");
    let before = bar();
    for _ in 0..3 {
        e.button(ShotButton::Bigger);
    }
    e.button(ShotButton::Smaller);
    assert_eq!(with(|s| s.width), an::WIDTH_DEFAULT + 2.0);
    e.button(ShotButton::Tool(Tool::Text));
    e.button(ShotButton::Bigger);
    assert_eq!(with(|s| s.font_size), an::SIZE_DEFAULT + an::SIZE_STEP);
    assert_eq!(bar(), before, "a barra fica do mesmo tamanho");
    e.key(Key::Escape);
    take_done();
}

/// O lápis guarda o caminho do mouse; o marca-texto também, largo e translúcido; a linha vai
/// de ponta a ponta sem ponta de seta; o Desfazer tira a última marca.
#[test]
fn lapis_marca_texto_linha_e_desfazer() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open();
    e.drag((100.0, 100.0), (700.0, 500.0));
    e.button(ShotButton::Tool(Tool::Pen));
    e.down(200.0, 200.0);
    for (x, y) in [(210.0, 205.0), (220.0, 215.0), (240.0, 230.0)] {
        e.mv(x, y);
    }
    e.up(240.0, 230.0);
    let pen = |m: &an::Mark| matches!(m, an::Mark::Stroke(k) if !k.marker && k.points == vec![(200.0, 200.0), (210.0, 205.0), (220.0, 215.0), (240.0, 230.0)]);
    assert!(e.marks().last().is_some_and(pen), "{:?}", e.marks());

    e.button(ShotButton::Tool(Tool::Marker));
    e.button(ShotButton::Color(1)); // amarelo
    e.drag((300.0, 300.0), (400.0, 300.0));
    let Some(an::Mark::Stroke(k)) = e.marks().last().cloned() else { panic!("marca-texto: {:?}", e.marks()) };
    assert!(k.marker && k.color == an::PALETTE[1]);

    e.button(ShotButton::Tool(Tool::Line));
    e.drag((150.0, 450.0), (650.0, 450.0));
    assert!(matches!(e.marks().last(), Some(an::Mark::Line(a)) if (a.x1, a.x2) == (150.0, 650.0)));

    e.button(ShotButton::Undo);
    assert_eq!(e.marks().len(), 2, "a linha saiu");
    e.ctrl('z');
    assert_eq!(e.marks().len(), 1, "o Ctrl+Z tira o marca-texto");
    e.button(ShotButton::Save);
    let d = take_done().pop().flatten().expect("salvo");
    // O lápis no meio do caminho (220, 215) sai na cor dele (o vermelho).
    let (r, g, b) = px(&d, 220 - 100, 215 - 100);
    assert!(r > 180 && g < 100 && b < 100, "o lápis: {:?}", (r, g, b));
}

/// O marca-texto começa onde o clique foi e termina no mouse: a pintura não passa para trás
/// do clique nem para a frente do fim (a ponta quadrada passava meia largura dos dois lados).
#[test]
fn marca_texto_comeca_no_clique() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open();
    e.drag((100.0, 100.0), (700.0, 500.0));
    e.button(ShotButton::Tool(Tool::Marker));
    e.button(ShotButton::Color(1)); // amarelo
    // Um clique sem arrasto: o marca-texto não tem lado para pintar, não vira marca.
    e.down(500.0, 400.0);
    e.up(500.0, 400.0);
    assert!(e.marks().is_empty(), "{:?}", e.marks());
    e.drag((300.0, 300.0), (400.0, 300.0));
    // Um arrasto curto (antes de o traço escolher a direção): só do clique para o mouse.
    e.down(300.0, 450.0);
    e.mv(304.0, 450.0);
    e.up(304.0, 450.0);
    assert_eq!(e.marks().len(), 2);
    e.button(ShotButton::Save);
    let d = take_done().pop().flatten().expect("salvo");
    let pintado = |x: i32, y: i32| px(&d, x - 100, y - 100) != (GRAY, GRAY, GRAY);
    assert!(pintado(303, 300) && pintado(397, 300), "o traço entre o clique e o fim");
    assert!(!pintado(296, 300), "nada antes do clique");
    assert!(!pintado(404, 300), "nada depois do fim");
    assert!(pintado(302, 450) && !pintado(297, 450), "o curto cresce para o lado do mouse");
    assert!(!pintado(500, 400), "o clique solto não pintou");
}

/// Sem ferramenta (clicar de novo na ativa), arrastar dentro move a área e as marcas ficam
/// onde estavam na tela; a alça do canto puxa os dois lados, com qualquer ferramenta.
#[test]
fn move_e_redimensiona_a_area() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open();
    e.drag((100.0, 100.0), (500.0, 400.0));
    e.drag((150.0, 150.0), (300.0, 200.0)); // uma seta
    e.button(ShotButton::Tool(Tool::Arrow)); // a ativa: solta
    assert_eq!(with(|s| s.tool), None);
    e.drag((300.0, 300.0), (350.0, 250.0));
    assert_eq!(with(|s| s.sel), Some(RectF::new(150.0, 50.0, 400.0, 300.0)));
    assert_eq!(e.marks(), vec![arrow(150.0, 150.0, 300.0, 200.0, an::WIDTH_DEFAULT)], "a seta fica no lugar");
    assert!(with(|s| s.editing), "a área continua escolhida");

    // A alça do canto de baixo à direita, com a seta escolhida de novo.
    e.button(ShotButton::Tool(Tool::Arrow));
    e.drag((550.0, 350.0), (600.0, 420.0));
    assert_eq!(with(|s| s.sel), Some(RectF::new(150.0, 50.0, 450.0, 370.0)));
    assert_eq!(e.marks().len(), 1, "puxar a alça não desenha");

    e.button(ShotButton::Save);
    let d = take_done().pop().flatten().expect("salvo");
    assert_eq!(d.area, AreaRect { x: 150 - 1920, y: 50, w: 450, h: 370 });
}

/// Retângulo: arrastar dentro da área demarca de canto a canto, na cor e na espessura
/// escolhidas; um risco sem altura não vira retângulo.
#[test]
fn retangulo_demarca_area() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open();
    e.drag((100.0, 100.0), (700.0, 500.0));
    e.button(ShotButton::Tool(Tool::Rect));
    e.button(ShotButton::Color(2));
    // Arrastado de baixo-direita para cima-esquerda, com a roda +2 no meio.
    e.down(400.0, 350.0);
    e.mv(300.0, 250.0);
    e.wheel(2);
    e.mv(200.0, 150.0);
    e.up(200.0, 150.0);
    e.drag((150.0, 300.0), (450.0, 301.0)); // sem altura: nada
    let w = an::WIDTH_DEFAULT + 2.0;
    assert_eq!(e.marks(), vec![an::Mark::Rect(an::Frame { x1: 400.0, y1: 350.0, x2: 200.0, y2: 150.0, width: w, color: an::PALETTE[2] })]);
    e.button(ShotButton::Save);
    let d = take_done().pop().flatten().expect("salvo");
    let green = |p: (u8, u8, u8)| p.1 > 120 && p.0 < 120 && p.2 < 120;
    assert!(green(px(&d, 200 - 100, 250 - 100)), "a borda esquerda: {:?}", px(&d, 100, 150));
    assert_eq!(px(&d, 300 - 100, 250 - 100), (GRAY, GRAY, GRAY), "o miolo fica como estava");
}

/// O texto é um Memo do Aylis: nasce no clique (a linha centrada nele), normal e com 20 px,
/// recebe o que se digita, a cor da barra e a roda (o tamanho); Ctrl+B, Ctrl+I e Ctrl+S
/// trocam o estilo; Enter termina (sem salvar); um texto deixado vazio some; e o texto sai
/// na imagem final.
#[test]
fn texto_no_memo() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open();
    e.drag((100.0, 100.0), (700.0, 500.0));
    e.button(ShotButton::Tool(Tool::Text));
    e.click(200.0, 200.0);
    let t = e.texts().pop().expect("o texto nasceu");
    assert_eq!(with(|s| s.typing), Some(t.memo), "digitando no texto novo");
    assert_eq!((t.size, t.style), (an::SIZE_DEFAULT, 0), "20 px, sem negrito");
    assert_eq!(e.h.ui.focused(e.form), Some(t.memo), "o Memo com o foco");
    e.h.type_text("Olá");
    assert_eq!(memo_text(&e.h.ui, t.memo), "Olá");
    e.button(ShotButton::Color(3)); // azul, no texto sendo digitado
    e.wheel(4); // +8 px
    e.ctrl('b');
    let t2 = e.texts()[0];
    assert_eq!((t2.color, t2.size, t2.style), (an::PALETTE[3], an::SIZE_DEFAULT + 8.0, an::STYLE_BOLD));
    assert_eq!(e.h.ui.get_prop(t.memo, "font_weight").ok(), Some(PropValue::Enum("bold".into())));
    assert_eq!(with(|s| s.typing), Some(t.memo), "a cor, a roda e o estilo não terminam o texto");
    e.key(Key::Enter);
    assert_eq!(with(|s| s.typing), None, "Enter termina o texto");
    assert!(take_done().is_empty(), "Enter digitando não salva");
    assert_eq!(memo_text(&e.h.ui, t.memo), "Olá", "o Enter não virou linha nova");

    // Um texto começado e deixado vazio some.
    e.click(500.0, 450.0);
    assert_eq!(e.texts().len(), 2);
    e.button(ShotButton::Tool(Tool::Arrow));
    assert_eq!(e.texts().len(), 1, "o texto vazio saiu");

    e.button(ShotButton::Save);
    let d = take_done().pop().flatten().expect("salvo");
    // Algum pixel azul em volta do texto (o canto dele em 200, 200 − meia linha).
    let blue = (90..130).flat_map(|y| (95..200).map(move |x| (x, y))).filter(|&(x, y)| {
        let (r, g, b) = px(&d, x, y);
        b > 160 && r < 100 && g < 160
    });
    assert!(blue.count() > 20, "o texto azul na imagem final");
}

/// A ferramenta, a cor e os tamanhos ficam para o próximo print (e no config); as marcas, não.
#[test]
fn lembra_ferramenta_e_cor() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open();
    assert_eq!(with(|s| (s.tool, s.color, s.width, s.font_size)), (Some(Tool::Arrow), an::COLOR, 4.0, an::SIZE_DEFAULT), "os padrões da instalação nova");
    e.drag((100.0, 100.0), (500.0, 400.0));
    e.button(ShotButton::Tool(Tool::Text));
    e.button(ShotButton::Color(1));
    e.wheel(-2);
    e.key(Key::Escape);
    take_done();
    // A memória zerada (fechar o NoOBS) lê de volta do config.
    with(|s| *s = State::default());
    with(reset);
    assert_eq!(with(|s| (s.tool, s.color, s.font_size, s.width, s.items.len())), (Some(Tool::Text), an::PALETTE[1], an::SIZE_DEFAULT - 4.0, an::WIDTH_DEFAULT, 0));
}

/// Mover: a área inteira anda e não sai do form. Alça: os lados puxados andam, sem passar do
/// oposto (fica com o mínimo) nem sair do form.
#[test]
fn mover_e_puxar_a_area() {
    let r = RectF::new(100.0, 100.0, 300.0, 200.0);
    assert_eq!(adjusted(r, Grab::Move, 50.0, -20.0, 1000.0, 800.0), RectF::new(150.0, 80.0, 300.0, 200.0));
    assert_eq!(adjusted(r, Grab::Move, 900.0, -500.0, 1000.0, 800.0), RectF::new(700.0, 0.0, 300.0, 200.0));
    let corner = Grab::Edges { left: false, top: false, right: true, bottom: true };
    assert_eq!(adjusted(r, corner, 30.0, 40.0, 1000.0, 800.0), RectF::new(100.0, 100.0, 330.0, 240.0));
    let left = Grab::Edges { left: true, top: false, right: false, bottom: false };
    assert_eq!(adjusted(r, left, 1000.0, 0.0, 1000.0, 800.0).x, 400.0 - MIN_SIZE, "não passa do lado oposto");
    assert_eq!(adjusted(r, left, -500.0, 0.0, 1000.0, 800.0).x, 0.0, "nem sai do form");
    assert_eq!(handles(r).count(), 8);
    assert_eq!(handle_at(r, PointF::new(402.0, 298.0), 1.0), Some(corner));
    assert_eq!(handle_at(r, PointF::new(250.0, 200.0), 1.0), None, "o meio da área não é alça");
}

/// "Abrir com a última área": o editor já nasce com ela escolhida (a barra e as alças à
/// vista), e salvar direto entrega a mesma área; uma área que não cabe mais na tela é
/// ignorada (abre como sempre).
#[test]
fn abre_com_a_ultima_area() {
    let _t = crate::testkit::begin();
    let mut e = Ed::open_at(Some(AreaRect { x: -1920 + 200, y: 150, w: 320, h: 240 }));
    assert!(with(|s| s.editing));
    assert_eq!(with(|s| s.sel), Some(RectF::new(200.0, 150.0, 320.0, 240.0)));
    assert!(with(|s| s.bar.is_some()), "a barra aparece");
    e.key(Key::Enter);
    let d = take_done().pop().flatten().expect("salvo");
    assert_eq!(d.area, AreaRect { x: -1920 + 200, y: 150, w: 320, h: 240 });

    let _e = Ed::open_at(Some(AreaRect { x: -1920 + 1100, y: 150, w: 320, h: 240 }));
    assert!(with(|s| !s.editing && s.sel.is_none()), "fora do form: a seleção começa vazia");
}
