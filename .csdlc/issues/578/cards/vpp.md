# Validation Planning Prompt

Template: 1.0.0

Issue: 578

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/578/design.md

Diagram: .csdlc/prepared/issues/578/diagram.mmd

## Selected Lanes

[
  {
    "lane": "provider-profile-tests",
    "proof_role": "Prove GLM-5.3-Flash profile expansion, deterministic model identity, provider model id, endpoint, and redaction.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "provider_tests",
      "profiles"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "zai-http-family-tests",
    "proof_role": "Prove exact Z.ai request body materialization, runtime override support, invalid parameter rejection, max-token bounds, and credential redaction.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "provider_tests",
      "http_family::zai_glm_5_3_flash_request_materializes_profile_defaults_and_runtime_overrides"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "reviewer-selection-smoke",
    "proof_role": "Prove a reviewer-agent fixture can select the general `z_ai:glm-5.3-flash` profile; live dispatch is skipped truthfully when `ZAI_API_KEY` is absent.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/578/reviewer-selection-smoke.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path adl/Cargo.toml --test provider_tests profiles`
- `cargo test --manifest-path adl/Cargo.toml --test provider_tests http_family::zai_glm_5_3_flash_request_materializes_profile_defaults_and_runtime_overrides`
- `bash .csdlc/prepared/issues/578/reviewer-selection-smoke.sh`

## Failure Semantics

Fail closed on stale provider facts, hidden defaults, invalid parameter fallback, credential leakage, zero-test validation, reviewer-selection drift, lifecycle tooling regression, or #446/#455 scope collision.

## Handoff

Retain typed evidence before convergence.
