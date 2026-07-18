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
    "disposition": "fixed",
    "fix_revision": "git-blake3:e2723c8b7ee47f698cf0de36ab06442521c1fbb5:72cb1f5e2addc68c27d5389ef00ae8bb5d65532b64f6bc3da774284bc158ba27",
    "route": null
  },
  {
    "id": "F-5494-10",
    "severity": "p2",
    "summary": "Previous-generation bearer overlap does not accept gateway identity signatures created with that same overlapping credential.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e2723c8b7ee47f698cf0de36ab06442521c1fbb5:72cb1f5e2addc68c27d5389ef00ae8bb5d65532b64f6bc3da774284bc158ba27",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted PR checks and lifecycle closeout remain pending; no external provider, cloud, API Gateway, GPU, or Runtime v3 integration claim is made.

## Review Result

Revision: Some("git-blake3:e2723c8b7ee47f698cf0de36ab06442521c1fbb5:72cb1f5e2addc68c27d5389ef00ae8bb5d65532b64f6bc3da774284bc158ba27")

Reviewer: Some("subagent:019f74ae-d602-7d20-b7ef-e8964d851ea2")

Result: pass
