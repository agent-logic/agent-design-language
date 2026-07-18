# Structured Output Record

Template: 1.0.0

Issue: 5494

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Aligned reported topology with the daemon-supervised cycle, derived readiness from observed required components and channels, replaced the static soak with supervised execution and recovery, and added bounded credential-generation overlap.

## Artifacts

- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/src/topology.rs
- adl/src/csm_runtime_api.rs
- docs/review-fixes/runtime/WP07A_REARCHITECTURE_REPAIR_5409.md
- docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md

## Execution

- Report the real daemon-supervised-cycle production model and remove static all-ready topology claims
- Normalize all sixteen component observations and fail readiness closed for required unhealthy components or typed channels
- Run 100 supervised Tokio task/channel cycles with injected failure, restart, recovery, and retained replay
- Retain one previous API bearer generation for five minutes while terminal revocation rejects both generations

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Prove supervision, topology, credential overlap, weather independence, and the real 100-cycle task/channel soak",
    "outcome": "passed",
    "evidence_ref": "local FastWork: 122 unit tests plus 1 independence test passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "csm_runtime_api"
    ],
    "purpose": "Prove observed component/channel readiness, fail-closed missing observations, and HTTP credential overlap/revocation",
    "outcome": "passed",
    "evidence_ref": "local FastWork: 44 focused CSM runtime API tests passed"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
