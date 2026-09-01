# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Four real Terraform launches have now repeatedly proved the GPU half and cleanup. The latest run passed immutable bootstrap, two-model VRAM residency, Git-free source restoration, pinned Rust installation, and Vector installation, then failed in guardian_lifecycle because the selective archive omitted three compile-time Runtime documents. The new bounded diagnostic receipt preserved the exact compiler errors. Both nodes and all volumes were destroyed and the lock was released. The source archive now includes the complete small Rust dependency and compile-input closure; final paid proof remains pending review and rerun.

## Artifacts

- infra/aws/runtime/gpu-proof
- adl-runtime-kernel/build.rs
- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .adl/local/issue345/adl-issue345-20260901-110251/gpu-ready.json
- .adl/local/issue345/adl-issue345-20260901-110251/runtime-final.json
- .adl/local/issue345/adl-issue345-20260901-110251/cleanup-on-exit.json

## Execution

- Retained the reviewed Terraform-owned two-node topology, one key pair, mandatory SSH /32, private Ollama routing, immutable artifacts, single-use authorization, cost bound, and three cleanup paths.
- Observed the fourth real GPU receipt prove llama3.1:8b and qwen3:8b simultaneously resident with exact digests and nonzero VRAM while Ollama remained non-public.
- Observed the Runtime node pass package setup including CMake, immutable source restoration, pinned Rust installation, and Vector installation before guardian_lifecycle compilation.
- Captured the exact bounded diagnostic proving three missing compile-time Runtime documents instead of losing the guest failure after termination.
- Extended the exact source archive with adl-spec plus the small Runtime API, parity-matrix, and stock-league compile inputs required by source-level include macros.
- Extended the archive contract test to assert every external compile input needed by the production Runtime and ADL builds.
- Updated the runbook to describe dependency and compile-input closure truthfully.
- Confirmed controller cleanup terminated both instances, removed all run volumes, destroyed Terraform state resources, and released the lock.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Prove the no-paid topology, bootstrap, dependency and compile-input archive closure, revision, diagnostics, IAM, authorization, recovery, and cleanup contracts.",
    "outcome": "passed",
    "evidence_ref": "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "run",
      "--commit",
      "ec75a382f9a354ccc4677b769b18043fc9c21567",
      "--run-id",
      "adl-issue345-20260901-110251",
      "--authorization-file",
      ".adl/local/issue345/operator-authorization-cargo-fix.json",
      "--execute"
    ],
    "purpose": "Run the real Terraform two-node qualification through repeated GPU residency, remediated native build setup, exact diagnostic failure, and cleanup.",
    "outcome": "failed",
    "evidence_ref": ".adl/local/issue345/adl-issue345-20260901-110251"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/aws/runtime/gpu-proof",
      "validate"
    ],
    "purpose": "Validate the complete two-node Terraform configuration.",
    "outcome": "passed",
    "evidence_ref": "infra/aws/runtime/gpu-proof"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject patch hygiene defects.",
    "outcome": "passed",
    "evidence_ref": "git diff"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
