# Structured Output Record

Template: 1.0.0

Issue: 622

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented production provider/profile hot loading through a provider reload owner, sidecar validation, immutable per-step runner snapshots, last-known-good preservation on invalid updates, credential-value rejection, and focused production/safety proof lanes.

## Artifacts

- adl/src/provider/reload.rs
- adl/src/provider/mod.rs
- adl/src/provider/local.rs
- adl/src/execute/runner.rs
- adl/src/execute/tests.rs
- adl-runtime-kernel/src/config_reload.rs
- .csdlc/prepared/issues/622/validate-provider-profile-hotload.sh
- docs/providers/provider-profile-hot-loading.md

## Execution

- Added a provider reload owner and global handle in adl/src/provider/reload.rs for provider-only sidecar loading, validation, diagnostics, stable provider digests, shutdown, and last-known-good preservation.
- Wired the production execution runner to resolve provider specs from the current reload snapshot for each step while preserving the in-flight step's start snapshot.
- Added deterministic mock-provider configuration hooks for fixed output and bounded sleep to prove reload behavior without external provider calls.
- Added focused provider reload, production runner, and runtime-kernel watcher tests covering valid reloads, invalid reload retention, credential-value rejection, duplicate debounce, shutdown, and in-flight snapshot isolation.
- Updated the #622 validation script to run focused production and safety lanes that avoid unrelated binary compile surfaces.
- Documented provider profile hot-loading behavior, accepted sidecar shape, credential boundary, snapshot semantics, validation evidence, and non-goals.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/622/validate-provider-profile-hotload.sh",
      "production"
    ],
    "purpose": "Prove the production provider/profile hot-loading lane, including existing provider profile invariants, accepted reloads without restart, invalid reload last-known-good retention, credential-value rejection, and in-flight step snapshot isolation.",
    "outcome": "passed",
    "evidence_ref": "terminal:issue-622-production-lane:provider_mod_profile 14 passed; provider_reload 4 passed; in-flight snapshot 1 passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/622/validate-provider-profile-hotload.sh",
      "safety"
    ],
    "purpose": "Prove the safety lane for reload debounce, shutdown, invalid-update last-known-good retention, credential boundary, focused production invariants, and whitespace/diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "terminal:issue-622-safety-lane:config_reload 2 passed; provider_mod_profile 14 passed; provider_reload 4 passed; in-flight snapshot 1 passed; git diff --check passed"
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
