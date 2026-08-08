# Structured Review Prompt

Template: 1.0.0

Issue: 61

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/lifecycle.rs
csdlc-v2/tests/gate2.rs

## Prompts

- Does every relative stored worktree path resolve from one canonical repository topology root rather than a scanned projection root?
- Can any unrelated historical dot record still trigger card or authored-artifact verification?
- Do requested issue, matching branch, and matching canonical worktree records remain fully verified and fail closed?
- Does the regression invoke the real bind binary and reproduce missing historical artifacts?
- Do surviving filesystem errors preserve typed category while naming issue and path?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:9f778dd2ba8a1aa52fb0673cbd5525afe62ab360:dd5b16d7d067aa7068f6846eaa4904b6879b09ed6f5d908f20237e583d293bd3")

Reviewer: Some("subagent:review-61-final")

Result: pass
