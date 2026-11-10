//! Unmodified, licensed third-party profiles embedded for offline use.
//! These are explicit choices, never replacements for existing document profiles or defaults.
//! Provenance, SHA-256 hashes and terms live in `profiles/sources.json` and adjacent licences.
use crate::{CmsError, ColorSpace, Profile};
use std::sync::{Arc, OnceLock};

pub struct BundledProfile {
    pub id: &'static str,
    pub description: &'static str,
    pub color_space: ColorSpace,
    pub bytes: &'static [u8],
    parsed: OnceLock<Result<Arc<Profile>, CmsError>>,
}

impl BundledProfile {
    /// Fail visibly if an embedded profile cannot be parsed; never substitute another profile.
    pub fn profile(&self) -> Result<Arc<Profile>, CmsError> {
        self.parsed.get_or_init(|| Profile::parse(self.bytes).map(Arc::new)).clone()
    }
}

pub static PROFILES: [BundledProfile; 3] = [
    BundledProfile {
        id: "icc-srgb-2014",
        description: "sRGB2014",
        color_space: ColorSpace::Rgb,
        bytes: include_bytes!("../profiles/sRGB2014.icc"),
        parsed: OnceLock::new(),
    },
    BundledProfile {
        id: "gracol-2013-crpc6",
        description: "GRACoL2013_CRPC6.icc",
        color_space: ColorSpace::Cmyk,
        bytes: include_bytes!("../profiles/GRACoL2013_CRPC6.icc"),
        parsed: OnceLock::new(),
    },
    BundledProfile {
        id: "swop-2013-c3-crpc5",
        description: "SWOP2013C3_CRPC5.icc",
        color_space: ColorSpace::Cmyk,
        bytes: include_bytes!("../profiles/SWOP2013C3_CRPC5.icc"),
        parsed: OnceLock::new(),
    },
];

pub fn find(spec: &str) -> Option<&'static BundledProfile> {
    PROFILES.iter().find(|p| p.id.eq_ignore_ascii_case(spec) || p.description.eq_ignore_ascii_case(spec))
}
