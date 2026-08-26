# Validation Planning Prompt

Template: 1.0.0

Issue: 272

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/272/design.md

Diagram: .csdlc/prepared/issues/272/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove current issue/card/design identity, exact prerequisite terminal-cache identities, frozen ownership/non-goals, declared post-bind target, and bounded lane contracts; this is preparation proof only.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/272/validate_preparation_bundle.py"
    ],
    "parallel_group": "272-serial-01-preparation",
    "defer_reason": "The validator is authored before bootstrap and runs after all six typed cards exist; it cannot substitute for product proof or review."
  },
  {
    "lane": "binding-contract",
    "proof_role": "Execute one positive and eight negative fixtures for exact adapter_kind and receipt_digest, fixed field set, duplicate/ambiguous keys and length, <=4096 bound, canonical framing, and prior/candidate digest swap rejection; this proves the verifier contract, not durable product behavior.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/272/validate_binding_contract.py"
    ],
    "parallel_group": "272-serial-02-binding-contract",
    "defer_reason": null
  },
  {
    "lane": "foundation-focused",
    "proof_role": "Prove AC-1 through AC-5 in the exact issue-owned foundation integration target with zero-test failure and serial execution.",
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
    "budget_tokens": 18000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_serving_authority_foundation",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "272-serial-03-focused",
    "defer_reason": "Deferred until typed bind and creation of the exact owned source/test target; fail closed on missing target, zero tests, ignored tests, or any failure."
  },
  {
    "lane": "foundation-clippy",
    "proof_role": "Reject warnings and API misuse for the exact foundation target after focused proof passes.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_serving_authority_foundation",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "272-serial-04-clippy",
    "defer_reason": "Deferred until the exact source/test target exists and focused proof passes."
  },
  {
    "lane": "foundation-scope",
    "proof_role": "Require the exact three tracked source/test paths and reject #203, #265, #300, #330, #114, parent #205, or any undeclared product change.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/272/validate_scope.py"
    ],
    "parallel_group": "272-serial-05-scope",
    "defer_reason": "Deferred until implementation creates a committed candidate; fail closed on any missing, extra, staged, or uncommitted tracked path."
  },
  {
    "lane": "foundation-diff-hygiene",
    "proof_role": "Reject whitespace, conflict-marker, and patch hygiene defects before exact-head review.",
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
    "parallel_group": "272-serial-06-diff",
    "defer_reason": "Deferred to the implementation candidate and rerun after every substantive fix."
  },
  {
    "lane": "terminal-authority",
    "proof_role": "After typed finish, require the canonical #272 merged terminal cache and prove the merge is ancestral to origin/main before successor release.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/272/validate_terminal.py"
    ],
    "parallel_group": "272-serial-07-terminal",
    "defer_reason": "Deferred until required ordinary CI is green and typed finish creates the canonical terminal cache; no optional or paid runner is authorized."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 .csdlc/prepared/issues/272/validate_preparation_bundle.py`
- `python3 .csdlc/prepared/issues/272/validate_binding_contract.py`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_serving_authority_foundation -- --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_serving_authority_foundation -- -D warnings`
- `python3 .csdlc/prepared/issues/272/validate_scope.py`
- `git diff --check`
- `python3 .csdlc/prepared/issues/272/validate_terminal.py`

## Failure Semantics

Fail closed on stale authority, wrong identity, receipt or prior-state mismatch, premature publication, conflicting retry, restart ambiguity, corruption, rollback, capacity, unsafe path, redaction leak, scope drift, lifecycle drift, review finding, CI failure, or noncanonical terminal ancestry.

## Handoff

Retain typed evidence before convergence.
