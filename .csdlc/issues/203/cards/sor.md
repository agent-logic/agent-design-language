# Structured Output Record

Template: 1.0.0

Issue: 203

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented authority-serving adapters, canonical lease-time state, migration/recovery authority-bound compatibility, and retained immutable issue #203 proof receipt.

## Artifacts

- .csdlc/evidence/203/v1/authority-store-proof.json
- .csdlc/evidence/203/v1/identity-authority.stdout.log
- .csdlc/evidence/203/v1/identity-authority.stderr.log
- .csdlc/evidence/203/v1/identity-clippy.stdout.log
- .csdlc/evidence/203/v1/identity-clippy.stderr.log

## Execution

- Sealed certificate, lease, and fencing access behind authority-bound adapters and permit checks.
- Removed node-local elapsed timestamps from canonical lease state and bound mutation authorization to wall-clock fields.
- Migrated normal-build migration and recovery consumers to authority-bound store handles while preserving cfg(test) raw fixtures.
- Raised generated test certificate duration defaults to a minimum ten minutes and kept shorter durations only for explicit expiry behavior tests.
- Added exact issue #203 44-case and 132-subassertion proof producer, validator, and immutable evidence.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/203/validate-proof-receipt.rb"
    ],
    "purpose": "Finalize issue #203 implemented truth using the retained local proof receipt without rerunning broad hosted or coverage jobs.",
    "outcome": "passed",
    "evidence_ref": "issue-203-retained-proof-receipt.log"
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
