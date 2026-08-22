# Validation Planning Prompt

Template: 1.0.0

Issue: 268

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/268/design.md

Diagram: .csdlc/prepared/issues/268/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove current-main dependency ancestry and exact operator authorization/denominator design before binding.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/268/validate_preparation_bundle.py"
    ],
    "parallel_group": "268-local-1",
    "defer_reason": null
  },
  {
    "lane": "six-hour-suite-contracts",
    "proof_role": "Prove the fixed minimum 21,600-second monotonic suite, bounded final-cycle overshoot, immutable duration, portable USD 20 request, receipt completeness, and fail-closed wrapper without provider mutation.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1500,
    "budget_tokens": 12000,
    "argv": [
      "bash",
      "adl/tools/test_run_issue268_six_hour_spot_qualification.sh"
    ],
    "parallel_group": "268-local-2",
    "defer_reason": "Deferred until implementation creates the wrapper and focused test target."
  },
  {
    "lane": "strict-clippy",
    "proof_role": "Reject warnings in the changed Runtime lifecycle soak binary.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "268-local-4",
    "defer_reason": "Deferred until implementation."
  },
  {
    "lane": "authorized-six-hour-launch",
    "proof_role": "Execute or idempotently resolve the explicitly authorized one-attempt asynchronous Spot launch. The remote 25,200-second timeout and USD 20 ceiling are enforced by the portable request; this control-plane command returns only after exact run ownership is established and never claims completion.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/run_issue268_six_hour_spot_qualification.sh",
      "authorized-launch"
    ],
    "parallel_group": "268-paid-launch",
    "defer_reason": "Deferred until implementation, local proof, fresh design approval, exact immutable revision, and final no-mutation AWS preflight pass. Operator authorization and USD 20 ceiling are already recorded."
  },
  {
    "lane": "six-hour-terminal-status",
    "proof_role": "Resolve the exact existing run identity and fail until the repository-native owner reports a terminal outcome; never launch or mutate a second attempt.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/run_issue268_six_hour_spot_qualification.sh",
      "terminal-status"
    ],
    "parallel_group": "268-paid-terminal",
    "defer_reason": "Deferred until the authorized asynchronous attempt exists and reaches a terminal owner state."
  },
  {
    "lane": "six-hour-receipt-cleanup-validation",
    "proof_role": "Validate the exact terminal attempt receipts for minimum elapsed exposure, overshoot <=600 seconds, faults, sampling, causal outcome, redaction, digest binding, exact-owner cleanup, and independent zero-instance readback.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 7000,
    "argv": [
      "bash",
      "adl/tools/run_issue268_six_hour_spot_qualification.sh",
      "validate"
    ],
    "parallel_group": "268-paid-validate",
    "defer_reason": "Deferred until the exact authorized run reaches a terminal state and owner cleanup completes."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 .csdlc/prepared/issues/268/validate_preparation_bundle.py`
- `bash adl/tools/test_run_issue268_six_hour_spot_qualification.sh`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --bin adl-runtime-lifecycle-soak -- -D warnings`
- `bash adl/tools/run_issue268_six_hour_spot_qualification.sh authorized-launch`
- `bash adl/tools/run_issue268_six_hour_spot_qualification.sh terminal-status`
- `bash adl/tools/run_issue268_six_hour_spot_qualification.sh validate`

## Failure Semantics

Fail closed before provider mutation on identity, revision, image, Spot, quota, cost, deadline, kill-switch, ownership, or residue failure; after launch preserve the first causal failure, clean exact resources, and require zero-instance proof.

## Handoff

Retain typed evidence before convergence.
