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

- The GitHub Linux CI lane must rerun against the repaired exact revision before integration can be claimed green.

## Review Result

Revision: Some("git-blake3:962847008e33859046897ce035b8bf8b784e82f8:3846a742e1976114d0243de1af566fa1e03f5e9e415545222af78a4885d9eb03")

Reviewer: Some("subagent:/root/issue_589_review")

Result: pass
