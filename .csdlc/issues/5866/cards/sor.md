# Structured Output Record

Template: 1.0.0

Issue: 5866

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented bounded authenticated seed discovery with caller-retained replay state at both request-proposal and proposal-acceptance boundaries.

## Artifacts

- adl-runtime/src/distributed/discovery.rs
- adl-runtime/tests/distributed_discovery.rs
- .csdlc/evidence/5866/remediation/execution-proof.json
- .csdlc/evidence/5866/remediation/negative-cases.json

## Execution

- Treat configured seeds only as bounded addresses and expected peer identities, never as enrollment or membership authority.
- Require live enrollment and authenticated transport identity, generation, trust domain, and protocol before emitting deterministic non-voting proposals.
- Persist bounded request and proposal replay observations across public calls through caller-owned discovery context.
- Fail closed on duplicate configured seeds, stale seed certificate generations, cross-call replay, timeout, cancellation, malformed input, wrong domain, revocation, rotation, expiry, and resource exhaustion.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_discovery",
      "--no-tests=fail"
    ],
    "purpose": "Run the exact issue-owned positive and fail-closed discovery target.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5866/remediation/distributed-discovery.stdout.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_discovery",
      "--",
      "-D",
      "warnings",
      "-A",
      "clippy::absurd_extreme_comparisons"
    ],
    "purpose": "Run strict focused Clippy for the repaired discovery surface.",
    "outcome": "passed",
    "evidence_ref": "local:strict-focused-clippy"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5866/validate-proof-receipt.rb",
      ".csdlc/evidence/5866/remediation/execution-proof.json"
    ],
    "purpose": "Validate the fresh two-revision remediation receipt.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5866/remediation/execution-proof.json"
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
