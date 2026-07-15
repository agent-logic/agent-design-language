# Structured Review Prompt

Template: 1.0.0

Issue: 5383

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

docs/milestones/v0.91.8
docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md
docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md
docs/milestones/v0.92/README.md

## Prompts

- Review whether the v0.91.8 package is complete enough for execution without implementation overclaims.
- Review whether #4641/#5384 routing is clear and not lossy.
- Review whether the issue-wave YAML preserves live issue topology and dependencies.

## Findings

[
  {
    "id": "review-5383-p2-v092-success-criterion",
    "severity": "p2",
    "summary": "v0.92 README success criteria initially omitted the v0.91.8 bridge prerequisite.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:2c4527bdfb6bd9572053d7a8387967be3d3123d4:b46e9be6bf70ef16cb6771742ecd2e814c115768a00d8f87655333cce20c094b",
    "route": null
  },
  {
    "id": "review-5383-p3-sidecar-wp-field",
    "severity": "p3",
    "summary": "WP-14 sidecar routes initially used wp: WP-14A for live WP-14-labeled issues instead of separating live and parent WP routing.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:2c4527bdfb6bd9572053d7a8387967be3d3123d4:b46e9be6bf70ef16cb6771742ecd2e814c115768a00d8f87655333cce20c094b",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC state is maintained by typed v2 lifecycle commands; publication review scope is limited to substantive planning docs.

## Review Result

Revision: Some("git-blake3:2c4527bdfb6bd9572053d7a8387967be3d3123d4:b46e9be6bf70ef16cb6771742ecd2e814c115768a00d8f87655333cce20c094b")

Reviewer: Some("subagent:019f66ad-cc78-76b3-ace1-6a3b78e3e01d")

Result: pass
