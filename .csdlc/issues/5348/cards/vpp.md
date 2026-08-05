# Validation Planning Prompt

Template: 1.0.0

Issue: 5348

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5348/design.md

Diagram: .csdlc/prepared/issues/5348/diagram.mmd

## Selected Lanes

[
  {
    "lane": "release-docs-and-evidence",
    "proof_role": "Validate WP-22 ancestry, required release files, supplemental JSON, and milestone YAML.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      "focused-release-docs-validator"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "release-diff-hygiene",
    "proof_role": "Prove the bounded docs/evidence diff has no whitespace errors.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 15,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "release-script-preflight",
    "proof_role": "Run the canonical ceremony script in check-only mode with the documented WP-23 circular closeout exception.",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/release_ceremony.sh",
      "--version",
      "v0.91.8",
      "--target-branch",
      "codex/5348-v0918-preparation",
      "--allow-dirty",
      "--skip-sor-gate"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "exact-head-review",
    "proof_role": "Obtain one bounded findings-first review of the exact docs/evidence revision before publication.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "subagent-review",
      "exact-head"
    ],
    "parallel_group": "review",
    "defer_reason": null
  },
  {
    "lane": "post-merge-release-verification",
    "proof_role": "Verify the pushed tag, published GitHub release, closed #5348/#5809, and final umbrella closure against the WP-23 merge commit.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "release-ceremony",
      "post-merge-live-verification"
    ],
    "parallel_group": "post-merge",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby focused-release-docs-validator`
- `git diff --check`
- `bash adl/tools/release_ceremony.sh --version v0.91.8 --target-branch codex/5348-v0918-preparation --allow-dirty --skip-sor-gate`
- `subagent-review exact-head`
- `release-ceremony post-merge-live-verification`

## Failure Semantics

Fail closed on missing live predecessor merge, stale ancestry, incomplete release evidence, or hidden implementation need.

## Handoff

Retain typed evidence before convergence.
