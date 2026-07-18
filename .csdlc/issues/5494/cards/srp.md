# Structured Review Prompt

Template: 1.0.0

Issue: 5494

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime/src/runtime_api_auth.rs
adl-runtime/src/topology.rs
adl/src/csm_runtime_api.rs
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
    "id": "F-5494-1",
    "severity": "p1",
    "summary": "Expired non-revoked credentials cannot recover after the renewal window is missed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5494-2",
    "severity": "p1",
    "summary": "Readiness exempts Audit and Evidence channels that channel policy classifies as required.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5494-3",
    "severity": "p1",
    "summary": "CSM weather readiness overclaims CPU, memory, and GPU observations from disk pressure only.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5494-4",
    "severity": "p1",
    "summary": "The synthetic supervisor test does not execute the production daemon cycle or typed channel fabric.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5494-5",
    "severity": "p2",
    "summary": "Credential audit events record a fixed overlap instead of the clipped actual duration.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5494-6",
    "severity": "p2",
    "summary": "Repair documentation and the sprint register overstate the first revision's proof.",
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

- Strict all-target Clippy is blocked by two pre-existing cav.rs test warnings outside #5494.

## Review Result

Revision: Some("git-blake3:6c779b1aba99312e79ff2cd801c8b9f8e166da6c:1bdeb0703abc8ad44b84baaee8272494b6df2eb4a440c78e31bf915025a811b8")

Reviewer: Some("subagent:019f7455-defb-7250-a22e-26c352e90d0d")

Result: changes_required
