//! English-only interface. Indian-language document text is handled independently by
//! the text engine: removing UI catalogs must never filter Unicode document content.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lang;

pub struct LangInfo {
    pub code: &'static str,
    pub name: &'static str,
    pub complete_menus: bool,
}

pub static LANGUAGES: [LangInfo; 1] = [LangInfo { code: "en", name: "English", complete_menus: false }];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Ltr,
    Rtl,
}

/// Document-language direction remains useful for Urdu and Sindhi text.
pub fn dir_for_code(code: &str) -> Dir {
    let primary = code.split(['-', '_']).next().unwrap_or("");
    if ["ar", "ckb", "dv", "fa", "he", "ps", "sd", "ug", "ur", "yi"].iter().any(|tag| tag.eq_ignore_ascii_case(primary)) { Dir::Rtl } else { Dir::Ltr }
}

impl Lang {
    pub const EN: Self = Self;
    pub fn code(self) -> &'static str {
        "en"
    }
    pub fn name(self) -> &'static str {
        "English"
    }
    pub fn from_code(code: &str) -> Option<Self> {
        code.eq_ignore_ascii_case("en").then_some(Self)
    }
    /// Older saved locale preferences always migrate to the English interface.
    pub fn from_pref(_: &str) -> Self {
        Self
    }
    pub fn all() -> impl Iterator<Item = Self> {
        std::iter::once(Self)
    }
    pub fn for_picker() -> Vec<Self> {
        vec![Self]
    }
    pub fn complete_menus(self) -> bool {
        false
    }
    pub fn dir(self) -> Dir {
        Dir::Ltr
    }
}

pub fn system_lang() -> Lang {
    Lang::EN
}
pub fn lang_from_tag(tag: &str) -> Option<Lang> {
    let primary = tag.split(['-', '_', '.', '@']).next().unwrap_or("");
    (primary.eq_ignore_ascii_case("en") || primary == "C" || primary == "POSIX").then_some(Lang::EN)
}
pub fn current() -> Lang {
    Lang::EN
}
pub fn set_current(_: Lang) {}
/// Temporarily draw in another language, restoring the previous one even on unwinding.
/// Preferences previews use this without changing the saved setting or other dialogs.
pub fn with_language<R>(lang: Lang, draw: impl FnOnce() -> R) -> R {
    let _restore = language_scope(lang);
    draw()
}

/// Keep the language active until this guard is dropped on the drawing thread.
#[must_use]
pub fn language_scope(lang: Lang) -> impl Drop {
    struct Restore {
        previous: Lang,
        _thread: std::marker::PhantomData<std::rc::Rc<()>>,
    }
    impl Drop for Restore {
        fn drop(&mut self) {
            set_current(self.previous);
        }
    }
    let restore = Restore { previous: current(), _thread: std::marker::PhantomData };
    set_current(lang);
    restore
}

pub fn sync_context(_: &egui::Context, _: &str) {}
pub fn has(_: Lang, _: &str) -> bool {
    false
}
pub fn t(s: &str) -> &str {
    s
}
pub fn tr(_: Lang, s: &str) -> &str {
    s
}
pub fn tr_ctx<'a>(_: Lang, _: &str, s: &'a str) -> &'a str {
    s
}
pub fn tr_id<'a>(_: Lang, _: &str, label: &'a str) -> &'a str {
    label
}
/// Fill `{name}` placeholders. Unknown placeholders are left as written.
pub fn fmt(template: &str, args: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (k, v) in args {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out
}

pub fn trn(_: Lang, n: u64, one: &str, other: &str) -> String {
    fmt(if n == 1 { one } else { other }, &[("n", &n.to_string())])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_english_is_registered_and_old_preferences_resolve_to_english() {
        assert_eq!(Lang::all().count(), 1);
        for code in ["auto", "hi", "te", "ja", "ru", "zh-hans", "unknown"] {
            assert_eq!(Lang::from_pref(code), Lang::EN);
            assert!(Lang::from_code(code).is_none());
        }
        assert_eq!(tr(Lang::EN, "Open…"), "Open…");
        assert_eq!(trn(Lang::EN, 2, "{n} layer", "{n} layers"), "2 layers");
    }
    #[test]
    fn indian_document_text_is_not_translated_or_filtered() {
        for text in ["తెలుగు", "हिन्दी", "বাংলা", "தமிழ்", "ಕನ್ನಡ", "മലയാളം", "ગુજરાતી", "ਪੰਜਾਬੀ", "ଓଡ଼ିଆ", "اردو"]
        {
            assert_eq!(t(text), text);
            assert_eq!(fmt("Text: {text}", &[("text", text)]), format!("Text: {text}"));
        }
        assert_eq!(dir_for_code("ur-IN"), Dir::Rtl);
        assert_eq!(dir_for_code("sd-IN"), Dir::Rtl);
        assert_eq!(Lang::EN.dir(), Dir::Ltr);
    }
}
