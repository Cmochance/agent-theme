#!/bin/bash
# Image-processing template for agent-theme themes.
#   build-theme.sh <source-image> <theme-id>
# Produces themes/<theme-id>/{bg.jpg, preview.jpg} from a source image:
#   - bg.jpg:      centre-cropped to square, 2048², JPEG q88 (the hero; theme.json
#                  backgroundPosition/Fit control framing in Codex's landscape viewport).
#   - preview.jpg: 640×400 landscape crop from the upper-centre (≈ what cover+top shows),
#                  JPEG q85 — the theme-picker thumbnail.
# Also prints a derived palette (darkest / most-saturated tones) to seed theme.json.
set -eu
SRC="${1:?usage: build-theme.sh <source-image> <theme-id>}"
ID="${2:?usage: build-theme.sh <source-image> <theme-id>}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/themes/$ID"
mkdir -p "$OUT"

[ -f "$SRC" ] || { echo "no such image: $SRC" >&2; exit 1; }

# hero: square centre-crop -> 2048², q88
magick "$SRC" -auto-orient \
  -resize 2048x2048^ -gravity center -extent 2048x2048 \
  -strip -quality 88 "$OUT/bg.jpg"

# preview: 16:10 landscape crop from the upper portion -> 640×400, q85
magick "$SRC" -auto-orient \
  -resize 640x400^ -gravity north -extent 640x400 \
  -strip -quality 85 "$OUT/preview.jpg"

echo "WROTE $OUT/bg.jpg ($(identify -format '%wx%h %b' "$OUT/bg.jpg"))"
echo "WROTE $OUT/preview.jpg ($(identify -format '%wx%h %b' "$OUT/preview.jpg"))"

# ---- derived palette (seed for theme.json style; review + fine-tune per theme) ----
echo "PALETTE (darkest 3 / brightest 3 prominent tones):"
magick "$SRC" -resize 200x200 -colors 8 -depth 8 -format "%c" histogram:info: 2>/dev/null \
  | sed -E 's/^ *([0-9]+):.*(#[0-9A-Fa-f]{6}).*/\1 \2/' | sort -rn | head -8
echo "most-saturated accent candidate:"
magick "$SRC" -resize 150x150 -colorspace HSL -channel G -separate +channel "$SRC" \
  -colorspace HSL null: 2>/dev/null || true
magick "$SRC" -resize 120x120 -colors 16 -unique-colors txt: 2>/dev/null \
  | grep -oE 'srgb\([^)]*\) *#[0-9A-Fa-f]{6}' | head -16 || \
  magick "$SRC" -resize 120x120 -colors 16 -unique-colors txt: 2>/dev/null | grep -oE '#[0-9A-Fa-f]{6}' | tr '\n' ' '
echo
