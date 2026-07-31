//! CJK font support for the egui-based dashboard/launcher.
//!
//! egui's default fonts have no CJK glyphs, so Chinese (and other CJK) strings
//! would render as empty boxes. We load a system CJK font at runtime and add it
//! as a fallback for the Proportional and Monospace families, keeping the small
//! Latin/emoji defaults for everything else. This avoids bundling a large font
//! file in the repository.
//!
//! Not available on wasm, where there are no system font files to read.

use std::path::Path;

// Candidate system fonts, checked in order. First readable file wins.
const CJK_FONT_CANDIDATES: &[&str] = &[
    // Windows: Microsoft YaHei (preferred), SimSun, SimHei
    "C:\\Windows\\Fonts\\msyh.ttc",
    "C:\\Windows\\Fonts\\msyh.ttf",
    "C:\\Windows\\Fonts\\simsun.ttc",
    "C:\\Windows\\Fonts\\simhei.ttf",
    // Linux: Noto Sans CJK, WenQuanYi
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/opentype/noto/NotoSansCJKsc-Regular.otf",
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.otf",
    "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
    "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
];

/// Add a system CJK font to the given egui context so localized strings render
/// correctly. No-op when no candidate font is found.
#[cfg(not(target_arch = "wasm32"))]
pub fn setup_fonts(ctx: &egui::Context) {
    for candidate in CJK_FONT_CANDIDATES {
        if Path::new(candidate).is_file() {
            match std::fs::read(candidate) {
                Ok(bytes) => {
                    let mut fonts = egui::FontDefinitions::default();
                    fonts
                        .font_data
                        .insert("cjk".to_owned(), egui::FontData::from_owned(bytes).into());
                    for family in [
                        egui::FontFamily::Proportional,
                        egui::FontFamily::Monospace,
                    ] {
                        fonts
                            .families
                            .entry(family)
                            .or_default()
                            .push("cjk".to_owned());
                    }
                    ctx.set_fonts(fonts);
                    return;
                }
                Err(err) => {
                    eprintln!("Failed to read CJK font {candidate}: {err}");
                }
            }
        }
    }
}
