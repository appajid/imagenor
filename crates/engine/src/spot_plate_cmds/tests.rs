use super::*;
use imagenor_color::{Color, ColorMode, PixelFormat, SampleType as Depth};
use imagenor_doc::{AlphaChannel, Document};
use imagenor_geom::{Rect, Size};
use imagenor_raster::Surface;
use std::sync::Arc;

fn session(depth: Depth) -> Session {
    let mut s = Session::new();
    let mut doc = Document::new("Plate test", Size::new(3, 1), ColorMode::Rgb, depth);
    doc.resolution_dpi = 300.0;
    let mut surface = Surface::new(PixelFormat::new(ColorMode::Grayscale, depth, false));
    surface.fill_rect(Rect::new(1, 0, 2, 1), &[0.5]);
    surface.fill_rect(Rect::new(2, 0, 3, 1), &[1.0]);
    let mut ch = AlphaChannel::new("../Original ink రంగు 🖨 C", surface);
    ch.spot = Some((Color { mode: ColorMode::Lab, c: [0.6, 0.8, 0.2, 0.0], alpha: 1.0 }, 0.2));
    doc.channels.push(ch);
    s.add_document(doc, None);
    s
}

fn decoded(reply: &Value) -> Image {
    let bytes = imagenor_paint::tile::b64_decode(reply["data"].as_str().unwrap()).unwrap();
    imagenor_codecs::decode(&bytes).unwrap()
}

#[test]
fn plate_exports_coverage_not_preview_color_or_solidity_at_all_depths() {
    for depth in Depth::ALL {
        let mut s = session(depth);
        let before = s.active().unwrap().history.entries().len();
        let reply = s.execute(CMD, json!({"channel":0})).unwrap();
        let image = decoded(&reply);
        assert_eq!(image.layout(), ChannelLayout::Gray);
        assert_eq!(image.sample_type(), SampleType::U16);
        assert!(image.icc.is_none(), "coverage must not acquire a display gray profile");
        assert_eq!(image.meta.dpi, Some((300.0, 300.0)));
        let samples = image.to_u16_samples().unwrap();
        assert_eq!((samples[0], samples[2]), (65535, 0), "white=no ink, black=full ink");
        for (x, sample) in samples.iter().enumerate() {
            let source = s.active().unwrap().doc.channels[0].surface.sample_channel(x as i32, 0, 0);
            assert!((1.0 - f32::from(*sample) / 65535.0 - source).abs() <= 0.000008);
        }
        let description = &image.meta.text.iter().find(|(key, _)| key == "Description").unwrap().1;
        assert!(description.is_ascii());
        let metadata: Value = serde_json::from_str(description).unwrap();
        assert_eq!(metadata, reply["plate"]);
        assert_eq!(metadata["inkName"], "../Original ink రంగు 🖨 C");
        assert_eq!(metadata["alternateColor"]["mode"], "Lab");
        let doc = Arc::make_mut(&mut s.active_mut().unwrap().doc);
        doc.channels[0].spot.as_mut().unwrap().1 = 0.9;
        doc.channels[0].spot.as_mut().unwrap().0 = Color::rgb(0.0, 1.0, 0.0);
        let second = decoded(&s.execute(CMD, json!({"channel": "../Original ink రంగు 🖨 C"})).unwrap());
        assert_eq!(image.data(), second.data(), "alternate/solidity affect display only");
        assert_eq!(s.active().unwrap().history.entries().len(), before);
    }
}

#[test]
fn bad_plate_requests_fail_without_editing() {
    let mut s = session(Depth::F32);
    for p in [
        json!({}),
        json!({"channel":null}),
        json!({"channel":-1}),
        json!({"channel":999}),
        json!({"channel":"missing"}),
        json!({"channel":0,"output":42}),
        json!({"channel":0,"output":"x.ignr"}),
    ] {
        assert!(s.execute(CMD, p).is_err());
    }
    let doc = Arc::make_mut(&mut s.active_mut().unwrap().doc);
    doc.channels.push(doc.channels[0].clone());
    assert!(s.execute(CMD, json!({"channel":"../Original ink రంగు 🖨 C"})).is_err(), "ambiguous names need an index");
    Arc::make_mut(&mut s.active_mut().unwrap().doc).channels[0].spot = None;
    assert!(s.execute(CMD, json!({"channel":0})).is_err());
    let mut s = session(Depth::F32);
    Arc::make_mut(&mut s.active_mut().unwrap().doc).size = Size::new(100_000, 100_000);
    assert!(s.execute(CMD, json!({"channel":0})).is_err(), "allocation bounded before sampling");
    let mut s = session(Depth::F32);
    Arc::make_mut(&mut s.active_mut().unwrap().doc).resolution_dpi = f32::NAN;
    assert!(s.execute(CMD, json!({"channel":0})).is_err());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn explicit_output_writes_tiff_without_marking_document_saved() {
    let mut s = session(Depth::U16);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("plate.tif");
    let reply = s.execute(CMD, json!({"channel":0,"output":path})).unwrap();
    assert!(reply.get("data").is_none());
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(reply["bytes"].as_u64().unwrap() as usize, bytes.len());
    assert_eq!(imagenor_codecs::decode(&bytes).unwrap().to_u16_samples().unwrap()[2], 0);
    assert!(s.active().unwrap().path.is_none());
    let bad_dir = dir.path().join("directory.tif");
    std::fs::create_dir(&bad_dir).unwrap();
    assert!(s.execute(CMD, json!({"channel":0,"output":bad_dir})).is_err());
    assert!(bad_dir.is_dir());
}

#[test]
fn float_coverage_clamps_with_a_report_but_rejects_nan() {
    let mut s = session(Depth::F32);
    let ch = &mut Arc::make_mut(&mut s.active_mut().unwrap().doc).channels[0];
    ch.surface.fill_rect(Rect::new(0, 0, 1, 1), &[-0.5]);
    ch.surface.fill_rect(Rect::new(1, 0, 2, 1), &[1.5]);
    let reply = s.execute(CMD, json!({"channel":0})).unwrap();
    assert_eq!(reply["plate"]["clampedSamples"], 2);
    assert_eq!(decoded(&reply).to_u16_samples().unwrap(), [65535, 0, 0]);
    Arc::make_mut(&mut s.active_mut().unwrap().doc).channels[0].surface.fill_rect(Rect::new(0, 0, 1, 1), &[f32::NAN]);
    assert!(s.execute(CMD, json!({"channel":0})).is_err());
}
