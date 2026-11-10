//! Explicit spot-coverage plate export. No process separation, halftoning or printer spooling.
//! Samples are ink coverage; display-only solidity and ICC display transforms must not alter it.
use crate::commands::CommandSpec;
use crate::{EngineError, Result, Session};
use imagenor_codecs::{ChannelLayout, EncodeOptions, Format, Image, SampleType};
use serde_json::{Value, json};

const CMD: &str = "file.exportSpotPlate";
const MAX_RAW_BYTES: usize = 128 * 1024 * 1024;
fn bad(message: impl Into<String>) -> EngineError {
    EngineError::BadParams { cmd: CMD.into(), msg: message.into() }
}

fn enabled(s: &Session) -> std::result::Result<(), String> {
    let d = s.active().ok_or("no document open")?;
    if d.doc.channels.iter().any(|c| c.spot.is_some()) { Ok(()) } else { Err("the document has no spot channels".into()) }
}

/// TIFF ImageDescription is ASCII. Escape Unicode as JSON UTF-16 escapes, keeping names lossless.
fn ascii_json(value: &Value) -> String {
    value.to_string().encode_utf16().map(|u| if u < 128 { char::from_u32(u32::from(u)).unwrap_or(' ').to_string() } else { format!("\\u{u:04x}") }).collect()
}

fn export(s: &mut Session, p: &Value) -> Result<Value> {
    let st = s.active().ok_or(EngineError::NoDocument)?;
    let doc = &st.doc;
    let index = match p.get("channel") {
        Some(Value::String(name)) => {
            let mut matches = doc.channels.iter().enumerate().filter(|(_, c)| c.name == *name);
            let index = matches.next().map(|(i, _)| i).ok_or_else(|| bad("no channel with that name"))?;
            if matches.next().is_some() {
                return Err(bad("channel name is ambiguous; use its numeric index"));
            }
            index
        }
        Some(v) => v.as_u64().and_then(|v| usize::try_from(v).ok()).ok_or_else(|| bad("channel must be an index or name"))?,
        None => return Err(bad("channel is required (see channel.list)")),
    };
    let ch = doc.channels.get(index).ok_or_else(|| bad("channel index is out of range"))?;
    if ch.name.len() > 64 * 1024 {
        return Err(bad("spot ink name exceeds the 64 KiB metadata budget"));
    }
    let (ink, solidity) = ch.spot.ok_or_else(|| bad("the selected channel is not a spot ink"))?;
    if !ink.c.iter().all(|v| v.is_finite()) || !ink.alpha.is_finite() || !solidity.is_finite() {
        return Err(bad("spot alternate colour and solidity must be finite"));
    }
    let output = match p.get("output") {
        None => None,
        Some(Value::String(path)) if !path.trim().is_empty() => {
            let ext = std::path::Path::new(path).extension().and_then(|x| x.to_str()).unwrap_or_default();
            if !ext.eq_ignore_ascii_case("tif") && !ext.eq_ignore_ascii_case("tiff") {
                return Err(bad("output must have a .tif or .tiff extension"));
            }
            Some(path.as_str())
        }
        Some(_) => return Err(bad("output must be a nonempty TIFF path")),
    };
    let (w, h) = (doc.size.width, doc.size.height);
    if w == 0 || h == 0 || w > i32::MAX as u32 || h > i32::MAX as u32 {
        return Err(bad("invalid plate dimensions"));
    }
    let len = (w as usize)
        .checked_mul(h as usize)
        .and_then(|n| n.checked_mul(2))
        .filter(|n| *n <= MAX_RAW_BYTES)
        .ok_or_else(|| bad("spot plate exceeds the 128 MiB raw-sample budget"))?;
    if !doc.resolution_dpi.is_finite() || doc.resolution_dpi <= 0.0 || doc.resolution_dpi > 1_000_000.0 {
        return Err(bad("document resolution must be finite, positive and at most 1,000,000 dpi"));
    }
    let mut raw = Vec::new();
    raw.try_reserve_exact(len).map_err(|e| bad(format!("cannot allocate plate: {e}")))?;
    let mut clamped = 0u64;
    for y in 0..h {
        for x in 0..w {
            let coverage = ch.surface.sample_channel(x as i32, y as i32, 0);
            if !coverage.is_finite() {
                return Err(bad("spot channel contains a non-finite coverage sample"));
            }
            if !(0.0..=1.0).contains(&coverage) {
                clamped += 1;
            }
            let gray = ((1.0 - coverage.clamp(0.0, 1.0)) * 65535.0).round() as u16;
            raw.extend_from_slice(&gray.to_ne_bytes());
        }
    }
    let metadata = json!({"schema":"imagenor.spot-plate.v1", "channel":index,"inkName":ch.name,
        "alternateColor":ink,"solidityPreviewOnly":solidity,"polarity":"blackIsFullInk", "bitsPerSample":16,
        "resolutionDpi":doc.resolution_dpi,"clampedSamples":clamped,"halftoned":false,"overprintApplied":false});
    let mut image = Image::from_raw(w, h, ChannelLayout::Gray, SampleType::U16, raw).map_err(|e| bad(e.to_string()))?;
    image.meta.dpi = Some((doc.resolution_dpi, doc.resolution_dpi));
    image.meta.text.push(("Description".into(), ascii_json(&metadata)));
    // Intentionally untagged: these are linear ink coverage values, not display gray values.
    let bytes = imagenor_codecs::encode(&image, Format::Tiff, &EncodeOptions::default()).map_err(|e| bad(e.to_string()))?;
    let mut reply = json!({"plate":metadata,"width":w,"height":h,"format":"tiff","bytes":bytes.len(),
        "warnings":["Spot coverage only; process plates, halftoning, trapping, registration marks and overprint are not applied."]});
    if let Some(path) = output {
        crate::file_cmds::write_file(path, &bytes)?;
        reply["output"] = json!(path);
    } else {
        reply["data"] = json!(imagenor_paint::tile::b64_encode(&bytes));
    }
    Ok(reply)
}

pub fn specs() -> Vec<CommandSpec> {
    vec![CommandSpec {
        id: CMD,
        label: "Export Spot Plate…",
        menu: &[],
        shortcut: None,
        params: r#"{"channel":index|name,"output":TIFF path?} → {plate,width,height,bytes,data:base64?}. Exports one 16-bit grayscale spot coverage plate, black=full ink. No colour conversion, preview-solidity scaling, halftoning or spooling. Without output returns TIFF bytes as base64."#,
        enabled,
        run: export,
        journal: false,
    }]
}

#[cfg(test)]
mod tests;
