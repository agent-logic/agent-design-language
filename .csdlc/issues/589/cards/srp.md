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

- Live AWS operations were not repeated for the lifecycle-only final commit; the retained live proof remains bound to the unchanged implementation.

## Review Result

Revision: Some("git-blake3:cb0d306132c2762e64f4a9d7a271ac4605e6d242:55d600c98932c61548e230c6e42075d07649b60d0efdf9235b14f2714a4f8d8e")

Reviewer: Some("subagent:/root/issue_589_review")

Result: pass
