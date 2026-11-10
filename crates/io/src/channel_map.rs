//! Alpha / spot channel options <-> PSD `DisplayInfo` image resources (1077, and the older 1007).
//!
//! Layout (Adobe PSD spec, "DisplayInfo"): resource 1077 starts with a 4-byte version (1), then
//! one 13-byte record per extra channel: colour space (u16), four u16 colour components, opacity
//! (u16, 0..100) and kind (u8: 0 = colour indicates selected areas, 1 = masked ("protected")
//! areas, 2 = spot channel). Resource 1007 has the same records without the version and the spot
//! kind, each followed by a padding byte (14 bytes).

use imagenor_color::{Color, ColorMode};
use imagenor_doc::{AlphaChannel, ColorIndicates};

pub(crate) const DISPLAY_INFO: u16 = 1077;
pub(crate) const DISPLAY_INFO_OLD: u16 = 1007;
/// Quick Mask information: channel id (u16) and an "initially empty" flag (u8).
pub(crate) const QUICK_MASK_INFO: u16 = 1022;

const SPACE_RGB: u16 = 0;
const SPACE_HSB: u16 = 1;
const SPACE_CMYK: u16 = 2;
const SPACE_LAB: u16 = 7;
const SPACE_GRAY: u16 = 8;

/// Resource 1077 for `channels` (in file order): a version, then 13 bytes per channel (colour
/// space, four components, opacity, kind). Every corpus file, Photoshop's included, uses 13;
/// 14-byte records (the older 1007 layout) shift every channel after the first.
/// `transparency`: the merged image's transparency channel comes first and gets a record too.
pub(crate) fn display_info(channels: &[&AlphaChannel], transparency: bool) -> Vec<u8> {
    let mut out = 1u32.to_be_bytes().to_vec();
    if transparency {
        // As Photoshop writes it: red, 100 %, masked areas.
        out.extend_from_slice(&[0, 0, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 100, 1]);
    }
    for c in channels {
        let (color, opacity, kind) = match c.spot {
            Some((ink, solidity)) => (ink, solidity, 2u8),
            None => (c.color, c.opacity, if c.indicates == ColorIndicates::SelectedAreas { 0 } else { 1 }),
        };
        if kind == 2 && color.mode == ColorMode::Lab {
            // Keep the device-independent ink alternate, including colours outside RGB.
            // PSD colour structures encode L* in hundredths and signed a*/b* in hundredths:
            // https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/ (Color space IDs).
            let finite = |v: f32, fallback: f32| if v.is_finite() { v } else { fallback };
            let l = (finite(color.c[0], 0.0).clamp(0.0, 1.0) * 10000.0).round() as u16;
            let chroma = |v: f32| ((finite(v, 128.0 / 255.0) * 255.0 - 128.0).clamp(-128.0, 127.0) * 100.0).round() as i16;
            out.extend_from_slice(&SPACE_LAB.to_be_bytes());
            out.extend_from_slice(&l.to_be_bytes());
            out.extend_from_slice(&chroma(color.c[1]).to_be_bytes());
            out.extend_from_slice(&chroma(color.c[2]).to_be_bytes());
            out.extend_from_slice(&0u16.to_be_bytes());
        } else {
            let rgb = color.to_rgb();
            out.extend_from_slice(&SPACE_RGB.to_be_bytes());
            for v in [rgb[0], rgb[1], rgb[2], 0.0] {
                out.extend_from_slice(&((v.clamp(0.0, 1.0) * 65535.0).round() as u16).to_be_bytes());
            }
        }
        out.extend_from_slice(&((opacity.clamp(0.0, 1.0) * 100.0).round() as u16).to_be_bytes());
        out.push(kind);
    }
    out
}

/// Applies a DisplayInfo resource (`versioned` = 1077: 13-byte records after a version; else the
/// older 1007 with 14-byte records) to the imported channels in order, after skipping `skip`
/// records (the merged transparency channel's).
pub(crate) fn apply_display_info(data: &[u8], versioned: bool, skip: usize, channels: &mut [AlphaChannel]) {
    let (body, size) = if versioned { (data.get(4..).unwrap_or(&[]), 13) } else { (data, 14) };
    for (c, rec) in channels.iter_mut().zip(body.chunks_exact(size).skip(skip)) {
        let u = |i: usize| u16::from_be_bytes([rec[i], rec[i + 1]]);
        let comps = [u(2), u(4), u(6), u(8)];
        let color = if rec[12] == 2 && u(0) == SPACE_LAB { decode_lab(comps) } else { decode_color(u(0), comps) };
        let opacity = f32::from(u(10).min(100)) / 100.0;
        match rec[12] {
            2 => c.spot = Some((color, opacity)),
            k => {
                // An alpha channel, not an ink (a Multichannel import starts every channel as one).
                c.spot = None;
                c.color = color;
                c.opacity = opacity;
                c.indicates = if k == 0 { ColorIndicates::SelectedAreas } else { ColorIndicates::MaskedAreas };
            }
        }
    }
}

/// Device-independent Lab alternate, without a gamut-clipping RGB intermediate.
fn decode_lab(c: [u16; 4]) -> Color {
    let l = f32::from(c[0].min(10000)) / 10000.0;
    let a = f32::from((c[1] as i16).clamp(-12800, 12700)) / 100.0;
    let b = f32::from((c[2] as i16).clamp(-12800, 12700)) / 100.0;
    Color { mode: ColorMode::Lab, c: [l, (a + 128.0) / 255.0, (b + 128.0) / 255.0, 0.0], alpha: 1.0 }
}

/// A PSD colour structure (space id + four u16 components) as an RGB [`Color`]; unknown spaces
/// (book colours…) read the first three components as RGB. See [`decode_known_color`].
pub(crate) fn decode_color(space: u16, c: [u16; 4]) -> Color {
    let n = |v: u16| f32::from(v) / 65535.0;
    decode_known_color(space, c).unwrap_or_else(|| Color::rgb(n(c[0]), n(c[1]), n(c[2])))
}

/// A PSD colour structure in one of the spaces Photoshop writes for plain colours (0 RGB,
/// 1 HSB, 2 CMYK, 7 Lab, 8 Grayscale) as an RGB [`Color`], or `None` for any other space id.
/// CMYK goes through the colour-managed conversion (the active document CMYK profile, see
/// `imagenor_color::convert::with_cmyk_space`), Lab through the D50 Lab → sRGB conversion.
pub(crate) fn decode_known_color(space: u16, c: [u16; 4]) -> Option<Color> {
    let n = |v: u16| f32::from(v) / 65535.0;
    Some(match space {
        SPACE_RGB => Color::rgb(n(c[0]), n(c[1]), n(c[2])),
        SPACE_HSB => {
            let rgb = hsb_to_rgb(n(c[0]) * 360.0, n(c[1]), n(c[2]));
            Color::rgb(rgb[0], rgb[1], rgb[2])
        }
        // Photoshop colour structures store CMYK inverted (65535 = no ink).
        SPACE_CMYK => {
            let cmyk = Color { mode: ColorMode::Cmyk, c: [1.0 - n(c[0]), 1.0 - n(c[1]), 1.0 - n(c[2]), 1.0 - n(c[3])], alpha: 1.0 };
            let rgb = cmyk.to_rgb();
            Color::rgb(rgb[0], rgb[1], rgb[2])
        }
        SPACE_LAB => {
            let l = f32::from(c[0].min(10000)) / 10000.0;
            let a = f32::from(c[1] as i16) / 100.0;
            let b = f32::from(c[2] as i16) / 100.0;
            let lab = Color { mode: ColorMode::Lab, c: [l, (a + 128.0) / 255.0, (b + 128.0) / 255.0, 0.0], alpha: 1.0 };
            let rgb = lab.to_rgb();
            Color::rgb(rgb[0], rgb[1], rgb[2])
        }
        SPACE_GRAY => {
            let v = 1.0 - f32::from(c[0].min(10000)) / 10000.0;
            Color::rgb(v, v, v)
        }
        _ => return None,
    })
}

/// HSB (hue in degrees, saturation and brightness 0..1) → RGB.
pub(crate) fn hsb_to_rgb(h: f32, s: f32, v: f32) -> [f32; 3] {
    let h = if h.is_finite() { h.rem_euclid(360.0) / 60.0 } else { 0.0 };
    let (s, v) = (s.clamp(0.0, 1.0), v.clamp(0.0, 1.0));
    let f = h - h.floor();
    let (p, q, t) = (v * (1.0 - s), v * (1.0 - s * f), v * (1.0 - s * (1.0 - f)));
    match h.floor() as u8 {
        0 => [v, t, p],
        1 => [q, v, p],
        2 => [p, v, t],
        3 => [p, q, v],
        4 => [t, p, v],
        _ => [v, p, q],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use imagenor_color::PixelFormat;
    use imagenor_raster::Surface;

    #[test]
    fn display_info_roundtrip() {
        let mut a = AlphaChannel::new("A", Surface::new(PixelFormat::GRAY8));
        a.color = Color::rgb(0.0, 1.0, 0.0);
        a.opacity = 0.3;
        a.indicates = ColorIndicates::SelectedAreas;
        let mut s = AlphaChannel::new("S", Surface::new(PixelFormat::GRAY8));
        s.spot = Some((Color::rgb(1.0, 0.0, 1.0), 0.8));
        let bytes = display_info(&[&a, &s], false);
        // 13 bytes per channel, as every corpus file (Photoshop's included) lays out 1077.
        assert_eq!(bytes.len(), 4 + 2 * 13);
        let mut back = vec![AlphaChannel::new("A", Surface::new(PixelFormat::GRAY8)), AlphaChannel::new("S", Surface::new(PixelFormat::GRAY8))];
        apply_display_info(&bytes, true, 0, &mut back);
        assert_eq!(back[0].color, a.color);
        assert!((back[0].opacity - 0.3).abs() < 1e-6);
        assert_eq!(back[0].indicates, ColorIndicates::SelectedAreas);
        assert_eq!(back[1].spot, s.spot);
        // With the merged transparency's record first, the channels still get their own.
        let with_t = display_info(&[&a, &s], true);
        assert_eq!(with_t.len(), 4 + 3 * 13);
        let mut back = vec![AlphaChannel::new("A", Surface::new(PixelFormat::GRAY8)), AlphaChannel::new("S", Surface::new(PixelFormat::GRAY8))];
        apply_display_info(&with_t, true, 1, &mut back);
        assert_eq!(back[0].indicates, ColorIndicates::SelectedAreas);
        assert_eq!(back[1].spot, s.spot);
        // Truncated data is ignored rather than misread.
        let mut c = vec![AlphaChannel::new("x", Surface::new(PixelFormat::GRAY8))];
        apply_display_info(&bytes[..10], true, 0, &mut c);
        assert_eq!(c[0].opacity, 0.5);
    }

    #[test]
    fn lab_spot_resource_retains_signed_components_without_rgb_clipping() {
        let ink = Color { mode: ColorMode::Lab, c: [0.6234, (85.67 + 128.0) / 255.0, (-79.12 + 128.0) / 255.0, 0.0], alpha: 1.0 };
        let mut channel = AlphaChannel::new("Original synthetic ink", Surface::new(PixelFormat::GRAY8));
        channel.spot = Some((ink, 0.7));
        let bytes = display_info(&[&channel], false);
        // Independently constructed public-spec record: version, Lab id, L*, signed a*/b*,
        // unused component, solidity and spot kind. No application-authored fixture.
        let mut expected = vec![0, 0, 0, 1, 0, 7];
        expected.extend_from_slice(&6234u16.to_be_bytes());
        expected.extend_from_slice(&8567i16.to_be_bytes());
        expected.extend_from_slice(&(-7912i16).to_be_bytes());
        expected.extend_from_slice(&[0, 0, 0, 70, 2]);
        assert_eq!(bytes, expected);
        let mut back = vec![AlphaChannel::new("ink", Surface::new(PixelFormat::GRAY8))];
        apply_display_info(&expected, true, 0, &mut back);
        let (actual, solidity) = back[0].spot.unwrap();
        assert_eq!(actual.mode, ColorMode::Lab);
        for (a, b) in actual.c.iter().zip(ink.c) {
            assert!((*a - b).abs() < 0.00001);
        }
        assert_eq!(solidity, 0.7);
        assert_eq!(display_info(&[&back[0]], false), expected);
    }

    #[test]
    fn lab_spot_bounds_and_nonfinite_components_encode_safely() {
        for (values, expected) in
            [([1.0, 0.0, 1.0, 0.0], [10000u16, (-12800i16) as u16, 12700, 0]), ([f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 0.0], [0, 0, 0, 0])]
        {
            let mut c = AlphaChannel::new("Synthetic bounds", Surface::new(PixelFormat::GRAY8));
            c.spot = Some((Color { mode: ColorMode::Lab, c: values, alpha: 1.0 }, 0.0));
            let bytes = display_info(&[&c], false);
            for (i, value) in expected.iter().enumerate() {
                assert_eq!(u16::from_be_bytes(bytes[6 + i * 2..8 + i * 2].try_into().unwrap()), *value);
            }
        }
        // Out-of-spec input is bounded within the colour structure's documented Lab range.
        let c = decode_lab([65535, (-30000i16) as u16, 30000, 0]);
        assert_eq!(c.c, [1.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn decodes_other_spaces() {
        let g = decode_color(SPACE_GRAY, [10000, 0, 0, 0]);
        assert_eq!(g.to_rgb(), [0.0; 3]);
        let lab = decode_color(SPACE_LAB, [10000, 0, 0, 0]).to_rgb();
        assert!(lab.iter().all(|v| *v > 0.95), "{lab:?}");
        // HSB: hue 120 degrees (65535 / 3), full saturation and brightness is pure green.
        let g = decode_color(SPACE_HSB, [21845, 65535, 65535, 0]).to_rgb();
        assert!((g[0]).abs() < 1e-3 && (g[1] - 1.0).abs() < 1e-3 && g[2].abs() < 1e-3, "{g:?}");
        let half = decode_color(SPACE_HSB, [0, 0, 32768, 0]).to_rgb();
        assert!(half.iter().all(|v| (v - 0.5).abs() < 1e-3), "{half:?}");
        // Unknown spaces (book colours) are not guessed.
        assert_eq!(decode_known_color(3, [1, 2, 3, 4]), None);
        assert_eq!(decode_known_color(SPACE_RGB, [65535, 0, 0, 0]), Some(Color::rgb(1.0, 0.0, 0.0)));
    }
}
