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
The capability envelope records both the exact verified continuity head and
canonical continuity-record digest inside its canonical envelope hash. Those
fields alone are not authority because a caller could rewrite them and
recompute the hash. A private provisioning capability owned by `LiveAssembly`
therefore binds
the provisioned `CapabilityEnvelopePolicy` digest and the exact verified
continuity head/record digest into an opaque `CapabilityAuthorityPolicy`.
Public capability build and validation require that opaque authority and the
same verified continuity value. Governed cognition invokes that authority-aware
capability validator before it builds or validates a profile, rather than
relying on the raw component validator.

Establishing a new opaque capability authority against token B is an explicit
Runtime reauthorization and may legitimately produce a new B-bound envelope.
Merely rewriting an A-bound envelope to token B, even with recomputed envelope,
cognitive, and caller-signature bytes, cannot create that authority and must
fail closed under the retained A authority.

## Owned paths

- `adl-runtime-kernel/src/capability_envelope.rs`
- `adl-runtime-kernel/src/assembly.rs`
- `adl-runtime-kernel/src/cognitive_profile.rs`
- `adl-runtime-kernel/src/birthday_continuity.rs`
- `adl-runtime-kernel/tests/capability_envelope.rs`
- `adl-runtime-kernel/tests/fixtures/capability_envelope/authority_tests.rs`
- `adl-runtime-kernel/tests/fixtures/cognitive_profile/authority_tests.rs`
- `.csdlc/issues/237/**`
- `.csdlc/evidence/237/**`

## Invariants

- The continuity record digest, identity root, identity-record digest, and
  opaque verified-cycle lineage are all validated before capability or
  cognition is accepted.
- A substituted record, head, identity root, or identity-record digest fails
  closed with typed rejection.
- Two independently valid continuity records that share the same identity and
  predecessor remain distinct authority values; a capability built against one
  cannot be replayed under the other.
- Caller-controlled envelope fields, hashes, policies, and downstream
  signatures cannot replace the opaque Runtime-established capability
  authority. Token changes require explicit Runtime reauthorization.
- Signature, policy-digest, evidence-digest, privacy, and caller authority
  boundaries remain unchanged.
- Positive proof uses real signed `LiveContinuity` checkpoints, not fixtures or
  cached packets.
- Raw-record capability and governed-cognitive builders/validators are
  crate-private component primitives; every public authoritative entrypoint
  requires `VerifiedBirthdayContinuity`.

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

The composition regression also creates two separately signed and verified
Runtime continuity histories with the same identity and predecessor. Runtime
establishes the capability authority against token A and builds the A-bound
capability. The attack then substitutes token B, rewrites and re-hashes the
capability to B, and rebuilds and re-signs the downstream cognitive input and
authority proof. Capability and governed cognition must both reject because
the retained opaque capability authority remains bound to token A. A separately
tested establishment against B is classified as authorized reauthorization,
not substitution.
