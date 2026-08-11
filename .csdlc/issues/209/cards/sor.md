# Structured Output Record

Template: 1.0.0

Issue: 209

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired the production kernel ACIP replay authority, typed pressure/error proof, and retained legacy signed-frame replay isolation required before #5834 can cite WP-14.

## Artifacts

- adl-runtime-kernel/src/acip.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/production_acip_wss.rs
- adl-runtime/src/runtime_api_auth.rs
- docs/api/runtime-v3/v1/openapi.json
- docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md
- .csdlc/prepared/issues/209/produce-native-receipt.rb
- .csdlc/prepared/issues/209/validate-native-receipts.rb
- .github/workflows/wp14-production-acip-repair.yml

## Execution

- Scoped production replay state by authenticated credential plus runtime/source domain and rejected terminal, excessive, replayed, and capacity-exhausting advances without eviction.
- Added real production binary dispatch, typed rejection rollback, corrected retry, replay-domain isolation, capacity fail-closed, and bounded canonical-ingress pressure proof.
- Scoped retained legacy signed-frame replay state by credential generation and bounded terminal/excessive progression.
- Aligned current OpenAPI and feature truth and added exact Linux/macOS native receipt surfaces.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "runtime_api_auth::tests::wss_admission_fails_before_dispatch_for_auth_origin_authority_and_replay",
      "--lib",
      "--",
      "--exact"
    ],
    "purpose": "Run the exact legacy admission authority regression.",
    "outcome": "passed",
    "evidence_ref": "legacy-signed-replay.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/209/produce-native-receipt.rb",
      "--self-test"
    ],
    "purpose": "Run both issue-local receipt contract self-tests.",
    "outcome": "passed",
    "evidence_ref": "native-contract-selftests.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--test",
      "production_acip_wss",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warning-bearing production changes.",
    "outcome": "passed",
    "evidence_ref": "production-acip-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "production_acip_wss"
    ],
    "purpose": "Run the focused production kernel ACIP target.",
    "outcome": "passed",
    "evidence_ref": "production-acip-wss.log"
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
