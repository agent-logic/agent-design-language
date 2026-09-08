# Structured Planning Prompt

Template: 1.0.0

Issue: 740

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #740 from current main, patch the GCP-B1 proof script to remove token-retaining backend files and shared auth cache, add focused validation, perform legacy-key disposition and Terraform-backend canary live proof, then review and publish.

## Plan

Revision 3

## Steps

[
  {
    "id": "step-1",
    "action": "Patch proof tooling so Terraform/GCloud auth is issue-run scoped, non-retained, and recovery sanitizes before retaining artifacts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "step-2",
    "action": "Replace self-issued authorization acceptance with authenticated operator-bound approval evidence tied to exact plan/head.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "step-3",
    "action": "Run redacted live GCP proof for legacy-key disposition, Terraform-backend canary write/recovery, and bucket readback.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "step-4",
    "action": "Record post-merge corrective truth, validate, obtain exact-head review, publish PR, and merge only when green.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- No `/private/tmp` diagnostics; keep proof in repo/worktree or Git-common issue-owned paths.
- No provider credential values, token contents, or key material are printed, committed, or retained.
- Root main remains inspection-only.
- Remote Terraform state bucket is not deleted after adoption.
- All live claims are bound to exact retained redacted evidence.

## Risks

- Accidentally retaining short-lived OAuth tokens in generated backend or recovery files.
- Mistaking a local JSON file for authenticated operator authorization.
- Using raw GCS object upload proof where Terraform backend state proof is required.
- Overreaching into unrelated GCP resources.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/740/design.md

Digest: d00bc8370459eb65fe3926900e715c56df7bb5088ca2662f2507a7f1f5095027

## Diagram

.csdlc/prepared/issues/740/diagram.mmd

Digest: a96391b12883a71c74bc78ece094ec081b9aedf166a9d18ca3c01e227b4486d8

## Stop Conditions

- GCP returns insufficient permission for the bounded Token Creator or key-disposition read/revoke action and no safe in-scope alternative can record a truthful time-bounded exception.
- A needed change would touch unrelated issue work or resources outside the declared GCP-B1 denominator.
- Validation detects retained credential material that cannot be sanitized without losing required evidence.

## Handoff

Proceed only after doctor readiness.
