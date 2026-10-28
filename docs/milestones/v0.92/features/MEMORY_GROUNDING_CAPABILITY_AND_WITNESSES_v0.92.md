# v0.92 Feature: Memory Grounding, Capability Envelope, and Witnesses

## Metadata

- Feature Name: Memory Grounding, Capability Envelope, and Witnesses
- Milestone Target: `v0.92`
- Status: implementation in progress; WP-12 capability-envelope and WP-15 birth-witness slices implemented by `#5829` and `#5833`
- Related issues: `#3377`, `#3434`, `#5825`, `#5826`, `#5829`, `#5833`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This feature doc remains the bounded contract for memory, capability,
witnesses, and receipts. The WP-12 capability-envelope slice is implemented by
`#5829`; the bounded WP-15 exact-candidate witness and redacted receipt slice is
implemented by `#5833`.

## Status

Mixed implementation feature contract for `v0.92`: the capability-envelope and
exact-candidate birth-witness runtime contracts and fixtures are implemented.
Broader governance, public launch, and legal-status work remains out of scope.

Related readiness issue: `#3377`.

## Purpose

Bind first-birthday identity claims to witnessed memory references, capability
envelopes, and citizen-facing evidence rather than to vocabulary alone.

## Context

The first birthday needs grounding in memory and capability, but reviewers
should not need raw private-state access to verify it.

## Coverage / Ownership

v0.92 owns redaction-safe memory references, capability envelope shape,
witness set, and receipt surface for the birthday packet.

## Overview

The feature binds identity claims to witnessed artifacts, bounded capabilities,
and reviewable receipts.

## Design

The design should use references and projections for memory grounding,
capability envelopes for provider/model/tool/skill/authority limits, and
witness/receipt records for review.

## Execution Flow

1. Resolve allowed memory-grounding references.
2. Build capability envelope.
3. Attach witness set and receipt.
4. Include all surfaces in the birthday packet.

## Determinism and Constraints

Memory grounding must not expose raw private state. Capability envelopes must
name limits and authority context rather than imply unlimited capacity.

## Integration Points

- Identity and continuity records.
- ObsMem/trace baseline.
- Governed tool evidence where applicable.
- Birthday review packet.

## Implemented Capability Envelope Input

Issue `#4761` supplies the pre-`v0.92` capability envelope input at
`.csdlc/evidence/4761/capability-envelope/envelope.v1.json`, with exact source
inventory in `.csdlc/evidence/4761/capability-envelope/inputs.v1.json`,
fail-closed validation in
`.csdlc/evidence/4761/capability-envelope/validation.v1.log`, and unsupported
claims in `.csdlc/evidence/4761/capability-envelope/non-claims.v1.md`.

Birthday packets may consume that envelope as the provider, model, tool, skill,
authority, and limit context surface. They must not treat the envelope as
birthday execution, Memory Palace completion, credentialed remote-provider
deployment, production citizenship, governance completion, or raw private-state
authority.

## WP-12 Runtime Capability Envelope

Issue `#5829` adds the versioned runtime contract at
`adl-runtime-kernel/src/capability_envelope.rs`. The contract consumes two
separately checked Birthday authorities:

- an accepted WP-08 `BirthdayCandidate` whose canonical digest is current; and
- a WP-09 `BirthdayIdentityRecord` whose schema and canonical record digest are
  current and whose stable name, identity root, and continuity head agree with
  the Birthday candidate.

The untrusted envelope input is evaluated against a separately provisioned
policy. The policy pins exact evidence path, content digest, source revision,
and issue provenance; allowed provider/model pairs, tools, skills, and grants;
required denials and unsupported claims; and explicit resource ceilings for
prompt/output tokens, tool calls, skill invocations, timeout, and recurrence.
The envelope describes capability context only. It neither grants authority nor
proves provider, model, tool, or skill invocation.

Canonical sorting and exact deduplication make equivalent inputs byte-stable.
Case-folded identifier collisions, unknown evidence, stale revisions,
undeclared provider/model/tool/skill selections, authority escalation,
grant/denial conflicts, omitted or zero limits, limit escalation, missing
provenance, and omitted non-claims fail closed. Every serialized contract uses
`deny_unknown_fields`. Paths must remain normalized and repository-relative;
secret-like material and private, home, host, or traversal paths are rejected.
Rejected untrusted identifiers are represented only by stable SHA-256
fingerprints in serializable and debug diagnostics; raw evidence, provider,
model, tool, skill, grant, denial, and unsupported-claim values are never
echoed. Path checks are lexical and platform-independent, including Windows
drive/UNC forms and leading private, home, users, or user namespaces.

`validate_capability_envelope` is the exported consumption boundary. It checks
the packet digest and reconstructs the complete expected envelope from the
original Birthday authorities plus the provisioned policy, so a caller cannot
make a forged packet acceptable merely by recomputing its digest.

## #448 Runtime Resident-Cycle Consumption

Issue `#448` wires the capability-envelope substrate into the live Runtime
resident cycle at `adl-runtime-kernel/src/resident_cycle.rs` and
`LiveAssembly::build_verified_resident_cycle`. The resident cycle provisions
capability authority through the live assembly, consumes exact Birthday
identity and verified continuity, builds the envelope from provider/model,
tool, skill, grant, denial, resource-limit, evidence, policy, and runtime
authority inputs, and returns a typed `VerifiedCapabilityEnvelopeHandle`.

This is a production integration slice, not a new capability-envelope
definition. It does not claim provider execution, ACC execution, birthday
ceremony completion, Adaptive Learning mutation execution, or broad Runtime v4
redesign. The retained proof for the slice is the focused resident-cycle test
lane in `adl-runtime-kernel/src/resident_cycle.rs`, including live-assembly
construction, restart/rehydration record validation, stale-continuity denial,
and authority-rotation denial.

## WP-15 Exact-Candidate Birth Witnesses

Issue `#5833` adds the versioned runtime contract at
`adl-runtime-kernel/src/birth_witness.rs`. It consumes an accepted, canonical
WP-08 `BirthdayCandidate` and its exact `BirthdayDecision`, then checks four
distinct signed witness roles against an opaque, runtime-established Ed25519 roster:
identity continuity, memory and capability, negative-case guard, and handoff
consumer. Every signature binds the exact candidate digest, reviewed evidence
set digest, current generation, role, witness identity, signing-key identity,
and accept-or-reject decision. External callers cannot construct or serialize
the authority policy or nominate its root keys. The candidate's
reviewer-visible WitnessSet reference must itself pin the established roster
digest.

The resulting witness set and citizen-facing receipt are canonical and
byte-stable under equivalent witness ordering. The receipt replaces source
paths with deterministic kind-and-digest evidence tokens, exposes no original
path text, and carries fixed caveats. Its birth
event status is always `not_claimed`: an all-accept witness set is review
evidence, not autonomous birth authority, legal personhood, citizenship,
governance approval, or public-launch authorization. A valid signed rejection
produces a deterministic rejected witness disposition with the same
`not_claimed` boundary.

Missing roles, duplicate or substituted identities/keys, stale generations,
candidate or evidence transplant, forged signatures, roster/policy mismatch,
private or host-local paths, secret-like identifiers, unknown fields, and
self-rehashed packet tampering fail closed. `validate_birth_witness_packet`
reconstructs the entire packet from the authoritative candidate, decision,
provisioned policy, and signed attestations rather than trusting caller-supplied
hashes.

## Validation

Validation includes required memory-reference fields, redaction checks,
capability-envelope checks, witness/receipt fixtures, and private-state denial
cases. The focused `capability_envelope` integration target covers deterministic
positive construction and comprehensive negative evidence, authorization,
limit, provenance, parsing, privacy, portability, and packet-forgery cases.
The focused crate-internal `birth_witness::authority_tests` lane contains 13
tests covering the exact four-role signed witness surface, deterministic
accept/reject receipts, executable fixture negatives, signature and candidate
substitution, generation freshness, roster/policy binding, redaction and path
hygiene, and complete packet reconstruction. The separate public
`birth_witness` integration target contains one serialization-boundary test for
unknown-field rejection; it is not the authority proof. A compile-fail doctest
separately proves that external callers cannot establish the opaque authority
root.

## Source Inputs

- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `#3377`

## Scope

This feature should establish:

- memory grounding tied to witnessed artifacts
- capability envelopes covering provider, model, tool, skill, authority, and
  limit context at birth
- birth witnesses and citizen-facing receipt surfaces
- redaction-safe review posture for grounded memory and witnessed capability
  claims
- clear separation between witnessed capability context and later reputation or
  governance judgment

## Acceptance Criteria

- Memory-grounding references are reviewable and redaction-safe.
- Capability envelope records provider, model, tool, skill, authority, denial,
  limit, provenance, and unsupported-claim context.
- Witness and receipt surfaces exist.
- Birthday packet can cite these surfaces without exposing raw private state.

## Risks

- Reviewers may ask for raw memory. Mitigation: provide witnessed references
  and redacted projections.
- Capability envelopes may overclaim. Mitigation: require explicit limits.

## Future Work

Later milestones can expand memory palace, reputation, economics, and richer
witness authority once governance work lands.

## Notes

This feature is the practical evidence bridge between identity and review.

## Non-goals

- raw private-state exposure
- unconstrained memory-palace implementation
- production contract-market or payments work

## Completion Target

`v0.92`
