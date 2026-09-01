# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Six real Terraform launches have now repeatedly proved the GPU half and cleanup. The latest run proved both models simultaneously resident, restored every reviewed Git-free source and validator input, and compiled the Guardian successfully in 1m05s. Lifecycle-soak then rejected the generated init template solely because the immutable source archive has no .git metadata. The retained receipt captured that exact failure. After the already-failed soak left its bounded HTTPS/WSS probe waiting for a readiness file that could never arrive, the probe alone was terminated so failure receipt and cleanup could proceed; both nodes and all volumes were then destroyed and the lock released. Lifecycle root discovery now preserves Git-worktree behavior and adds a narrow fallback requiring the generated template under .adl/runtime-v3 plus all canonical ADL archive markers. Final paid proof remains pending exact-head review and rerun.

## Artifacts

- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .adl/local/issue345/adl-issue345-20260901-120543/gpu-ready.json
- .adl/local/issue345/adl-issue345-20260901-120543/runtime-final.json
- .adl/local/issue345/adl-issue345-20260901-120543/cleanup-on-exit.json

## Execution

- Retained the reviewed Terraform-owned two-node topology, one key pair, mandatory SSH /32, private Ollama routing, immutable artifacts, single-use authorization, USD 20 hard budget, and three cleanup paths.
- Observed the sixth real GPU receipt again prove llama3.1:8b and qwen3:8b simultaneously resident with exact digests and nonzero VRAM while Ollama remained non-public.
- Observed the Runtime node restore the complete archive and compile the Guardian successfully in 1m05s before lifecycle-soak rejected absent .git metadata.
- Captured the exact bounded diagnostic proving the lifecycle qualification root finder, rather than archive contents, was the blocker.
- Terminated only the already-orphaned readiness probe after soak failure so the failure trap and cost-bounded cleanup could proceed instead of waiting ten minutes for an impossible readiness file.
- Extended lifecycle root discovery with a narrow Git-free fallback requiring the init template under .adl/runtime-v3 and canonical adl, adl-runtime, adl-runtime-kernel, and infra/runtime-v3 markers.
- Added positive Git-free root, incomplete-layout rejection, and exact lock-placement/removal unit tests.
- Confirmed cleanup terminated both instances, removed all run volumes, destroyed Terraform state resources, and released the lock; direct AWS inventory returned zero issue instances and volumes.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "git_free_archive"
    ],
    "purpose": "Prove Git-free canonical archive root acceptance, incomplete-layout rejection, and archive-contained qualification lock placement and removal.",
    "outcome": "passed",
    "evidence_ref": "adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Prove the no-paid topology, bootstrap, complete archive, revision, diagnostics, IAM, authorization, recovery, and cleanup contracts.",
    "outcome": "passed",
    "evidence_ref": "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "run",
      "--commit",
      "f841da7db7011a3908a69c608e1decff29d4b87a",
      "--run-id",
      "adl-issue345-20260901-120543",
      "--authorization-file",
      ".adl/local/issue345/operator-authorization-runtime-init-fix.json",
      "--execute"
    ],
    "purpose": "Run the real Terraform two-node qualification through repeated GPU residency, successful Guardian compilation, exact Git-free lifecycle-root failure, and cleanup.",
    "outcome": "failed",
    "evidence_ref": ".adl/local/issue345/adl-issue345-20260901-120543"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--",
      "--check"
    ],
    "purpose": "Verify Rust formatting for the lifecycle-soak remediation.",
    "outcome": "passed",
    "evidence_ref": "adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs"
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
