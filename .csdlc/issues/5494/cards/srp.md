# Structured Review Prompt

Template: 1.0.0

Issue: 5494

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/runtime_api_auth.rs
adl-runtime/src/supervision.rs
adl-runtime/src/topology.rs
adl/src/cli/csmctl_cmd.rs
adl/src/csm_runtime_api.rs
adl/src/long_lived_agent/tests.rs
docs/review-fixes/runtime/WP07A_REARCHITECTURE_REPAIR_5409.md
docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md

## Prompts

- Does production execute rather than merely describe the assembly?
- Can any required missing or unhealthy observation leave readiness green?
- Does the soak drive real tasks, channels, failure, and recovery?
- Is credential overlap bounded without weakening revocation?

## Findings

[
  {
    "id": "F-5494-11",
    "severity": "p2",
    "summary": "The soak claims durable replay without asserting retained sequence continuity and the failure-to-restart-to-ready lifecycle transition.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5494-12",
    "severity": "p3",
    "summary": "Ordinary authorization takes the exclusive credential mutation lock before acquiring the shared authorization lock.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI must confirm the revised coverage lane after both findings are fixed.

## Review Result

Revision: Some("git-blake3:b1c45a6a74e37c7517f9c916ee16f308508b4e60:7637be99e867490334876bcc8882bf913f0d6d9648bbe901bafe8eea86043ed9")

Reviewer: Some("subagent:019f7581-a4bf-7fb3-a900-3d71dfea4abc")

Result: changes_required
