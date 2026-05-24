# Crate Layout

Copy each `Cargo.toml.sample` to `Cargo.toml` as you create the workspace.

`fe_reader_core` must stay pure and dependency-light. Platform/UI/rendering/automation/plugin/intelligence dependencies belong in adapter crates.
