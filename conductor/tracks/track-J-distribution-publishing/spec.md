# Track J: Distribution & Publishing

## Theme

Release

## Scope

- Windows winget/scoop/choco/msix/nsis
- NuGet
- Homebrew/MAS
- Flatpak/Snap/AUR/deb/rpm/AppImage
- Google Play/App Store

## Contract files

- `contracts/README.md`
- Relevant `contracts/rust/*.rs`
- Relevant platform/application/web/MCP contracts
- Relevant schemas in `schemas/`

## Hard rules

- Map high-risk actions to `FeOperationIntent`.
- Return patch plans for destructive or high-risk mutations.
- Update CLI tests for core operations.
- Run phase gate after every phase.
- Do not introduce ML/RAG dependency unless this is Track M in Wave 6.

## Deliverables

- Compileable crate/module skeletons.
- Unit tests and at least one golden/smoke test.
- Documentation update.
- Contract/schema update where public shape changes.
