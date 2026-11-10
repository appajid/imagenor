# Imagenor language scope

Last reviewed and updated: 2026-10-25. Owner decision: English-only UI; Indian-language document support.

The interface uses English only. UI translation catalogs, automatic OS language selection and language selectors are removed. Older saved UI locale preferences resolve to English. No international UI localization work is planned.

Document text remains Unicode. Indian-language priorities include Devanagari (Hindi, Marathi, Sanskrit, Nepali, Konkani and others), Bengali/Assamese, Gurmukhi, Gujarati, Odia, Tamil, Telugu, Kannada, Malayalam, Urdu and Sindhi. Languages share scripts; script detection alone does not establish complete language support. Font availability, shaping, keyboard/IME input, caret/selection behavior and layered `.ignr` save/reopen must be verified separately. Preserve RTL shaping and editing where required. Do not filter existing document text by language.

No font files are added to this repository. Use installed fonts or the optional CRAFT_FONTS_DIR input. Current evidence includes the `.ignr` layer-name round trip with Telugu text; this does not prove complete Indic shaping or IME fidelity. Those remain validation work.

## Revision history

- 2026-10-25: replaced inherited international UI-localization goals with the owner’s English-interface/Indian-document-text scope. No readiness score increased.

Verification (2026-10-25): English fallback, removed Preferences/native-menu selectors, menu structure and Preferences search tests pass. Indian-script classification and installed-font Indic shaping/fallback tests pass, including Odia; no font-unavailable skips were reported. Workspace library/binary Clippy with warnings denied, formatting and wasm checks pass. Preferences was rendered offscreen and visually inspected. Full IME/caret fidelity across all Indian languages is still unmeasured. Local evidence: plan/verification/language-results.json and english-preferences.png. No upload or commit was made.

COMP-2 evidence, 2026-10-25: `crates/io/tests/ignr_text.rs` adds actual editable text-layer tests for 15 samples, including Urdu/Sindhi RTL paragraph metadata, character language/style runs, UTF-8 run lengths, box geometry, transforms and re-saving edits. Both tests pass. These are font-independent data-persistence checks; they do not prove IME or cursor fidelity.
