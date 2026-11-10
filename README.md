# Imagenor

Active execution list: [Competing with Photoshop](docs/competitive-plan.md) — start with build reliability, `.ignr` data safety and Indian-language editing.

Owner language scope (2026-10-25): English-only interface; no translation catalogs or language selectors. Prioritize Indian-language document text (Unicode input, fonts, shaping, editing and `.ignr` save/reopen), including Urdu/Sindhi direction. Inherited international-localization parity rows are historical and outside this fork’s scope.

<p align="center"><img src="assets/brand/Logo-Primary.svg" width="400" alt="Imagenor"></p>

Imagenor is a Rust-native image editor forked from [PhotoCraft](https://github.com/storytold/photocraft), starting at commit `14a4ab9`. It uses native egui/wgpu on the desktop and Rust/WebAssembly for the web.

Native documents use **`.ignr`**. The inherited layered archive schema is retained. Existing `.pcraft` archives can still be read by content detection; new native saves use `.ignr`.

## Status

Local fork established on 2026-10-25. Product names, Rust crates, app directories, packaging names, icons, Home/About branding and the desktop startup splash now use Imagenor. The supplied SVG originals are in `assets/brand`. About retains Contributors and Models and displays the actual build version.

The inherited project is alpha. Feature and parity estimates in [ROADMAP.md](ROADMAP.md) and [docs/target-app-parity.md](docs/target-app-parity.md) are upstream estimates; this branding change does not increase them. See [local rename notes](docs/imagenor-rebrand.md) for scope and verification.

The first Imagenor release is **0.1.0-alpha.1**: an experimental preview, with known test failures and incomplete print-production workflows. Source and release files are hosted at [appajid/imagenor](https://github.com/appajid/imagenor). Repository access is private unless the owner explicitly changes its visibility. See [release notes](docs/releases/0.1.0-alpha.1.md).

In-app update transport remains disabled. Upstream PhotoCraft binaries are not offered as Imagenor updates.

## Build and run

Use Rust stable and the platform dependencies described in [the development guide](docs/development.md).

```sh
cargo run -p imagenor
cargo run -p imagenor -- --control 7979
cargo run -p imagenor-cli -- --help
cargo xtask layers
cargo xtask wasm
```

Workspace design: [architecture](docs/architecture.md). Automation: [control protocol](docs/control-protocol.md). Agent instructions: [AGENTS.md](AGENTS.md).

## Licence and attribution

Source code retains the upstream [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE) licences.

Copyright (c) 2026 ArtCraft Team and the PhotoCraft contributors. Required notices are in [NOTICE](NOTICE).

Every retained or imported asset has provenance in [ATTRIBUTION.md](ATTRIBUTION.md). Owner-supplied Imagenor brand artwork is authorized for local integration; its authorship and redistribution licence have not separately been declared. See [the brand notice](assets/brand/LICENSE.txt). This does not change the source code licences. Upstream proprietary ArtCraft marks were removed; their original licence notice is preserved for provenance.

Adobe and Photoshop are trademarks or registered trademarks of Adobe Inc. Imagenor is independent and is not affiliated with, sponsored by or endorsed by Adobe Inc. These names describe compatible workflows only.
