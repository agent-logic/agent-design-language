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

- Fresh exact-head CI must confirm strict hosted Clippy after publication; live Runtime and cloud readbacks remain mutable point-in-time evidence.

## Review Result

Revision: Some("git-blake3:2a2ab485271cdcda11a2f34bb2c97a507f09fe38:3625684ad2c52c228371524fc80ea793a6a87c8690e7e9a7081633944b3b7ce0")

Reviewer: Some("subagent:/root/issue_589_review")

Result: pass
