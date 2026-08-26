# Structured Review Prompt

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/conversation_sessions_tests.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/src/parity.rs
adl-runtime-kernel/src/bin/adl-runtime-shadow-fixture.rs
adl-runtime-kernel/tests/parity.rs
.csdlc/issues/244
.csdlc/prepared/issues/244
.csdlc/evidence/244

## Prompts

- Does queuing the duplicate behind re-authentication preserve server frame order and the authentication-generation transition?
- Does the test attach to the existing in-flight turn before its existing deadline without changing production behavior?
- Are duplicate attachment and exactly-once terminal semantics preserved, with no #112 authority changes?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:f186a0dc3915a9528d81d5b96c858d1882c5fc7e:961651c7b436c3ae4637c2bfa148d12ba4381bb1fad5a71842f385cf55a328d3")

Reviewer: Some("codex-subagent:review_244_248_combined")

Result: pass
