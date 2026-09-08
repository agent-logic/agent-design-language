# Structured Output Record

Template: 1.0.0

Issue: 725

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Migrated operational C-SDLC v3 authority to a native selector and digest-bound receipt and removed the csdlc-v2 Cargo dependency.

## Artifacts

- csdlc-v3/operator/authority-selector.json
- csdlc-v3/operator/native-authority-receipt.json
- csdlc-v3/src/authority.rs
- csdlc-v3/Cargo.toml
- csdlc-v3/Cargo.lock
- csdlc-v3/README.md

## Execution

- Added native v3 authority selector and cutover provenance receipt.
- Migrated local, remote, proof/install, and terminal authority reads to the native selector.
- Replaced the v2-generated exact-head review test fixture with a native typed review receipt.
- Removed the csdlc-v2 dev-dependency and regenerated the standalone lockfile.
- Documented one-way post-cutover migration and explicit fail-closed rollback.

## Validation

[
  {
    "command": [
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets"
    ],
    "purpose": "fresh archive compile with csdlc-v2 physically absent",
    "outcome": "passed",
    "evidence_ref": "repo-local .adl/canary-725 output; no-v2 all-target compile passed at eaf71785c"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--lib"
    ],
    "purpose": "native authority, install and authenticated reconciliation behavior without v2 source",
    "outcome": "passed",
    "evidence_ref": "fresh repo-local .adl/canary-725 checkout; 36 passed at eaf71785c"
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
