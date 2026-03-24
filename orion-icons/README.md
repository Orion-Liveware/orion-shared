# Orion Icons

Shared icon system for all Orion ecosystem apps. Each app uses the same Orion constellation base with a unique initials badge.

## Files

| File | Purpose |
|------|---------|
| `orion-base.svg` | Base icon — Orion constellation only, no badge |
| `orion-mol.svg` | My Orion Life (MOL, blue badge `#4a6a9a`) |
| `orion-oc.svg` | Orion Coding (OC, green badge `#5a7a6a`) |
| `orion-vd.svg` | Video Docs (VD, rose badge `#7a5a6a`) |
| `orion-s.svg` | Scraper (S, olive badge `#6a6a5a`) |
| `generate-app-icon.sh` | Script to generate platform icons from SVGs |

## Generating Platform Icons for an App

```bash
# Install librsvg (one-time)
brew install librsvg

# Generate all platform icons (ico, icns, png variants)
# from the app's SVG into its icons directory:
rsvg-convert -w 1024 -h 1024 orion-oc.svg -o /tmp/icon.png
cd /path/to/app
cargo tauri icon /tmp/icon.png --output crates/tauri-app/icons
```

Or use the helper script:
```bash
./generate-app-icon.sh "OC" ../../orion-coding/crates/tauri-app/icons "#5a7a6a"
```

## Adding a New App

1. Pick 1-3 letter initials and a muted badge color
2. Create the SVG variant:
   ```bash
   # The script injects a badge into orion-base.svg
   ./generate-app-icon.sh "XX" /path/to/app/icons "#color"
   ```
3. Or manually: copy `orion-base.svg`, add the badge group before `</svg>`
4. Add `bundle.icon` paths to the app's `tauri.conf.json`:
   ```json
   "bundle": {
     "icon": [
       "icons/32x32.png",
       "icons/128x128.png",
       "icons/128x128@2x.png",
       "icons/icon.icns",
       "icons/icon.ico"
     ]
   }
   ```

## Design

- **Background:** Deep space gradient (`#1a1a2e` → `#0d0d1a`), rounded corners (180px at 1024)
- **Stars:** 7 main Orion stars with accurate relative positions and colors
  - Betelgeuse (top-left, warm orange) — largest
  - Bellatrix (top-right, blue-white)
  - Belt: Alnitak, Alnilam, Mintaka (center, blue-white)
  - Saiph (bottom-left, pale blue)
  - Rigel (bottom-right, bright blue) — second largest
- **Effects:** Per-star glow filters, faint constellation lines, subtle nebula in belt area, scattered background stars
- **Badge:** Bottom-right oval with app initials, semi-transparent white border, drop shadow
