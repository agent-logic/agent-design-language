# Validation Planning Prompt

Template: 1.0.0

Issue: 528

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/528/design.md

Diagram: .csdlc/prepared/issues/528/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-vertex-ai-provider-packet",
    "proof_role": "Proves #528 design packet readiness, #514 dependency truth, provider substrate inputs, credential-redaction boundary, and live-smoke defer posture before bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      ".csdlc/prepared/issues/528/validate-vertex-ai-provider-transport.sh"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "vertex-ai-provider-rust-focused",
    "proof_role": "After bind, runs focused Rust provider tests covering shared Gemini codec reuse, Vertex AI transport config/auth boundaries, UTS tools, streaming, errors, cancellation, and redaction.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--lib",
      "provider"
    ],
    "parallel_group": "postbind-local",
    "defer_reason": "Deferred until #528 is bound and implementation creates the Vertex AI transport and shared Gemini codec proof."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "After bind, verifies whitespace and patch hygiene for the exact #528 implementation diff.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "postbind-local",
    "defer_reason": "Deferred until #528 has an implementation diff."
  },
  {
    "lane": "live-vertex-smoke",
    "proof_role": "Optional external provider qualification proving a live Vertex AI call only after explicit operator authorization, ADC/workload identity, project, region, model, quota, and cost ceiling are set.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 1500,
    "argv": [
      ".csdlc/prepared/issues/528/run-live-vertex-smoke.sh"
    ],
    "parallel_group": "external-provider",
    "defer_reason": "Deferred: live Vertex AI calls require separate operator authorization plus configured ADC/workload identity, project, location, model, quota, and cost ceiling."
  },
  {
    "lane": "exact-head-review",
    "proof_role": "Fresh exact-head review proving no actionable P0-P3 findings before publication.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-review",
      "record"
    ],
    "parallel_group": "review",
    "defer_reason": "Deferred until #528 is implemented and locally validated."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `.csdlc/prepared/issues/528/validate-vertex-ai-provider-transport.sh`
- `cargo test --manifest-path adl/Cargo.toml --lib provider`
- `git diff --check origin/main...HEAD`
- `.csdlc/prepared/issues/528/run-live-vertex-smoke.sh`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-review record`

## Failure Semantics

Fail closed on credential disclosure, embedded API keys, duplicated divergent Gemini semantic codecs, missing explicit project/location/model resolution, unsafe tool argument mapping, unbounded live provider calls, or #509/GCP infrastructure scope absorption.

## Handoff

Retain typed evidence before convergence.
