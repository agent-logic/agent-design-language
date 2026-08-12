# Structured Review Prompt

Template: 1.0.0

Issue: 237

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/capability_envelope.rs
adl-runtime-kernel/src/cognitive_profile.rs
adl-runtime-kernel/tests/capability_envelope.rs
adl-runtime-kernel/tests/fixtures/cognitive_profile/authority_tests.rs

## Prompts

- Does capability verify the actual BirthdayContinuityRecord rather than trust a caller digest?
- Can any substituted head or identity binding pass?
- Did the change weaken authority signatures, policy/evidence digests, or privacy?
- Is the real positive free of fixtures and cached proof?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:834be4fe9a99ad90aabe7f627f6ed9699efa55db:3929bcd1f3a03eebf8738702e70f6510776338f05b2723448cd15c55c5e3d59a")

Reviewer: Some("/root/review_237_opaque_authority")

Result: pass
