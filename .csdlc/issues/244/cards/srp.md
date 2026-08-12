# Structured Review Prompt

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/Cargo.toml
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

Revision: Some("git-blake3:b0383b06ccd782f0d32ba9d0c1bb9336ea3b884a:415831f8e539fcd90e9102ebef92a6d5ffd63d2f927312b70e00c6bf9bedc0f7")

Reviewer: Some("subagent:review_244_cleanup_race")

Result: pass
