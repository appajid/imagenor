//! Native text persistence uses synthetic documents and requires no installed fonts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

use imagenor_color::{ColorMode, SampleType};
use imagenor_doc::text::{CharStyle, ParagraphRun, ParagraphStyle, TextDirection, TextRun, TextShape};
use imagenor_doc::{Document, Layer, LayerContent, Size, TextLayer};
use imagenor_geom::Affine;

const SAMPLES: &[(&str, &str, TextDirection)] = &[
    ("hi", "हिन्दी भाषा क्षत्रिय श्री", TextDirection::Ltr),
    ("mr", "मराठी भाषा महाराष्ट्र", TextDirection::Ltr),
    ("sa", "संस्कृतम् क्षत्रिय श्री", TextDirection::Ltr),
    ("ne", "नेपाली भाषा", TextDirection::Ltr),
    ("bn", "বাংলা ভাষা ক্ষত্রিয় শ্রী", TextDirection::Ltr),
    ("as", "অসমীয়া ভাষা", TextDirection::Ltr),
    ("pa", "ਪੰਜਾਬੀ ਭਾਸ਼ਾ ਸਤਿ ਸ੍ਰੀ", TextDirection::Ltr),
    ("gu", "ગુજરાતી ભાષા ક્ષત્રિય શ્રી", TextDirection::Ltr),
    ("or", "ଓଡ଼ିଆ ଭାଷା", TextDirection::Ltr),
    ("ta", "தமிழ் மொழி ஸ்ரீ கொடை", TextDirection::Ltr),
    ("te", "తెలుగు భాష క్షత్రియ శ్రీ", TextDirection::Ltr),
    ("kn", "ಕನ್ನಡ ಭಾಷೆ ಕ್ಷತ್ರಿಯ ಶ್ರೀ", TextDirection::Ltr),
    ("ml", "മലയാളം ഭാഷ ക്ഷത്രിയ ശ്രീ", TextDirection::Ltr),
    ("ur", "اردو زبان", TextDirection::Rtl),
    ("sd", "سنڌي ٻولي", TextDirection::Rtl),
];

fn document(language: &str, sample: &str, direction: TextDirection) -> Document {
    let first = format!("{sample}\n");
    let latin = "Imagenor 123";
    let style = CharStyle {
        // Persistence must not depend on resolving a face or rasterizing cached text.
        font_family: "Synthetic Missing Indian Font".into(),
        language: Some(language.into()),
        size_pt: 24.0,
        ..Default::default()
    };
    let mut text = TextLayer {
        text: format!("{first}{latin}"),
        runs: vec![
            TextRun { len: first.len(), style: style.clone() },
            TextRun { len: latin.len(), style: CharStyle { language: Some("en".into()), underline: true, ..style } },
        ],
        paragraphs: vec![
            ParagraphRun { len: first.len(), style: ParagraphStyle { direction, space_after_pt: 6.0, ..Default::default() } },
            ParagraphRun { len: latin.len(), style: ParagraphStyle { direction: TextDirection::Ltr, ..Default::default() } },
        ],
        shape: TextShape::Box { x: 0.0, y: 0.0, width: 280.0, height: 160.0 },
        transform: Affine::translate(12.0, 18.0),
        ..Default::default()
    };
    text.sync_summary();
    let mut doc = Document::new("Indian text persistence", Size::new(320, 200), ColorMode::Rgb, SampleType::U8);
    doc.resolution_dpi = 144.0;
    doc.layers.push(Layer::new(sample, LayerContent::Text(text)));
    doc
}

fn reopened(doc: &Document) -> Document {
    let out = imagenor_io::export(doc, "indian-text.ignr", &Default::default()).unwrap();
    assert!(out.warnings.is_empty(), "{:?}", out.warnings);
    let imported = imagenor_io::import("indian-text.ignr", &out.bytes).unwrap();
    assert!(!imported.source_read_only && !imported.preview_only);
    imported.document
}

fn text_layer(doc: &Document) -> &TextLayer {
    let LayerContent::Text(text) = &doc.layers[0].content else { panic!("editable type must not become a flattened layer") };
    text
}

#[test]
fn indian_text_styles_and_paragraph_direction_survive_ignr_reopen_and_resave() {
    for &(language, sample, direction) in SAMPLES {
        let original = document(language, sample, direction);
        let first = reopened(&original);
        let second = reopened(&first);
        for back in [&first, &second] {
            let expected = text_layer(&original);
            let actual = text_layer(back);
            assert_eq!(actual.text, expected.text, "{language}");
            assert_eq!(actual.runs, expected.runs, "{language}: styles/language and UTF-8 run boundaries");
            assert_eq!(actual.paragraphs, expected.paragraphs, "{language}: paragraph direction");
            assert_eq!(actual.font_family, expected.font_family, "{language}: missing font metadata");
            assert_eq!(actual.shape, expected.shape, "{language}: editable box geometry");
            assert_eq!(actual.transform, expected.transform, "{language}: placement");
            assert_eq!(back.resolution_dpi, original.resolution_dpi);
            assert_eq!(back.layers[0].name, sample);
        }
    }
}

#[test]
fn reopened_indian_text_remains_editable_and_saves_the_edit() {
    for &(language, sample, direction) in SAMPLES {
        let mut doc = reopened(&document(language, sample, direction));
        let LayerContent::Text(text) = &mut doc.layers[0].content else { panic!("editable type") };
        text.text.push('!');
        text.runs.last_mut().unwrap().len += 1;
        text.paragraphs.last_mut().unwrap().len += 1;
        let back = reopened(&doc);
        assert_eq!(text_layer(&back).text, format!("{sample}\nImagenor 123!"), "{language}");
        assert_eq!(text_layer(&back).runs, text_layer(&doc).runs, "{language}");
        assert_eq!(text_layer(&back).paragraphs, text_layer(&doc).paragraphs, "{language}");
    }
}
