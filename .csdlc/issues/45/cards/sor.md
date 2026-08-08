# Structured Output Record

Template: 1.0.0

Issue: 45

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented explicit typed split-repository authority for bind, doctor, and publication consistency.

## Artifacts

- csdlc-v2/src/doctor.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/model.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/operator/skills
- docs/tooling/C_SDLC_V2_ISSUE_CREATION_AND_BINDING_RUNBOOK.md
- docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
- docs/tooling/adl_pr_cycle_skill.md
- csdlc-v2/src/doctor.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/model.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/operator/skills
- docs/tooling

## Execution

- Record optional code_repository identity in canonical issue state through csdlc-bind.
- Compare doctor Git origin evidence against explicit code authority without inferring a split from remote names.
- Reject publication-time code repository substitution against the bound canonical record.
- Add deterministic gate2 coverage for same-repository, valid split, absent split, and mismatched split cases.
- Update active typed skills, lifecycle runbooks, and adl_pr_cycle installation contract.
- Record optional code_repository identity through typed bind.
- Accept same-repository and explicit valid split routes while rejecting absent or mismatched drift.
- Prevent publication-time code repository substitution.
- Update focused regression coverage, active skills, and runbooks.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove all C-SDLC v2 targets remain warning-free with the expanded typed schema.",
    "outcome": "passed",
    "evidence_ref": "doctor-contract-and-lint.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Prove same-repository acceptance, explicit valid split acceptance, and fail-closed drift rejection.",
    "outcome": "passed",
    "evidence_ref": "doctor-repository-identity.log"
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
