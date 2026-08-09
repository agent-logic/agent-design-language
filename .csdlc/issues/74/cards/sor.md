# Structured Output Record

Template: 1.0.0

Issue: 74

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added the exact claim-bearing foreign topology regression without changing production behavior.

## Artifacts

- csdlc-v2/tests/gate2.rs

## Execution

- Extended the real csdlc-bind Gate 2 canary with an unrelated legacy projection containing the retired claim field.
- Proved bind leaves the unrelated claim-bearing projection byte-for-byte unchanged.
- Proved the same retired field on the relevant issue remains strict corruption while existing collision checks remain fail closed.

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
      "gate2",
      "bind_topology_scan_uses_canonical_record_identity"
    ],
    "purpose": "Prove unrelated claim-bearing legacy records are ignored without mutation while relevant corruption and ownership collisions remain fail closed.",
    "outcome": "passed",
    "evidence_ref": "local:issue-74-gate2:1-passed"
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
