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
.csdlc/evidence/244
.csdlc/issues/244

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

Revision: Some("git-blake3:379a08a71e112368b79e6bef6ea083cc767562fd:da2510ad78ed130513f25aa89de02e8b3501f8aea774ee2e32c0dd4d3b287967")

Reviewer: Some("subagent:review_244_cleanup_race")

Result: pass
