# Structured Output Record

Template: 1.0.0

Issue: 194

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the private Wuji/AWS recovery qualification harness and recorded the current AWS quota gate truth: Wuji local restart recovery, private two-voter TCP mesh, private one-GPU model-health, S3 endpoint artifact flow, and zero-instance cleanup are proven; simultaneous two model-capable AWS GPU voters remain blocked by current Agent Logic AWS quota/capacity and are explicitly not claimed.

## Artifacts

- adl/tools/issue194_private_network.cloudformation.json
- adl/tools/issue194_private_wuji_aws_runner.sh
- adl/tools/private_wuji_aws_recovery_qualification.py
- adl/tools/issue194_model_health_command.py
- adl/tools/test_issue194_private_network_template.sh
- adl/tools/test_private_wuji_aws_recovery_qualification.sh
- .csdlc/evidence/194/private-wuji-aws-recovery-live-summary.redacted.json
- .csdlc/evidence/194/live-runs/issue-194-wuji-local-recovery-20260817/wuji-recovery.redacted.json
- .csdlc/evidence/194/live-runs/issue-194-gpu-feasibility-live/gpu-feasibility.redacted.json
- .csdlc/evidence/194/live-runs/issue-194-quota-live-g6xlarge-two-voters/quota-preflight.redacted.json

## Execution

- Added a private CloudFormation topology for #194 with private subnets, SSM/S3 VPC endpoints, no public ingress, and self-security-group voter TCP mesh.
- Added the #194 private Wuji/AWS runner with zero-instance assertion, network preflight, quota/GPU feasibility checks, private voter launch, SSM smoke, direct TCP mesh smoke, S3 artifact smoke, model-health smoke, Wuji local recovery receipt, and fail-closed serial hybrid gate.
- Added deterministic local validation for the private network template and qualification runner fail-closed behavior, including quota insufficiency, undersized fractional GPU rejection, structured Wuji receipt validation, and insufficient one-GPU serial proof rejection.
- Recorded redacted live evidence for Wuji local recovery, private AWS two-voter mesh, one-GPU private model-health, current quota infeasibility for two model-capable AWS voters, and current zero-instance cleanup.

## Validation

[
  {
    "command": [
      "/bin/bash",
      "adl/tools/test_issue194_private_network_template.sh"
    ],
    "purpose": "Validate private CloudFormation topology guardrails for #194.",
    "outcome": "passed",
    "evidence_ref": "network_template_contract.log"
  },
  {
    "command": [
      "/bin/bash",
      "adl/tools/test_private_wuji_aws_recovery_qualification.sh"
    ],
    "purpose": "Validate #194 fail-closed quota, Wuji receipt, and serial hybrid proof contracts.",
    "outcome": "passed",
    "evidence_ref": "qualification_fail_closed.log"
  },
  {
    "command": [
      "python3",
      "-m",
      "json.tool",
      ".csdlc/evidence/194/private-wuji-aws-recovery-live-summary.redacted.json"
    ],
    "purpose": "Validate retained redacted #194 live-summary JSON syntax.",
    "outcome": "passed",
    "evidence_ref": "redacted_summary_json.log"
  },
  {
    "command": [
      "/bin/bash",
      "-n",
      "adl/tools/issue194_private_wuji_aws_runner.sh"
    ],
    "purpose": "Validate #194 runner shell syntax.",
    "outcome": "passed",
    "evidence_ref": "runner_shell_syntax.log"
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
