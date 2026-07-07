# ADR 0043: ADL Platform CLI Binary Taxonomy

- Status: Accepted
- Date: 2026-07-06
- Accepted in: v0.91.7
- Related issues: #4983, #4989, #4995, #4726, #4806, #4950
- Related ADRs: ADR 0024, ADR 0028, ADR 0033, ADR 0037, ADR 0038, ADR 0039
- Source evidence:
  - `https://github.com/danielbaustin/agent-design-language/issues/4983`
  - `.adl/v0.91.7/bodies/issue-4989-v0-91-7-adr-write-v0-91-7-architecture-decision-records.md`
  - `docs/milestones/v0.91.7/WBS_v0.91.7.md`
  - `docs/milestones/v0.91.7/review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md`
  - `docs/milestones/v0.91.7/review/tooling_closeout/TOOLING_SPRINT_4806_CLOSEOUT_TRUTH_4959.md`

## Context

ADL now has several operational surfaces that should not keep accumulating
inside one monolithic command. The language/compiler surface, CSM runtime,
C-SDLC workflow control plane, runtime administration, validation helpers, and
future product tools have different audiences, authority boundaries, and proof
requirements.

v0.91.7 build-throughput and workflow-stabilization work also exposed the cost
of treating all ADL commands as one large binary surface. When unrelated command
families share one operational shape, validation lanes, ownership, and release
readiness become harder to reason about.

## Decision

ADL should use a platform CLI taxonomy with explicit ownership boundaries:

- `adl`: language compiler/manager and stable user-facing ADL entrypoint
- `csm`: Cognitive Spacetime runtime daemon/execution binary
- `csmctl`: CSM runtime administration and operator-control client
- `csdlc`: C-SDLC workflow control plane for issues, cards, PR lifecycle,
  validation planning, closeout, and review surfaces
- `tools/*`: bounded helper utilities, scripts, and validation adapters that do
  not own product authority

Future binaries such as `aptitude`, `obsmem`, `polis`, or `guild` require an
explicit ownership boundary, operational need, validation lane, and reviewable
handoff before they become first-class public surfaces.

## Consequences

### Positive

- Reduces the blast radius of changes by aligning binaries with ownership.
- Helps validation lanes target only the command family affected by a change.
- Keeps runtime administration out of language tooling and C-SDLC workflow
  control out of runtime/product binaries.
- Gives future binary additions a decision gate instead of allowing utility
  sprawl.

### Negative

- Command migration needs careful compatibility handling.
- Documentation and skills must teach the binary taxonomy consistently.
- Some near-term helpers may remain transitional until their owning binary is
  implemented.

## Alternatives Considered

### Keep expanding `adl`

Rejected. A single all-purpose binary makes ownership, validation, and operator
expectations harder to keep deterministic.

### Put all runtime operations under `csmctl`

Rejected. `csmctl` should administer the runtime, not absorb C-SDLC workflow
control, language compilation, validation helpers, or unrelated future products.

## Validation Notes

Validate this ADR by checking that future issue plans and docs route command
surfaces to the proper owner, and that validation profiles do not treat changes
to one binary family as proof obligations for every command family.

## Non-Claims

- This ADR does not implement command migration.
- This ADR does not remove compatibility entrypoints.
- This ADR does not claim future binaries are ready before their owning issues
  produce code and proof.
- Implementation is tracked separately by #4995.
