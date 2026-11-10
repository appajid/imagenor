//! Alpha / spot channels, channel options and the Quick Mask through PSD.

use imagenor_color::{Color, ColorMode, PixelFormat, SampleType};
use imagenor_doc::{AlphaChannel, ColorIndicates, Document};
use imagenor_geom::{Rect, Size};
use imagenor_io::{document_to_psd, psd_to_document};
use imagenor_raster::Surface;

fn channel(depth: SampleType, x1: i32) -> Surface {
    let mut s = Surface::new(PixelFormat::new(ColorMode::Grayscale, depth, false));
    s.fill_rect(Rect::new(0, 0, x1, 8), &[1.0]);
    s
}

#[test]
fn channel_options_spot_and_quick_mask_roundtrip() {
    for depth in SampleType::ALL {
        let mut d = Document::with_background("c", Size::new(16, 8), ColorMode::Rgb, depth, Color::WHITE);
        let mut a = AlphaChannel::new("Alpha 1", channel(depth, 4));
        a.color = Color::rgb(0.0, 0.0, 1.0);
        a.opacity = 0.7;
        a.indicates = ColorIndicates::SelectedAreas;
        d.channels.push(a);
        d.channels.push(AlphaChannel { spot: Some((Color::rgb(1.0, 0.0, 1.0), 0.25)), ..AlphaChannel::new("PANTONE X", channel(depth, 8)) });
        d.quick_mask = Some(AlphaChannel::new("Quick Mask", channel(depth, 12)));
        let (back, warnings) = psd_to_document(&document_to_psd(&d));
        assert!(warnings.is_empty(), "{warnings:?}");
        assert_eq!(back.channels.len(), 2, "{depth:?}");
        let (a, s) = (&back.channels[0], &back.channels[1]);
        assert_eq!((a.name.as_str(), a.color, a.indicates), ("Alpha 1", Color::rgb(0.0, 0.0, 1.0), ColorIndicates::SelectedAreas));
        assert!((a.opacity - 0.7).abs() < 1e-6);
        assert_eq!(a.surface.sample_channel(2, 2, 0), 1.0);
        assert_eq!(a.surface.sample_channel(6, 2, 0), 0.0);
        assert_eq!(s.name, "PANTONE X");
        assert_eq!(s.spot, Some((Color::rgb(1.0, 0.0, 1.0), 0.25)));
        let q = back.quick_mask.as_ref().expect("quick mask restored");
        assert_eq!(q.surface.sample_channel(10, 2, 0), 1.0);
        assert_eq!(q.surface.sample_channel(14, 2, 0), 0.0);
        // Re-exporting doesn't duplicate the display-info resources.
        let f2 = document_to_psd(&back);
        assert_eq!(f2.resources.iter().filter(|r| r.id == 1077).count(), 1);
        assert_eq!(f2.resources.iter().filter(|r| r.id == 1022).count(), 1);
    }

    let mut full = Document::with_background("full", Size::new(1, 1), ColorMode::Rgb, SampleType::U8, Color::WHITE);
    full.channels = (0..53).map(|i| AlphaChannel::new(format!("Alpha {i}"), Surface::new(PixelFormat::GRAY8))).collect();
    full.quick_mask = Some(AlphaChannel::new("Quick Mask", Surface::new(PixelFormat::GRAY8)));
    let out = imagenor_io::export(&full, "full.psd", &Default::default()).unwrap();
    assert!(out.warnings.iter().any(|warning| warning.contains("Quick Mask")), "{:?}", out.warnings);
    let file = imagenor_psd::PsdFile::from_bytes(&out.bytes).unwrap();
    assert_eq!(file.header.channels, 56);
    assert!(file.resources.iter().all(|resource| resource.id != 1022));
}

/// Original synthetic inks, not Pantone/library data or an Adobe-created file.
#[test]
fn lab_spot_alternates_survive_psd_and_psb_bytes_at_every_depth() {
    let lab = [62.34, 85.67, -79.12];
    let ink = Color { mode: ColorMode::Lab, c: [lab[0] / 100.0, (lab[1] + 128.0) / 255.0, (lab[2] + 128.0) / 255.0, 0.0], alpha: 1.0 };
    for mode in [ColorMode::Rgb, ColorMode::Cmyk] {
        for depth in SampleType::ALL {
            for transparent in [false, true] {
                for ext in ["synthetic.psd", "synthetic.psb"] {
                    let mut d =
                        Document::with_background("Synthetic spot", Size::new(16, 8), mode, depth, if transparent { Color::TRANSPARENT } else { Color::WHITE });
                    let mut channel = AlphaChannel::new("Original Ink – రంగు C", channel(depth, 8));
                    channel.spot = Some((ink, 0.7));
                    d.channels.push(channel);
                    for _ in 0..2 {
                        let out = imagenor_io::export(&d, ext, &Default::default()).unwrap();
                        assert!(out.warnings.is_empty(), "{mode:?}/{depth:?}/{ext}: {:?}", out.warnings);
                        let back = imagenor_io::import(ext, &out.bytes).unwrap();
                        assert!(back.warnings.is_empty(), "{:?}", back.warnings);
                        let ch = &back.document.channels[0];
                        assert_eq!(ch.name, "Original Ink – రంగు C");
                        let (got, solidity) = ch.spot.unwrap();
                        assert_eq!(got.mode, ColorMode::Lab, "{mode:?}/{depth:?}/{ext}");
                        let decoded = [got.c[0] * 100.0, got.c[1] * 255.0 - 128.0, got.c[2] * 255.0 - 128.0];
                        for (actual, expected) in decoded.iter().zip(lab) {
                            assert!((*actual - expected).abs() <= 0.0051);
                        }
                        assert_eq!(solidity, 0.7);
                        assert_eq!(ch.surface.sample_channel(2, 2, 0), 1.0);
                        assert_eq!(ch.surface.sample_channel(12, 2, 0), 0.0);
                        d = back.document;
                    }
                }
            }
        }
    }
}
