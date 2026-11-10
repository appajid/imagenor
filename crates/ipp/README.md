# imagenor-ipp

A small IPP (Internet Printing Protocol, RFC 8010/8011) client for the local CUPS server, in pure
Rust with no dependencies. It does in-process what `lp` does from the command line, so Imagenor
can print where there is no `lp` binary (a Flatpak sandbox with `--socket=cups`, a minimal
container) and can read the printer list and the default printer first instead of handing
CUPS's errors to the user.

```rust
let ep = imagenor_ipp::Endpoint::detect().ok_or("no CUPS server")?;   // socket, then localhost:631
let printers = imagenor_ipp::printers(&ep)?;                            // name, info, state, default
let id = imagenor_ipp::print_job(&ep, &imagenor_ipp::Job { printer: "Office", title: "photo", user: "me", copies: 2, document: &pdf, format: "application/pdf" })?;
```

Three operations: `CUPS-Get-Printers`, `CUPS-Get-Default`, `Print-Job`. HTTP/1.1 over the CUPS
domain socket (`/run/cups/cups.sock`, or `CUPS_SERVER` as a path) or TCP (`localhost:631`, or
`CUPS_SERVER` as `host[:port]`), chunked replies included. Replies are capped at 16 MiB and
parsed with bounds checks; an IPP error status becomes `Error::Status` with the server's own
message. Tests run against an in-process fake server over TCP and a Unix socket.

Standalone: it depends on no other Imagenor crate. Used by `imagenor-engine`'s print
commands on Unix; Windows printing lives in `imagenor-winprint`.
