# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Three real Terraform launches have now repeatedly proved the GPU half and cleanup. The latest run passed immutable bootstrap, two-model VRAM residency, Git-free source restoration, and Vector installation, then the Runtime node exited 101 during Cargo work. The deleted node left only the old terse failure receipt. Both nodes and volumes were destroyed. The bootstrap now installs CMake for aws-lc-sys, the kernel build accepts the authorization-bound archive revision without Git, and future failure receipts retain a bounded stage and diagnostic tail. Final paid proof remains pending exact-head review and rerun.

## Artifacts

- infra/aws/runtime/gpu-proof
- adl-runtime-kernel/build.rs
- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .adl/local/issue345/adl-issue345-20260901-102827/gpu-ready.json
- .adl/local/issue345/adl-issue345-20260901-102827/runtime-final.json
- .adl/local/issue345/adl-issue345-20260901-102827/cleanup-on-exit.json

## Execution

- Retained the reviewed Terraform-owned two-node topology, one key pair, mandatory SSH /32, private Ollama routing, immutable artifacts, single-use authorization, cost bound, and three cleanup paths.
- Observed the third real GPU receipt prove llama3.1:8b and qwen3:8b simultaneously resident with exact digests and nonzero VRAM while Ollama remained non-public.
- Observed the Runtime node pass immutable source restoration and the remediated Vector stage, then exit 101 during Cargo work.
- Added CMake to Runtime package setup because the locked aws-lc-sys native dependency requires it on the Ubuntu guest.
- Changed the runtime-kernel build script to accept only an exact lowercase forty-hex ADL_RUNTIME_SOURCE_REVISION and prefer it over unavailable Git metadata in the immutable source archive.
- Added bounded stage and 4096-byte diagnostic-tail fields to Runtime failure receipts so a terminated node does not erase the actionable failure cause.
- Extended the generated-bootstrap contract test to cover CMake, archive revision propagation, and diagnostic receipt fields.
- Confirmed controller cleanup drove both instances to termination, removed all run volumes, destroyed Terraform state resources, and released the lock after the third attempt.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Prove the no-paid topology, bootstrap, native-build prerequisite, revision, diagnostic, IAM, authorization, recovery, and cleanup contracts.",
    "outcome": "passed",
    "evidence_ref": "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "run",
      "--commit",
      "7deec6a83a4e7578f86208e07a7c21008cb47a29",
      "--run-id",
      "adl-issue345-20260901-102827",
      "--authorization-file",
      ".adl/local/issue345/operator-authorization-vector-fix.json",
      "--execute"
    ],
    "purpose": "Run the real Terraform two-node qualification through repeated GPU residency, remediated Vector setup, Runtime Cargo failure, and cleanup.",
    "outcome": "failed",
    "evidence_ref": ".adl/local/issue345/adl-issue345-20260901-102827"
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
