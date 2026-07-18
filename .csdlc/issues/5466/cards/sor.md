# Structured Output Record

Template: 1.0.0

Issue: 5466

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added a typed reconcile-merged publication command that observes an explicit merged PR and fails closed on final-head or identity drift.

## Artifacts

- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/tests/gate6.rs

## Execution

- Add reconcile-merged CLI request with explicit PR number
- Require current clean final-head review through the existing publication guard
- Validate merged state, final SHA, repository route, base, head, title, body, and draft state
- Record final merged publication evidence for normal readiness and closeout

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Prove merged-head identity rejection, unchanged draft publication behavior, and full C-SDLC v2 regression safety",
    "outcome": "passed",
    "evidence_ref": "local:5466-gate6-full-suite-clippy-fmt-help"
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
