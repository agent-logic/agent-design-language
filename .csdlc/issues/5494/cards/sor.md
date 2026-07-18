# Structured Output Record

Template: 1.0.0

Issue: 5494

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Eliminate the near-expiry credential-store race by using one captured timestamp for the complete rotation transaction.

## Artifacts

- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/src/topology.rs
- adl/src/csm_runtime_api.rs
- docs/review-fixes/runtime/WP07A_REARCHITECTURE_REPAIR_5409.md
- docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md
- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/src/supervision.rs
- adl-runtime/src/topology.rs
- adl/src/csm_runtime_api.rs
- adl/src/long_lived_agent.rs
- adl/src/long_lived_agent/tests.rs
- docs/review-fixes/runtime/WP07A_REARCHITECTURE_REPAIR_5409.md
- docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md
- adl-runtime/src/runtime_api_auth.rs

## Execution

- Report the real daemon-supervised-cycle production model and remove static all-ready topology claims
- Normalize all sixteen component observations and fail readiness closed for required unhealthy components or typed channels
- Run 100 supervised Tokio task/channel cycles with injected failure, restart, recovery, and retained replay
- Retain one previous API bearer generation for five minutes while terminal revocation rejects both generations
- Renew expired non-revoked credentials and report the actual clipped overlap duration
- Derive required readiness directly from runtime channel policy, including Audit and Evidence
- Keep weather in Runtime v3 and limit the CSM production assembly to its fifteen owned components
- Exercise the exact production daemon cycle for 100 real ticks with injected failure and recovery
- Correct the retained repair document and sprint register to match the proving implementation
- Delegate public rotation to a deterministic rotate_at transaction
- Use the same timestamp for prior-generation overlap and replacement creation
- Add a deterministic one-second-overlap regression test

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
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Prove credential recovery, exact overlap reporting, CSM ownership boundaries, and Runtime v3 weather independence",
    "outcome": "passed",
    "evidence_ref": "local FastWork: 121 unit tests plus 1 independence test passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "csm_runtime_api"
    ],
    "purpose": "Prove all policy-required channel blockers, fifteen CSM component observations, credential renewal, and revocation",
    "outcome": "passed",
    "evidence_ref": "local FastWork: 44 focused CSM runtime API tests passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "long_lived_agent::tests::production_daemon_cycle_soak_runs_real_ticks_channels_and_recovery",
      "--",
      "--exact",
      "--nocapture"
    ],
    "purpose": "Prove 100 production tick cycles over typed Runtime v3 channels with one injected failure and recovery on the same context",
    "outcome": "passed",
    "evidence_ref": "local FastWork: 100 successful production cycles, one injected failure, recovery, 27.78 seconds"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "runtime_api_auth::tests"
    ],
    "purpose": "Prove credential creation, renewal, expired recovery, overlap, revocation, and the exact one-second rotation boundary",
    "outcome": "passed",
    "evidence_ref": "local FastWork: 9 credential-store tests passed"
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
