# Imagenor rename — 2026-10-25

Baseline: PhotoCraft main, 14a4ab9. Product names, Rust crate identifiers, app directories, packaging names and app IDs now use Imagenor. Upstream URLs and copyright lines remain as provenance; no source was uploaded. Native documents now use `.ignr`; the underlying archive schema is unchanged. Older `.pcraft` archives retain their readable structure. Supplied SVG originals live in assets/brand; the About reference has illustrative version text, so the live About dialog uses the logo and actual build version instead. The source repository origin still identifies the upstream checkout.

Brand redistribution rights remain undeclared; local use only. Feature parity estimates remain upstream estimates and were not increased by this branding change.

The desktop release transport has no endpoint configured, so it never offers upstream PhotoCraft binaries as Imagenor updates.

The Home and About views omit inherited community/promotional controls, while About retains Contributors and Models. Help identifies the upstream source explicitly. Windows installer upgrade/component GUIDs are separate from PhotoCraft. The supplied document icon is used for macOS and Linux native file associations.

Upstream licence texts and NOTICE are preserved verbatim (NOTICE has an appended local-fork note). Proprietary ArtCraft mark files were removed and the README now describes the local fork rather than upstream download/community offerings.

## Verification (2026-10-25)

Passed: workspace library/binary build checks; Clippy for workspace libraries/binaries with warnings denied; formatting; layer rules; wasm (24 crates and HEIF-enabled codecs); 67 native-format tests; supplied artwork/splash and title-bar texture tests; source-link and narrow Home layout tests; five desktop icon tests; local-update endpoint test; `.ignr` export/reopen with retained layers, visibility and Telugu layer names. Home and About were rendered offscreen and visually inspected. Local evidence is in the ignored `plan/verification/` directory.

The broader `cargo check --workspace --all-targets` is not green: the inherited Unix FIFO plug-in test at `crates/plugins/tests/abi.rs:351` calls `rustix::fs::mknodat`, which rustix does not expose on macOS. The same call exists at upstream baseline `14a4ab9`; it was not changed by this task. Full CI is therefore not claimed. Changes remain local and uncommitted.

Update 2026-10-25 (COMP-1): the macOS FIFO test compile blocker described above is resolved. All workspace targets now pass `cargo check --workspace --all-targets`, and all 17 plug-in ABI tests pass. The Rust example fixture was rebuilt with the Imagenor manifest ID. Full CI status is tracked in docs/competitive-plan.md.
