---
name: fe-reader-review-skill
description: Project-local shim for the Conductor review lifecycle. Use the installed conductor-review skill and Fe Reader workflow rules.
license: Apache-2.0
---

# Fe Reader Review Skill

Use the installed `conductor-review` skill as the authoritative review loop.

Project-specific requirements:

- Load the active track `spec.md`, `plan.md`, and `conductor/workflow.md`.
- Apply high-confidence fixes automatically.
- Rerun focused validation after fixes.
- Record review evidence in the commit body and in `refs/notes/conductor`.
- Do not mark a track complete while implementation tasks are unchecked unless the remaining items are documented external gates such as signing, notarization, registry review, credentials, or maintainer approval.
- Return control to implementation automatically after validation passes.
