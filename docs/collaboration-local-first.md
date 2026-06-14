# Local-First Collaboration Roadmap

## Position

Real-time cloud collaboration is not an early priority. Local-first collaboration can still be planned through portable annotation/workflow bundles.

## Initial collaboration features

- export annotations as JSON/FDF-like sidecar
- import annotations from sidecar
- review bundle with document hash binding
- comment summary export to Markdown
- workflow receipt export

## Later options

- local network review session
- Git-backed annotation bundles
- CRDT-based annotation sidecars
- self-hosted team workspace

## Rule

Collaboration never weakens redaction, metadata scrub or automation policy. A shared bundle must not expose hidden source content unless explicitly included by the user.

Cloud collaboration remains behind the `opt_in_collaboration_sync` feature gate, disabled by default. Sync providers must not silently upload, sync, collect analytics or phone home; they start unavailable until the user explicitly opts in and provider capability discovery succeeds.
