//! Print da tela (o modo `screenshot` dos atalhos).
//!
//! * [`capture`]: `BitBlt` do retângulo pedido, em pixels FÍSICOS da área de
//!   trabalho virtual (o processo é per-monitor DPI aware v2, como a seleção de
//!   área). `CAPTUREBLT` entra pra pegar as janelas em camadas (menus, dicas).
//! * A seleção na hora congela antes: o atalho tira o print da área de trabalho
//!   inteira e abre a seleção; o escolhido é RECORTADO desse print ([`Shot::crop`]).
//!   Assim sai o que estava na tela quando o atalho foi apertado, sem o escuro da
//!   seleção e com os menus abertos ainda lá.
//! * [`deliver`]: grava o PNG na pasta de gravações e copia a imagem para a área de
//!   transferência, numa thread (codificar um 4K leva centenas de ms), e avisa no fim.

use std::ffi::c_void;
use std::mem::size_of;
use std::path::{Path, PathBuf};

use windows::Win32::Foundation::{GlobalFree, HANDLE, POINT};
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleDC, CreateDIBSection, DeleteDC, DeleteObject, GetDC, GetMonitorInfoW, MonitorFromPoint, ReleaseDC, SelectObject, BITMAPINFO,
    BITMAPINFOHEADER, BI_RGB, CAPTUREBLT, DIB_RGB_COLORS, HDC, HGDIOBJ, MONITORINFO, MONITOR_DEFAULTTONEAREST, SRCCOPY,
};
use windows::Win32::System::DataExchange::{CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData};
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use windows::Win32::System::Ole::CF_DIB;
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN};

use crate::engine::AreaRect;
use crate::{lang, log, mainthread};

/// Os prints na biblioteca: listados junto das gravações (miniatura, tamanho, data,
/// mover, excluir), mas sem as ações de vídeo (player, transcrever, exportar, unir).
pub const PRINT_EXTS: [&str; 1] = [".png"];

/// O arquivo é um print (pela extensão)?
pub fn is_print_file(path: &str) -> bool {
    let ext = Path::new(path).extension().map(|e| format!(".{}", e.to_string_lossy().to_lowercase())).unwrap_or_default();
    PRINT_EXTS.contains(&ext.as_str())
}

/// A miniatura de um print (`dest`, JPEG de `height` px de altura) para o card da biblioteca.
pub fn write_thumbnail(path: &str, dest: &Path, height: u32) -> bool {
    let Ok(img) = image::open(path) else { return false };
    let h = height.min(img.height()).max(1);
    let w = ((img.width() as u64 * h as u64) / img.height().max(1) as u64).max(1) as u32;
    let thumb = image::imageops::thumbnail(&img.to_rgb8(), w, h);
    thumb.save_with_format(dest, image::ImageFormat::Jpeg).is_ok()
}

/// Um print: RGBA 8 bits, linhas de cima pra baixo, e onde ele estava na tela.
pub struct Shot {
    pub area: AreaRect,
    pub rgba: Vec<u8>,
}

impl Shot {
    /// O pedaço `r` (coordenadas de tela) deste print; `None` se não houver interseção.
    pub fn crop(&self, r: AreaRect) -> Option<Shot> {
        let a = self.area;
        let (l, t) = (r.x.max(a.x), r.y.max(a.y));
        let (rt, b) = ((r.x + r.w).min(a.x + a.w), (r.y + r.h).min(a.y + a.h));
        let (w, h) = (rt - l, b - t);
        if w <= 0 || h <= 0 {
            return None;
        }
        let stride = a.w as usize * 4;
        let mut rgba = Vec::with_capacity(w as usize * h as usize * 4);
        for y in 0..h as usize {
            let start = (t - a.y) as usize * stride + y * stride + (l - a.x) as usize * 4;
            rgba.extend_from_slice(&self.rgba[start..start + w as usize * 4]);
        }
        Some(Shot { area: AreaRect { x: l, y: t, w, h }, rgba })
    }
}

/// A área de trabalho virtual (todos os monitores), em pixels físicos.
pub fn virtual_screen() -> AreaRect {
    unsafe {
        AreaRect {
            x: GetSystemMetrics(SM_XVIRTUALSCREEN),
            y: GetSystemMetrics(SM_YVIRTUALSCREEN),
            w: GetSystemMetrics(SM_CXVIRTUALSCREEN),
            h: GetSystemMetrics(SM_CYVIRTUALSCREEN),
        }
    }
}

/// O monitor onde está o mouse (o mais perto, se ele estiver entre dois).
pub fn cursor_monitor() -> Option<AreaRect> {
    unsafe {
        let mut p = POINT::default();
        GetCursorPos(&mut p).ok()?;
        let mon = MonitorFromPoint(p, MONITOR_DEFAULTTONEAREST);
        let mut mi = MONITORINFO { cbSize: size_of::<MONITORINFO>() as u32, ..Default::default() };
        if !GetMonitorInfoW(mon, &mut mi).as_bool() {
            return None;
        }
        let r = mi.rcMonitor;
        Some(AreaRect { x: r.left, y: r.top, w: r.right - r.left, h: r.bottom - r.top })
    }
}

/// Objetos GDI com liberação garantida (cada handle tem um dono).
struct ScreenDc(HDC);
impl Drop for ScreenDc {
    fn drop(&mut self) {
        unsafe { ReleaseDC(None, self.0) };
    }
}
struct MemDc(HDC);
impl Drop for MemDc {
    fn drop(&mut self) {
        unsafe {
            let _ = DeleteDC(self.0);
        }
    }
}
struct Gdi(HGDIOBJ);
impl Drop for Gdi {
    fn drop(&mut self) {
        unsafe {
            let _ = DeleteObject(self.0);
        }
    }
}

/// O que está na tela em `r` agora. `None` se o GDI falhar ou a área for vazia.
pub fn capture(r: AreaRect) -> Option<Shot> {
    if r.w <= 0 || r.h <= 0 {
        return None;
    }
    unsafe {
        let screen = GetDC(None);
        if screen.0.is_null() {
            return None;
        }
        let screen = ScreenDc(screen);
        let mem = CreateCompatibleDC(Some(screen.0));
        if mem.0.is_null() {
            return None;
        }
        let mem = MemDc(mem);
        // DIB 32 bpp top-down (altura negativa): as linhas já saem na ordem do PNG.
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: r.w,
                biHeight: -r.h,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut bits: *mut c_void = std::ptr::null_mut();
        let bmp = CreateDIBSection(Some(mem.0), &bmi, DIB_RGB_COLORS, &mut bits, None, 0).ok()?;
        let bmp = Gdi(HGDIOBJ(bmp.0));
        if bits.is_null() {
            return None;
        }
        let old = SelectObject(mem.0, bmp.0);
        let ok = BitBlt(mem.0, 0, 0, r.w, r.h, Some(screen.0), r.x, r.y, SRCCOPY | CAPTUREBLT).is_ok();
        let mut rgba = Vec::new();
        if ok {
            let px = std::slice::from_raw_parts(bits as *const u8, r.w as usize * r.h as usize * 4);
            rgba.reserve(px.len());
            for p in px.chunks_exact(4) {
                rgba.extend_from_slice(&[p[2], p[1], p[0], 255]); // BGRA → RGBA opaco
            }
        }
        SelectObject(mem.0, old); // desfaz antes de o DIB ser destruído
        ok.then_some(Shot { area: r, rgba })
    }
}

/// Grava o PNG em `dir` e copia para a área de transferência, numa thread; no fim,
/// o aviso na tela (com o nome do arquivo) ou o erro.
pub fn deliver(shot: Shot, dir: PathBuf) {
    let spawned = std::thread::Builder::new().name("noobs-print".into()).spawn(move || {
        let copied = copy_to_clipboard(&shot);
        let saved = save_png(&shot, &dir);
        let (w, h) = (shot.area.w, shot.area.h);
        mainthread::queue(move || match saved {
            Ok(path) => {
                log!("Print: {w}x{h} salvo em {} (area de transferencia: {copied}).", path.display());
                let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                let msg = if copied { lang::tr_with("{{file}} — também copiado para a área de transferência.", &[("file", &name)]) } else { name };
                crate::view::call(move |v| v.toast(&lang::tr("Print salvo"), &msg, false, 0));
                // O card na lista de gravações (com a miniatura, que chega depois).
                crate::bridge::push_print_added(&path.to_string_lossy());
            }
            Err(e) => {
                log!("Print: falhou ao salvar ({e}).");
                crate::bridge::post_error(lang::tr_with("Não foi possível salvar o print: {{error}}", &[("error", &e)]));
            }
        });
    });
    if let Err(e) = spawned {
        log!("Print: thread nao subiu ({e}).");
    }
}

/// `<pasta>/2026-11-10 09-15-02.png` (com ` (2)`, ` (3)`... se já existir).
fn file_path(dir: &Path) -> PathBuf {
    file_path_at(dir, &chrono::Local::now().format("%Y-%m-%d %H-%M-%S").to_string())
}

fn file_path_at(dir: &Path, stamp: &str) -> PathBuf {
    let mut path = dir.join(format!("{stamp}.png"));
    let mut n = 2;
    while path.exists() {
        path = dir.join(format!("{stamp} ({n}).png"));
        n += 1;
    }
    path
}

fn save_png(shot: &Shot, dir: &Path) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let path = file_path(dir);
    let img = image::RgbaImage::from_raw(shot.area.w as u32, shot.area.h as u32, shot.rgba.clone()).ok_or("tamanho da imagem")?;
    img.save_with_format(&path, image::ImageFormat::Png).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(path)
}

/// `CF_DIB` (BGRA 32 bpp, linhas de baixo pra cima, que é o que todo app lê).
fn copy_to_clipboard(shot: &Shot) -> bool {
    let (w, h) = (shot.area.w, shot.area.h);
    let header = BITMAPINFOHEADER {
        biSize: size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: w,
        biHeight: h,
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB.0,
        ..Default::default()
    };
    let head = size_of::<BITMAPINFOHEADER>();
    let stride = w as usize * 4;
    let total = head + stride * h as usize;
    unsafe {
        let Ok(mem) = GlobalAlloc(GMEM_MOVEABLE, total) else { return false };
        let p = GlobalLock(mem) as *mut u8;
        if p.is_null() {
            let _ = GlobalFree(Some(mem));
            return false;
        }
        std::ptr::copy_nonoverlapping(&header as *const BITMAPINFOHEADER as *const u8, p, head);
        let dst = std::slice::from_raw_parts_mut(p.add(head), stride * h as usize);
        for y in 0..h as usize {
            let src = &shot.rgba[y * stride..(y + 1) * stride];
            let row = &mut dst[(h as usize - 1 - y) * stride..(h as usize - y) * stride];
            for (d, s) in row.chunks_exact_mut(4).zip(src.chunks_exact(4)) {
                d.copy_from_slice(&[s[2], s[1], s[0], 255]); // RGBA → BGRA
            }
        }
        let _ = GlobalUnlock(mem);
        if OpenClipboard(None).is_err() {
            let _ = GlobalFree(Some(mem));
            return false;
        }
        let _ = EmptyClipboard();
        // Aceito, a memória passa a ser do sistema; recusado, ainda é nossa.
        let ok = SetClipboardData(CF_DIB.0 as u32, Some(HANDLE(mem.0))).is_ok();
        let _ = CloseClipboard();
        if !ok {
            let _ = GlobalFree(Some(mem));
        }
        ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shot() -> Shot {
        // 4x3 em (-2, 10): cada pixel guarda (coluna, linha) no R e no G.
        let area = AreaRect { x: -2, y: 10, w: 4, h: 3 };
        let rgba = (0..3).flat_map(|y| (0..4).flat_map(move |x| [x as u8, y as u8, 0, 255])).collect();
        Shot { area, rgba }
    }

    #[test]
    fn recorte_pega_o_pedaco_certo() {
        let c = shot().crop(AreaRect { x: -1, y: 11, w: 2, h: 2 }).unwrap();
        assert_eq!(c.area, AreaRect { x: -1, y: 11, w: 2, h: 2 });
        assert_eq!(c.rgba, vec![1, 1, 0, 255, 2, 1, 0, 255, 1, 2, 0, 255, 2, 2, 0, 255]);
    }

    #[test]
    fn recorte_fica_dentro_do_print() {
        let c = shot().crop(AreaRect { x: -10, y: 0, w: 9, h: 11 }).unwrap();
        assert_eq!(c.area, AreaRect { x: -2, y: 10, w: 1, h: 1 });
        assert!(shot().crop(AreaRect { x: 50, y: 50, w: 5, h: 5 }).is_none());
    }

    #[test]
    fn print_e_o_png() {
        assert!(is_print_file("C:\\v\\2026-11-10 09-15-02.PNG"));
        assert!(!is_print_file("C:\\v\\a.mkv"));
        assert!(!is_print_file("semextensao"));
    }

    #[test]
    fn miniatura_do_print() {
        let dir = std::env::temp_dir().join(format!("noobs_thumb_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let png = dir.join("p.png");
        image::RgbaImage::from_pixel(1920, 1080, image::Rgba([10, 200, 30, 255])).save(&png).unwrap();
        let jpg = dir.join("p.jpg");
        assert!(write_thumbnail(&png.to_string_lossy(), &jpg, 240));
        let t = image::open(&jpg).unwrap();
        assert_eq!((t.width(), t.height()), (426, 240));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn nome_nao_sobrescreve() {
        let dir = std::env::temp_dir().join(format!("noobs_print_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let a = file_path_at(&dir, "2026-11-10 09-15-02");
        assert!(a.to_string_lossy().ends_with("2026-11-10 09-15-02.png"));
        std::fs::write(&a, b"x").unwrap();
        let b = file_path_at(&dir, "2026-11-10 09-15-02");
        assert!(b.to_string_lossy().ends_with("2026-11-10 09-15-02 (2).png"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
