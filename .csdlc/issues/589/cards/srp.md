# Structured Review Prompt

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/cli/csm_runtime_v3_cmd.rs
.csdlc/issues/589

## Prompts

- Verify startup no longer requires the separate continuity channel while Guardian ownership remains intact.
- Verify stale-state recovery cannot remove a lock owned by a live writer.
- Verify reload preserves the last known-good running configuration on candidate failure.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted Linux CI remains the proving execution environment for the systemd command path; live Runtime and cloud readbacks are mutable point-in-time evidence.

## Review Result

Revision: Some("git-blake3:83b003b9ae24ec6a4f328a98154b3735642a61fa:37988ab7dddd698291ebce7ab2d51b0c8775cf72639846404074a880e8d3bcff")

Reviewer: Some("subagent:/root/issue_589_review")

Result: pass
