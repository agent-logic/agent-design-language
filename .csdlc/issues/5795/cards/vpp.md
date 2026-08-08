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
    "proof_role": "Prove governed admission, strict request and output bounds, execution classification, saturation, timeout, cancellation, recovery, unavailable behavior, configuration rejection, runner-byte pinning, and failure redaction.",
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
    "proof_role": "Compile the kernel contract and cross-crate real-model harness with Clippy warnings denied so dependency and portability drift fail closed.",
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
    "lane": "mac-mlx-real-local-model-smoke",
    "proof_role": "Invoke the explicitly configured Mac MLX gemma4:12b model through the bounded adapter and require a correlated non-retained real_local_model receipt with runner and model hashes and no cloud fallback.",
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
      "real_local_model_smoke",
      "--",
      "--ignored",
      "--exact",
      "--nocapture"
    ],
    "parallel_group": "local-model",
    "defer_reason": null
  },
  {
    "lane": "aws-portable-model-preflight",
    "proof_role": "Verify the exact versioned S3 model manifest, all eight retained artifact identities, the fixed g6.xlarge and DLAMI contract, live price cap, quota state, and zero residual issue instances without launching paid compute.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/run_wp5795_aws_gpu_proof.sh",
      "preflight"
    ],
    "parallel_group": "aws-portable-model",
    "defer_reason": null
  },
  {
    "lane": "aws-cuda-real-local-model-smoke",
    "proof_role": "On one fixed On-Demand g6.xlarge, restore only exact S3 object versions, verify NVIDIA L4 driver and Ollama CUDA 12 libraries, run the exact-head real_local_model smoke with positive VRAM use, and prove instance, volume, and lock cleanup.",
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
    "budget_tokens": 12000,
    "argv": [
      "bash",
      "adl/tools/run_wp5795_aws_gpu_proof.sh",
      "run",
      "--commit",
      "<exact-head-sha>",
      "--run-id",
      "<adl-wp5795-run-id>",
      "--execute"
    ],
    "parallel_group": "aws-portable-model",
    "defer_reason": "The us-west-2 On-Demand G and VT service quota is 0 vCPUs and the fixed g6.xlarge proof requires 4; no GPU instance may launch until the approved business account quota reaches that value."
  },
  {
    "lane": "post-wp14-wss-browser-integration",
    "proof_role": "After issue 5832 freezes authenticated command and WSS transport, prove negative Runtime carrier behavior and one correlated real Shepherd Observatory browser round trip without changing the foundation adapter contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 8000,
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
    "defer_reason": "Issue 5832 has not frozen the authenticated command and WSS carrier contract; implementation or browser proof before that gate would be speculative."
  },
  {
    "lane": "exact-head-hygiene",
    "proof_role": "Reject unrelated changes and support exact-head independent review of the bounded Shepherd foundation and proof runner.",
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
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model real_local_model_smoke -- --ignored --exact --nocapture`
- `bash adl/tools/run_wp5795_aws_gpu_proof.sh preflight`
- `bash adl/tools/run_wp5795_aws_gpu_proof.sh run --commit <exact-head-sha> --run-id <adl-wp5795-run-id> --execute`
- `node adl/tools/validate_v092_shepherd_browser_roundtrip.mjs --browser chrome --require-real-local-model --require-governed-ingress --require-correlated-browser-result`
- `git diff --check`

## Failure Semantics

Fail closed on unavailable model, timeout, malformed command, unauthorized mutation, policy bypass, status ambiguity, or fake-only success; keep the Runtime and Observatory usable after failure.

## Handoff

Retain typed evidence before convergence.
