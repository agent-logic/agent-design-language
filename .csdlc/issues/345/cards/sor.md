# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved all exact-head review findings in the optional AWS GPU proof runner; local and real read-only AWS proofs pass, while paid GPU guest execution remains pending fresh exact-head review and typed publication.

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
- Require the authorization's full immutable review revision to equal the current passing typed C-SDLC review and reject substantive proof-surface drift after that reviewed source commit.
- Resolve the security group, AMI, and subnet once, authorize their hashes through preflight, and reuse those exact values for launch to remove the TOCTOU gap.
- Execute focused local fixtures for canonical authorization identity, fabricated review revision, authorization mismatches, reaper bounds, resolved AMI drift, both IAM trust policies, and cleanup ownership.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Execute eleven local no-AWS safety cases and the runner contract without paid launch.",
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
    "purpose": "Execute the real read-only AWS preflight plus five live drift cases and eleven local safety cases without paid launch.",
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
