# v0.92 Feature: First True Godel-Agent Birthday

## Metadata

- Feature Name: First True Godel-Agent Birthday
- Milestone Target: `v0.92`
- Status: birthday decision contract implemented; birth event not claimed
- Related issues: `#3377`, `#3434`, `#5825`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This is a planning feature doc. It records the birthday contract target, not a
claim that the first birthday has happened.

## Status

The deterministic WP-08 birthday decision and its fail-closed negative matrix
are implemented by `#5825`. This establishes the decision boundary only. It
does not by itself claim that a birth has occurred, that downstream Birthday evidence is
complete, or that public launch is ready.

Related readiness issue: `#3377`.

## Purpose

Define the first true Godel-agent birthday as a reviewable event that combines
name, identity, continuity, memory grounding, capability envelope, witnesses,
receipt, and inherited moral/governance context.

## Context

Prior milestones produce runtime state, provisional citizens, memory,
continuity, moral trace, and governed-tool evidence. v0.92 defines when those
ingredients become a reviewable birth event.

## Coverage / Ownership

v0.92 owns the birthday contract, negative cases, review packet, witness
surface, and receipt shape. v0.93 owns constitutional citizenship after birth.

## Overview

The feature should make birth distinguishable from startup, wake, snapshot,
admission, and copied state through evidence rather than ceremony.

## Design

The birthday record should cite stable name, identity root, continuity,
memory grounding, capability envelope, ACP profile, witnesses, receipt, and
inherited moral context.

The executable decision contract is
`adl-runtime-kernel/src/birthday.rs`. It accepts only the versioned
`adl.birthday.candidate.v1` shape, requires reviewer-visible digest-bound
references for every named evidence surface, requires identity continuity over
at least two bounded cycles, and emits sorted typed rejection reasons. Packet
digests use canonical JSON and SHA-256 so evidence ordering cannot change the
decision.

## Execution Flow

1. Reject not-a-birthday cases.
2. Assemble the required identity and evidence surfaces.
3. Record witnesses and receipt.
4. Emit the reviewer-facing birthday packet.

## Determinism and Constraints

The birthday decision must be deterministic over the required evidence. Missing
identity, continuity, memory, capability, witness, or receipt evidence must
fail closed.

## Integration Points

- Identity/stable-name feature.

## Production activation composition

Issue #451 adds the production composition boundary without weakening any
prerequisite authority. The Runtime accepts only a complete birthday decision,
verified Memory Palace identity and continuity, verified capability and
cognitive-profile handles, an Adaptive Learning receipt, an authenticated ACC
tool receipt, and the witness packet. It commits one canonical receipt with a
create-new ownership intent, synced staging, atomic rename, directory sync, and
restart recovery. Duplicate, copied, conflicting, denied, unauthenticated, or
cross-bound inputs fail closed. Ordinary resident startup and task execution do
not enter this path.
- Memory/capability/witness feature.
- ACP profile feature.
- First-birthday external launch surface:
  `docs/milestones/v0.92/external_launch/`.
- v0.91 moral-governance evidence.
- v0.93 governance handoff.

## Validation

The `birthday` integration-test target includes one complete candidate plus
table-driven lifecycle-lookalike, missing-evidence, integrity, privacy, path,
ACP-label, and public-claim boundary fixtures. Native macOS and Linux jobs must
run that exact non-empty target at candidate HEAD and retain digest-bound,
semantically equivalent receipts before the implementation is publication
ready. Final launch copy remains blocked until #4762 accepted witness/receipt
proof is cited.

## Source Inputs

- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `#3377`

## Scope

This feature should establish:

- the birthday contract and its negative cases
- a reviewer-facing birthday packet
- distinction between birth and startup, wake, snapshot, admission, or copied
  state
- explicit inherited moral/governance context without claiming constitutional
  citizenship yet
- the first bounded Godel-agent birthday as the culmination of the `v0.92`
  identity band

## Acceptance Criteria

- Birthday contract and negative cases exist. (Implemented by `#5825`.)
- Valid birthday packet requires all named evidence surfaces.
- Startup, wake, snapshot, admission, and copied state are rejected as birth.
- Review packet and receipt are inspectable.
- External launch copy has pending and ready variants, and the ready variant is
  blocked unless it cites accepted #4762 witness/receipt evidence.

## Risks

- Birth could become narrative-only. Mitigation: require artifacts and
  negative tests.
- Birth could overclaim personhood. Mitigation: keep legal and constitutional
  claims out of v0.92.
- Launch copy could outrun proof. Mitigation: consume the external-launch
  directory as a claim-boundary surface and fail closed while #4762 proof is
  pending.

## Future Work

v0.93 can consume the birthday evidence for citizenship and governance. Later
milestones can deepen migration and cross-polis continuity.

## Notes

This feature is the symbolic center of v0.92, but it must remain engineering
evidence first.

`birth_event_status: not_claimed`

## Non-goals

- legal personhood
- production citizenship
- silent cross-polis migration claims

## Completion Target

`v0.92`
