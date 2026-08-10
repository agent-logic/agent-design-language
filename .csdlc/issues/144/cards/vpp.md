# Validation Planning Prompt

Template: 1.0.0

Issue: 144

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/144/design.md

Diagram: .csdlc/prepared/issues/144/diagram.mmd

## Selected Lanes

[
  {
    "lane": "cognitive-profile-authority",
    "proof_role": "Prove trusted authority anchoring, full revision lineage, governed rotation, privacy, and deterministic projection with a nonzero focused target.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "cognitive_profile",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "parallel_group": "144-core",
    "defer_reason": null
  },
  {
    "lane": "cognitive-profile-strict-clippy",
    "proof_role": "Reject warning-bearing product or focused-test integration.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "cognitive_profile",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "144-core",
    "defer_reason": null
  },
  {
    "lane": "cognitive-profile-native-linux-macos",
    "proof_role": "Produce exact-head native Linux and macOS receipts, independently verify digests, runner provenance, sanitized logs, positive inventory, and semantic equivalence, then require the reviewed green qualified PR before terminal delivery.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/144/validate-native-receipts.rb",
      ".csdlc/evidence/144/native-platform/macos.json",
      ".csdlc/evidence/144/native-platform/linux.json"
    ],
    "parallel_group": "144-native",
    "defer_reason": "Runs on native GitHub Actions macOS and Linux after publication; terminal delivery follows exact reviewed green proof."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test cognitive_profile --no-tests=fail --status-level all`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test cognitive_profile -- -D warnings`
- `ruby .csdlc/prepared/issues/144/validate-native-receipts.rb .csdlc/evidence/144/native-platform/macos.json .csdlc/evidence/144/native-platform/linux.json`

## Failure Semantics

Fail closed on untrusted authority roots, signature or digest mismatch, incomplete lineage, forged deep history, invalid rotation, privacy leakage, zero-test proof, stale evidence, or missing exact-head review.

## Handoff

Retain typed evidence before convergence.
