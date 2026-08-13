# Structured Output Record

Template: 1.0.0

Issue: 331

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented initialized/unbound code_repository migration authority for legacy issue records without widening existing bound migration behavior.

## Artifacts

- csdlc-v2/src/migration.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/tests/code_repository_migration.rs
- .csdlc/prepared/issues/331/validate_initialized_code_repository_migration.py

## Execution

- Added a typed initialized_code_repository migration request/report/evidence contract for initialized, unbound issue records.
- Authorized migration only when exact generation/digest, source issue repository, canonical target repository, clean initialized topology, and digest-bound collision evidence all match.
- Preserved existing bound/implemented/reviewed code-repository migration report schema and behavior while adding a separate initialized/unbound route.
- Exposed the route through csdlc-issue and the public schema surface.
- Added issue-owned regression proof for digest-bound evidence, initialized/unbound evidence shape, doctor/validate readiness, and existing bound migration compatibility.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "code_repository_migration"
    ],
    "purpose": "Run the full code_repository_migration integration target covering initialized migration and existing bound migration compatibility.",
    "outcome": "passed",
    "evidence_ref": "code-repository-migration-target.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Confirm the changed C-SDLC v2 crate compiles.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-check.log"
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
    "evidence_ref": "csdlc-v2-fmt.log"
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
    "purpose": "Reject warning regressions across all C-SDLC v2 targets.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-strict-clippy.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/331/validate_initialized_code_repository_migration.py",
      "--mode",
      "initialized-nonzero"
    ],
    "purpose": "Run the issue-owned validator that verifies its own digest, exact nonzero regression tests, and doctor plus csdlc-validate readiness coverage.",
    "outcome": "passed",
    "evidence_ref": "initialized-nonzero-validator.log"
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
