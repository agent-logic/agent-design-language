# Structured Output Record

Template: 1.0.0

Issue: 693

Repository: agent-logic/agent-design-language

Card: sor

Status: ready

## Summary

Implemented Runtime-owned model-backed A2A selection through provider-native tool calls, with governed dispatch, sender-bound Layer8 authority for runtime-internal resident A2A, public raw A2A initiation fail-closed, distinct initiating and recipient results, correlated completion observability, safe ordinary-reply fallback when a model rejects tools, and CI-compatible config-generation receipt setup for production-style Runtime v3 subprocess tests.

## Artifacts

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/telemetry.rs
- adl-runtime-kernel/tests/guardian_soak.rs
- adl-runtime-kernel/tests/observatory.rs
- .csdlc/evidence/693/ci-red-remediation.md
- .csdlc/evidence/693/runtime-guardian-soak-ci-remediation.log
- .csdlc/evidence/693/review-finding-remediation.md
- .csdlc/evidence/693/public-a2a-impersonation-remediation.md
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
- Separated public Observatory WebSocket raw agent-initiation input from Runtime-internal model-selected A2A by refusing public payloads with agent_initiation_requires_runtime_authority unless a future verifiable per-agent authority envelope exists.
- Added direct configured-signer and authenticated production Observatory WebSocket regressions proving a bearer-token client cannot impersonate the configured resident sender by supplying sender_id.

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
    "evidence_ref": ".csdlc/evidence/693/public-a2a-impersonation-remediation.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observatory",
      "observatory_websocket_rejects_public_agent_initiation_sender_impersonation",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove the authenticated production Observatory WebSocket path refuses raw public A2A sender impersonation instead of dispatching recipient work.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/public-a2a-impersonation-remediation.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Prove the full Runtime kernel lane after the public/raw A2A initiation authority split.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/public-a2a-impersonation-remediation.md"
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
    "evidence_ref": ".csdlc/evidence/693/public-a2a-impersonation-remediation.md"
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
    "evidence_ref": ".csdlc/evidence/693/public-a2a-impersonation-remediation.md"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Diff hygiene.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/public-a2a-impersonation-remediation.md"
  },
  {
    "command": [
      "csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "693"
    ],
    "purpose": "Validate typed C-SDLC issue state after the public A2A impersonation remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/public-a2a-impersonation-remediation.md"
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
