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
PREVIEW_DIR="$ROOT_DIR/target/native-preview"
PREVIEW_IMAGE=""
PREVIEW_ARTIFACT="$ROOT_DIR/artifacts/screenshots/fe-reader-native-preview.png"
FIXTURE_PATH=""
UPDATE_ARTIFACT=0
SKIP_CLI=0

usage() {
  cat <<'EOF'
Usage: script/build_and_run.sh [--verify] [--fixture path] [--update-artifact] [--skip-cli]

Builds and launches the unsigned local macOS preview app.
EOF
}

VERIFY=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --help)
      usage
      exit 0
      ;;
    --verify)
      VERIFY=1
      shift
      ;;
    --fixture)
      if [[ $# -lt 2 ]]; then
        usage >&2
        exit 2
      fi
      FIXTURE_PATH="$2"
      shift 2
      ;;
    --update-artifact)
      UPDATE_ARTIFACT=1
      shift
      ;;
    --skip-cli)
      SKIP_CLI=1
      shift
      ;;
    *)
      usage >&2
      exit 2
      ;;
  esac
done

if [[ ! -f "$SOURCE_FILE" ]]; then
  echo "Missing macOS source file: $SOURCE_FILE" >&2
  exit 1
fi

mkdir -p "$APP_BUNDLE/Contents/MacOS" "$APP_BUNDLE/Contents/Resources" "$PREVIEW_DIR"

if [[ "$SKIP_CLI" -eq 0 ]]; then
  cargo build -q -p fe_reader_cli
  cp "$ROOT_DIR/target/debug/fe-reader" "$APP_BUNDLE/Contents/Resources/fe-reader"
  chmod +x "$APP_BUNDLE/Contents/Resources/fe-reader"
fi

if pgrep -x "$APP_NAME" >/dev/null 2>&1; then
  pkill -x "$APP_NAME" || true
  sleep 1
fi

if [[ -n "$FIXTURE_PATH" && ! -f "$FIXTURE_PATH" ]]; then
  echo "Missing verification fixture: $FIXTURE_PATH" >&2
  exit 1
fi

if [[ "$VERIFY" -eq 1 ]]; then
  if [[ -n "$FIXTURE_PATH" ]]; then
    PREVIEW_IMAGE="$PREVIEW_DIR/fixture-open.png"
  else
    PREVIEW_IMAGE="$PREVIEW_DIR/empty.png"
  fi
  rm -f "$PREVIEW_MARKER" "$PREVIEW_IMAGE"
  touch "$PREVIEW_MARKER"
fi

swiftc_args=(
  -parse-as-library
  -framework AppKit
  -framework CryptoKit
  -framework Foundation
  -framework PDFKit
  -o "$EXECUTABLE_PATH"
  "$SOURCE_FILE"
)
if [[ "$VERIFY" -eq 0 ]]; then
  swiftc_args=(-O -whole-module-optimization "${swiftc_args[@]}")
fi
swiftc "${swiftc_args[@]}"

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
app_args=()
if [[ "$VERIFY" -eq 1 ]]; then
  app_args+=(--verify-output "$PREVIEW_IMAGE")
  if [[ -n "$FIXTURE_PATH" ]]; then
    app_args+=(--verify-fixture "$FIXTURE_PATH")
  fi
fi

if [[ "$VERIFY" -eq 1 ]]; then
  "$EXECUTABLE_PATH" "${app_args[@]}" &
else
  open -n "$APP_BUNDLE" --args "${app_args[@]}"
fi

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

  preview_mode="empty"
  if [[ -n "$FIXTURE_PATH" ]]; then
    preview_mode="open"
  fi
  python3 "$ROOT_DIR/scripts/native_preview_check.py" "$PREVIEW_IMAGE" --mode "$preview_mode"
  if [[ "$UPDATE_ARTIFACT" -eq 1 ]]; then
    mkdir -p "$(dirname "$PREVIEW_ARTIFACT")"
    cp "$PREVIEW_IMAGE" "$PREVIEW_ARTIFACT"
  fi
fi
