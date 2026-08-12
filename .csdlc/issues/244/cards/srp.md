# Structured Review Prompt

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/tests/parity.rs
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

Revision: Some("git-blake3:6fe1a034113416ab245b5335ee43c229add4083e:d50487a39b0306f290198dfca02dbcf7cb78b20c7b543ab597b228c8fcbd0b24")

Reviewer: Some("subagent:issue244_ci_fix_review")

Result: pass
