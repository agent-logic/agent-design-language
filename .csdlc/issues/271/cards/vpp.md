# Validation Planning Prompt

Template: 1.0.0

Issue: 271

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Preparation validation runs now; runtime/browser/scope wrappers are required after bind and before implementation review/publication.

## Lane Inputs

Design: .csdlc/prepared/issues/271/design.md

Diagram: .csdlc/prepared/issues/271/diagram.mmd

## Selected Lanes

[
  {
    "lane": "dependency-scope-and-review-readiness",
    "proof_role": "Prove canonical terminal dependencies, exact current-main base, declared path scope, exact lock scope, and deterministic packet readiness for separate fresh design review.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/271/validate_preparation_bundle.py"
    ],
    "parallel_group": "271-serial-01",
    "defer_reason": null
  },
  {
    "lane": "recipient-ack-runtime-existing-handler",
    "proof_role": "Run the existing real adl-runtime-kernel recipient_ack handler target, parse the nonzero denominator, and retain the authentic public handler-output artifact consumed by browser fixtures under issue-local evidence.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "recipient_ack",
      "--",
      "--nocapture"
    ],
    "parallel_group": "271-serial-02",
    "defer_reason": "Deferred until bound implementation captures the existing-handler denominator and source-grounded handler-output artifact under issue-local evidence."
  },
  {
    "lane": "observatory-browser-exact-eight",
    "proof_role": "Exercise actual Observatory assets through the literal eight-case exact-set browser wrapper with per-test nonzero assertions and zero/ignored/skipped/missing/duplicated case rejection while consuming authentic issue-local handler-output evidence rather than loopback-only data.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5500,
    "argv": [
      "bash",
      "adl/tools/validate_layer8_authority_observatory_ui.sh"
    ],
    "parallel_group": "271-serial-03",
    "defer_reason": "Deferred until bound implementation creates the exact browser wrapper and consumes authentic handler-derived public response fixtures from .csdlc/evidence/271 rather than mocked conversation frames."
  },
  {
    "lane": "post-bind-exact-three-path-scope",
    "proof_role": "After bind, compare the bound branch from its exact execution base to current main and reject any product/test path outside app.js, styles.css, and validate_layer8_authority_observatory_ui.sh, while allowing issue-local lifecycle/evidence surfaces.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 90,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/evidence/271/validate_exact_three_path_scope.py"
    ],
    "parallel_group": "271-serial-04",
    "defer_reason": "Deferred until bind establishes the execution base and evidence directory."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove clean patch structure before review.",
    "acceptance_ids": [
      "AC-5",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 90,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "271-serial-05",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/271/validate_preparation_bundle.py`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml recipient_ack -- --nocapture`
- `bash adl/tools/validate_layer8_authority_observatory_ui.sh`
- `python3 .csdlc/evidence/271/validate_exact_three_path_scope.py`
- `git diff --check`

## Failure Semantics

Fail closed on unknown schema/status, malformed response, unavailable Runtime, redaction uncertainty, scope drift, missing proof, denominator mismatch, zero/ignored/skipped browser cases, non-authentic handler-output fixture evidence, or a new non-issue-local runtime wrapper script.

## Handoff

Retain typed evidence before convergence.
