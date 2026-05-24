# Conductor Workflow

1. Maintain context in `conductor/*.md`.
2. Use `conductor/waves.yaml` to identify the active wave.
3. Use `conductor/tracks/<track>/spec.md` and `plan.md` to implement track work.
4. After each phase, run `scripts/conductor_phase_gate.sh --phase <phase-id> --auto-fix`.
5. If the phase gate fails architecture, security, redaction, platform permission, or distribution signing checks, stop and create a fix plan.
6. Do not auto-proceed through high-risk failures.
