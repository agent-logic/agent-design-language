# Structured Task Prompt

Template: 1.0.0

Issue: 604

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Restore governed draft-to-ready and reconcile-ready publication operations in the v2 publication owner, with tests and skill/inventory alignment.

## Deliverables

- Typed csdlc-publish ready request/result command.
- Typed csdlc-publish reconcile-ready request/result command.
- Durable exact-identity lifecycle truth after successful remote readback.
- csdlc-v2/tests/publication_ready.rs focused tests for success, uncertainty/recovery, identity drift, stale CAS, pre-state rejection, and zero-write failure paths.
- Updated publication skill and operator inventory documentation.

## Acceptance

1. AC-1: Restore a typed exact-identity draft-to-ready operation under the authoritative v2 publication owner.
2. AC-2: Re-observe exact repository, PR, head SHA, open state, and draft state before and after mutation.
3. AC-3: Record ready publication truth atomically and provide reconciliation after uncertain remote response.
4. AC-4: Reject mismatched repository, PR, head, non-draft pre-state, closed state, stale generation, or stale digest without lifecycle mutation.
5. AC-5: Align publication skill text and installed binary inventory with the implemented command surface.
6. AC-6: Add focused tests for success, remote uncertainty, recovery, identity drift, and zero-write failure paths.

## Dependencies

- Live GitHub issue #604
- Current Gate 10D2 v2 authority contract

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/604
- csdlc-v2/src/publication.rs
- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/github.rs
- csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md
- csdlc-v2/operator/skills.json

## Non Goals

- Runtime v3 behavior changes
- Raw GitHub fallback
- Merge, finish, cleanup, or issue closeout
- Weakening reviewed or publication prerequisites
