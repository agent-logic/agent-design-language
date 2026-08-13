# Structured Task Prompt

Template: 1.0.0

Issue: 300

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #300 deterministic integrated proof only; no production redesign or parent/upstream lifecycle mutation.

## Deliverables

- csdlc-v2/tests/projection_recovery_integration.rs
- Issue-local exact matrix evidence
- Truthful VPP/SOR proof boundaries
- Fresh #119 exact-head review

## Acceptance

1. AC-1: Every approved recovery and cleanup durability/mutation boundary has deterministic interruption-before and interruption-after proof against production code.
2. AC-2: Every restart reaches the same exact result or fails closed while preserving evidence without sleeps or scheduler luck.
3. AC-3: Identity, topology, ownership, permission, mount, hardlink, type, corruption, CAS, collision, substitution, and ambiguity negatives are exercised.
4. AC-4: Repeated classify/recover/cleanup and a subsequent ordinary typed commit prove idempotency and intended gate release.
5. AC-5: No unrelated file or replacement inode is deleted and immutable ledgers remain.
6. AC-6: Existing initialized/ready recovery and #291 behavior remain green.
7. AC-7: VPP/SOR name only tests actually run and results observed.
8. AC-8: Exact-head #119 review has no unresolved actionable finding.

## Dependencies

- Validated typed terminal #298 and head/merge ancestral to #300 execution base
- Validated typed terminal #299 and head/merge ancestral to #300 execution base
- Exact integrated production API and failpoint registry from terminal #298/#299

## Inputs

- agent-logic/agent-design-language#300
- .git/csdlc-v2/derived-terminal/298.json
- Future validated .git/csdlc-v2/derived-terminal/299.json
- .csdlc/prepared/issues/300/design.md
- .csdlc/prepared/issues/300/diagram.mmd

## Non Goals

- Changing production recovery or cleanup semantics
- Editing shared gate5 coverage
- Mutating #291, #294, #296, #297, #298, or #299
- Mock, fabricated, or self-authored receipt authority
- Paid runners without authority
- Publication, merge, or closeout during preparation
