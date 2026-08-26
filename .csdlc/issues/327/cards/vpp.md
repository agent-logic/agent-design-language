# Validation Planning Prompt

Template: 1.0.0

Issue: 327

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/327/design.md

Diagram: .csdlc/prepared/issues/327/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-327-preparation",
    "proof_role": "Prove exact pre-bind issue/card/design bindings.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/327/validate_preparation_bundle.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "removed-tooling-routing-regression",
    "proof_role": "Directly exercise extant ADL and adl-review tooling dispatches and prove they remain fail closed without restoring a C-SDLC compatibility binary.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "issue_327_removed_tooling"
    ],
    "parallel_group": "local",
    "defer_reason": "The issue-owned integration target is created only after typed bind."
  },
  {
    "lane": "issue-327-scope-allowlist",
    "proof_role": "Prove every committed path is in the exact #327 allowlist and all #259/lifecycle surfaces remain untouched.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/327/validate_changed_paths.py"
    ],
    "parallel_group": "local",
    "defer_reason": "Runs after the bounded substantive commit exists."
  },
  {
    "lane": "adl-strict-clippy",
    "proof_role": "Reject the dead-code regression and all affected ADL target warnings.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 750,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "fresh-exact-head-review-record",
    "proof_role": "Record authoritative typed fresh-session exact-head PASS.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 90,
    "budget_tokens": 2000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-review",
      "record",
      "--request",
      ".git/csdlc-v2/requests/327-review-record.json"
    ],
    "parallel_group": "review",
    "defer_reason": "Runs after typed assignment and completed fresh review."
  },
  {
    "lane": "hosted-required-checks-live",
    "proof_role": "Collect live required-check state on the reviewed published head.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-shepherd",
      "--pr-state-request",
      ".git/csdlc-v2/requests/327-pr-state.json"
    ],
    "parallel_group": "hosted",
    "defer_reason": "Runs only after reviewed publication."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `python3 .csdlc/prepared/issues/327/validate_preparation_bundle.py`
- `cargo test --manifest-path adl/Cargo.toml --test issue_327_removed_tooling`
- `python3 .csdlc/prepared/issues/327/validate_changed_paths.py`
- `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
- `.adl/bin/csdlc-v2/csdlc-review record --request .git/csdlc-v2/requests/327-review-record.json`
- `.adl/bin/csdlc-v2/csdlc-shepherd --pr-state-request .git/csdlc-v2/requests/327-pr-state.json`

## Failure Semantics

Fail closed on caller discovery, scope collision, stale topology, validation failure, stale review, hosted check failure, or any need to widen beyond the single obsolete helper.

## Handoff

Retain typed evidence before convergence.
