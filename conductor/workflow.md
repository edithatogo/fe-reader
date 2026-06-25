# Conductor Workflow

1. Maintain context in `conductor/*.md`.
2. Use `conductor/waves.yaml` to identify the active wave.
3. Use `conductor/tracks/<track>/spec.md` and `plan.md` to implement track work.
4. Before starting new implementation work, check `git status -sb`. If `main` is ahead of `origin/main`, push the completed backlog before opening a new track.

## Task Workflow

Every task follows this lifecycle:

1. Implement the smallest task-scoped change that satisfies the track plan.
2. Run the narrowest meaningful validation for the task.
3. Run `conductor-review` against the task diff.
4. Apply high-confidence review fixes automatically.
5. Rerun focused validation after fixes.
6. Commit the task with a message body containing these trailers:
   - `Conductor-Track: <track-id>`
   - `Conductor-Phase: <phase-id>`
   - `Conductor-Task: <task-id>`
   - `Review: conductor-review passed`
   - `Validation: <commands>`
7. Add a git note to the task commit:

```bash
git notes --ref=conductor add -f -m "Conductor-Track: <track-id>
Conductor-Phase: <phase-id>
Conductor-Task: <task-id>
Review: conductor-review passed
Validation: <commands>
CI: pending" HEAD
```

8. Push after every task commit:

```bash
git push origin HEAD:main
git push origin refs/notes/conductor
```

## Phase Workflow

After each phase:

1. Run `scripts/conductor_phase_gate.sh --phase <phase-id> --auto-fix`.
2. Run `conductor-review` for the whole phase diff.
3. Apply high-confidence review fixes automatically and rerun validation.
4. Run `python3 scripts/conductor_lifecycle_check.py`.
5. Commit and add a `refs/notes/conductor` note for the phase checkpoint.
6. Push commits and notes.
7. If the phase gate fails architecture, security, redaction, platform permission, or distribution signing checks, stop only when the failure is high-risk and cannot be fixed without a design decision.

## Track Closeout Workflow

Every track must finish with:

1. All implementation tasks either checked off or explicitly recorded as deferred external gates.
2. `conductor-review` run for the full track.
3. High-confidence fixes applied automatically.
4. `python3 scripts/conductor_lifecycle_check.py --require-git-note` passing locally.
5. Required track validation passing.
6. Track archived automatically after review and validation pass.
7. A closeout commit with Conductor trailers and a `refs/notes/conductor` note.
8. `git push origin HEAD:main` and `git push origin refs/notes/conductor`.
9. GitHub Actions passing for the final SHA before claiming the track is complete.

Do not auto-proceed through unresolved high-risk failures. Do auto-proceed through review, fixes, validation, notes, pushes, and archive cleanup when the remaining work is deterministic.
