# Rust Systems Styleguide

- Prefer small crates and traits at boundaries.
- Use `thiserror` for library errors and `anyhow` for binaries/adapters.
- Use `serde` contracts only at API boundaries.
- Do not unwrap in library code.
- Hash documents before planning and before applying.
- Keep platform, UI, MCP, plugins, and ML outside `fe_reader_core`.
