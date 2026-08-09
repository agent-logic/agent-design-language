# Structured Output Record

Template: 1.0.0

Issue: 75

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Add a typed closing or part_of publication linkage mode and preserve it through request, intent, remote observation, evidence, and fail-closed finish authority.

## Artifacts

- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/model.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/tests/gate6.rs
- csdlc-v2/tests/gate_finish.rs

## Execution

- Add a strum-backed PublicationLinkageMode with backward-compatible closing default.
- Require exact same-repository or qualified split-repository part_of references and reject mixed linkage.
- Retain linkage mode through publication intent, remote observation, and canonical evidence.
- Reject part_of evidence as terminal closeout authority.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish"
    ],
    "purpose": "Prove fail-closed finish behavior.",
    "outcome": "passed",
    "evidence_ref": "finish-linkage.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate6"
    ],
    "purpose": "Prove publication linkage behavior.",
    "outcome": "passed",
    "evidence_ref": "publication-linkage.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "schema::tests"
    ],
    "purpose": "Prove public schema propagation.",
    "outcome": "passed",
    "evidence_ref": "schema.log"
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
