# Releasing Imagenor

Imagenor's first repository/release publication was requested by the owner on 2026-11-11. The repository is https://github.com/appajid/imagenor and was made public at the owner's explicit request on 2026-11-11. The first release is `v0.1.0-alpha.1`, explicitly marked pre-release.

Use the release's notes in `docs/releases/` for validated platforms, signing state and limitations. Full workspace CI is not currently claimed green. Inherited upstream workflow templates moved to `.github/upstream-workflows/`; they are references and must not be enabled unchanged against the upstream organisation's private/reusable infrastructure. The manual quality workflow runs the complete gate without concealing its known failures.

## Local package

```sh
cargo xtask version set 0.1.0-alpha.1
bash packaging/macos/package.sh --arch aarch64
```

Without an Apple Developer identity, macOS packages are ad-hoc signed and not notarized. Record that in the release notes. Windows/Linux/Intel/web artifacts require their own builds and validation; do not rename a macOS artifact for another platform.

Create a draft GitHub pre-release, attach only validated files plus SHA256SUMS.txt, verify uploaded checksums and the exact source tag, then publish it. Every upload/publish needs an owner request; the initial repository and first release were explicitly requested. Preserve all source and third-party licence notices. In-app update checks remain disabled until an appropriate release policy/endpoint is separately configured. Official Pantone libraries remain unbundled.
