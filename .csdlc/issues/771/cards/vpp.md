# Validation Planning Prompt

Template: 1.0.0

Issue: 771

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/771/design.md

Diagram: .csdlc/prepared/issues/771/diagram.mmd

## Selected Lanes

[
  {
    "lane": "current-mapping",
    "proof_role": "Required current exact-source review and full detached-suite binding for all four V3-F rows.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "python3",
      "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py",
      "--v3f-current"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "mapping-negative",
    "proof_role": "Required stale SHA/blob/suite substitution denial.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "python3",
      "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py",
      "--v3f-current",
      "--negative"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "mapping-contract",
    "proof_role": "Required synthetic validator anti-forgery and drift contracts; not source review proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "python3",
      "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/test_validate.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "template-schemas",
    "proof_role": "Required Python-readable schema parity smoke; native renderer validation is in exact detached suite.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "python3",
      "adl/tools/test_prompt_template_structure_schemas.py",
      "--template-set",
      "1.0.5"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "clippy",
    "proof_role": "Required native source static validation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py --v3f-current`
- `python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py --v3f-current --negative`
- `python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/test_validate.py`
- `python3 adl/tools/test_prompt_template_structure_schemas.py --template-set 1.0.5`
- `cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on any missing identity, path or mutation-isolation proof; hosted CI remains required for integration.

## Handoff

Retain typed evidence before convergence.
