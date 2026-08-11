# ADR 0063: ACP Cognitive Profile Evidence Boundary

## Status

Status: **Proposed**

## Context

ACP profile code, focused tests, and corrective authority issue #144 are merged
and ancestral to this packet.

## Decision

ACP remains an evidence-grounded profile projection whose authority is anchored
to runtime-owned identity, continuity, capability, and predecessor evidence.
Public projection is strictly narrower than the private governed profile and
does not become identity or rights authority.

## Consequences

Consumers receive a canonical, integrity-checked profile and bounded public
projection without inheriting raw evidence or ambient authority.

## Alternatives Considered

Treating a profile as self-authorizing identity, accepting a caller-nominated
authority anchor, or projecting private evidence publicly was rejected.

## Source Evidence

- `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md`
- `adl-runtime-kernel/src/cognitive_profile.rs`

## Validation Evidence

- `adl-runtime-kernel/tests/cognitive_profile.rs`
- `adl-runtime-kernel/tests/fixtures/cognitive_profile/authority_tests.rs`
- `.csdlc/evidence/144/cognitive-profile-authority-v1.log`
- `.csdlc/evidence/144/cognitive-profile-public-integration.log`
- `.csdlc/evidence/144/cognitive-profile-compile-fail.log`
- `.csdlc/evidence/144/local-validation-manifest.json`

## Supersession Relationships

Refines ADR 0016 and ADR 0019 without granting their broader moral or social
authority.

## Non-Claims

No reputation, consciousness, rights, personhood, or public standing is
claimed; the profile is not an identity root or governance decision.

## Approval Boundary

Human review must separately promote this candidate into `docs/adr/`.
