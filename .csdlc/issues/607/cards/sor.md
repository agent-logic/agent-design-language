# Structured Output Record

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: sor

Status: draft

## Summary

Implemented and live-qualified the restart-safe warm two-node AWS Polis on the current four-vCPU G-family quota using r7i.2xlarge plus one-L4 g6.xlarge; all local readiness, service readiness, two-model Shepherd, six-agent ACC, resilience, cleanup, and cost gates passed.

## Artifacts

- adl/tools/run_issue607_warm_polis.sh
- adl/tools/issue607_probe_runtime.py
- adl/tools/issue607_qualify_warm_polis.sh
- adl/tools/issue607_validate_saved_plan.sh
- adl/tools/test_issue607_warm_polis.sh
- infra/aws/runtime/gpu-proof
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/evidence/607/aws-current-quota-qualification-6a8dfdd80.json
- .csdlc/evidence/607/aws-paid-action-cost-audit.json

## Execution

- Separated retained warm storage, disposable preparation, and disposable compute ownership under exact Terraform plans and single-use authorization.
- Prepared immutable Runtime and GPU AMIs plus integrity-bound persistent EBS volumes once, with no launch-time build, package installation, Git access, model download, or mutable dependency resolution.
- Kept Guardian and Runtime alive independently of qualification failures and kept Ollama private to the Runtime-to-GPU path.
- Changed the current-quota GPU default and saved-plan guard to g6.xlarge while retaining r7i.2xlarge for Runtime.
- Qualified Runtime local_ready in 4.640 seconds, GPU local_ready with two resident models in 96.090 seconds, and Terraform apply-to-service_ready in 235 seconds.
- Executed all six real Runtime agent ACC cycles and passed Guardian restart, state preservation, degradation recovery, Vector recovery, clean-log, and clean-shutdown assertions.
- Destroyed all disposable compute, network, security-group, key-pair, and attachment resources while retaining exactly the two prepared warm volumes.
- Reserved the final action at USD 0.242333 for a projected conservative issue ledger total of USD 19.842333 under the USD 20 ceiling.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/607/validate-preparation.sh"
    ],
    "purpose": "Prove the issue-specific acceptance, plan, validation, shape, and budget contract.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/final-pre-review-validation-6a8dfdd80.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_issue607_warm_polis.sh",
      "all"
    ],
    "purpose": "Prove Terraform topology, immutable activation, current-quota thresholds, authorization, cleanup, residue, and cost controls without another AWS mutation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/final-pre-review-validation-6a8dfdd80.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_spp_risk_repair_requires_review_recovery_epoch"
    ],
    "purpose": "Prove implemented-phase SPP risk correction is permitted only within a current review-recovery epoch.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/final-pre-review-validation-6a8dfdd80.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue607_warm_polis.sh",
      "qualification-quota-recovery",
      "--commit",
      "7be87dd22260d30a7966d1b129123e84bb761074",
      "--run-id",
      "adl-issue607-e8925c1dc8b0-quota-recovery",
      "--storage-id",
      "adl-issue607-warm-v6",
      "--authorization-file",
      ".adl/local/issue607/runs/adl-issue607-e8925c1dc8b0-quota-recovery/authorization.json",
      "--execute"
    ],
    "purpose": "Live-qualify the exact r7i.2xlarge plus g6.xlarge warm Polis under current quota, including two models, six ACC agents, resilience, teardown, and zero residue.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/aws-current-quota-qualification-6a8dfdd80.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject branch diff hygiene defects before exact-head review.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/final-pre-review-validation-6a8dfdd80.json"
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
