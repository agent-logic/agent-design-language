# Structured Review Prompt

Template: 1.0.0

Issue: 5494

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

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
    "id": "F-5494-1",
    "severity": "p1",
    "summary": "Expired non-revoked credentials cannot recover after the renewal window is missed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:4d876fcd0168c633bb48584fb3f91ca00a800b24:1144caf3ba38b2b2b78179bc8eefffbe59656c57f42904084b82e21712856927",
    "route": null
  },
  {
    "id": "F-5494-2",
    "severity": "p1",
    "summary": "Readiness exempts Audit and Evidence channels that channel policy classifies as required.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:4d876fcd0168c633bb48584fb3f91ca00a800b24:1144caf3ba38b2b2b78179bc8eefffbe59656c57f42904084b82e21712856927",
    "route": null
  },
  {
    "id": "F-5494-3",
    "severity": "p1",
    "summary": "CSM weather readiness overclaims CPU, memory, and GPU observations from disk pressure only.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:4d876fcd0168c633bb48584fb3f91ca00a800b24:1144caf3ba38b2b2b78179bc8eefffbe59656c57f42904084b82e21712856927",
    "route": null
  },
  {
    "id": "F-5494-4",
    "severity": "p1",
    "summary": "The synthetic supervisor test does not execute the production daemon cycle or typed channel fabric.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:4d876fcd0168c633bb48584fb3f91ca00a800b24:1144caf3ba38b2b2b78179bc8eefffbe59656c57f42904084b82e21712856927",
    "route": null
  },
  {
    "id": "F-5494-5",
    "severity": "p2",
    "summary": "Credential audit events record a fixed overlap instead of the clipped actual duration.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:4d876fcd0168c633bb48584fb3f91ca00a800b24:1144caf3ba38b2b2b78179bc8eefffbe59656c57f42904084b82e21712856927",
    "route": null
  },
  {
    "id": "F-5494-6",
    "severity": "p2",
    "summary": "Repair documentation and the sprint register overstate the first revision's proof.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:4d876fcd0168c633bb48584fb3f91ca00a800b24:1144caf3ba38b2b2b78179bc8eefffbe59656c57f42904084b82e21712856927",
    "route": null
  },
  {
    "id": "F-5494-7",
    "severity": "p1",
    "summary": "Near-expiry rotation can retain a previous credential that does not outlive the replacement creation timestamp.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:4d876fcd0168c633bb48584fb3f91ca00a800b24:1144caf3ba38b2b2b78179bc8eefffbe59656c57f42904084b82e21712856927",
    "route": null
  },
  {
    "id": "F-5494-8",
    "severity": "p1",
    "summary": "Concurrent rotation can overwrite a terminal revocation and restore an active credential.",
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

Revision: Some("git-blake3:4d876fcd0168c633bb48584fb3f91ca00a800b24:1144caf3ba38b2b2b78179bc8eefffbe59656c57f42904084b82e21712856927")

Reviewer: Some("subagent:019f746f-a6ca-7333-809c-ccce6be3f702")

Result: changes_required
