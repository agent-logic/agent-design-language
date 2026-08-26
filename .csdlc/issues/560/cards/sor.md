# Structured Output Record

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Stabilized the runtime_v2 unified-kernel workspace coverage gate by granting only the three timed-out correlation/drift proofs a bounded ci-coverage nextest timeout override while preserving the default 120s ceiling for unrelated tests.

## Artifacts

- adl/.config/nextest.toml
- .csdlc/prepared/issues/560/validate-focused-proof.sh
- .csdlc/prepared/issues/560/validate-lifecycle-evidence.sh
- .csdlc/evidence/560/focused-runtime-v2-unified-kernel-coverage.log
- .csdlc/evidence/560/diff-hygiene.log

## Execution

- Added an exact ci-coverage nextest override for the three runtime_v2 unified-kernel tests that timed out at 120s in hosted workspace coverage run 33017588921.
- Set the override ceiling to 240s with terminate-after = 1, leaving the profile-wide ci-coverage timeout at 120s for all unrelated tests.
- Kept Runtime v2 product semantics and test assertions unchanged; the change is instrumentation-aware test/profile scheduling only.
- Added issue-owned focused coverage and lifecycle evidence validators for #560.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and patch hygiene drift.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/560/validate-focused-proof.sh"
    ],
    "purpose": "Run the issue-owned validator that exercises the three exact runtime_v2 unified-kernel correlation/drift tests under cargo llvm-cov nextest ci-coverage.",
    "outcome": "passed",
    "evidence_ref": "focused-runtime-v2-unified-kernel-coverage.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/560/validate-lifecycle-evidence.sh"
    ],
    "purpose": "Verify issue-local lifecycle state and evidence directory exist before implementation finalization.",
    "outcome": "passed",
    "evidence_ref": "lifecycle-evidence-hygiene.log"
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
