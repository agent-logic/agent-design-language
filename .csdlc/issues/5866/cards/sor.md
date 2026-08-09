# Structured Output Record

Template: 1.0.0

Issue: 5866

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented bounded authenticated seed discovery with atomic request-plus-proposal replay admission and expiry-bounded caller-retained replay state.

## Artifacts

- adl-runtime/src/distributed/discovery.rs
- adl-runtime/tests/distributed_discovery.rs
- .csdlc/evidence/5866/replay-window/execution-proof.json
- .csdlc/evidence/5866/replay-window/negative-cases.json

## Execution

- Treat configured seeds only as bounded addresses and expected peer identities, never as enrollment or membership authority.
- Atomically retain accepted request and proposal identifiers so the same request cannot bypass replay denial through a different valid seed.
- Retain live replay entries through their signed validity horizon, deny capacity overflow while all entries are live, and recover capacity only after trusted time passes expiry.
- Fail closed on duplicate seeds, stale certificate generations, cross-call replay, timeout, cancellation, malformed input, wrong domain, revocation, rotation, expiry, and resource exhaustion.

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
    "purpose": "Run the exact issue-owned positive and fail-closed discovery target including cross-seed replay and bounded-window recovery.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5866/replay-window/distributed-discovery.stdout.log"
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
    "purpose": "Run strict focused Clippy for the final discovery surface while allowing only the qualified pre-existing ACIP lint.",
    "outcome": "passed",
    "evidence_ref": "local:strict-focused-clippy"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5866/validate-proof-receipt.rb",
      ".csdlc/evidence/5866/replay-window/execution-proof.json"
    ],
    "purpose": "Validate the final two-revision replay-window receipt.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5866/replay-window/execution-proof.json"
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
