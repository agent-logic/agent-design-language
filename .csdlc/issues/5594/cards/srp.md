# Structured Review Prompt

Template: 1.0.0

Issue: 5594

Repository: danielbaustin/agent-design-language

Card: srp

Status: ready

## Scope

README.md
docs/planning/ADL_FEATURE_LIST.md
docs/milestones/v0.91.8
.csdlc/prepared/issues/5594

## Prompts

- Does every sprint have one real umbrella and a complete non-overlapping child set?
- Do canonical docs agree with live issue, PR, card, and dependency truth?
- Are parallel assignments collision-safe and dependency-correct?
- Did WP-01 avoid implementation and scope expansion?
- Are external-agent and merge authorities correctly bounded?

## Findings

[
  {
    "id": "WP01-ER7",
    "severity": "p1",
    "summary": "Provider transport signed-trace and five standalone tooling rows retained semantically incorrect or incomplete cutover owners",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "issue_5594"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Structural validation cannot establish semantic ownership correctness; exact reviewer inspection remains required.

## Review Result

Revision: Some("git-blake3:157b7608ec77cf7b008f388186848b418ee7690f:e4eba2757f882ed66fe9afcb0fa70cf609c5b2cafd3f4005b5755537e868cdce")

Reviewer: Some("subagent:Halley:019f7e77-cc24-7532-a693-7b04a46fe7d7")

Result: changes_required
