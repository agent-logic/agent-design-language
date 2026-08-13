# Validation Planning Prompt

Template: 1.0.0

Issue: 273

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/273/design.md

Diagram: .csdlc/prepared/issues/273/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove typed identity, exact predecessor terminal ancestry, disjoint file ownership, serial registration order, and declared post-bind targets.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/273/validate_preparation_bundle.py"
    ],
    "parallel_group": "273-serial-01",
    "defer_reason": "Runs after bootstrap; preparation proof only."
  },
  {
    "lane": "shepherd-focused",
    "proof_role": "Prove acquire/replace/revoke/expiry, retry/restart, rejection, capacity, receipt, and redaction behavior in the exact issue-owned target.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 16000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_shepherd_serving_eligibility",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "273-serial-02",
    "defer_reason": "Deferred until typed bind and target creation."
  },
  {
    "lane": "shepherd-clippy",
    "proof_role": "Reject warnings/API misuse in the exact library and integration target.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_shepherd_serving_eligibility",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "273-serial-03",
    "defer_reason": "Deferred until focused proof target exists."
  },
  {
    "lane": "shepherd-scope",
    "proof_role": "Require the exact four product paths and reject #274, parent, and unrelated changes.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/273/validate_scope.py"
    ],
    "parallel_group": "273-serial-04",
    "defer_reason": "Deferred until a committed implementation candidate exists."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace, conflict marker, and patch hygiene defects before exact-head review.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "273-serial-05",
    "defer_reason": "Deferred to implementation and rerun after substantive repair."
  },
  {
    "lane": "terminal-authority",
    "proof_role": "Require canonical merged #273 terminal cache and merge ancestry before #274 shared registration.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/273/validate_terminal.py"
    ],
    "parallel_group": "273-serial-06",
    "defer_reason": "Deferred until ordinary required CI is green and typed finish creates terminal authority; no optional or paid runner is authorized."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 .csdlc/prepared/issues/273/validate_preparation_bundle.py`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_shepherd_serving_eligibility -- --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_shepherd_serving_eligibility -- -D warnings`
- `python3 .csdlc/prepared/issues/273/validate_scope.py`
- `git diff --check`
- `python3 .csdlc/prepared/issues/273/validate_terminal.py`

## Failure Semantics

Fail closed on stale authority, wrong foundation binding, dual eligibility, replay, partial mutation, redaction leak, scope collision, lifecycle drift, review finding, CI failure, or nonancestral terminal state.

## Handoff

Retain typed evidence before convergence.
