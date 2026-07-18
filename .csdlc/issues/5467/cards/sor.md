# Structured Output Record

Template: 1.0.0

Issue: 5467

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Made the backend snapshot contract reachable, removed false setup assertions, and extracted backend selection into a locally testable fail-closed helper.

## Artifacts

- .github/workflows/ci.yaml
- adl/tools/resolve_ci_backend.sh
- adl/tools/test_run_aws_spot_ci_profile.sh

## Execution

- Update the stale builder validation assertion to its current owner contract
- Remove ten false setup assertions whose target strings never existed in the named script
- Count and require execution of all seventeen backend workflow snapshot assertions
- Prove default hosted, explicit hosted, Spot-selected, and invalid backend behavior locally
- Keep CI backend selection semantics in a pure helper used by the workflow

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_ci_profile.sh"
    ],
    "purpose": "Prove all backend snapshots are reachable and default hosted, explicit hosted, Spot-selected, and invalid backend values behave locally without AWS",
    "outcome": "passed",
    "evidence_ref": "local:5467-backend-snapshot-contract-shell-syntax-behavior-exit2-diff-check"
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
