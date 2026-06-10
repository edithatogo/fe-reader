# Platform Test Workflows

Fe Reader keeps platform code behind adapter and contract boundaries. The local
and GitHub Actions platform tests therefore validate the current runnable
surfaces: Rust workspace behavior, platform automation contracts, mobile binding
contracts, and target-specific compilation.

## Local Linux Container

Run:

```bash
bash scripts/linux_container_smoke.sh
```

The script builds `ci/linux-rust/Dockerfile` as
`fe-reader-linux-rust:1.95.0`, mounts the checkout read/write, uses temporary
Cargo directories inside the container, and runs the Linux smoke subset without
leaving root-owned build artifacts in the workspace.

## Local Android Emulator

Run:

```bash
bash scripts/android_emulator_smoke.sh
```

The script uses a running `adb` device if one is already attached. If no device
is attached, it creates and boots an AVD when these Android SDK tools are
available:

- `adb`
- `sdkmanager`
- `avdmanager`
- `emulator`

On Apple Silicon the default local system image is
`system-images;android-35;google_apis;arm64-v8a`. On Intel hosts the default is
`system-images;android-35;google_apis;x86_64`.

## GitHub Actions

`.github/workflows/09-platform-tests.yml` runs:

- Linux container smoke through `scripts/linux_container_smoke.sh`.
- macOS native workspace tests and x86_64 macOS CLI compilation.
- Windows Rust smoke tests.
- Android emulator boot plus mobile contract and Android target compilation.
- iOS simulator target compilation for the UniFFI mobile boundary.
