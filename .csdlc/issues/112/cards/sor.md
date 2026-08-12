# Structured Output Record

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented Runtime-owned Layer 8 conversation authority, one signed identity-message contract for operator and agent senders, recipient-signed acknowledgement verification, and a reachable disclosure-safe Observatory conversation surface.

## Artifacts

- adl-runtime-kernel/src/layer8_authority.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/conversation_sessions.rs
- adl-runtime/tests/layer8_authority.rs
- adl/tests/layer8_authority_runtime_api.rs
- adl/tools/validate_layer8_authority_observatory_ui.sh
- demos/html-observatory/app.js
- demos/html-observatory/styles.css
- docs/milestones/v0.92/features/LAYER8_CONVERSATION_AUTHORITY.md

## Execution

- Enforced least-privilege identity, capability, policy, replay, refusal, and redacted hash-chained audit decisions before conversation reservation and provider dispatch.
- Added one canonical signed ACIP identity-message contract with externally held per-principal key verification and exact recipient binding for human-agent and agent-agent senders.
- Added recipient-signed acknowledgement binding and adversarial signature substitution, rotation, revocation, expiry, recipient widening, replay, restart, and audit corruption coverage.
- Restored reachability of the merged Observatory conversation panel through a dedicated communication dashboard surface and real-browser proof.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "layer8_authority",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Run the exact nonzero authority integration target.",
    "outcome": "passed",
    "evidence_ref": "layer8-authority-contract.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/validate_layer8_authority_observatory_ui.sh"
    ],
    "purpose": "Run the exact real-browser Observatory contract.",
    "outcome": "passed",
    "evidence_ref": "layer8-observatory-ui.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_sessions",
      "--no-fail-fast"
    ],
    "purpose": "Run the exact production conversation integration target.",
    "outcome": "passed",
    "evidence_ref": "layer8-production-conversation-boundary.log"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "layer8_authority_runtime_api",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Run the exact nonzero Runtime API integration target.",
    "outcome": "passed",
    "evidence_ref": "layer8-runtime-api-integration.log"
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
