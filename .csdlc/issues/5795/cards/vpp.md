# Validation Planning Prompt

Template: 1.0.0

Issue: 5795

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5795/design.md

Diagram: .csdlc/prepared/issues/5795/diagram.mmd

## Selected Lanes

[
  {
    "lane": "shepherd-foundation-contract",
    "proof_role": "Prove signed governed admission, strict request and output bounds, real versus test classification, one-request saturation, timeout, cancellation, explicit recovery, unavailable behavior, configuration rejection, and failure redaction.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "shepherd"
    ],
    "parallel_group": "shepherd-foundation",
    "defer_reason": null
  },
  {
    "lane": "shepherd-denied-warning-build",
    "proof_role": "Compile the kernel contract and cross-crate real-model harness with Clippy warnings denied so hidden dependency and portability drift fail closed.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "shepherd",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "shepherd-foundation",
    "defer_reason": null
  },
  {
    "lane": "real-local-model-smoke",
    "proof_role": "Invoke the explicitly configured local gemma4:12b-mlx model through the bounded process adapter and require a correlated non-retained real_local_model response without cloud fallback.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "shepherd_local_model",
      "--",
      "--ignored",
      "--exact",
      "real_local_model_smoke"
    ],
    "parallel_group": "local-model",
    "defer_reason": null
  },
  {
    "lane": "runtime-wss-negative",
    "proof_role": "After WP-14 freezes the carrier contract, prove malformed, unauthorized, wrong-runtime, timeout, and post-failure read-stream behavior over authenticated Runtime API/WSS.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_api_wss"
    ],
    "parallel_group": "post-wp14-integration",
    "defer_reason": "Issue 5832 has not frozen the authenticated command and WSS carrier contract; running or modifying that integration now would cross the declared serialization gate."
  },
  {
    "lane": "real-shepherd-browser-roundtrip",
    "proof_role": "After WP-14 integration, submit one uniquely correlated governed message from Chrome and prove the Observatory renders the same non-retained real_local_model result while Runtime remains usable.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "node",
      "adl/tools/validate_v092_shepherd_browser_roundtrip.mjs",
      "--browser",
      "chrome",
      "--require-real-local-model",
      "--require-governed-ingress",
      "--require-correlated-browser-result"
    ],
    "parallel_group": "post-wp14-integration",
    "defer_reason": "Issue 5832 remains unresolved, the Shepherd WSS route is intentionally absent, and the Runtime feed on port 20997 is currently unavailable; none of those conditions may be reported as passing browser proof."
  },
  {
    "lane": "exact-head-hygiene",
    "proof_role": "Reject unrelated changes and support exact-head review of the bounded foundation.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test shepherd`
- `cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --test shepherd -- -D warnings`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model -- --ignored --exact real_local_model_smoke`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss`
- `node adl/tools/validate_v092_shepherd_browser_roundtrip.mjs --browser chrome --require-real-local-model --require-governed-ingress --require-correlated-browser-result`
- `git diff --check`

## Failure Semantics

Fail closed on unavailable model, timeout, malformed command, unauthorized mutation, policy bypass, status ambiguity, or fake-only success; keep the Runtime and Observatory usable after failure.

## Handoff

Retain typed evidence before convergence.
