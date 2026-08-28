# Issue #501 Design: V3-B C-SDLC v3 foundation

## Purpose

Produce the first deterministic C-SDLC v3 foundation slice after the V3-A
contract. The slice exposes repository context, state loading, and projection
replay as non-authoritative library/application/repository surfaces.

## Authority boundary

C-SDLC v2 remains the sole operational lifecycle authority. The #501 foundation
slice must not create lifecycle execution, GitHub mutation, publication,
finish, cleanup, or authority-cutover behavior.

## Inputs

- `agent-logic/agent-design-language#501`
- `agent-logic/agent-design-language#500`
- retained predecessor issues `#164`, `#165`, `#166`, and `#167`
- `docs/csdlc-v3/CONTRACT.md`
- `docs/csdlc-v3/predecessor-coverage.json`
- `docs/csdlc-v3/proportional-lifecycle.json`
- `docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#V3-B`
- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml`
- `docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md`

## Deliverables

- `csdlc-v3/src/repository/**`: explicit repository-context model and path
  resolution.
- `csdlc-v3/src/application/**`: deterministic state/projection application
  boundary.
- `csdlc-v3/src/bin/**`: a narrow inspection/demo command for the foundation
  surface, with machine-readable output and no lifecycle mutation.
- `csdlc-v3/tests/foundation/**`: focused tests proving deterministic replay,
  repository context, and retained requirements `#164` through `#167`.

## Design decisions

1. Repository context is an explicit value, not ambient process state.
2. State and projections are loaded through deterministic repository adapters.
3. Projection replay has stable ordering and stable serialization.
4. Foundation command output is read-only and machine-readable.
5. Startup ergonomics include a bounded issue-start readiness surface that can
   prove a prepared issue is ready quickly without bypassing v2 authority.

## Validation lanes

- `foundation-unit`
- `repository-context`
- `state-projection`
- `strict-clippy`
- `diff-hygiene`

## Stop conditions

- Hidden process state is required.
- Projection replay diverges across repeated runs.
- The implementation requires GitHub mutation or lifecycle execution.
- The slice grants C-SDLC v3 operational authority.
