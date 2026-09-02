# Structured Output Record

Template: 1.0.0

Issue: 648

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the corrective provider reload ownership repair after PR #646 merged stale by replaying the run-scoped handle fix through issue #648.

## Artifacts

- adl/src/provider/reload.rs
- adl/src/execute/mod.rs
- adl/src/execute/runner.rs
- adl/src/execute/tests.rs
- adl/src/long_lived_agent.rs
- .csdlc/prepared/issues/648/validate-provider-reload-corrective.sh
- adl/src/provider/reload.rs
- adl/src/execute/mod.rs
- adl/src/execute/runner.rs
- adl/src/execute/tests.rs
- adl/src/long_lived_agent.rs
- .csdlc/prepared/issues/648/validate-provider-reload-corrective.sh

## Execution

- Added run-scoped ProviderReloadHandle propagation through sequential, concurrent, retry, and called-workflow execution paths.
- Changed production CSM adl_workflow hotload ownership to retain and pass an explicit run-scoped provider reload handle instead of depending on process-global registration.
- Retained compatibility global registration with identity-aware guard clearing so older guards cannot clear newer registrations.
- Added overlap, shutdown-order, in-flight snapshot, and direct global guard regression coverage.
- Kept validation local and offline; no live provider, AWS, paid runner, Runtime restart, cutover, or merge action was performed.
- Added run-scoped ProviderReloadHandle propagation through sequential, concurrent, retry, and called-workflow execution paths.
- Changed production CSM adl_workflow hotload ownership to retain and pass an explicit run-scoped provider reload handle instead of depending on process-global registration.
- Retained compatibility global registration with identity-aware guard clearing so older guards cannot clear newer registrations.
- Added overlap, shutdown-order, in-flight snapshot, and direct global guard regression coverage.
- Kept proof local and offline without live provider, AWS, paid runner, Runtime restart, cutover, or merge action.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/648/validate-provider-reload-corrective.sh",
      "production"
    ],
    "purpose": "Prove production provider profile, provider reload, in-flight snapshot, and CSM hotload-owner behavior after the run-scoped reload handle repair.",
    "outcome": "passed",
    "evidence_ref": "terminal:2026-09-02:#648-production-validation:14-provider-profile-7-provider-reload-1-inflight-1-csm-passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/648/validate-provider-reload-corrective.sh",
      "safety"
    ],
    "purpose": "Prove config reload safety, provider reload ownership regressions, fmt, clippy, and diff hygiene without live provider, AWS, paid runner, Runtime restart, cutover, or merge action.",
    "outcome": "passed",
    "evidence_ref": "terminal:2026-09-02:#648-safety-validation:2-config-14-profile-7-reload-1-inflight-1-csm-fmt-clippy-diffcheck-passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/648/validate-provider-reload-corrective.sh",
      "production"
    ],
    "purpose": "Prove production provider profile, provider reload, in-flight snapshot, and CSM hotload-owner behavior after the run-scoped reload handle repair.",
    "outcome": "passed",
    "evidence_ref": "provider-reload-corrective-production.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/648/validate-provider-reload-corrective.sh",
      "safety"
    ],
    "purpose": "Prove config reload safety, provider reload ownership regressions, fmt, clippy, and diff hygiene without live provider, AWS, paid runner, Runtime restart, cutover, or merge action.",
    "outcome": "passed",
    "evidence_ref": "provider-reload-corrective-safety.log"
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
