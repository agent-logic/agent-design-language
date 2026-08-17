# Structured Output Record

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated the remaining C-SDLC standalone CI stale design-review fixture by aligning initialized decomposition recovery approval with canonical fresh-session UUID requirements.

## Artifacts

- csdlc-v2/tests/initialized_decomposition_recovery.rs
- .csdlc/issues/296

## Execution

- Update initialized_decomposition_recovery recovered-design approval fixture from a noncanonical fresh-session label to a canonical fresh-session UUID.
- Reproduce the exact CI failure class locally and prove the initialized decomposition recovery integration target passes.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "initialized_decomposition_recovery"
    ],
    "purpose": "Prove the remaining stale canonical fresh-session reviewer fixture exposed by CI.",
    "outcome": "passed",
    "evidence_ref": "local:r8-initialized-decomposition-recovery-2"
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
