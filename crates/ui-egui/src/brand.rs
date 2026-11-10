//! Imagenor branding from the owner-supplied artwork in assets/brand.

use egui::{Color32, Context, Id, Rect, TextureHandle, TextureOptions, Ui, pos2};
use imagenor_codecs::{ChannelLayout, SampleType};

/// 128 px: sharp at 20 pt on a 3× display, and the mipmaps keep it clean at 1×.
const ICON_PNG: &[u8] = include_bytes!("../../../assets/app-icon/hicolor/128x128/apps/ai.storyteller.imagenor.png");

/// The icon's pixels (one transparent pixel if it couldn't be decoded, which a test rules out).
fn decode() -> egui::ColorImage {
    decode_bytes(ICON_PNG)
}

fn decode_bytes(bytes: &[u8]) -> egui::ColorImage {
    let rgba = imagenor_codecs::decode(bytes).ok().map(|img| img.convert(ChannelLayout::Rgba, SampleType::U8));
    match rgba {
        Some(img) if img.data().len() == img.width() as usize * img.height() as usize * 4 => {
            egui::ColorImage::from_rgba_unmultiplied([img.width() as usize, img.height() as usize], img.data())
        }
        _ => egui::ColorImage::filled([1, 1], Color32::TRANSPARENT),
    }
}

/// The icon texture, uploaded on first use and kept in the context.
fn texture(ctx: &Context) -> TextureHandle {
    let id = Id::new("imagenor-brand-mark");
    if let Some(tex) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return tex;
    }
    let options = TextureOptions { mipmap_mode: Some(egui::TextureFilter::Linear), ..TextureOptions::LINEAR };
    let tex = ctx.load_texture("imagenor-brand-mark", decode(), options);
    ctx.data_mut(|d| d.insert_temp(id, tex.clone()));
    tex
}

/// Where the mark was painted this frame (tests and screenshots).
fn rect_id() -> Id {
    Id::new("imagenor-brand-mark-rect")
}

/// Paint the mark into `r` (square).
pub fn paint_mark(ui: &Ui, r: Rect) {
    let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    ui.painter().image(texture(ui.ctx()).id(), r, uv, Color32::WHITE);
    ui.ctx().data_mut(|d| d.insert_temp(rect_id(), r));
}

/// The rect the mark was last painted into.
#[cfg(test)]
pub fn mark_rect(ctx: &Context) -> Option<Rect> {
    ctx.data(|d| d.get_temp(rect_id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_icon_decodes_to_a_rounded_128_px_tile() {
        let img = decode();
        assert_eq!(img.size, [128, 128]);
        // Rounded corners: the corner pixel is transparent, the centre opaque.
        assert_eq!(img.pixels[0].a(), 0);
        assert_eq!(img.pixels[64 * 128 + 64].a(), 255);
    }

    #[test]
    fn the_title_bar_paints_the_icon_from_one_cached_texture() {
        let ctx = Context::default();
        crate::ImagenorApp::setup_context(&ctx, crate::theme::ThemeKind::ALL[0]);
        let mut app = crate::ImagenorApp::new(imagenor_engine::Session::new(), crate::Services::default());
        // The system title bar hides the in-app mark (the OS draws its own icon), so paint the
        // app-drawn bar here.
        app.custom_titlebar = true;
        let mut frame = || {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| crate::panels::title_bar(&mut app, ui));
            let uploads = out.textures_delta.set.values().flat_map(|d| d.iter()).filter(|d| d.image.size() == [128, 128]).count();
            out.textures_delta.clear();
            uploads
        };
        assert_eq!(frame(), 1, "uploaded on first use");
        assert_eq!(frame(), 0, "then reused");
        assert!(mark_rect(&ctx).is_some());
    }
}

/// Cache supplied raster artwork through the existing codec path (no extra image loader).
fn artwork(ctx: &Context, name: &'static str, bytes: &[u8]) -> TextureHandle {
    let id = Id::new(name);
    if let Some(texture) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return texture;
    }
    let texture = ctx.load_texture(name, decode_bytes(bytes), TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(id, texture.clone()));
    texture
}

/// Brand logo for the start screen and About. Version and credits remain live UI.
pub fn logo(ui: &mut Ui) {
    let (name, bytes): (&str, &[u8]) = if ui.visuals().dark_mode {
        ("imagenor-logo-inverse", include_bytes!("../../../assets/brand/Logo-Inverse.png"))
    } else {
        ("imagenor-logo-primary", include_bytes!("../../../assets/brand/Logo-Primary.png"))
    };
    let texture = artwork(ui.ctx(), name, bytes);
    ui.add(egui::Image::new((texture.id(), texture.size_vec2())).max_width(300.0));
}

/// Desktop-only startup presentation; headless and embedded clients opt out by default.
/// Escape, a click, or one second dismisses it, with no filesystem or document changes.
pub fn startup(ui: &mut Ui) -> bool {
    let id = Id::new("imagenor-splash-start");
    let now = ui.input(|i| i.time);
    let start = ui.ctx().data_mut(|d| *d.get_temp_mut_or_insert_with(id, || now));
    let dismissed = ui.input(|i| i.key_pressed(egui::Key::Escape) || i.pointer.any_click());
    if dismissed || now - start >= 1.0 {
        return false;
    }
    let texture = artwork(ui.ctx(), "imagenor-splash", include_bytes!("../../../assets/brand/Splash-Screen.png"));
    ui.centered_and_justified(|ui| {
        ui.add(egui::Image::new((texture.id(), texture.size_vec2())).max_width(900.0).max_height(580.0).shrink_to_fit());
    });
    ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));
    true
}

#[cfg(test)]
mod imagenor_tests {
    use super::*;

    #[test]
    fn supplied_splash_decodes_at_its_original_size() {
        let image = imagenor_codecs::decode(include_bytes!("../../../assets/brand/Splash-Screen.png")).unwrap();
        assert_eq!((image.width(), image.height()), (900, 580));
    }

    #[test]
    fn startup_finishes_after_one_second() {
        let ctx = Context::default();
        egui_extras::install_image_loaders(&ctx);
        let mut showing = false;
        let mut output = ctx.run_ui(egui::RawInput { time: Some(0.0), ..Default::default() }, |ui| showing = startup(ui));
        output.textures_delta.clear();
        assert!(showing);
        let mut output = ctx.run_ui(egui::RawInput { time: Some(1.1), ..Default::default() }, |ui| showing = startup(ui));
        output.textures_delta.clear();
        assert!(!showing);
    }
}
