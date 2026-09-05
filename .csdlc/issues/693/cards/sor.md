# Structured Output Record

Template: 1.0.0

Issue: 693

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented Runtime-owned first-class A2A action selection for model-backed conversation ingress. Ollama-style provider chat responses now expose a provider-native initiate_agent tool channel that Runtime normalizes into the existing governed A2A initiation intent. Free-form prose remains an operator reply and cannot authorize dispatch. Legacy explicit action envelopes remain accepted through the same validation boundary.

## Artifacts

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/control.rs
- .csdlc/evidence/693

## Execution

- adl-runtime-kernel/src/assembly.rs routes initial operator-to-agent provider conversations through a typed ProviderConversationOutput while preserving existing provider-model execution for initiated peer work.
- adl-runtime-kernel/src/assembly.rs changes the model prompt from brittle exact JSON emission to a first-class initiate_agent tool contract and retains legacy JSON envelope compatibility.
- adl-runtime-kernel/src/control.rs adds provider conversation output/action structs plus Ollama /api/chat tool-call invocation and fail-closed normalization.
- adl-runtime-kernel/src/control.rs adds a production-ingress live-style local HTTP fixture proving Beacon selects A2A through provider-native tool output and Ember executes through the governed recipient provider route.
- adl-runtime-kernel/src/control.rs adds coverage for ordinary prose as non-action, provider-native action projection, JSON-object and JSON-encoded tool arguments, ambiguous/unknown tool-call rejection, replay/conflict/failure semantics, and existing governed initiation primitive behavior.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Whitespace and diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "diff-check.log"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/.tmp",
      "cargo",
      "test",
      "--manifest-path",
      "/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/adl-runtime-kernel/Cargo.toml",
      "agent_to_agent_initiation",
      "--",
      "--nocapture"
    ],
    "purpose": "Existing #662 governed A2A primitive compatibility lane covering configured provider work/activity, replay/conflict, and terminal missing/stale-recipient failures.",
    "outcome": "passed",
    "evidence_ref": "runtime-a2a-primitive-compatibility.log"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/.tmp",
      "cargo",
      "clippy",
      "--manifest-path",
      "/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict lint check for the touched runtime crate and tests.",
    "outcome": "passed",
    "evidence_ref": "runtime-clippy.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Rust formatting hygiene for the touched runtime crate.",
    "outcome": "passed",
    "evidence_ref": "runtime-fmt.log"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/.tmp",
      "cargo",
      "test",
      "--manifest-path",
      "/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/adl-runtime-kernel/Cargo.toml",
      "agent_to_agent_model_action_from_conversation_delivers_peer_response",
      "--",
      "--nocapture"
    ],
    "purpose": "Production conversation ingress proof for Beacon selecting A2A through local Ollama-style /api/chat tool output, Runtime-governed dispatch, Ember provider execution, and correlated A2A activity.",
    "outcome": "passed",
    "evidence_ref": "runtime-production-ingress-a2a.log"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/.tmp",
      "cargo",
      "test",
      "--manifest-path",
      "/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/adl-runtime-kernel/Cargo.toml",
      "provider_conversation",
      "--",
      "--nocapture"
    ],
    "purpose": "Provider conversation normalization lane proving native tool-call selection, JSON-object and JSON-encoded arguments, prose non-action, ambiguous tool rejection, and legacy envelope compatibility.",
    "outcome": "passed",
    "evidence_ref": "runtime-provider-conversation.log"
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
