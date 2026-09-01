# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved all four exact-head review findings in the optional AWS GPU proof runner; local and real read-only AWS proofs pass, while paid GPU guest execution remains pending a fresh exact-head review and typed publication.

## Artifacts

- adl-runtime/tests/shepherd_local_model.rs
- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/345/issue345-runner-contract.log
- .csdlc/evidence/345/issue345-live-preflight.json
- .csdlc/evidence/345/issue345-diff-hygiene.log

## Execution

- Normalize Ollama's optional sha256 digest prefix before the governed real-model smoke test compares the installed model with the manifest digest.
- Derive the retained single-use marker from canonical JSON so key ordering and whitespace cannot replay an authorization.
- Require authorization schema v2 to bind the business-account hash, immutable manifest, exact IAM and security-group pins, reaper identity, and resolved AMI and subnet hashes.
- Extract pure IAM-trust and cleanup-owner validators and execute focused fixtures for canonical authorization identity, authorization mismatches, reaper bounds, resolved AMI drift, both trust policies, and cleanup ownership.
- Correct AC-8 and the runbook to separate local executable negative proof, live read-only AWS-state verification, and the still-pending paid GPU lane.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Execute ten local no-AWS safety cases and the runner contract without paid launch.",
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
    "purpose": "Execute the real read-only AWS preflight plus five live drift cases and ten local safety cases without paid launch.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/345/issue345-live-preflight.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "shepherd_local_model",
      "--no-run"
    ],
    "purpose": "Compile the corrected real Ollama model smoke test.",
    "outcome": "passed",
    "evidence_ref": "adl-runtime/tests/shepherd_local_model.rs"
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
