# Structured Review Prompt

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

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

- Live Runtime and cloud readbacks remain mutable point-in-time evidence.

## Review Result

Revision: Some("git-blake3:96f81c6ef45b96495a2a07f68a26f303f6a73eb3:b9d678f48cd8d2a4f18e2dc6e42800137c54b09f05e906f4e806f889126e89c7")

Reviewer: Some("subagent:/root/issue_589_review")

Result: pass
