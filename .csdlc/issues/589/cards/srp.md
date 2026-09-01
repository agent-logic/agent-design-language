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

[
  {
    "id": "F-589-LIVE-1",
    "severity": "p2",
    "summary": "The SOR omits the required SSM instance ID and combines SSM and live roster claims under one command that cannot reproduce both evidence surfaces.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The live roster is a mutable point-in-time surface.

## Review Result

Revision: Some("git-blake3:ac8b65c7314f548d8b15a8501916d828d8d249d5:126abf6a62e38078483f74a2a73c3fac18a7c2d44f7cabaf06937be06f810a20")

Reviewer: Some("subagent:/root/issue_589_review")

Result: changes_required
