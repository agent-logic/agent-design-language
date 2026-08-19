# Validation Planning Prompt

Template: 1.0.0

Issue: 425

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/425/design.md

Diagram: .csdlc/prepared/issues/425/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-recordless-closeout-tests",
    "proof_role": "Prove typed recordless closeout positive/negative behavior, including no product writes, no GitHub writes, no historical issue/card rewrite, and no synthesized review/implementation evidence.",
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
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_recordless_closeout"
    ],
    "parallel_group": "425-serial-01-tests",
    "defer_reason": "Deferred until implementation."
  },
  {
    "lane": "retained-v092-residual-dry-run",
    "proof_role": "Run a retained classify-only dry-run over #204/#207/#211/#248/#266/#267/#373/#374/#401 before applying eligible recordless closeouts, proving exact live evidence and fail-closed #248 precedence.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-finish",
      "recordless-closeout",
      "--request",
      ".git/csdlc-v2/requests/425-v092-residual-dry-run.json"
    ],
    "parallel_group": "425-serial-02-live-dry-run",
    "defer_reason": "Deferred until the typed recovery command exists and focused tests pass."
  },
  {
    "lane": "csdlc-v2-check",
    "proof_role": "Compile touched C-SDLC v2 owner binaries and ensure the recovery route integrates with existing typed finish/clean code.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "check",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "parallel_group": "425-serial-03-check",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace or conflict artifacts and confirm no unintended product files remain in the diff.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "425-serial-04-diff",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_recordless_closeout`
- `.adl/bin/csdlc-v2/csdlc-finish recordless-closeout --request .git/csdlc-v2/requests/425-v092-residual-dry-run.json`
- `cargo check --manifest-path csdlc-v2/Cargo.toml`
- `git diff --check`

## Failure Semantics

Fail closed on stale issue state, wrong repository/PR/SHA/linkage, contradictory retained evidence, missing token source, test failure, review finding, or publication drift.

## Handoff

Retain typed evidence before convergence.
