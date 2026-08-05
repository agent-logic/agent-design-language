# Validation Planning Prompt

Template: 1.0.0

Issue: 5802

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5802/design.md

Diagram: .csdlc/prepared/issues/5802/diagram.mmd

## Selected Lanes

[
  {
    "lane": "native-recursive-live",
    "proof_role": "Prove complete authenticated recursive create-or-update and exact post-write verification",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 3600,
    "budget_tokens": 4000,
    "argv": [
      "adl-gws-context-mirror",
      "--repo-root",
      ".",
      "--drive-root-folder-id",
      "1IrNmsxyDLBD0d2jk5f9GiaTCsVm86BAx",
      "--drive-seed-folder-id",
      "1aIoNRhGlhZLjfM_WOPYmkjIbvWBajTz0"
    ],
    "parallel_group": "live-drive",
    "defer_reason": null
  },
  {
    "lane": "independent-drive-readback",
    "proof_role": "Verify company Drive folder topology, uniqueness, inventory count, and exact CodeFriend content",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 4000,
    "argv": [
      "company-drive-api",
      "list-and-fetch-exact"
    ],
    "parallel_group": "readback",
    "defer_reason": null
  },
  {
    "lane": "focused-mirror-contracts",
    "proof_role": "Prove local recursive, auth failure, ambiguity, and exact-readback contracts if code changes",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "adl_gws_context_mirror"
    ],
    "parallel_group": "local-contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `adl-gws-context-mirror --repo-root . --drive-root-folder-id 1IrNmsxyDLBD0d2jk5f9GiaTCsVm86BAx --drive-seed-folder-id 1aIoNRhGlhZLjfM_WOPYmkjIbvWBajTz0`
- `company-drive-api list-and-fetch-exact`
- `cargo test --manifest-path adl/Cargo.toml adl_gws_context_mirror`

## Failure Semantics

Fail closed on authentication, scope, listing, ambiguity, upload, readback, digest, or recursion mismatch; retain one actionable failure and keep the schedule paused.

## Handoff

Retain typed evidence before convergence.
