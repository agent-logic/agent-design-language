# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

The Terraform two-node path launched successfully and proved two-model GPU residency, then exposed an unset-HOME bootstrap defect before Runtime proof. Terraform cleanup terminated both nodes and removed their volumes. The worktree now explicitly defines HOME for both bootstraps and Ollama and reduces the exact source archive from 335 MiB to 23 MiB; final paid proof remains pending review and rerun.

## Artifacts

- infra/aws/runtime/gpu-proof
- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .adl/local/issue345/adl-issue345-20260901-090918/gpu-ready.json
- .adl/local/issue345/adl-issue345-20260901-090918/runtime-final.json
- .adl/local/issue345/adl-issue345-20260901-090918/terraform.tfstate

## Execution

- Retained the reviewed Terraform-owned two-node topology, one key pair, mandatory SSH /32, private Ollama routing, immutable artifacts, single-use authorization, cost bound, and three cleanup paths.
- Replaced the hung single PUT with multipart upload plus fail-closed immutable VersionId verification.
- Observed a real GPU receipt proving llama3.1:8b and qwen3:8b simultaneously resident with exact digests and nonzero VRAM while Ollama remained non-public.
- Observed Runtime cloud-init fail before Rust execution because HOME was unset and recorded the failed receipt without overstating Guardian or six-agent proof.
- Made HOME=/root explicit in both bootstrap scripts and the Ollama systemd unit.
- Reduced the exact reviewed source archive to the tracked adl and adl-runtime build/proof trees, excluding unrelated media and historical evidence.
- Confirmed controller cleanup drove both instances to termination and removed run volumes before another authorization.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Prove twenty-four no-paid topology, bootstrap, archive, IAM, authorization, input-drift, recovery, and cleanup contracts.",
    "outcome": "passed",
    "evidence_ref": "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "run",
      "--commit",
      "e54a3da961b8f65eeebe6e463f7d99bfbbc05668",
      "--run-id",
      "adl-issue345-20260901-090918",
      "--execute"
    ],
    "purpose": "Run the real Terraform two-node qualification through GPU residency, Runtime bootstrap, failure receipt, and cleanup.",
    "outcome": "failed",
    "evidence_ref": ".adl/local/issue345/adl-issue345-20260901-090918"
  },
  {
    "command": [
      "git",
      "archive",
      "--format=tar",
      "HEAD",
      "--",
      "adl",
      "adl-runtime"
    ],
    "purpose": "Verify the selective exact-source archive contains both required build trees and is 23 MiB instead of 335 MiB.",
    "outcome": "passed",
    "evidence_ref": ".adl/local/issue345/selective-source-test.tar"
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
