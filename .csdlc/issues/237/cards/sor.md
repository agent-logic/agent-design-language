# Structured Output Record

Template: 1.0.0

Issue: 237

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

A successfully constructed LiveAssembly exclusively owns the unforgeable provisioning capability that establishes opaque capability authority for an exact verified continuity record; callers cannot self-authorize token substitution and publication remains pending.

## Artifacts

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/capability_envelope.rs
- adl-runtime-kernel/src/cognitive_profile.rs
- adl-runtime-kernel/tests/capability_envelope.rs
- adl-runtime-kernel/tests/fixtures/cognitive_profile/authority_tests.rs
- .csdlc/evidence/237/continuity-public-api-target.log
- .csdlc/evidence/237/continuity-authority-lib.log
- .csdlc/evidence/237/continuity-public-boundary-doc.log
- .csdlc/evidence/237/continuity-strict-lib-clippy.log

## Execution

- Add private RuntimeCapabilityProvisioner state to LiveAssembly and expose reauthorization only as LiveAssembly::provision_capability_authority.
- Keep RuntimeCapabilityProvisioner construction/provisioning and CapabilityAuthorityPolicy construction crate-private with compile-fail external boundaries.
- Require the opaque capability authority at every public verified-continuity capability and governed-cognition build and validation entrypoint.
- Prove an actual production LiveAssembly provisions authority, retained authority A rejects a fully rewritten token-B capability and rebuilt cognition, and explicit assembly-owned reauthorization can authorize B.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--test",
      "capability_envelope"
    ],
    "purpose": "Prove public verified-continuity entrypoints remain exported.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/237/continuity-public-api-target.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--lib"
    ],
    "purpose": "Prove 80 Runtime library tests including LiveAssembly provisioning, real composition, token-B rewrite rejection, explicit reauthorization, authority, and privacy negatives.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/237/continuity-authority-lib.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--doc"
    ],
    "purpose": "Prove ten compile-fail public authority boundaries including provisioner construction and direct new access.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/237/continuity-public-boundary-doc.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across the changed Runtime library surface.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/237/continuity-strict-lib-clippy.log"
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
