# Validation Planning Prompt

Template: 1.0.0

Issue: 142

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/142/design.md

Diagram: .csdlc/prepared/issues/142/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-runtime-integration",
    "proof_role": "Prove production launcher, real multi-process convergence, authority failures, coherent single-Observatory projection, and teardown behavior with nonzero exact tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 3600,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_runtime_operational",
      "--no-tests=fail"
    ],
    "parallel_group": "local-only",
    "defer_reason": "Target is declared during planning and must exist before implementation validation."
  },
  {
    "lane": "serial-live-demos",
    "proof_role": "Run Demo A to full cleanup, then Demo B, retaining live Observatory, authority, failure/recovery, account, and teardown evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-10"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 10800,
    "budget_tokens": 30000,
    "argv": [
      "bash",
      "adl/tools/run_v092_distributed_runtime_observatory_demo.sh",
      "--serial"
    ],
    "parallel_group": "exclusive-live",
    "defer_reason": "The live runner is an issue deliverable and AWS execution requires verified business-account authority."
  },
  {
    "lane": "exact-receipt-and-review",
    "proof_role": "Verify exact source, argv parity, process/test denominators, redaction, serial non-overlap, cleanup, live captures, and independent reviewer provenance.",
    "acceptance_ids": [
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/142/validate-proof-receipt.rb"
    ],
    "parallel_group": "post-demo",
    "defer_reason": "The issue-owned validator is authored and reviewed with the implementation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_operational --no-tests=fail`
- `bash adl/tools/run_v092_distributed_runtime_observatory_demo.sh --serial`
- `ruby .csdlc/prepared/issues/142/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on singleton substitution, duplicate identity or state, authority drift, incoherent Observatory cut, public/plaintext exposure, wrong AWS account, phase overlap, incomplete cleanup, invalid exact proof, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
