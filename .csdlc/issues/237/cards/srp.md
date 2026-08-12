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
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/conversation_sessions_tests.rs
adl-runtime-kernel/src/parity.rs
adl-runtime-kernel/src/bin/adl-runtime-shadow-fixture.rs
adl-runtime-kernel/tests/parity.rs
.csdlc/issues/237

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

Revision: None

Reviewer: None

Result: pre_review
