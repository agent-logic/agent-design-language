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
adl/src/csm_runtime_api.rs
adl/src/long_lived_agent.rs
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
    "id": "F-5494-9",
    "severity": "p1",
    "summary": "Concurrent terminal revocation can commit after the initial revocation check and before authorize returns authenticated.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5494-10",
    "severity": "p2",
    "summary": "Previous-generation bearer overlap does not accept gateway identity signatures created with that same overlapping credential.",
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

- none

## Review Result

Revision: Some("git-blake3:51f97887673f338ba861282a976dd8578f345864:82d9a9a6b7f812166547662659949b3fc8a8fabaf79e6b3e4c74ae40e3ff75bb")

Reviewer: Some("subagent:019f74ae-d602-7d20-b7ef-e8964d851ea2")

Result: changes_required
