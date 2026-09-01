# Structured Review Prompt

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/589
adl-runtime-kernel
adl-runtime
adl
infra/aws/csm-runtime-health
infra/runtime-v3/runtime-init.toml

## Prompts

- Verify startup no longer requires the separate continuity channel while Guardian ownership remains intact.
- Verify stale-state recovery cannot remove a lock owned by a live writer.
- Verify reload preserves the last known-good running configuration on candidate failure.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:384c8230545e340703c07122cea9aee9795dcce0:75f9bf27302cc4d2e81170582b1ee25dc4eabca1609b3606bd9cafbcec50c0e6")

Reviewer: Some("subagent:/root/issue_589_review")

Result: pass
