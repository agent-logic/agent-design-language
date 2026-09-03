# Structured Output Record

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: sor

Status: draft

## Summary

The final source-bound warm two-node AWS Polis qualification passed in 240 seconds apply-to-service-ready for the current two-8B-model configuration. Runtime local readiness was 5.950 seconds and GPU local readiness was 97.340 seconds. Both models remained resident, all six Runtime agents executed governed ACC tools, Guardian and degradation recovery passed, all disposable resources were removed, and both warm EBS volumes remain detached and available. The exact run-bound operator extension from USD 20 to USD 21 is retained as immutable evidence. One later AWS read observed both exact instances terminated and conservatively bounds Runtime at 3001 seconds and GPU at 3015 seconds, producing a USD 20.983286 issue total with USD 0.016714 remaining. The final substantive revision 54aa257b0 has retained green local, Terraform, typed-card, and diff-hygiene proof.

## Artifacts

- adl/tools/run_issue607_warm_polis.sh
- adl/tools/test_issue607_warm_polis.sh
- infra/aws/runtime/gpu-proof
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/607/operator-budget-extension.json
- .csdlc/evidence/607/aws-terminal-state-observation.json
- .csdlc/evidence/607/aws-paid-action-cost-audit.json
- .csdlc/evidence/607/aws-payload-recovery-qualification.json
- .csdlc/evidence/607/final-validation-54aa257b0.json

## Execution

- Implemented prebuilt immutable Runtime and GPU launch artifacts, persistent EBS data volumes, strict saved-plan identity, no launch-time builds or downloads, SSH, private Ollama, Guardian resilience, and exact cleanup.
- Retained the operator's exact USD 21 extension bound to the payload-recovery run, controller, plan, and prior authorization digest.
- Used one observed-terminal AWS response for both exact instances and aligned the audit as_of timestamp and freshness validator with that evidence.
- Conservatively charged Runtime for 3001 seconds and GPU for 3015 seconds through the shared terminal observation; the issue total remains under USD 21.
- Future launches reserve 900 seconds for the full billable lifetime, keep a separate 420-second service-operation deadline, wait for both exact instances to terminate, and are now rejected under the remaining issue budget.
- Retained final validation for exact substantive revision 54aa257b0 and tree 4e4b767c7.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_issue607_warm_polis.sh",
      "all"
    ],
    "purpose": "Prove terminal observation, audit freshness, full-lifetime reservation, extension binding, cleanup, authorization, cost, residue, and recovery contracts.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/final-validation-54aa257b0.json"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/aws/runtime/gpu-proof",
      "test",
      "-filter=tests/issue607_warm.tftest.hcl"
    ],
    "purpose": "Prove rendered warm Runtime payload and Terraform topology.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/final-validation-54aa257b0.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue607_warm_polis.sh",
      "qualification-payload-recovery",
      "--execute"
    ],
    "purpose": "Prove the source-bound current two-8B-model warm Polis on real AWS GPU infrastructure.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/aws-payload-recovery-qualification.json"
  },
  {
    "command": [
      "aws",
      "ec2",
      "describe-instances",
      "--instance-ids",
      "i-00da9426786754a6c",
      "i-03bb0138945501212"
    ],
    "purpose": "Prove both exact final-run instances reached terminal state and bind a symmetric conservative lifetime upper bound.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/aws-terminal-state-observation.json"
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
