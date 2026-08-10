# v0.92 Feature: ACP / Cognitive Profiles

## Metadata

- Feature Name: ACP / Cognitive Profiles
- Milestone Target: `v0.92`
- Status: implemented locally; native dual-platform proof pending CI
- Related issues: `#3377`, `#3434`, `#5830`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This is a planning feature doc. It records intended scope and proof surfaces,
not implementation closeout.

## Status

The bounded WP-13 contract is implemented by `adl-runtime-kernel` for `#5830`.
Publication and dual-platform native receipts remain separate integration proof.

## Purpose

Define runtime-visible ACP / cognitive profile records that stay grounded in
identity, continuity, memory, capability, Theory of Mind, intelligence, and
governed-learning evidence without collapsing into reputation or public
standing.

## Context

ACP profiles bind the accepted WP-08 birthday candidate, WP-09 identity record,
WP-10 continuity record, and WP-12 capability envelope. Memory, capability,
Theory-of-Mind, intelligence, and governed-learning entries remain explicit
evidence categories rather than inferred labels. Profiles support birthday
review without replacing identity, reputation, moral standing, or citizenship.

## Coverage / Ownership

v0.92 owns the first bounded ACP profile contract and fixtures needed for the
birthday packet. Later milestones may expand profile usage after governance
rules mature.

## Overview

The profile should be a runtime-visible evidence map: what memory, capability,
continuity, ToM, intelligence, and learning evidence is available, what is
private, and which claims remain unsupported.

## Design

The versioned `adl.cognitive_profile.v1` record contains a stable profile ID,
monotonic revision, predecessor digest, update actor and rationale, exact
authority digests, an evidence map, bounded fields, redaction-policy digest,
explicit nonclaims, and canonical input/policy/profile digests.

Every evidence reference carries a closed category, repository-relative path,
content digest, revision digest, and `public` or `internal_redacted` visibility.
Policy and input identifiers are collision-checked case-insensitively. Public
fields may cite only public evidence. The generated public projection omits all
evidence links and update metadata and must be strictly narrower than the
internal profile.

Updates bind the previous profile ID, identity root, policy digest, previous
profile digest, and exact field additions/removals. The exported validator
reconstructs the complete record from its authorities and rejects any mismatch.

## Execution Flow

1. Revalidate the accepted birthday, identity, continuity, and capability
   authorities through their exported validators and digests.
2. Match every input evidence reference and field against the canonical policy.
3. Validate nonclaims, privacy, revision linkage, and exact update delta.
4. Canonicalize and hash the input, policy, internal record, and public
   projection.
5. Reconstruct the complete exported record during validation.

## Determinism and Constraints

Profiles are deterministic over cited evidence. Unknown JSON fields, stale or
missing evidence, policy collisions, authority substitution, unexplained
updates, unsafe paths, secrets, raw/private state, diagnosis, standing,
personhood, reputation, consciousness, citizenship, and rights inference fail
closed. Rejection payloads contain only closed error codes/categories and never
echo attacker-controlled strings.

## Integration Points

- v0.91.1 memory/identity, ToM, intelligence, and learning evidence.
- v0.92 birthday packet and capability envelope.
- v0.93 governance handoff as a consumer, not an owner.

## Validation

The focused `cognitive_profile` integration target contains 11 deterministic
tests covering positive creation/update/round-trip behavior and the complete
negative matrix above. Its fixture inventory is retained under
`adl-runtime-kernel/tests/fixtures/cognitive_profile/`.

Issue-local native receipt tooling runs that exact target on macOS and Linux,
records its exact structured test inventory and semantic digest, binds source
and producer digests to the candidate head, normalizes checkout paths, and
rejects retained machine-local paths. Native receipts are integration evidence;
they are not claimed until the GitHub workflow produces and validates them.

## Source Inputs

- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `#3377`

## Scope

This feature should establish:

- ACP/profile records as bounded runtime-visible contracts
- profile update rules tied to witnessed evidence rather than free-floating
  labels
- privacy and projection boundaries for profile use
- distinction between profile, identity, reputation, and public standing
- consumption of `v0.91.1` memory, capability, ToM, intelligence, and
  governed-learning evidence

## Acceptance Criteria

- ACP/profile schema and fixtures exist.
- Profile claims cite allowed evidence references.
- Unsupported reputation, personhood, and standing claims are rejected.
- Review packet includes profile evidence and non-claims.

## Risks

- Profiles could become horoscope-like labels. Mitigation: require evidence
  links and explicit non-claims.
- Profiles could leak private state. Mitigation: require redaction policy and
  allowed projection boundaries.

## Future Work

Future milestones may connect ACP profiles to governance, reputation, and
cross-polis exchange after v0.93 rules exist.

## Notes

This feature is intentionally narrower than citizenship or reputation.

## Non-goals

- scalar moral verdicts
- reputation replacement
- public standing by profile inference alone
- unaudited private-state exposure

## Completion Target

`v0.92`
