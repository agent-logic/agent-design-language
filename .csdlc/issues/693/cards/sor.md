# Structured Output Record

Template: 1.0.0

Issue: 693

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented Runtime-owned model-backed A2A selection through Ollama-native tool calls, with governed dispatch, distinct initiating and recipient results, correlated completion observability, and safe ordinary-reply fallback when a model rejects tools.

## Artifacts

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/telemetry.rs
- .csdlc/evidence/693/local-validation.md

## Execution

- Replaced prompt-dependent exact JSON action selection with an Ollama-native initiate_agent tool contract normalized into the existing governed Runtime initiation intent.
- Kept Runtime authoritative for identifiers, admission, Layer8 dispatch, replay, cancellation, and terminal outcomes; malformed, unknown, or ambiguous actions fail closed.
- Preserved Beacon's operator-facing reply independently from Ember's provider result and emitted a separately correlated agent_to_agent_completed Runtime event.
- Added bounded fallback to ordinary Ollama generation for 400/404 tool-unsupported responses without inferring or dispatching A2A.
- Added an isolated production-ingress Beacon-to-Ember acceptance using native provider tool output, real governed recipient execution, and correlated initiation/completion observation.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "control::layer8_conversation_ingress_tests::agent_to_agent_model_action_from_conversation_delivers_peer_response",
      "--",
      "--exact"
    ],
    "purpose": "Prove production-ingress native action selection, governed recipient execution, distinct replies, and correlated terminal observability.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/local-validation.md"
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
    "evidence_ref": ".csdlc/evidence/693/local-validation.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib"
    ],
    "purpose": "Prove the complete Runtime kernel library surface.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/693/local-validation.md"
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
    "evidence_ref": ".csdlc/evidence/693/local-validation.md"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
