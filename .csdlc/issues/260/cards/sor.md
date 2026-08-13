# Structured Output Record

Template: 1.0.0

Issue: 260

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Migrated the remaining PlacementFencingSnapshot lease/fencing seam to governed adapters, extended the issue guard, and reconciled SPP/VPP execution truth.

## Artifacts

- adl-runtime/src/distributed/placement.rs
- adl-runtime/tests/distributed_authority_adapter_callers_260.rs
- .csdlc/issues/260/cards/spp.values.json
- .csdlc/issues/260/cards/vpp.values.json

## Execution

- Production PlacementFencingSnapshot capture now consumes AuthorityBoundLeaseLedger and AuthorityBoundFencingStore.
- Adapter failures map fail-closed to inconsistent evidence; raw placement stores remain cfg(test)-only.
- Issue guard covers placement lease/fencing type aliases and helper routing.
- SPP steps and VPP lanes now agree with actual implemented validation.

## Validation

[
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Compile production Runtime caller migrations.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/260/runtime-lib-clippy.log"
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
    "purpose": "Prove placement behavior after governed capture migration.",
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
      "distributed_migration",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove migration retry and transition behavior.",
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
    "purpose": "Prove recovery retry and fail-closed behavior.",
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

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
