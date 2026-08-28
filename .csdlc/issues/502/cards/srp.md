# Structured Review Prompt

Template: 1.0.0

Issue: 502

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/AGENTS.md
csdlc-v3/Cargo.toml
csdlc-v3/Cargo.lock
csdlc-v3/src/adapters/mod.rs
csdlc-v3/src/lib.rs
csdlc-v3/src/lifecycle/mod.rs
csdlc-v3/src/storage/mod.rs
csdlc-v3/tests/transactions.rs
csdlc-v3/tests/foundation.rs

## Prompts

- Does every lifecycle command/state pair have an explicit capability-checked allowed or rejected outcome?
- Can any partial or interrupted write acquire authority?
- Does recovery replay preserve audit provenance and converge deterministically?
- Do typed adapters preserve argv/status/stdout/stderr/timeout/cancellation and credential-scope boundaries?
- Does csdlc-v3/AGENTS.md preserve the v2 authority boundary while making future v3 issue starts faster and simpler?
- Can any command or API surface be misread as C-SDLC v3 operational authority before cutover?

## Findings

[
  {
    "id": "pr-body-refresh",
    "severity": "p3",
    "summary": "PR #572 body still cites older review/validation truth and keeps Closes #502 at the bottom; route through typed publication refresh.",
    "actionable": true,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "typed csdlc-publish refresh before publication is considered current"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 remains construction-only; typed C-SDLC v2 remains live lifecycle and GitHub authority until explicit V3-F cutover.

## Review Result

Revision: Some("git-blake3:5d05309d3ad48deedfa86f43edef5fe84a3dad13:fcec36c2a16ad1204cc04eb9f4563271e650986231b0aa26bb6ef532eda1e5f7")

Reviewer: Some("issue_502_5d_review")

Result: pass
