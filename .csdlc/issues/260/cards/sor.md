# Structured Output Record

Template: 1.0.0

Issue: 260

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Migrated all declared non-transport Runtime authority callers to governed adapters and retained unambiguous command-bound proof, including the final placement seam.

## Artifacts

- adl-runtime/src/distributed
- adl-runtime/tests/distributed_authority_adapter_callers_260.rs
- .csdlc/evidence/260

## Execution

- Production callers use AuthorityBound certificate, lease, and fencing adapters.
- Raw store/token seams are cfg(test)-only.
- Placement capture adapter failures deny closed.
- SPP/VPP and retained evidence agree with implemented truth.

## Validation

[
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Compile production Runtime caller migrations; SHA256 02fd4914b45cb8a562d4256938704a3729cbdb794ff85e216ac2a0d13ed06d82.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/260/cargo-check-r2.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authority_adapter_callers_260",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove governed caller boundaries.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/260/authority-adapter-callers-260.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_placement",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove placement behavior; SHA256 3aec3a55871f8f01acbd38706e32a678289716c0375ed251701739bb3177e601.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/260/distributed-placement-r2.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_migration",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove migration transitions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/260/migration-recovery.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_recovery",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove recovery transitions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/260/migration-recovery.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject production lint regressions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/260/runtime-lib-clippy.log"
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
