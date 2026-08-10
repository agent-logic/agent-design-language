# Structured Task Prompt

Template: 1.0.0

Issue: 5826

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver only the WP-09 stable-name and identity-root contract, validation fixtures, and exact-revision evidence.

## Deliverables

- adl-runtime-kernel/src/birthday_identity.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/fixtures/birthday_identity/authority_tests.rs
- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- .csdlc/prepared/issues/5826/validate-native-receipts.rb
- .csdlc/prepared/issues/5826/produce-native-receipt.rb
- .csdlc/evidence/5826
- .github/workflows/wp09-native-birthday-identity.yml
- Complete canonical identity, alias, provenance, collision, substitution, disclosure, attacker-root compile-fail, and fail-closed negative fixture matrix under adl-runtime-kernel/tests/fixtures/birthday_identity/
- Focused deterministic root, canonicalization, privacy, authority-boundary, and negative tests
- Digest-bound native macOS and Linux exact-revision proof
- Independent exact-head security/privacy review and rollback evidence

## Acceptance

1. The WP-09 record deterministically binds stable name, identity root, aliases, origin evidence, continuity head, provenance, and redaction policy while rejecting ambiguous or substituted identity; external callers cannot establish BirthdayAuthorityPolicy or construct/deserialize VerifiedBirthdayEvidence, so self-consistent attacker roots fail at the crate boundary.
2. WP-08/#5825 terminal proof and current lineage authority are verified before implementation begins.
3. Implementation is confined to adl-runtime-kernel/src/birthday_identity.rs, lib.rs module registration, tests/fixtures/birthday_identity/ including the internal authority proof, the identity feature contract, issue-local native receipt tooling/workflow, and .csdlc/evidence/5826/.
4. Canonical serialization, root derivation, and alias ordering replay identically and are retained at the exact reviewed revision.
5. Empty roots, alias collision, provenance mismatch, substituted continuity, raw private state, and absolute or path-unsafe references fail closed.
6. One fresh independent exact-head security/privacy SRP review records no unresolved actionable findings.
7. The implementation PR targets the intended base and includes Closes #5826 without claiming completion of downstream Birthday work.
8. The exact internal birthday_identity authority lane runs a positive test count on native GitHub Actions macOS and Linux at the exact candidate product head; issue-local producers retain hashed source manifests, complete command logs, and canonical semantic outputs, and independent validation recomputes every digest and requires semantic equivalence.

## Dependencies

- WP-08 / issue #5825 terminal proof
- Current Runtime v3 identity_memory.rs and private_state.rs authority

## Inputs

- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md
- adl-runtime-kernel/src/identity_memory.rs
- adl-runtime-kernel/src/private_state.rs

## Non Goals

- Multi-cycle continuity, migration, citizenship, reputation, legal personhood, or birthday approval
- Using display name, boot admission, wake, snapshot, or copied state as identity proof
- Exposing raw private state or rewriting prior lineage evidence
