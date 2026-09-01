# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

The ninth authorized Terraform run launched the Runtime and GPU nodes in us-west-2d with mandatory SSH and one key pair. The GPU node verified the immutable S3 artifacts and again proved llama3.1:8b and qwen3:8b simultaneously resident with exact digests and nonzero VRAM. The Runtime node restored the Git-free archive and compiled Guardian successfully, then its authenticated WSS fanout failed because the lifecycle client requested the unversioned Observatory feed, which defaults to v2, while asserting the v3 feed schema. The exact orphaned probe was terminated after the immediate failure to avoid idle billing. Cleanup destroyed both nodes and all volumes, emptied Terraform state, and released the lock. The lifecycle client now explicitly requests schema=v3 and a focused regression test passes. Final paid proof remains pending exact-head review and rerun.

## Artifacts

- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- .adl/local/issue345/adl-issue345-20260901-141836/gpu-ready.json
- .adl/local/issue345/adl-issue345-20260901-141836/runtime-final.json
- .adl/local/issue345/adl-issue345-20260901-141836/cleanup-on-exit.json

## Execution

- Retained the reviewed Terraform-owned two-node topology, one SSH key, mandatory /32 SSH ingress, private Ollama routing, immutable artifacts, single-use authorization, and USD 20 hard ceiling.
- Observed llama3.1:8b and qwen3:8b simultaneously resident on the L4 with exact expected digests and 5271715839 and 5578204118 bytes of nonzero VRAM.
- Observed the Runtime node restore the exact Git-free source archive, compile Guardian in 1m10s, pass HTTPS fanout, and reach authenticated WSS fanout.
- Captured the exact WSS failure: the client requested the unversioned route, which defaults to the previous v2 feed, while requiring the v3 feed schema.
- Changed only the lifecycle WSS upgrade request to include ?schema=v3 and added a focused exact request regression test.
- Terminated only the already-orphaned WSS probe after the deterministic failure so cleanup could proceed without ten idle minutes.
- Confirmed cleanup left zero instances and volumes, zero Terraform resources, and no active S3 lock.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "tests::observatory_wss_qualification_requests_current_feed_schema",
      "--",
      "--exact"
    ],
    "purpose": "Prove the lifecycle WSS upgrade explicitly selects the v3 feed and retains the configured Observatory origin.",
    "outcome": "passed",
    "evidence_ref": "adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs"
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
    "purpose": "Verify Rust formatting for the WSS schema-selection remediation.",
    "outcome": "passed",
    "evidence_ref": "adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "run",
      "--commit",
      "9d1a1184c2117a81c7ef0f04b24012de584dfd0f",
      "--run-id",
      "adl-issue345-20260901-141836",
      "--authorization-file",
      ".adl/local/issue345/operator-authorization-final-2d.json",
      "--execute"
    ],
    "purpose": "Run the real us-west-2d two-node qualification through repeated GPU residency, successful Guardian compilation, exact WSS feed-version failure, and zero-residue cleanup.",
    "outcome": "failed",
    "evidence_ref": ".adl/local/issue345/adl-issue345-20260901-141836"
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
