# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved all exact-head review findings in the optional AWS GPU proof runner; thirteen local and eighteen combined read-only safety cases pass, while paid GPU execution remains pending fresh exact-head review and typed publication.

## Artifacts

- adl-runtime/tests/shepherd_local_model.rs
- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/345/issue345-runner-contract.log
- .csdlc/evidence/345/issue345-live-preflight.json
- .csdlc/evidence/345/issue345-diff-hygiene.log

## Execution

- Document execution from the clean reviewed or published lifecycle head with --commit naming its unchanged reviewed substantive ancestor.
- Execute the typed-review equality guard and real-Git substantive-drift rejection against isolated review indexes.
- Build the EC2 run-instances argv through one pure function and prove that the authorized security-group, AMI, and subnet values each reach launch exactly once.
- Retain canonical authorization identity, exact infrastructure binding, full review revision equality, bounded cost, multi-model residency, Guardian, Runtime-agent ACC, cleanup, and 24/7 non-claim boundaries.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Execute thirteen local no-AWS safety cases including exact-review, drift, and launch-argv regressions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/345/issue345-runner-contract.log"
  },
  {
    "command": [
      "env",
      "ADL_ISSUE345_LIVE_PREFLIGHT=1",
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Execute the real read-only AWS preflight plus all eighteen combined safety cases without paid launch.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/345/issue345-live-preflight.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject patch hygiene defects.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/345/issue345-diff-hygiene.log"
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
