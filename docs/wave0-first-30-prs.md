# Wave 0 First 30 Pull Requests

These PRs are intentionally small. They allow parallel agents to work without creating a tangled, half-built monolith.

| PR | Owner track | Title | Acceptance criteria |
|---:|---|---|---|
| 1 | Governance | Repository bootstrap and root workspace | Root `Cargo.toml`, rust-toolchain, license files and `cargo metadata` succeed. |
| 2 | Core | Core contract crate skeleton | `fe_reader_core` compiles; no forbidden dependencies. |
| 3 | CLI | CLI doctor command | `fe-reader doctor` prints CLI/core versions. |
| 4 | CLI | Read-only inspect plan stub | `fe-reader inspect --json` emits a patch-plan-shaped JSON object without reading/mutating the PDF. |
| 5 | Schemas | Schema validation script hardening | `scripts/validate_schemas.py` validates all schemas and reports failures clearly. |
| 6 | Safety | Error taxonomy | Error envelope schema and Rust contract are added; CLI uses safe messages. |
| 7 | Safety | Transaction journal contract | Operation journal schema and Rust contract are added. |
| 8 | Safety | Write-mode policy stub | Incremental append and sanitising rewrite modes are modelled; redaction defaults to sanitising rewrite. |
| 9 | Security | Security policy check | `scripts/security_policy_check.sh` verifies read-only default automation policy. |
| 10 | Corpus | Fixture manifest contract | Add first placeholder fixture manifest and validation. |
| 11 | Testing | nextest config | `.config/nextest.toml` exists and `cargo nextest run` is documented. |
| 12 | CI | Quality matrix sample | CI sample runs fmt, clippy, tests, schema validation and bootstrap check. |
| 13 | Performance | Perf smoke scaffold | `scripts/perf_smoke.sh` runs without requiring real PDF rendering. |
| 14 | Architecture | Core dependency firewall | Architecture check fails if core imports renderer/UI/platform/automation crates. |
| 15 | Metadata | Metadata contract stub | Metadata operation schema and Rust contract compile. |
| 16 | Render | RenderBackend trait crate | `fe_reader_render` owns only trait types and no PDFium dependency. |
| 17 | Render | PDFium adapter placeholder | `fe_reader_render_pdfium` compiles behind adapter boundary. |
| 18 | Platform | PlatformIntegration trait stub | Platform contracts compile; no OS-specific code in core. |
| 19 | Automation | Local automation read-only contract | OpenAPI/JSON-RPC schemas expose read-only methods first. |
| 20 | MCP | MCP read-only tool manifest | Tool list includes inspect/read/search only; mutating tools disabled. |
| 21 | Plugins | Plugin manifest validator | WASM plugin manifests validate capability requests. |
| 22 | Workflows | Workflow pack schema validator | Template/workflow packs validate and are risk-classified. |
| 23 | Redaction | Redaction recipe schema | Redaction recipes distinguish markup-only from secure redaction. |
| 24 | Updates | Update manifest schema | Signed update manifest contract validates. |
| 25 | Build | Reproducible dev environment smoke | devcontainer/Nix docs and scripts are consistent. |
| 26 | Docs | First contributor guide | CONTRIBUTING references Wave 0 commands and boundaries. |
| 27 | Governance | ADR workflow | ADR schema/checker exists; first ADRs cover core/UI/render separation. |
| 28 | Release | Release evidence placeholder | Release evidence schema and checker are wired. |
| 29 | UX | User-facing error copy review | Error messages avoid leaking document text or paths by default. |
| 30 | Wave gate | Wave 0 phase gate | `scripts/conductor_phase_gate.sh` runs Wave 0 checks and reports a single pass/fail summary. |

## Rule for coding agents

Do not combine unrelated PRs. If a task touches more than two architectural layers, split it.
