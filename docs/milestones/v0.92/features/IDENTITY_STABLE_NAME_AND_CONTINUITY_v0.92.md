# v0.92 Feature: Identity, Stable Name, and Continuity

## Metadata

- Feature Name: Identity, Stable Name, and Continuity
- Milestone Target: `v0.92`
- Status: WP-09 identity record and WP-10 bounded multi-cycle continuity implemented
- Related issues: `#3377`, `#3434`, `#5826`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This is a planning feature doc. It defines required identity surfaces without
claiming implementation has landed.

## Status

WP-09 implements the deterministic stable-name and identity-root record. WP-10
implements bounded evidence-backed continuity across signed runtime cycles.
Neither work package claims birthday approval, citizenship, or a completed
governance identity.

Related readiness issue: `#3377`.

## Purpose

Define the identity-bearing substrate that turns bounded runtime activity into
durable named continuity rather than anonymous or purely process-local
execution.

## Context

Birth needs identity that is more durable than process state. v0.92 should use
prior lineage and citizen-state primitives while defining the birthday-specific
identity root, stable name, and continuity proof.

## Coverage / Ownership

v0.92 owns the identity record and continuity evidence needed for the first
birthday. v0.90.3 retains ownership of lower-level citizen-state primitives.

## Overview

The feature should define how a named agent remains the same identity across
bounded cycles and how ambiguous continuity is represented.

## Design

The identity record should include stable name, identity root, aliases,
origin event, continuity head, memory grounding references, capability
reference, witness reference, and redaction policy.

The executable WP-09 contract is
`adl-runtime-kernel/src/birthday_identity.rs`. Identity-root authority derives
only from the canonical stable name and reviewer-visible origin provenance.
The origin provenance identifier must resolve to the same digest as the
declared origin reference, and the continuity head must equal its declared
evidence-reference digest; either substitution fails closed.
Aliases are canonical provenance-bearing labels: input order does not affect
the retained record, adding an alias does not rotate the root, and an alias can
never replace root authority. Continuity heads remain references to prior
evidence. The executable WP-10 contract is
`adl-runtime-kernel/src/birthday_continuity.rs`: a crate-private runtime policy
pins the accepted Birthday Identity record, trusted checkpoint signer, runtime
topology/configuration, service schema, and first generation. At least two
signed checkpoint cycles must advance monotonically from the Birthday Identity
continuity head. Caller-created keys, restart or snapshot narratives,
duplicated cycles, reordered generations, unsafe paths, and record tampering
fail closed.

## Execution Flow

1. Create identity root and stable name.
2. Attach continuity evidence across bounded cycles.
3. Reject lifecycle events that lack continuity.
4. Feed identity evidence into the birthday packet.

## Determinism and Constraints

Continuity must be evidence-based. Copied state, wake, and process restart
must not become identity continuity without the required record and witnesses.

## Integration Points

- v0.90.3 lineage/state primitives.
- v0.92 memory grounding and capability envelope.
- v0.92 birthday record.
- v0.93 governance handoff.

## Validation

The exact `birthday_identity` integration target covers canonical replay,
alias ordering, origin and alias provenance, root and continuity substitution,
path portability, private-state redaction, unknown-field rejection, and the
display-name, boot-admission, wake, snapshot, and copied-state negatives.
Native macOS and Linux jobs must retain semantically equivalent exact-head
receipts before portability is claimed.

The exact `birthday_continuity` integration target covers deterministic
two-cycle replay, signer and generation policy, predecessor continuity,
runtime-witness and provenance binding, copied-state rejection, path
portability, record tamper, and unknown-field rejection. Native macOS and Linux
jobs retain and independently compare exact-head semantic receipts.

## Source Inputs

- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md`
- `#3377`

## Scope

This feature should establish:

- stable names and alias policy
- identity root and continuity head semantics
- evidence-based continuity across bounded cycles
- separation between startup, wake, snapshot, admission, and true identity
  continuity
- downstream handoff into `v0.93` governance rather than governance-by-name
  alone

## Acceptance Criteria

- Identity record contract exists. (Implemented by `#5826`.)
- Stable names and aliases are represented.
- Continuity across bounded cycles is evidence-backed.
- Startup, wake, snapshot, admission, and copied-state cases do not pass as
  continuity without evidence.

## Risks

- A display name could be mistaken for identity. Mitigation: require identity
  root and continuity head.
- Continuity could become magical. Mitigation: require lineage and witness
  evidence.

## Future Work

v0.93 can use identity evidence for governance. Later milestones can expand
cross-polis migration and portability.

## Notes

This feature should keep the identity surface practical and auditable.

`birthday_event_status: not_claimed`

## Non-goals

- legal personhood
- constitutional citizenship by mere existence
- replacing `v0.90.3` state/lineage primitives

## Completion Target

`v0.92`
