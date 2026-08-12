# Structured Output Record

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and remediated Runtime-owned Layer 8 conversation authority with independently loaded current identity, capability, and agent and Polis policy evidence; production signed requests; exact recipient-signed acknowledgements; serialized tamper-evident audit; retry-safe refusal handling; and exact-current conversation integration after merged issue 244.

## Artifacts

- adl-runtime-kernel/src/layer8_authority.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs
- adl-runtime/src/layer8_authority.rs
- adl-runtime/tests/layer8_authority.rs
- adl/src/csm_runtime_api.rs
- adl/tests/layer8_authority_runtime_api.rs
- adl/tools/validate_layer8_authority_observatory_ui.sh
- demos/html-observatory/app.js
- demos/html-observatory/styles.css
- docs/milestones/v0.92/features/LAYER8_CONVERSATION_AUTHORITY.md

## Execution

- Load current identity evidence, pre-existing capabilities, and separate agent and Polis policies from an external authority profile instead of constructing request-shaped authority.
- Load externally held per-principal Ed25519 keys and require a verified signed request before dispatch plus an exact recipient-key acknowledgement before reporting delivery.
- Serialize audit appends across store handles with an exclusive file lock and current-head reload, preserving a valid chain before returning grants.
- Keep retryable policy refusals from consuming replay identities and preflight conversation capacity before durable authorization.
- Reconcile the production conversation test with merged issue 244's internal test-module migration while retaining its cleanup-race semantics.
- Preserve disclosure-safe Observatory authority-state presentation and document the external authority and signing profile boundary.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "conversation_sessions_tests::authenticated_selected_agent_conversation_uses_canonical_wss_ingress",
      "--",
      "--exact"
    ],
    "purpose": "Run the exact-current production conversation boundary after issue 244's test-module migration.",
    "outcome": "passed",
    "evidence_ref": "terminal: production conversation boundary 1 passed"
  },
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
    "purpose": "Run identity, least-privilege authority, signed-message, acknowledgement, replay, audit, and refusal proof.",
    "outcome": "passed",
    "evidence_ref": "terminal: authority contract 11 passed"
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
    "purpose": "Run the Runtime API signed request and exact acknowledgement integration proof.",
    "outcome": "passed",
    "evidence_ref": "terminal: Runtime API integration 3 passed"
  },
  {
    "command": [
      "bash",
      "adl/tools/validate_layer8_authority_observatory_ui.sh"
    ],
    "purpose": "Run the real-browser Observatory disclosure and authority-state proof with the configured local Playwright and Chrome runtimes.",
    "outcome": "passed",
    "evidence_ref": "terminal: Observatory browser contract PASS"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-kernel",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Enforce strict lint hygiene for the production Runtime source.",
    "outcome": "passed",
    "evidence_ref": "terminal: kernel strict clippy PASS"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "layer8_authority_runtime_api",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Enforce strict lint hygiene for the Runtime API integration target.",
    "outcome": "passed",
    "evidence_ref": "terminal: API strict clippy PASS"
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
