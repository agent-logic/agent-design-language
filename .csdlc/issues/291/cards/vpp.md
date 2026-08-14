# Validation Planning Prompt

Template: 1.0.0

Issue: 291

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/291/design.md

Diagram: .csdlc/prepared/issues/291/diagram.mmd

## Selected Lanes

[
  {
    "lane": "initialized-decomposition-recovery-focused",
    "proof_role": "Prove happy initialized recovery plus negatives for stale CAS, unsupported phase, missing/drifted design or diagram evidence, false-approval old/new audit capture, digest-bound R2 historical evidence retention, bootstrap-overwrite rejection, validation-before-write, content-addressed staged blobs, prepared manifest as durable point of no return, preimage/postimage hash verification, fsync ordering, unsupported durability fallback, commit-marker semantics, crash injection before prepared-manifest fsync/after prepared-manifest fsync/after target replacement/before parent-directory fsync boundary/after commit-marker fsync, abandon only transactions without durable prepared manifest, roll forward every prepared transaction to exact post-state, fail closed on unexpected target hashes, journal cleanup and idempotency, request-root/cwd mismatch, repository containment, issue/path identity, symlink and path escape rejection, isolated #114 golden-root copy, live #114 and root .csdlc/locks/114.lock non-mutation, generic typed graph nodes/roles/directed edges/parent owner/acyclic order/in-scope validation, missing/inverted/out-of-scope/duplicate/trust-redefinition graph failures, closed semantic field set, identity propagation across all cards, preparation-only SOR, normal initialized edit rejection, exact one-entry audit consistency, and preservation of already-repaired fields unless intentionally superseded.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "initialized_decomposition_recovery"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": "Implementation must create csdlc-v2/tests/initialized_decomposition_recovery.rs before proof execution; this readiness deferral is not validation evidence and must fail closed until the target exists."
  },
  {
    "lane": "csdlc-v2-full",
    "proof_role": "Prove existing C-SDLC v2 lifecycle/edit behavior remains intact.",
    "acceptance_ids": [
      "AC-1",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-clippy",
    "proof_role": "Reject warnings across all C-SDLC v2 targets.",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "lint",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors and support exact-head review.",
    "acceptance_ids": [
      "AC-10"
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
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test initialized_decomposition_recovery`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on stale generation/digest, unsupported phase, missing or drifted preserved design/diagram evidence, missing digest-bound R2 historical evidence, false-approval old/new audit ambiguity, bootstrap overwrite, unsafe scope, request-root/cwd mismatch, repository containment failure, issue/path identity mismatch, symlink or path escape, unsupported fsync/durability fallback, missing or inconsistent write-ahead journal, staged blob, prepared manifest, preimage/postimage hash, commit marker, crash recovery, journal cleanup, or idempotency behavior, partial values/rendered/index/audit visibility, graph mismatch or forbidden trust redefinition, closed-field-surface violation, #114 live fixture or root-lock mutation, normal initialized edit regression, warning, failed proof, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
