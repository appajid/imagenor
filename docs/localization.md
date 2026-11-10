# English interface and Indian-language document text

Last reviewed/updated: 2026-10-25. Owner scope decision.

Imagenor’s menus, dialogs, tooltips, errors and automation labels use English. No UI translation catalog, OS-language negotiation or language picker is shipped. Legacy locale preferences are accepted for reading older settings and resolve to English.

Indian-language support concerns document content: Unicode keyboard/IME input, font discovery, script shaping, conjuncts and matras, caret movement, selection and `.ignr` save/reopen. Preserve Urdu/Sindhi RTL behavior independently of the left-to-right interface. Never delete or transliterate a user’s text because its language is outside the current product focus.

Use installed fonts and the existing optional CRAFT_FONTS_DIR integration; do not add font binaries to the repository. Odia fallback families join the existing Indic fallback list. Tests that rely on installed fonts must report unavailable fonts rather than claim shaping coverage they did not exercise.

[Language scope and remaining verification](localization-parity.md) distinguishes script classification, font fallback, actual shaping, IME behavior and native persistence. Full Indian-language readiness has not been measured. International UI localization is outside scope.

## Revision history

- 2026-10-25: removed inherited catalog/live-switching instructions and recorded the owner’s language scope.
