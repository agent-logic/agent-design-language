# Structured Output Record

Template: 1.0.0

Issue: 5873

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented deterministic bounded placement from committed membership, verified capability evidence, admitted resource-weather projections, and fencing receipts.

## Artifacts

- adl-runtime/src/distributed/placement.rs
- adl-runtime/tests/distributed_placement.rs
- .csdlc/evidence/5873/execution-proof.json
- .csdlc/evidence/5873/operator-v2/negative-cases.json

## Execution

- Rank eligible voters deterministically by pressure, remaining capacity, and stable node identity.
- Fail closed on stale membership or advertisements, wrong trust domains, fenced nodes, inconsistent evidence, policy exhaustion, and absent eligible targets.
- Retain exact 12-test proof, strict focused Clippy, and ten machine-derived negative cases in a digest-bound two-revision receipt.

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
      "distributed_placement",
      "--no-tests=fail"
    ],
    "purpose": "Run the exact issue-owned distributed placement target.",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5873/validate-proof-receipt.rb"
    ],
    "purpose": "Validate the issue 5873 two-revision proof receipt.",
    "outcome": "passed",
    "evidence_ref": "exact-revision-proof-receipt.log"
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
