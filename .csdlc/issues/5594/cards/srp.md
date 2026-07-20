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
    "id": "WP01-ER6",
    "severity": "p1",
    "summary": "Signing verification and trust policy was routed to Runtime Parity-C instead of ADL v2 WP-07 signing owner #5342",
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

- Structural validation cannot by itself prove semantic ownership decisions.

## Review Result

Revision: Some("git-blake3:c43967edbe804b8e89509ae2d59fcb22e67f98b0:1af897753270b2f2ffd86e141e33713c529a47010d706aee7d680334579370aa")

Reviewer: Some("subagent:Popper:019f7ddf-df65-7ec2-a4be-6c5b3efa28b1")

Result: changes_required
