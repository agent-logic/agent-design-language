# Issue #237 design: canonical Birthday continuity binding

## Problem

The retained capability validator binds `BirthdayCandidate.continuity_head` to
`BirthdayIdentityRecord.continuity.head_sha256`, while the retained cognitive
validator binds the same value to `BirthdayContinuityRecord.continuity_head`.
A real signed Runtime continuity record has its own canonical head, so the two
predicates cannot both hold for the integrated WP-18 composition.

## Decision

The canonical Birthday continuity binding used by downstream capability and
cognitive authority is an opaque `VerifiedBirthdayContinuity` value. It is
created only by a shared verifier that consumes a `BirthdayContinuityRecord`,
the bound `BirthdayIdentityRecord`, and opaque `VerifiedBirthdayCycle` values
from `verify_birthday_cycles`, then runs the full continuity-record validator.
Both capability and governed cognition consume that same verified value; a
public record or matching digest alone confers no authority. No constructor for
trusted authority is made public and no caller-supplied trust root is added.

## Owned paths

- `adl-runtime-kernel/src/capability_envelope.rs`
- `adl-runtime-kernel/src/cognitive_profile.rs`
- `adl-runtime-kernel/src/birthday_continuity.rs`
- `adl-runtime-kernel/tests/capability_envelope.rs`
- `adl-runtime-kernel/tests/fixtures/cognitive_profile/authority_tests.rs`
- `.csdlc/issues/237/**`
- `.csdlc/evidence/237/**`

## Invariants

- The continuity record digest, identity root, identity-record digest, and
  opaque verified-cycle lineage are all validated before capability or
  cognition is accepted.
- A substituted record, head, identity root, or identity-record digest fails
  closed with typed rejection.
- Signature, policy-digest, evidence-digest, privacy, and caller authority
  boundaries remain unchanged.
- Positive proof uses real signed `LiveContinuity` checkpoints, not fixtures or
  cached packets.

## Proof

Add one focused crate-internal regression under the existing cognitive-profile
authority test surface. It constructs real signed Birthday identity and
continuity evidence, obtains the opaque verified binding, then composes
capability and governed cognition without exposing trust constructors. Attack
cases substitute the record head, identity root, identity-record digest, and
whole record, then recompute every caller-controlled candidate, record,
envelope, and profile digest and applicable caller signature. Each must still
fail with the typed continuity-binding rejection because the attacker cannot
forge the opaque verified cycles/runtime authority. Run the retained capability
and cognitive authority tests in the same required job to prove authority and
privacy behavior was not weakened.
