# Validation Planning Prompt

Template: 1.0.0

Issue: 74

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/74/design.md

Diagram: .csdlc/prepared/issues/74/diagram.mmd

## Selected Lanes

[
  {
    "lane": "stale-topology-bind",
    "proof_role": "Through the real csdlc-bind binary, prove an unrelated claim-bearing record is skipped while relevant malformed records and real collisions fail closed.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 360,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "bind_topology_scan_uses_canonical_record_identity"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2 bind_topology_scan_uses_canonical_record_identity`

## Failure Semantics

Fail closed if the claim-bearing record is relevant, if any ownership identity collides, or if the test mutates foreign evidence.

## Handoff

Retain typed evidence before convergence.
