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
    "id": "WP01-ER1",
    "severity": "p1",
    "summary": "The 123-row feature-preservation gate lacks retained per-row owner and cutover dispositions",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "issue_5594"
  },
  {
    "id": "WP01-ER2",
    "severity": "p2",
    "summary": "Exact commit patch contains four trailing-whitespace lines while SOR records diff hygiene passed",
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

- Feature dispositions require a row-level retained artifact before Runtime v2 deletion planning can be trusted.

## Review Result

Revision: Some("git-blake3:de1011a8bfc95cea511782a66228f2ff83359b43:c463e796007f3e8491305624eb0001b85872cebe386a8727321a95fb032a7326")

Reviewer: Some("subagent:Planck:019f8057-e16d-76f1-99c1-5ef2ea96f133")

Result: changes_required
