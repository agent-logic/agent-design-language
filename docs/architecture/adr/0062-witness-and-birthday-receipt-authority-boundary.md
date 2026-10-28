# ADR 0062: Witness And Birthday Receipt Authority Boundary

## Status

Status: **Proposed**

## Context

WP-15 now provides an opaque runtime-established Ed25519 witness roster,
exact-candidate attestations, and a redacted receipt boundary beyond the
structural birthday decision.

## Decision

Witness authority is established by an opaque runtime policy that binds four
distinct roles, trusted verifying keys, the exact candidate and evidence-set
digests, and the current generation. Canonical receipts expose redacted
evidence tokens and always retain `not_claimed` birth-event status. Missing,
substituted, stale, forged, equivocal, or self-nominated witness inputs fail
closed.

## Consequences

Consumers can verify a bounded exact-candidate witness packet without receiving
raw evidence paths or gaining birth, governance, or public-launch authority.

## Alternatives Considered

Allowing callers to nominate trust roots, accepting unsigned fixture shapes,
or treating all-accept signatures as autonomous birth authority was rejected.

## Source Evidence

- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
- `adl-runtime-kernel/src/birth_witness.rs`

## Validation Evidence

- `adl-runtime-kernel/tests/fixtures/birth_witness/authority_tests.rs`
- `adl-runtime-kernel/tests/birth_witness.rs`
- `.csdlc/evidence/5833/birth-witness-authority-runtime-v3.log`
- `.csdlc/evidence/5833/birth-witness-public-boundary.log`
- `.csdlc/evidence/5833/birth-witness-compile-fail.log`
- `.csdlc/evidence/5833/local-validation-manifest.json`

## Supersession Relationships

Refines ADR 0016 and ADR 0053 without superseding either record.

## Non-Claims

Does not claim a networked external witness service, certificate revocation,
legal attestation, autonomous birth authority, personhood, citizenship,
governance approval, public deployment, public-launch authorization, or a
real-world birth event.

## Approval Boundary

Human review must separately promote this candidate into `docs/adr/`.
