# Validation Planning Prompt

Template: 1.0.0

Issue: 693

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/693/design.md

Diagram: .csdlc/prepared/issues/693/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-a2a-live-style",
    "proof_role": "Prove production conversation ingress selects and completes governed Beacon-to-Ember A2A from an Ollama-native tool call while preserving the distinct initiating reply and correlated recipient completion.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "control::layer8_conversation_ingress_tests::agent_to_agent_model_action_from_conversation_delivers_peer_response",
      "--",
      "--exact"
    ],
    "parallel_group": "693-runtime",
    "defer_reason": null
  },
  {
    "lane": "runtime-a2a-governance",
    "proof_role": "Preserve failure replay cancellation admission legacy primitive behavior and ordinary replies for models without tool support.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "provider_conversation"
    ],
    "parallel_group": "693-runtime",
    "defer_reason": null
  },
  {
    "lane": "runtime-quality",
    "proof_role": "Prove formatting and lint hygiene for the Runtime kernel change.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 800,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "693-quality",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib control::layer8_conversation_ingress_tests::agent_to_agent_model_action_from_conversation_delivers_peer_response -- --exact`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib provider_conversation`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed: model prose is never treated as delivery truth, and any invalid action is rejected without bypassing governed admission or corrupting the ordinary reply path.

## Handoff

Retain typed evidence before convergence.
