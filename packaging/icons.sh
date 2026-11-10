#!/usr/bin/env bash
# Regenerate every app icon from assets/app-icon/imagenor.svg (the canonical master).
#
# Needs: resvg (brew install resvg / cargo install resvg). On macOS, iconutil also writes the
# .icns. The outputs are committed, so packaging never needs these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
SVG="$DIR/imagenor.svg"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

command -v resvg >/dev/null || { echo "error: resvg not found (brew install resvg)" >&2; exit 1; }

# The supplied Imagenor tile is 512 units square. macOS gets transparent padding;
# Windows and Linux retain the complete tile.
grep -q 'viewBox="0 0 512 512"' "$SVG" || { echo "error: expected viewBox=\"0 0 512 512\" in $SVG" >&2; exit 1; }
MAC="$TMP/mac.svg"
sed 's/viewBox="0 0 512 512"/viewBox="-62 -62 636 636"/' "$SVG" >"$MAC"
TIGHT="$TMP/tight.svg"
sed 's/viewBox="0 0 512 512"/viewBox="0 0 512 512"/' "$SVG" >"$TIGHT"

render() { resvg -w "$2" -h "$2" "$1" "$3" </dev/null; }

render "$MAC" 1024 "$DIR/imagenor-1024.png"

# Linux hicolor theme.
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  render "$TIGHT" "$s" "$DIR/hicolor/${s}x${s}/apps/ai.storyteller.imagenor.png"
done
mkdir -p "$DIR/hicolor/scalable/apps"
# The lighter trace (imagenor-small.svg) keeps the scalable theme icon cheap to render.
cp "$DIR/imagenor-small.svg" "$DIR/hicolor/scalable/apps/ai.storyteller.imagenor.svg"

# Windows .ico.
ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  render "$TIGHT" "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/imagenor.ico" "${ICO_PNGS[@]}")

# macOS .icns.
if command -v iconutil >/dev/null; then
  SET="$TMP/imagenor.iconset"
  mkdir -p "$SET"
  for s in 16 32 128 256 512; do
    render "$MAC" "$s" "$SET/icon_${s}x${s}.png"
    render "$MAC" $((s * 2)) "$SET/icon_${s}x${s}@2x.png"
  done
  iconutil -c icns -o "$DIR/imagenor.icns" "$SET"
else
  echo "warning: iconutil not found (macOS only); imagenor.icns not regenerated" >&2
fi
# Native document artwork gets its own file-association icon.
DOC="$TMP/document.svg"
sed 's/viewBox="0 0 440 560"/viewBox="-85 -25 610 610"/' "$ROOT/assets/brand/Document-Icon.svg" >"$DOC"
render "$DOC" 1024 "$DIR/imagenor-document-1024.png"
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/mimetypes"
  render "$DOC" "$s" "$DIR/hicolor/${s}x${s}/mimetypes/application-x-imagenor.png"
done
if command -v iconutil >/dev/null; then
  SET="$TMP/imagenor-document.iconset"
  mkdir -p "$SET"
  for s in 16 32 128 256 512; do
    render "$DOC" "$s" "$SET/icon_${s}x${s}.png"
    render "$DOC" $((s * 2)) "$SET/icon_${s}x${s}@2x.png"
  done
  iconutil -c icns -o "$DIR/imagenor-document.icns" "$SET"
fi
echo "icons written to $DIR"
