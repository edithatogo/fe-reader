#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_NAME="FeReader"
DISPLAY_NAME="Fe Reader"
SOURCE_FILE="$ROOT_DIR/native/macos/FeReaderNativeApp.swift"
DIST_DIR="$ROOT_DIR/dist"
APP_BUNDLE="$DIST_DIR/${APP_NAME}.app"
EXECUTABLE_PATH="$APP_BUNDLE/Contents/MacOS/$APP_NAME"
PLIST_PATH="$APP_BUNDLE/Contents/Info.plist"
PREVIEW_MARKER="/tmp/fe-reader-capture-preview"
PREVIEW_IMAGE="/tmp/fe-reader-native-preview.png"
PREVIEW_ARTIFACT="$ROOT_DIR/artifacts/screenshots/fe-reader-native-preview.png"

usage() {
  cat <<'EOF'
Usage: script/build_and_run.sh [--verify]

Builds and launches the unsigned local macOS preview app.
EOF
}

VERIFY=0
if [[ "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi
if [[ "${1:-}" == "--verify" ]]; then
  VERIFY=1
  shift
fi
if [[ $# -gt 0 ]]; then
  usage >&2
  exit 2
fi

if [[ ! -f "$SOURCE_FILE" ]]; then
  echo "Missing macOS source file: $SOURCE_FILE" >&2
  exit 1
fi

mkdir -p "$APP_BUNDLE/Contents/MacOS" "$APP_BUNDLE/Contents/Resources"

if pgrep -x "$APP_NAME" >/dev/null 2>&1; then
  pkill -x "$APP_NAME" || true
  sleep 1
fi

rm -f "$PREVIEW_MARKER" "$PREVIEW_IMAGE"
touch "$PREVIEW_MARKER"

swiftc \
  -O \
  -parse-as-library \
  -whole-module-optimization \
  -framework AppKit \
  -framework CryptoKit \
  -framework Foundation \
  -framework PDFKit \
  -o "$EXECUTABLE_PATH" \
  "$SOURCE_FILE"

cat >"$PLIST_PATH" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleExecutable</key>
  <string>$APP_NAME</string>
  <key>CFBundleIdentifier</key>
  <string>com.edithatogo.fe-reader.native</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>$DISPLAY_NAME</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0-preview.1</string>
  <key>CFBundleVersion</key>
  <string>1</string>
  <key>LSMinimumSystemVersion</key>
  <string>13.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>NSPrincipalClass</key>
  <string>NSApplication</string>
</dict>
</plist>
EOF

chmod +x "$EXECUTABLE_PATH"
open -n "$APP_BUNDLE"

if [[ "$VERIFY" -eq 1 ]]; then
  for _ in {1..10}; do
    if pgrep -x "$APP_NAME" >/dev/null 2>&1; then
      break
    fi
    sleep 1
  done

  if [[ ! -f "$PREVIEW_IMAGE" ]]; then
    for _ in {1..20}; do
      if [[ -f "$PREVIEW_IMAGE" ]]; then
        break
      fi
      sleep 1
    done
  fi

  if [[ ! -f "$PREVIEW_IMAGE" ]]; then
    echo "Preview snapshot was not written: $PREVIEW_IMAGE" >&2
    exit 1
  fi

  python3 "$ROOT_DIR/scripts/native_preview_check.py" "$PREVIEW_IMAGE"
  mkdir -p "$(dirname "$PREVIEW_ARTIFACT")"
  cp "$PREVIEW_IMAGE" "$PREVIEW_ARTIFACT"
fi
