# Structured Output Record

Template: 1.0.0

Issue: 693

Repository: agent-logic/agent-design-language

Card: sor

Status: ready

## Summary

Implemented Runtime-owned model-backed A2A selection through provider-native tool calls, with governed dispatch, sender-bound Layer8 authority for runtime-internal resident A2A, distinct initiating and recipient results, correlated completion observability, safe ordinary-reply fallback when a model rejects tools, and CI-compatible config-generation receipt setup for production-style Runtime v3 subprocess tests.

## Artifacts

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/telemetry.rs
- adl-runtime-kernel/tests/guardian_soak.rs
- adl-runtime-kernel/tests/parity_b_live_kernel.rs
- adl-runtime-kernel/tests/production_acip_wss.rs
- .csdlc/evidence/693/ci-red-remediation.md
- .csdlc/evidence/693/runtime-guardian-soak-ci-remediation.log
- .csdlc/evidence/693/review-finding-remediation.md
- .csdlc/evidence/693/runtime-v3-fast-ci-remediation.md

## Execution

- Replaced prompt-dependent exact JSON action selection with an Ollama-native initiate_agent tool contract normalized into the existing governed Runtime initiation intent.
- Kept Runtime authoritative for identifiers, admission, Layer8 dispatch, replay, cancellation, and terminal outcomes; malformed, unknown, or ambiguous actions fail closed.
- Preserved Beacon's operator-facing reply independently from Ember's provider result and emitted a separately correlated agent_to_agent_completed Runtime event.
- Exposed Ember's provider-generated output as the distinct initiated_reply field on the authoritative conversation result, while retaining Beacon's reply unchanged.
- Emitted agent_to_agent_initiated when governed dispatch is accepted, agent_to_agent_completed only after delivery, and agent_to_agent_failed for terminal non-delivery outcomes.
- Added bounded fallback to ordinary Ollama generation for 400/404 tool-unsupported responses without inferring or dispatching A2A.
- Added an isolated production-ingress Beacon-to-Ember acceptance using native provider tool output, real governed recipient execution, and correlated initiation/completion observation.
- Provisioned Runtime v3 configuration-generation environment in guardian soak subprocess tests so PR CI exercises the real configured runtime entrypoint.
- Bound runtime-internal A2A initiation to the active Layer8 signed exchange sender identity and added a mismatch regression so one resident's signer cannot authorize another resident as sender.
- Provisioned and activated config-generation receipts for the production-style parity-b live kernel and production ACIP WSS subprocess tests before spawning CARGO_BIN_EXE_adl-runtime-kernel.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "agent_to_agent_",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove Runtime-owned A2A action selection, runtime-internal resident-pair communication with sender-bound Layer8 authority, mismatch refusal, recipient delivery, and correlated observability.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/review-finding-remediation.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "guardian_soak",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove the CI-red guardian soak subprocess lane after provisioning runtime configuration generation for the real serve entrypoint.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/runtime-guardian-soak-ci-remediation.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "parity_b_live_kernel",
      "live_graph_executes_through_guardian_canonical_ingress",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove the CI-red parity-b live kernel subprocess starts the real runtime after config-generation receipt provisioning.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/runtime-v3-fast-ci-remediation.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "production_acip_wss",
      "production_binary_acip_wss_produces_observed_receipt",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove the sibling production ACIP WSS subprocess starts the real runtime after config-generation receipt provisioning.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/runtime-v3-fast-ci-remediation.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Prove the full Runtime kernel lane exercised by the runtime-v3-fast CI job after the production-style subprocess fixes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/runtime-v3-fast-ci-remediation.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "provider_conversation"
    ],
    "purpose": "Prove provider action normalization, failure behavior, and unsupported-tools ordinary-reply fallback.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/ci-red-remediation.md"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Rust formatting.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/runtime-v3-fast-ci-remediation.md"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict lint validation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/runtime-v3-fast-ci-remediation.md"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Diff hygiene.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/runtime-v3-fast-ci-remediation.md"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
