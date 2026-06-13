# Track AX: Advanced Roadmap Continuation

## Overview

Create post-launch advanced roadmap tracks after stable desktop launch gates are implemented. Advanced work must not block launch unless it fixes a launch-critical defect.

## Scope

- Post-launch roadmap decomposition.
- Advanced PDF capabilities.
- Optional frontier lanes.
- Mobile launch follow-up.
- Cloud/collaboration follow-up, if separately approved.

## Functional Requirements

- Create follow-on Conductor tracks for advanced roadmap work after launch gates are in place.
- Keep ML/RAG/frontier intelligence feature-gated and disabled by default until evidence supports promotion.
- Keep Android/iOS public launch work separate from desktop stable.
- Keep cloud sync/collaboration separate and opt-in.
- Add acceptance criteria for each follow-on track before implementation.

## Non-Functional Requirements

- Advanced tracks must preserve core architecture boundaries.
- Frontier dependencies require owner, feature gate, rollback plan and exit criteria.
- No advanced track may weaken release evidence, automation safety, privacy or signing gates.

## Acceptance Criteria

- Advanced roadmap backlog is represented as granular Conductor tracks.
- Each track has spec, plan, metadata and registry entry.
- Dependencies and non-blocking relationship to desktop launch are explicit.

## Out of Scope

- Implementing ML/RAG, mobile public launch or cloud services inside the desktop stable launch programme.

