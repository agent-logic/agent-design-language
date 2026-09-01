# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

After AWS refused g6.xlarge placement in 2a and then transiently in 2b and 2c, authorization-bound preferred-subnet routing launched both nodes in us-west-2d. The GPU node again proved both configured models simultaneously resident with exact digests and nonzero VRAM. The Runtime node restored the complete Git-free archive, compiled Guardian in 1m06s, passed Git-free root and polis-domain validation, then failed because the validator localized the Observatory allowed-origin entry without localizing polis.observatory_public_origin. The orphaned probe alone was terminated after the immediate soak failure. Cleanup destroyed both nodes and volumes and released the lock. The validator now localizes both Observatory-origin fields together and rejects missing or duplicate polis origin fields. Final paid proof remains pending review and rerun.

## Artifacts

- adl/tools/validate_v092_runtime_guardian_lifecycle.sh
- adl/tools/test_run_issue268_six_hour_spot_qualification.sh
- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- .adl/local/issue345/adl-issue345-20260901-134726/gpu-ready.json
- .adl/local/issue345/adl-issue345-20260901-134726/runtime-final.json
- .adl/local/issue345/adl-issue345-20260901-134726/cleanup-on-exit.json

## Execution

- Retained the reviewed two-node Terraform topology, one SSH key, /32 SSH ingress, private Ollama, immutable artifacts, single-use authorization, and USD 20 ceiling.
- Used the reviewed preferred-subnet path to route around transient g6.xlarge capacity refusals while preserving VPC, GPU offering, route, network ACL, preflight, and authorization binding.
- Observed the us-west-2d GPU receipt prove llama3.1:8b and qwen3:8b simultaneously resident with exact digests and nonzero VRAM while Ollama remained non-public.
- Observed Runtime restore the archive, compile Guardian in 1m06s, and pass the previously repaired Git-free root and polis-domain checks.
- Captured the exact remaining policy error: polis.observatory_public_origin was not present in the localized Observatory allowed-origin set.
- Localized the polis Observatory origin to the same test origin as the allowed-origin entry and added fail-closed missing and duplicate cases.
- Confirmed cleanup left zero issue instances and volumes, destroyed Terraform state resources, and released the lock.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue268_six_hour_spot_qualification.sh"
    ],
    "purpose": "Run the extracted Runtime-init localization contract, including coupled Observatory origins and missing or duplicate negative cases.",
    "outcome": "failed",
    "evidence_ref": "adl/tools/test_run_issue268_six_hour_spot_qualification.sh"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Prove the issue-owned no-paid topology, capacity subnet, authorization, recovery, and cleanup contracts.",
    "outcome": "passed",
    "evidence_ref": "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "run",
      "--commit",
      "993cd46101709a400a2d3e3615921219bea85ede",
      "--run-id",
      "adl-issue345-20260901-134726",
      "--authorization-file",
      ".adl/local/issue345/operator-authorization-capacity-subnet-2d.json",
      "--execute"
    ],
    "purpose": "Run the real us-west-2d two-node qualification through repeated GPU residency, Guardian compilation, exact Observatory-origin policy failure, and cleanup.",
    "outcome": "failed",
    "evidence_ref": ".adl/local/issue345/adl-issue345-20260901-134726"
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
