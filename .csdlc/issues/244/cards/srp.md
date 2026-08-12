# Structured Review Prompt

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/tests/conversation_sessions.rs
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

Revision: Some("git-blake3:60c14bdad340ddcdbcd47a967efaa43553d6ca8a:b85895c30db1869a131cc0c1df514b46cdecbaa0f66135d716f7f9adb8e55334")

Reviewer: Some("subagent:review_244_cleanup_race")

Result: pass
