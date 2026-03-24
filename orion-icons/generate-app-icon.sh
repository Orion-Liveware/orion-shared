#!/bin/bash
# Generate an app-specific Orion icon with initials badge.
#
# Usage:
#   ./generate-app-icon.sh <initials> <output-dir> [badge-color]
#
# Examples:
#   ./generate-app-icon.sh "MOL" ../my-orion-life/crates/tauri-app/icons
#   ./generate-app-icon.sh "OC"  ../orion-coding/crates/tauri-app/icons
#   ./generate-app-icon.sh "VD"  ../video-docs/crates/tauri-app/icons
#   ./generate-app-icon.sh "S"   ../scraper/src-tauri/icons "#6a8a5a"
#
# Requires: rsvg-convert (librsvg) or sips (macOS built-in)

set -euo pipefail

INITIALS="${1:?Usage: $0 <initials> <output-dir> [badge-color]}"
OUTPUT_DIR="${2:?Usage: $0 <initials> <output-dir> [badge-color]}"
BADGE_COLOR="${3:-#4a6a9a}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BASE_SVG="$SCRIPT_DIR/orion-base.svg"
TEMP_SVG="$(mktemp /tmp/orion-icon-XXXXXX.svg)"
TEMP_PNG="$(mktemp /tmp/orion-icon-XXXXXX.png)"

# Font size scales with number of characters
CHAR_COUNT=${#INITIALS}
if [ "$CHAR_COUNT" -le 1 ]; then
  FONT_SIZE="90"
  BADGE_RX="70"
  BADGE_RY="70"
elif [ "$CHAR_COUNT" -le 2 ]; then
  FONT_SIZE="72"
  BADGE_RX="85"
  BADGE_RY="65"
else
  FONT_SIZE="56"
  BADGE_RX="95"
  BADGE_RY="60"
fi

# Build the badge SVG snippet (bottom-right corner)
BADGE_SVG="
  <!-- App initials badge -->
  <g>
    <!-- Badge shadow -->
    <ellipse cx=\"802\" cy=\"842\" rx=\"${BADGE_RX}\" ry=\"${BADGE_RY}\" fill=\"#000000\" opacity=\"0.3\"/>
    <!-- Badge background -->
    <ellipse cx=\"800\" cy=\"840\" rx=\"${BADGE_RX}\" ry=\"${BADGE_RY}\" fill=\"${BADGE_COLOR}\"/>
    <!-- Badge border -->
    <ellipse cx=\"800\" cy=\"840\" rx=\"${BADGE_RX}\" ry=\"${BADGE_RY}\" fill=\"none\" stroke=\"#ffffff\" stroke-width=\"3\" opacity=\"0.3\"/>
    <!-- Initials text -->
    <text x=\"800\" y=\"840\" text-anchor=\"middle\" dominant-baseline=\"central\"
      font-family=\"-apple-system, 'SF Pro Rounded', 'Helvetica Neue', Arial, sans-serif\"
      font-size=\"${FONT_SIZE}\" font-weight=\"700\" fill=\"#ffffff\" letter-spacing=\"2\">
      ${INITIALS}
    </text>
  </g>"

# Inject badge before closing </svg> tag
sed "s|</svg>|${BADGE_SVG}\n</svg>|" "$BASE_SVG" > "$TEMP_SVG"

# Convert SVG to 1024x1024 PNG
if command -v rsvg-convert &>/dev/null; then
  rsvg-convert -w 1024 -h 1024 "$TEMP_SVG" -o "$TEMP_PNG"
elif command -v /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome &>/dev/null; then
  # Fallback: use sips on macOS (limited SVG support, may not render filters)
  echo "Warning: rsvg-convert not found, falling back to sips (filters may not render)"
  sips -s format png -z 1024 1024 "$TEMP_SVG" --out "$TEMP_PNG" 2>/dev/null || {
    echo "Error: Cannot convert SVG. Install librsvg: brew install librsvg"
    exit 1
  }
else
  echo "Error: No SVG converter found. Install librsvg: brew install librsvg"
  rm -f "$TEMP_SVG" "$TEMP_PNG"
  exit 1
fi

# Generate platform icons using cargo tauri icon
mkdir -p "$OUTPUT_DIR"
cargo tauri icon "$TEMP_PNG" --output "$OUTPUT_DIR" 2>&1

# Clean up
rm -f "$TEMP_SVG" "$TEMP_PNG"

echo ""
echo "Icons generated in $OUTPUT_DIR for app '$INITIALS'"
echo "Files: icon.ico, icon.icns, icon.png, and all size variants"
