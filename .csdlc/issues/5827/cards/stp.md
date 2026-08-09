# Structured Task Prompt

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver only the WP-10 multi-cycle continuity chain, fixtures, validator, and retained replay/negative evidence.

## Deliverables

- adl-runtime-kernel/src/birthday_continuity.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/birthday_continuity.rs
- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- .csdlc/prepared/issues/5827/validate-native-receipts.rb
- .csdlc/prepared/issues/5827/produce-native-receipt.rb
- .csdlc/evidence/5827
- Complete canonical multi-cycle, substitution, discontinuity, duplicate, reorder, copied-state, and fail-closed negative fixture matrix under adl-runtime-kernel/tests/fixtures/birthday_continuity/
- Focused deterministic replay, continuity-head, and negative tests
- Digest-bound native macOS and Linux exact-revision proof
- Independent exact-head review and rollback evidence

## Acceptance

1. The WP-10 record links at least two bounded cycles to the same identity root and deterministically derives a continuity head or stable rejection reason.
2. WP-09/#5826 terminal proof and current lineage/wake evidence are verified before implementation begins.
3. Implementation is confined to adl-runtime-kernel/src/birthday_continuity.rs, lib.rs module registration, tests/birthday_continuity.rs, tests/fixtures/birthday_continuity/, the identity feature contract, and .csdlc/evidence/5827/.
4. Identical predecessor and cycle evidence replay to byte-equivalent semantic continuity output retained at exact revision.
5. Missing predecessor, root substitution, discontinuous or reordered cycles, duplicate cycles, forged witness, copied state, private paths, and host paths fail closed.
6. One bounded exact-head SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5827 without claiming completion of downstream Birthday work.
8. The exact birthday_continuity nextest target runs a positive test count on native GitHub Actions macOS and Linux at exact candidate HEAD; issue-local producers retain hashed source manifests, complete command logs, and canonical semantic outputs, and independent validation recomputes every digest and requires semantic equivalence.

## Dependencies

- WP-09 / issue #5826 terminal proof
- Current Runtime v3 continuity.rs and live_continuity.rs authority

## Inputs

- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md
- adl-runtime-kernel/src/continuity.rs
- adl-runtime-kernel/src/live_continuity.rs
- adl-runtime-kernel/tests/live_continuity.rs

## Non Goals

- Memory Palace retrieval, capability profiles, migration, citizenship, or birthday approval
- Metaphysical sameness or narrative-only continuity claims
- Rewriting predecessor lineage or wake evidence
