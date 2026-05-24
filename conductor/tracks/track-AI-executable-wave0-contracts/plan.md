# Executable Wave 0 Contracts plan

1. Run `cargo metadata --format-version=1`.
2. Run `cargo test --workspace --all-targets`.
3. Fix compile/test failures with smallest possible changes.
4. Run CLI smoke commands against `fixtures/minimal/minimal.pdf`.
5. Update `docs/wave0-contract-acceptance-tests.md` only when acceptance criteria change.
