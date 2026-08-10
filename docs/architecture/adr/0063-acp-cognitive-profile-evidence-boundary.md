# ADR 0063: ACP Cognitive Profile Evidence Boundary

## Status

Status: **Deferred**

## Context

ACP has landed profile code and focused tests, but its canonical feature
contract identifies corrective authority issue #144.

## Decision

Defer the architecture decision until #144 resolves profile authority and the
result is reviewed at an exact revision. ACP should remain an evidence-grounded
profile projection, not identity or rights authority.

## Consequences

Existing implementation remains usable evidence but does not receive durable
architecture approval from this packet.

## Alternatives Considered

Ignoring the corrective authority issue was rejected as review-hostile.

## Source Evidence

- `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md`
- `adl-runtime-kernel/src/cognitive_profile.rs`

## Validation Evidence

- `adl-runtime-kernel/tests/cognitive_profile.rs`
- `.csdlc/evidence/5830/cognitive-profile-runtime-v3.log`

## Supersession Relationships

May refine ADR 0016 and ADR 0019 after corrective proof.

## Non-Claims

No identity, reputation, consciousness, rights, public standing, or corrected
authority contract is claimed.

## Approval Boundary

Issue #144 must land and receive human architecture review before Proposed.
