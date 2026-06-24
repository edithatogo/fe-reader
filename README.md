# Fe Reader

Fe Reader is a local-first PDF workflow platform focused on privacy,
verification, metadata transparency, automation safety and cross-platform native
integration.

The repository is currently a technical preview implementation checkpoint. It
contains a headless Rust core, executable Wave 0 contracts, platform automation
contracts, release-evidence tooling, package manifests and an Astro/Starlight
documentation site. It is not yet a production PDF application or a published
app-store package.

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
- `fe-reader doctor` reports core/pdf/security identities and stays within the
  registry claim boundary in `docs/pdf-parity-registry.md`.
- `fe-reader inspect fixtures/minimal/minimal.pdf --json` emits a read-only intent, patch plan and PDF summary. See `docs/pdf-parity-registry.md`.
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
cargo run -p fe_reader_cli -- inspect fixtures/minimal/minimal.pdf --json # See docs/pdf-parity-registry.md for the claim boundary for inspect/read.
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
| macOS | Homebrew Cask, DMG, Mac App Store | signed and notarized DMG evidence required before public registry publication |
| Linux | Flatpak, Snap, AUR, distro packages, AppImage | manifests/checklists exist; publishing deferred |
| Android | Google Play, F-Droid evaluation | checklist and emulator CI exist; publishing deferred |
| iOS | TestFlight, App Store | checklist and simulator target CI exist; publishing deferred |

Package registry links become authoritative only after the first signed preview
artifacts are published. Until then, GitHub Releases are the canonical release
index.

Stable desktop release instructions live in
[`docs/stable-desktop-release.md`](docs/stable-desktop-release.md). A desktop
release should publish platform artifacts, `SHA256SUMS`, signatures, release
notes and the GitHub Actions `release-evidence` bundle before registry or
homepage claims are treated as authoritative.

Stable reader readiness is tracked separately in
[`docs/stable-reader-readiness.md`](docs/stable-reader-readiness.md) and
validated by `scripts/stable_reader_readiness_check.py`.
The usable stable bleeding-edge PDF reader contract is documented in
[`docs/usable-stable-bleeding-edge-pdf-reader-contract.md`](docs/usable-stable-bleeding-edge-pdf-reader-contract.md).
The macOS public-quality signed/notarized launch contract is documented in
[`docs/macos-public-quality-signed-notarized-launch-contract.md`](docs/macos-public-quality-signed-notarized-launch-contract.md).
The Windows/Linux beta installers contract is documented in
[`docs/windows-linux-beta-installers-contract.md`](docs/windows-linux-beta-installers-contract.md).
The contract is enforced by `scripts/windows_linux_beta_installers_check.py`.
The exhaustive PDF parity taxonomy and contracts are documented in
[`docs/exhaustive-pdf-parity-taxonomy-contract.md`](docs/exhaustive-pdf-parity-taxonomy-contract.md).
The contract is enforced by `scripts/exhaustive_pdf_parity_taxonomy_check.py`.
The corpus/oracle evidence factory is documented in
[`docs/corpus-oracle-evidence-factory-contract.md`](docs/corpus-oracle-evidence-factory-contract.md).
The contract is enforced by `scripts/corpus_oracle_evidence_factory_check.py`.
The reader/render/search/accessibility parity contract is documented in
[`docs/reader-render-search-accessibility-parity-contract.md`](docs/reader-render-search-accessibility-parity-contract.md).
The contract is enforced by `scripts/reader_render_search_accessibility_parity_check.py`.
The professional workflow parity contract is documented in
[`docs/professional-workflow-parity-contract.md`](docs/professional-workflow-parity-contract.md).
The contract is enforced by `scripts/professional_workflow_parity_check.py`.
The advanced PDF family parity contract is documented in
[`docs/advanced-pdf-family-parity-contract.md`](docs/advanced-pdf-family-parity-contract.md).
The contract is enforced by `scripts/advanced_pdf_family_parity_check.py`.
The marketing claim governance contract is documented in
[`docs/marketing-claim-governance-contract.md`](docs/marketing-claim-governance-contract.md).
The contract is enforced by `scripts/marketing_claim_governance_check.py`.
The stable release cutover and registries contract is documented in
[`docs/stable-release-cutover-registries-contract.md`](docs/stable-release-cutover-registries-contract.md).
The contract is enforced by `scripts/stable_release_cutover_registries_check.py`.

Marketing readiness levels and claim boundaries are documented in
[`docs/marketing-readiness.md`](docs/marketing-readiness.md) and validated by
`scripts/marketing_claim_evidence_governance_check.py`.

The v2 roadmap foundation is documented in
[`docs/v2-roadmap-foundation.md`](docs/v2-roadmap-foundation.md) and validated
by `scripts/advanced_roadmap_check.py`.

Desktop distribution gates are documented in
[`docs/desktop-distribution-publication.md`](docs/desktop-distribution-publication.md),
with machine-readable state in
[`packaging/registry-status.yaml`](packaging/registry-status.yaml) and
[`packaging/desktop-distribution.yaml`](packaging/desktop-distribution.yaml).

Launch QA is summarized by:

```bash
python3 scripts/launch_qa_check.py
```

Known launch limitations and support routes are documented in
[`docs/launch-limitations-support.md`](docs/launch-limitations-support.md),
[`SUPPORT.md`](SUPPORT.md) and [`SECURITY.md`](SECURITY.md).

Post-launch PDF capability claims are governed by the exhaustive parity
registry in [`docs/pdf-parity-registry.md`](docs/pdf-parity-registry.md) and
the nested baseline matrix in
[`docs/pdf-baseline-parity-matrix.md`](docs/pdf-baseline-parity-matrix.md).
Run `python3 scripts/pdf_parity_registry_check.py` before expanding public PDF
capability claims.

Run `python3 scripts/stable_reader_readiness_check.py` before broad marketing
claims about the reader baseline.

Mobile public launch readiness is tracked separately behind the
`mobile_public_launch` feature gate in
[`docs/mobile-public-launch-readiness.md`](docs/mobile-public-launch-readiness.md).
Run `python3 scripts/mobile_public_launch_check.py` before changing Android or
iOS publishing status.

Optional local ML/RAG work remains disabled by default behind the
`frontier_intelligence_preview` feature gate. Governance lives in
[`docs/frontier-intelligence-governance.md`](docs/frontier-intelligence-governance.md);
run `python3 scripts/frontier_intelligence_governance_check.py` before changing
frontier intelligence defaults or promotion criteria.

Opt-in collaboration and sync remains disabled by default behind the
`opt_in_collaboration_sync` feature gate. Governance lives in
[`docs/opt-in-collaboration-sync.md`](docs/opt-in-collaboration-sync.md);
run `python3 scripts/opt_in_collaboration_sync_check.py` before changing
collaboration defaults, provider behavior or support-bundle exclusions.

Rendering and GPU performance promotion remains disabled by default behind the
`rendering_performance_promotion` feature gate. Governance lives in
[`docs/rendering-performance-promotion.md`](docs/rendering-performance-promotion.md);
run `python3 scripts/rendering_performance_promotion_check.py` before expanding
rendering, GPU or performance claims.

SDK, plugin, workflow-pack and marketplace expansion remains disabled by default
behind the `ecosystem_integrations_marketplace` feature gate. Governance lives
in [`docs/ecosystem-integrations-marketplace.md`](docs/ecosystem-integrations-marketplace.md);
run `python3 scripts/ecosystem_integrations_marketplace_check.py` before
expanding ecosystem publication claims.

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
