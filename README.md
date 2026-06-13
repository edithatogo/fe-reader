# Fe Reader

Fe Reader is a local-first PDF workflow platform focused on privacy,
verification, metadata transparency, automation safety and cross-platform native
integration.

The repository is currently a preview implementation checkpoint. It contains a
headless Rust core, executable Wave 0 contracts, platform automation contracts,
release-evidence tooling, package manifests and an Astro/Starlight documentation
site. It is not yet a production PDF application or a published app-store
package.

## Status

- Current release line: `0.1.0-preview.1`
- Rust crate API version: `0.1.0`
- Documentation: <https://edithatogo.github.io/fe-reader/>
- Repository: <https://github.com/edithatogo/fe-reader>
- Releases and package status: <https://github.com/edithatogo/fe-reader/releases>
- License: `Apache-2.0 OR MIT`

## Architecture

Fe Reader keeps the pure document/workflow core separate from UI, platform,
renderer, plugin, AI and automation adapters.

```text
fe_reader_core
  pure document/workflow core
  no UI, no platform, no renderer, no AI, no MCP, no plugin runtime

adapters
  rendering, platform, app integrations, MCP, plugins, web, native shell
```

All write-capable flows must use:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

Automation surfaces such as MCP, COM, AppleScript, D-Bus, Android intents, iOS
App Intents, browser extensions and plugins are read-only or plan-only by
default. Mutation requires document hash matching, policy approval, review and
audit evidence.

## What Works Today

- Rust workspace builds and tests.
- `fe-reader doctor` reports core/pdf/security identities.
- `fe-reader inspect fixtures/minimal/minimal.pdf --json` emits a read-only
  intent, patch plan and PDF summary.
- An unsigned local macOS preview bundle can be built and launched with
  `./script/build_and_run.sh`.
- The launcher also writes a local preview snapshot to
  `/tmp/fe-reader-native-preview.png` when verification mode is used.
- Core mutation contracts, transaction journaling and audit receipts have tests.
- Platform/mobile automation contracts are checked for read-only or plan-only
  behavior.
- Release-readiness workflows emit SBOM, provenance, signing-readiness and
  release evidence.
- Platform CI covers Linux container, macOS, Windows, Android emulator and iOS
  simulator target checks.

## Quick Start

```bash
cargo metadata --format-version=1
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo run -p fe_reader_cli -- doctor
cargo run -p fe_reader_cli -- inspect fixtures/minimal/minimal.pdf --json
python3 scripts/validate_schemas.py
bash scripts/wave0_bootstrap_check.sh
```

For platform-specific checks:

```bash
bash scripts/linux_container_smoke.sh
bash scripts/android_emulator_smoke.sh
```

The Android emulator script requires Android SDK command-line tools locally:
`adb`, `sdkmanager`, `avdmanager` and `emulator`.

## Packages And Registries

Packaging manifests are present for the intended registries, but publishing is
gated on signed artifacts, release evidence and registry credentials.

| Surface | Registry or channel | Status |
| --- | --- | --- |
| Rust crates | crates.io | crate manifests exist; publishing deferred |
| Native .NET wrapper | NuGet | preview package metadata exists; publishing deferred |
| Windows | winget, Chocolatey, Scoop, MSIX/MSI/NSIS | manifests/checklists exist; publishing deferred |
| macOS | Homebrew Cask, DMG, Mac App Store | cask/notarization notes exist; Apple credentials deferred |
| Linux | Flatpak, Snap, AUR, distro packages, AppImage | manifests/checklists exist; publishing deferred |
| Android | Google Play, F-Droid evaluation | checklist and emulator CI exist; publishing deferred |
| iOS | TestFlight, App Store | checklist and simulator target CI exist; publishing deferred |

Package registry links become authoritative only after the first signed preview
artifacts are published. Until then, GitHub Releases are the canonical release
index.

Desktop distribution gates are documented in
[`docs/desktop-distribution-publication.md`](docs/desktop-distribution-publication.md),
with machine-readable state in
[`packaging/registry-status.yaml`](packaging/registry-status.yaml) and
[`packaging/desktop-distribution.yaml`](packaging/desktop-distribution.yaml).

## Documentation

The docs site lives in `docs-site/` and uses Astro with Starlight:

```bash
cd docs-site
npm ci
npm run build
```

Architecture, contracts and release-governance docs are under `docs/`,
`contracts/` and `schemas/`.

## Contributing

Read `CONTRIBUTING.md`, `AGENTS.md`, `docs/v9-coding-agent-start-here.md` and
`contracts/README.md` before opening implementation work.

Contributions must preserve the core/adapters boundary, include tests or
evidence, and update compatibility/versioning notes when public contracts
change.

## Security

See `SECURITY.md`. Do not attach private PDFs, document text, credentials,
support bundles or exploit fixtures to public issues.

## Citation

Use `CITATION.cff` for citation metadata.
