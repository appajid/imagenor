# Competing with Photoshop: Imagenor execution list

Last reviewed/updated: 2026-10-25. Status: active local work. Owner: Appaji Ambarisha.

Success means reliable completion of real image-editing workflows, with strong Indian-language document editing and direct agent automation. Menu count and inherited parity percentages do not establish competitiveness. UI remains English-only. Use public specifications/documentation and original synthetic fixtures; never run or inspect Adobe software. No upload or publication without an explicit owner request.

## Ranked work and acceptance gates

| Rank / task | Work | Acceptance evidence | Current state |
|---|---|---|---|
| 1 / COMP-1 | Establish a trustworthy build/test baseline | Fix the macOS FIFO-test compile failure, rebuild renamed example fixtures, check all workspace targets; run required CI and record any remaining failures without hiding them | All targets compile; FIFO/ABI/native menu checks pass; parser stack overflow fixed; full CI blocked by additional test failures |
| 2 / COMP-2 | Protect native `.ignr` documents | Actual editable Indian-script text layers, styling, paragraph direction and transforms survive export/reopen/re-save; malformed/truncated inputs fail safely; atomic save, recovery and undo tests remain green | Two persistence regressions pass across 15 language samples; broader recovery/IME acceptance remains open |
| 3 / COMP-3 | Complete Indian-language typography | Script matrix for Devanagari, Bengali/Assamese, Gurmukhi, Gujarati, Odia, Tamil, Telugu, Kannada, Malayalam and Urdu/Sindhi; conjunct/matra shaping, IME composition, caret navigation, deletion, selection, copy/paste and save/reopen. Explicitly assess additional scripts such as Meetei Mayek and Ol Chiki; do not equate shared-script coverage with complete language support. Mark installed-font, synthetic, native-IME and web evidence separately | Font/shaping tests exist; interactive coverage incomplete |
| 4 / COMP-4 | Stabilize core editing workflows | Repeatable photo correction, cut-out, composite, painting and Indian-language poster workflows complete with undo/redo, cancellation and save/export; cover failed commands without losing document state | Audit and fix the first failing workflow |
| 5 / COMP-5 | Make PSD/PSB interchange trustworthy | Synthetic and permitted independently sourced fixtures preserve supported layers, masks, type, profiles and smart objects; explicit warnings for unsupported data; independently generated structure checks, no Adobe-runtime oracle | Known major gap |
| 6 / COMP-6 | Make interaction responsive | New idle-machine baseline for brush latency, pan/zoom, transforms, layer updates, large files and cancellation; meet documented per-scenario budgets and guard regressions. No Photoshop speed claim without comparable permitted evidence | Inherited measurements are stale/noisy |
| 7 / COMP-7 | Deepen painting, masking and retouching | Usable pressure/smoothing, clean mask edges, dependable healing/clone/content-aware operations, interactive transform/warp handles and predictable previews | Partial implementations exist |
| 8 / COMP-8 | Professional colour, raw and output | Test high-bit-depth/CMYK/Lab/ICC paths, soft proof, camera-raw support, ordinary export fidelity and print behavior; document each format's direction and limitations | Partial; prioritize common workflows |
| 9 / COMP-9 | Finish workspace, accessibility and hardware | Remove ineffective preferences; keyboard-accessible dialogs, useful error messages, panel persistence, scaling, tablet pressure/tilt and monitor handling verified on actual targets | Open |
| 10 / COMP-10 | Reliable automation and extension points | CLI/MCP/control workflows match GUI semantics, actions replay across documents, cancellation/errors are safe, filter plug-ins stay sandboxed; do not claim Photoshop plug-in compatibility | Strong foundation; incomplete coverage |
| 11 / COMP-11 | Produce installable, tested local builds | Local macOS app first, then actual Windows/Linux and browser validation; clean-machine opening/saving, `.ignr` associations, fonts and recovery. Publication is a separate owner-authorized step | Source/wasm checks exist; packaged validation incomplete |
| 12 / COMP-12 | Add useful optional AI after core reliability | Choose a specific user task, permitted model/licence, local-resource budget and reproducible quality/latency evaluation. Keep core editing functional without AI | Deferred; no provider/model selected |

## Immediate execution tranche

1. Fix the existing Unix FIFO regression test on macOS while retaining its anti-blocking assertion.
2. Rebuild the original Rust example plug-in so its generated fixture matches Imagenor's renamed source and manifest.
3. Add a real `.ignr` editable text-layer persistence matrix, beyond the existing Telugu **layer-name** test.
4. Run targeted regressions and all-target compilation, then the CI gate. Record actual results and next blockers.
5. Advance to IME/caret/selection acceptance after the baseline and persistence contract are established.

## Tracking rules

- A task is complete only when its acceptance evidence passes; preserve failures and skips in the record.
- Update this list, the relevant gap/checklist and local dev log when work lands. Do not inflate inherited overall readiness estimates.
- Commit locally only after required gates pass. Do not combine unrelated feature tasks into a single commit.
- Current evidence lives under ignored `plan/verification/`; references to inherited tests/estimates are labelled as inherited.

## Revision history

- 2026-10-25: created ranked competition plan and began COMP-1/COMP-2. This is an engineering execution list, not a claim of Photoshop equivalence.

## Execution evidence (2026-10-25)

- Fixed the macOS FIFO test without removing its timeout assertion; 17 plug-in ABI tests pass.
- Rebuilt `invert.wasm` from the renamed Rust example and corrected the native AppKit integration assertion for the English-only menu.
- Added actual editable text-layer `.ignr` persistence and edit/re-save tests across 15 Indian-language samples; both pass. Font shaping and IME fidelity are separate acceptance items.
- Full CI exposed a stack overflow in the Affinity tagged-object parser. Replaced recursive object traversal with bounded heap-backed continuations, retaining the existing 384-level limit. All six object encodings accept the boundary and reject excess nesting on a 128 KiB stack; shared class chains/self-references remain intact. 31 active Affinity tests pass, including mutation properties (one child-process helper is intentionally ignored by direct runs).
- Formatting, all-target Clippy, dependency layers and generated scorecard checks passed in the CI attempt. Full CI is not green; no commit or release claim.
- Corrected the renamed description in the project's own synthetic CC0 CMYK profile, preserving all eight other ICC tag payloads byte-for-byte; 33 active CMS integration tests pass (one ignored).
- Corrected the synthetic EXIF fixture's manufacturer length/offset after renaming; its targeted regression passes.
- Open COMP-1 blockers from the wider unoptimized diagnostic run: stack overflows in engine `same_pixels_ignores_the_selection_but_not_layers` and native-format integration tests, plus SVG `a_single_path_clip_becomes_the_groups_vector_mask`. The format failure's unnamed worker still needs isolation. These are observed failures, not yet attributed causes.
- The CMYK effects property test also overflowed; its test strategies are now boxed without changing generated cases. Its targeted 120-case property test passes. This is test-harness work, not evidence of a production renderer fix.

- Current WASM check passes for all 24 wasm-safe crates and the codecs HEIF feature.

## Colour and print priority (owner direction, 2026-10-25)

COMP-8 is now the active feature priority, alongside crash/data-safety blockers. The goal is natural, accurate display and predictable printing, measured against ICC specifications and independent implementations; no promise that arbitrary screens and printers match.

1. **Profile integrity and access:** bundle licensed, unchanged RGB and characterized CMYK profiles; preserve copyright, IDs and all tags; expose explicit assignment/conversion/working-space/proof choices. Never replace a document's existing profile merely because another profile is bundled.
2. **Display accuracy:** verify source → display conversion, near-black gradients, neutral grays, skin-tone patches, alpha and wide-gamut clipping at 8/16/32-bit. Follow the active monitor and surface changes; report sRGB fallback. OS calibration belongs to the monitor path, never the document profile.
3. **Print proofing:** use the actual printer/ink/paper ICC or the print shop's specified exchange condition; validate rendering intents, black-point compensation, paper-white/black simulation and gamut warning. Prevent double colour management in the print path. GRACoL/SWOP characterize specific reference conditions, not every Indian press or desktop printer.
4. **Filter colour fidelity:** audit blur, sharpening, denoise, curves, hue/saturation and colour balance for profile retention, high-bit-depth precision, neutrality, alpha fringes and CMYK black handling; make encoded-versus-linear processing explicit where relevant. Synthetic ramps and colour patches must have numerical assertions before aesthetic comparisons.
5. **Measured print validation:** create original patch charts and record calibrated-display conditions, printer/ink/paper, measurement instrument, reference values and colour differences. Physical print matching remains unverified until measured. No hardware purchase or print job has been authorized or initiated.

First evidence: a mandatory portable CMYK oracle compares 1,649 original ink patches against moxcms across sRGB/Display P3 and relative/perceptual/saturation intents. It passes the existing interpolation budget; maximum differences remain 16/12 eight-bit code values respectively, so this is implementation-agreement evidence, not proof of physical colour accuracy. New downloaded profiles pass parse/byte-retention and all-intent conversion/proof smoke tests; alpha is retained in 8/16/float buffers. 40 CMS unit tests and 35 integration tests pass (one benchmark ignored).

Official sources: https://registry.color.org/rgb-registry/srgbprofiles ; https://registry.color.org/profile-registry/GRACoL2013_CRPC6 ; https://registry.color.org/profile-registry/SWOP2013C3_CRPC5 ; https://www.color.org/displaycalibration/ .

Profile-embedding validation: engine offline resolver/info, New Document mode filtering and Assign Profile mode filtering pass; rendered RGB menu inspected. Strict CMS/engine/UI Clippy, formatting, dependency layers (31 crates), WASM (24 crates plus HEIF codecs) and the full adversarial-command sweep pass. Full workspace CI remains blocked by the previously recorded baseline failures. No commit/upload.

## Named spot colours and Pantone integration — 2026-10-25

Owner requested Pantone support. Official Pantone colour libraries/API embedding requires appropriate software-integration rights; a consumer subscription is not assumed to grant product redistribution rights. Source: https://www.pantone.com/license/software-and-web . No official Pantone colour tables, unofficial scraped copies or Adobe assets were downloaded or bundled.

Implemented the generic authorised-library path: existing ASE spot-swatch imports can feed `channel.newSpot` via `swatch` and optional `group`; the default channel name comes from the swatch. Alternate colours are retained as Lab D50 instead of clipping through an RGB hex preview. RGB/HSB/CMYK/gray alternates convert through the selected working-space ICC profiles. Unknown/non-spot/unevaluable book entries and conflicting explicit colours return errors without document edits. New Spot Channel offers a selector when the library contains spot swatches. `.ignr` retains the ink and name at 8/16/32-bit and creation remains undoable.

Open: official Pantone licence/API agreement and permitted delivery format, library provenance/version metadata, tint/overprint and measured print validation, and measured interchange with external consumers. Lab spot alternates now survive synthetic PSD/PSB export/import within the public format’s 0.01-unit encoding precision (2026-10-25). Individual unhalftoned spot-coverage TIFF plate export is now available (2026-10-31); combined process/spot print separation workflows remain unfinished. This is authorised ASE spot-library support, not certified/full Pantone integration. Tests use original synthetic inks only.

Spot-library validation: 3 engine and 11 channel-options dialog tests pass; strict engine/UI Clippy, layers, WASM24+HEIF, formatting and adversarial command sweep (62.98s) pass. Custom and selected-library-ink dialog states were rendered and visually inspected. Full workspace CI remains blocked by existing failures; no commit/upload.

## PSD/PSB Lab spot alternates — 2026-10-25

Fixed the next spot-colour interchange gap: write Lab spot inks as Lab DisplayInfo records and retain them as Lab on import, avoiding an RGB conversion. Other consumers of the shared plain-colour RGB decoder are unchanged. Signed a*/b* components follow the public colour-structure specification, with bounded values for malformed input and explicit handling of non-finite source components. Behaviour reference: https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/ (Color space IDs); no Adobe software, profiles, library colour data or application-generated fixtures used.

Evidence: independent exact-byte record regression (L*62.34, a*85.67, b*−79.12), signed/boundary/non-finite regression, and actual serialized PSD+PSB export/import twice across 24 configurations (RGB/CMYK documents × 8/16/32-bit × opaque/transparent × PSD/PSB). Ink names including Telugu, Lab alternates, solidity and coverage survive. This is a bounded improvement in spot metadata fidelity; it does not implement press separations, overprint or certify Pantone equivalence.

Owner confirmed authorised imports only, with no Pantone software/library licence; official Pantone tables remain unbundled.

Final PSD Lab spot validation (2026-10-25): 16 related unit +25 integration tests pass; strict IO Clippy, formatting/diff, layers31 and WASM24+HEIF pass. Fetched SHA-verified psd-tools corpus at 96eb134c17b2c65edf4c4151c0f00b802ada86c2 (309 files): existing gate passes with 249 reference matches, 56 differences, 4 skips, 0 errors/crashes; 308 export/re-import renders unchanged, 0 changed, 0 invalid tagged blocks. This retains the existing floor, not a readiness-score increase. Evidence: ignored plan/verification/lab-spot-results.json and logs. Full workspace CI retains earlier blockers; no commit/upload.

## Individual spot plate export — 2026-10-31

Added `file.exportSpotPlate` for an explicitly selected spot channel (numeric index or unique name). It returns a base64 16-bit grayscale TIFF, or atomically writes an explicit `.tif`/`.tiff` output in the native engine/CLI. Automation allows the byte-returning form and rejects ambient output paths. This is a CLI/control/MCP foundation; no new graphical export dialog is claimed.

Ink coverage maps to black-is-full-ink and white-is-no-ink. No display/working-space ICC transform is applied, no grayscale ICC profile is assigned, and alternate colour/preview solidity never scale the samples. The TIFF retains document DPI and ASCII JSON metadata containing the original Unicode ink name, alternate colour, preview solidity, polarity and clamped-sample count. Signed/out-of-range float coverage is clamped and reported; NaN/infinite coverage is refused. Duplicate names require an index. Raw samples are capped at 128 MiB; names at 64 KiB; dimensions/DPI are checked before allocation. Export does not mark the document saved or change undo history.

Example:

```sh
imagenor-cli run artwork.ignr --cmd file.exportSpotPlate --params '{"channel":"Brand Ink C","output":"brand-ink.tif"}'
```

Four engine tests cover 8/16/32-bit source precision/polarity, Unicode metadata roundtrip, independence from preview colour/solidity, invalid requests/resource limits, float clamping/NaN and safe native writes. Automation path-restriction regression passes. An original three-tone plate was generated through the CLI and visually checked; macOS sips independently reports 16-bit samples, 300×100 pixels. The sample's default source resolution is 72 dpi; separate tests verify 300 dpi retention.

The downstream RIP must consume the TIFF as raw single-ink coverage, not colour-managed grayscale artwork. `alternateColor` is identification/preview metadata in the native Color representation (Lab D50: L*=c[0]×100, a*=c[1]×255−128, b*=c[2]×255−128), not a physical ink characterization.

Remaining: process CMYK separations, all-plate packages, registration marks, trapping, halftoning, overprint and measured press/RIP validation. This command never spools a printer job and does not claim complete production separations or physical print accuracy. Official Pantone tables remain unbundled; authorised imports only.

Final plate-export checks: 4 engine tests, automation path-policy regression, strict engine/automation Clippy, layering31, WASM24+HEIF, formatting and full adversarial command sweep (69.66s) pass. Evidence and sample: ignored plan/verification/spot-plate-results.json and spot-plate-demo.tif. Full workspace CI is not claimed green; earlier blockers remain. No source uploaded.

## Reviewed upstream backports — 2026-11-11

Searched 95 distinct recent merged PR candidates across ICC, CMYK, filter, crash and print queries, then inspected eight relevant PRs' details (including the unmerged Average-memory follow-up). Selected three small merged fixes whose old behaviour is still present in the local baseline. Ported source and synthetic regressions into Imagenor's pre-reorganisation paths; retained its branding, native extension, local profiles and authorised-imports-only scope. No remote branch was merged wholesale and nothing was uploaded.

| PR | Local action | Evidence / limits |
|---|---|---|
| [#9076](https://github.com/storytold/photocraft/pull/9076) | Applied Average transparency correction | Mean weights selection coverage rather than alpha; hidden RGB contributes; each pixel's alpha stays intact. 8/16/32-bit, asymmetric selection and history regressions pass. Does not add Smart Filter support or bounded large-image processing. |
| [#12260](https://github.com/storytold/photocraft/pull/12260) | Applied Lab/8 Color Replacement correction | Color mode preserves stored lightness and changes a/b; guard tests retain other modes and Lab/16 behaviour. Upstream intentionally restricts this correction to Lab/8. |
| [#12381](https://github.com/storytold/photocraft/pull/12381) | Applied picked-CMYK eraser matching | Foreground protection/background-swatch sampling use valid original ink picks, quantized to stored sample precision. Stale picks/non-CMYK targets retain fallback. Adapted to the older local eraser without importing separate falloff rewrites. |
| [#9077](https://github.com/storytold/photocraft/pull/9077) | Deferred integration | Smart Average changes the shared algorithm/renderer; upstream identifies unresolved large-image memory findings until #9200 accompanies it. |
| [#9200](https://github.com/storytold/photocraft/pull/9200) | Deferred | Not merged at review; stacked algorithm and memory-admission changes need a separate evaluated integration. |
| [#10260](https://github.com/storytold/photocraft/pull/10260) | Queued for careful port | Flat PSD spot/alpha-channel recovery touches seven files and overlaps Imagenor's new Lab alternate preservation. |
| [#10276](https://github.com/storytold/photocraft/pull/10276) | Queued after #10260 | Spot overprint preview spans compositor/UI/cache/thumbnail paths; a 16-file integration requiring preservation of local profiles and plate export. |
| [#10463](https://github.com/storytold/photocraft/pull/10463) | Queued memory work | Seventeen-file Photomerge/algorithm/RAM-detection refactor; evaluate with its prerequisite architecture rather than applying individual fragments. |

Verified source heads and merge SHAs are recorded in ignored plan/verification/upstream-pr-review-2026-11-11.json. Current evidence: paint127 tests pass, Average10 and eraser10 engine tests pass (one unrelated eraser benchmark ignored). No inherited parity scores changed; broader baseline failures remain separately tracked.

Backport batch final checks: 147 paint/related engine tests pass, one benchmark ignored; strict Clippy, layers31, WASM24+HEIF, formatting/diff and adversarial command sweep (65.76s) pass. Full workspace CI has earlier baseline blockers and is not claimed green. Evidence: plan/verification/upstream-pr-review-2026-11-11.json and adjacent logs (local/ignored). No upload or commit.
