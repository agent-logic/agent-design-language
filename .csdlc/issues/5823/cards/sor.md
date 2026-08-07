# Structured Output Record

Template: 1.0.0

Issue: 5823

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a provider-neutral bounded remote-validation contract across local, Nessus, and AWS adapters with exact revision, provenance, timeout, cost, artifact, cleanup, redaction, and same-profile fallback semantics.

## Artifacts

- tools/remote_validation
- tools/aws_remote_validation/src/aws_remote_validation.rs
- tools/aws_remote_validation/tests/portable_adapter.rs
- adl/tools/run_nessus_remote_validation.sh
- adl/tools/test_run_nessus_remote_validation.sh
- adl/tools/run_aws_spot_remote_validation_lane.sh
- adl/tools/test_run_aws_spot_remote_validation_lane.sh
- .csdlc/evidence/5823

## Execution

- Added the typed portable request, adapter plan, local execution, and result-validation crate with exact revision and command-profile binding.
- Integrated portable requests with the Nessus and AWS wrappers, including advertised-ref plus immutable-revision enforcement and fail-closed budget propagation.
- Added focused positive and negative contract, timeout, cancellation, stale-revision, malformed-result, ref-mismatch, fallback, and cleanup tests.
- Retained native live macOS and AWS Linux proof, an explicitly non-native Windows fixture, redacted provider receipts, exact cost bounds, and complete resource cleanup evidence.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "tools/aws_remote_validation/Cargo.toml",
      "--test",
      "portable_adapter"
    ],
    "purpose": "Prove portable request mapping, advertised-ref checkout, cost ceiling, provider failures, and cleanup semantics.",
    "outcome": "passed",
    "evidence_ref": "final-aws-adapter.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_remote_validation_lane.sh"
    ],
    "purpose": "Prove portable AWS wrapper mapping, exact ref/revision mismatch rejection, timeout, cost ceiling, and cleanup controls without provider use.",
    "outcome": "passed",
    "evidence_ref": "final-aws-shell.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace errors on the bounded worktree diff.",
    "outcome": "passed",
    "evidence_ref": "final-diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_nessus_remote_validation.sh"
    ],
    "purpose": "Prove portable Nessus mapping, quoting, exact revision, and same-profile fallback behavior.",
    "outcome": "passed",
    "evidence_ref": "final-nessus-shell.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5823/validate-platform-matrix.rb"
    ],
    "purpose": "Require native live Linux and macOS receipts plus an explicitly non-native Windows fixture and reject blocked rows.",
    "outcome": "passed",
    "evidence_ref": "final-platform-matrix.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "tools/remote_validation/Cargo.toml",
      "--test",
      "contract"
    ],
    "purpose": "Prove exact revision, source-ref, profile digest, adapter, artifact, redaction, timeout, cancellation, cleanup, and fallback behavior.",
    "outcome": "passed",
    "evidence_ref": "final-portable-contract.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
