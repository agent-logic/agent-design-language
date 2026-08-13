# Structured Output Record

Template: 1.0.0

Issue: 331

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented initialized/unbound code_repository migration authority for legacy issue records and repaired review-found evidence disposition reporting without widening existing bound migration behavior.

## Artifacts

- csdlc-v2/src/migration.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/tests/code_repository_migration.rs
- .csdlc/prepared/issues/331/validate_initialized_code_repository_migration.py
- .csdlc/evidence/331/post-p1-initialized-validator.log
- .csdlc/evidence/331/post-p1-code-repository-migration.log
- .csdlc/evidence/331/post-p1-fmt.log
- .csdlc/evidence/331/post-p1-check.log
- .csdlc/evidence/331/post-p1-clippy.log

## Execution

- Added a typed initialized_code_repository migration request/report/evidence contract for initialized, unbound issue records.
- Authorized migration only when exact generation/digest, source issue repository, canonical target repository, clean initialized topology, and digest-bound collision evidence all match.
- Recorded the parsed canonical collision evidence disposition in initialized migration evidence.
- Recorded the explicit cross-repository authority disposition as legacy_issue_authority_with_canonical_code_repository in initialized migration evidence.
- Preserved existing bound/implemented/reviewed code-repository migration report schema and behavior while adding a separate initialized/unbound route.
- Exposed the route through csdlc-issue and the public schema surface.
- Added and reran issue-owned regression proof for digest-bound evidence, initialized/unbound evidence shape including disposition fields, doctor/validate readiness, and existing bound migration compatibility.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/331/validate_initialized_code_repository_migration.py",
      "--mode",
      "initialized-nonzero"
    ],
    "purpose": "Run the issue-owned validator that verifies its own digest, exact nonzero initialized migration regressions, and doctor plus csdlc-validate readiness coverage.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/331/post-p1-initialized-validator.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "code_repository_migration"
    ],
    "purpose": "Run the full code_repository_migration integration target covering initialized migration evidence disposition fields and existing bound migration compatibility.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/331/post-p1-code-repository-migration.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--check"
    ],
    "purpose": "Reject Rust formatting drift in the changed C-SDLC v2 crate.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/331/post-p1-fmt.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Confirm the changed C-SDLC v2 crate compiles after the evidence disposition remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/331/post-p1-check.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warning regressions across all C-SDLC v2 targets after the evidence disposition remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/331/post-p1-clippy.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
