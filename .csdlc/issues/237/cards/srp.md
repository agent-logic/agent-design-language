# Structured Review Prompt

Template: 1.0.0

Issue: 237

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/birthday_continuity.rs
adl-runtime-kernel/src/capability_envelope.rs
adl-runtime-kernel/src/cognitive_profile.rs
adl-runtime-kernel/tests/capability_envelope.rs
adl-runtime-kernel/tests/fixtures/birthday_continuity/authority_tests.rs
adl-runtime-kernel/tests/fixtures/capability_envelope/authority_tests.rs
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

- Required hosted Runtime CI and live PR mergeability remain publication gates.

## Review Result

Revision: Some("git-blake3:7ce5852bcae2cb1f7c418a0d833c788841c0f0f1:bd1b292b3a57c60f7c940f9eac61f8cbb91d3b886fcf3fd0655d601318b017a4")

Reviewer: Some("/root/fix_5833_birth_witness_runtime/review_237_fresh_integrated")

Result: pass
