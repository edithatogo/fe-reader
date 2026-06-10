#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
API="${FE_ANDROID_API:-35}"
TARGET="${FE_ANDROID_TARGET:-aarch64-linux-android}"
AVD_NAME="${FE_ANDROID_AVD_NAME:-fe-reader-api-${API}}"
TOOLCHAIN="${FE_RUST_TOOLCHAIN:-}"

find_android_tool() {
  local name="$1"
  if command -v "${name}" >/dev/null 2>&1; then
    command -v "${name}"
    return 0
  fi
  for base in "${ANDROID_HOME:-}" "${ANDROID_SDK_ROOT:-}" "$HOME/Library/Android/sdk" "$HOME/Android/Sdk"; do
    [[ -n "${base}" ]] || continue
    for candidate in \
      "${base}/platform-tools/${name}" \
      "${base}/emulator/${name}" \
      "${base}/cmdline-tools/latest/bin/${name}" \
      "${base}/cmdline-tools/bin/${name}"; do
      if [[ -x "${candidate}" ]]; then
        printf '%s\n' "${candidate}"
        return 0
      fi
    done
  done
  return 1
}

ADB="$(find_android_tool adb || true)"
if [[ -z "${ADB}" ]]; then
  echo "adb is required for the Android emulator smoke test" >&2
  exit 1
fi

if [[ -n "${TOOLCHAIN}" ]]; then
  RUSTC_BIN="$(rustup which rustc --toolchain "${TOOLCHAIN}" 2>/dev/null || true)"
  if [[ -n "${RUSTC_BIN}" ]]; then
    export RUSTC="${RUSTC_BIN}"
  fi
fi

has_booted_device() {
  "${ADB}" devices | awk 'NR > 1 && $2 == "device" { found=1 } END { exit(found ? 0 : 1) }'
}

wait_for_boot() {
  "${ADB}" wait-for-device
  local booted=""
  for _ in $(seq 1 120); do
    booted="$("${ADB}" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r' || true)"
    if [[ "${booted}" == "1" ]]; then
      return 0
    fi
    sleep 2
  done
  echo "Android emulator did not finish booting" >&2
  exit 1
}

EMULATOR_PID=""
cleanup() {
  if [[ -n "${EMULATOR_PID}" ]]; then
    kill "${EMULATOR_PID}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

if ! has_booted_device; then
  if [[ "${FE_ANDROID_EMULATOR_MANAGED:-0}" == "1" ]]; then
    wait_for_boot
  else
    SDKMANAGER="$(find_android_tool sdkmanager || true)"
    AVDMANAGER="$(find_android_tool avdmanager || true)"
    EMULATOR="$(find_android_tool emulator || true)"
    if [[ -z "${SDKMANAGER}" || -z "${AVDMANAGER}" || -z "${EMULATOR}" ]]; then
      echo "No running Android device and sdkmanager/avdmanager/emulator were not all found" >&2
      exit 1
    fi

    case "$(uname -m)" in
      arm64|aarch64) SYSTEM_IMAGE="system-images;android-${API};google_apis;arm64-v8a" ;;
      *) SYSTEM_IMAGE="system-images;android-${API};google_apis;x86_64" ;;
    esac

    yes | "${SDKMANAGER}" "platform-tools" "platforms;android-${API}" "emulator" "${SYSTEM_IMAGE}" >/dev/null
    if ! "${EMULATOR}" -list-avds | grep -Fxq "${AVD_NAME}"; then
      printf 'no\n' | "${AVDMANAGER}" create avd --force --name "${AVD_NAME}" --package "${SYSTEM_IMAGE}" >/dev/null
    fi
    "${EMULATOR}" -avd "${AVD_NAME}" -no-window -no-audio -no-boot-anim -gpu swiftshader_indirect >/tmp/fe-reader-android-emulator.log 2>&1 &
    EMULATOR_PID="$!"
    wait_for_boot
  fi
fi

"${ADB}" shell getprop ro.build.version.sdk | tr -d '\r'
"${ADB}" shell getprop ro.product.cpu.abi | tr -d '\r'

cd "${ROOT}"
python3 scripts/mobile_smoke_bindings_check.py
rustup target add "${TARGET}" >/dev/null
cargo check -p fe_reader_uniffi --target "${TARGET}"
