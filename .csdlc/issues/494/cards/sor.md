# Structured Output Record

Template: 1.0.0

Issue: 494

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the #494 GCP-E GPU readiness smoke as split stable-support and disposable-instance Terraform modules/roots, retained successful live L4 readback evidence, and hardened repeat execution so only the per-run instance is recreated and destroyed.

## Artifacts

- infra/gcp/workloads/modules/gpu-smoke-support
- infra/gcp/workloads/modules/gpu-smoke-instance
- infra/gcp/workloads/gpu-smoke-support
- infra/gcp/workloads/gpu-smoke-instance
- docs/milestones/v0.92.1/evidence/cloud/gcp-e
- .csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh

## Execution

- Split the GCP-E smoke infrastructure into stable support resources and a per-run disposable L4 VM so repeated smoke runs do not recreate service account and IAP firewall support.
- Added reusable Terraform modules and roots for gpu-smoke-support and gpu-smoke-instance with explicit budget, quota, OS Login, IAP SSH, readiness marker, and per-run VM/disk cleanup contracts.
- Updated the runbook and live proof script to default to the proven us-central1-a L4 target, preserve normal gcloud OS Login/key propagation, route SSH key and known-hosts paths to approved private locations, and avoid writing credential material into tracked evidence.
- Retained live GCP proof evidence showing an NVIDIA L4 VM was created, read over IAP SSH, and destroyed with no per-run VM/disk residue while stable support resources remained for subsequent runs.
- Added and updated the issue-owned validator to prove the split-root layout, fail-closed cost/quota boundaries, cleanup selectors, and no --plain SSH regression.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Reject whitespace errors and conflict markers.",
    "outcome": "passed",
    "evidence_ref": "gcp-e-diff-hygiene.log"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/gcp/workloads/gpu-smoke-instance",
      "validate"
    ],
    "purpose": "Validate the disposable instance Terraform root.",
    "outcome": "passed",
    "evidence_ref": "gcp-e-instance-terraform-validate.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh",
      "--lane=all"
    ],
    "purpose": "Run the #494 issue-owned validator after implementation changes.",
    "outcome": "passed",
    "evidence_ref": "gcp-e-issue-validator.log"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/gcp/workloads/gpu-smoke-support",
      "validate"
    ],
    "purpose": "Validate the stable support Terraform root.",
    "outcome": "passed",
    "evidence_ref": "gcp-e-support-terraform-validate.log"
  },
  {
    "command": [
      "terraform",
      "fmt",
      "-check",
      "-recursive",
      "infra/gcp/workloads/modules/gpu-smoke-support",
      "infra/gcp/workloads/modules/gpu-smoke-instance",
      "infra/gcp/workloads/gpu-smoke-support",
      "infra/gcp/workloads/gpu-smoke-instance"
    ],
    "purpose": "Verify Terraform formatting for all #494 GCP-E modules and roots.",
    "outcome": "passed",
    "evidence_ref": "gcp-e-terraform-fmt.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
