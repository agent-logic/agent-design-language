# Structured Task Prompt

Template: 1.0.0

Issue: 5600

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement only issue #5600 typed preparation-to-implementation replanning and its complete proof surface.

## Deliverables

- typed replacement operations for every required SIP, STP, SPP, and SRP planning collection
- phase, ownership, generation, digest, atomicity, and audit enforcement
- cross-card acceptance-coverage enforcement after replanning
- #5337 preparation-to-implementation conversion fixture
- complete focused and regression proof

## Acceptance

1. AC-1: A prepared Bound issue can replace every SIP, STP, SPP, VPP, and SRP planning collection required for full implementation without direct Markdown or values-file edits
2. AC-2: Every replacement remains card-owned, generation-aware, digest-guarded, claim-authorized, atomically regenerated, and durably audited
3. AC-3: Cross-card acceptance coverage rejects stale, missing, duplicate, or extra acceptance IDs after replanning without partial mutation
4. AC-4: Negative tests reject empty replacements, wrong-card ownership, stale generation, stale digest, invalid claim, and mutation outside authorized Bound replanning
5. AC-5: A #5337 preparation-to-implementation fixture completes through typed operations and passes typed validation and doctor
6. AC-6: Existing operation serialization, compact card rendering, and lifecycle behavior remain compatible
7. AC-7: Focused tests, all-target tests, formatting, strict Clippy, exact-revision review, and typed draft publication complete with no unresolved findings

## Dependencies

- issue #5597 integrated generation-aware preparation semantics
- issue #5337 consumes the integrated result

## Inputs

- live issue #5600
- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/tests
- issue #5337 prepared card shape

## Non Goals

- ADL language or Runtime implementation
- direct editing of rendered cards or values files
- generic untyped JSON mutation
- AWS
- raw gh
- unrelated lifecycle or documentation refactors
