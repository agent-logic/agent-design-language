# Structured Output Record

Template: 1.0.0

Issue: 415

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Retain exact redacted builder preflight diagnostics on early failure, identify the failed check and executable, and preserve remote summary and cleanup authority without launching AWS resources.

## Artifacts

- adl/tools/run_aws_spot_builder_image_validation.sh
- adl/tools/test_run_aws_spot_builder_image_validation.sh
- tools/aws_remote_validation/scripts/remote_validation_runner.sh
- .csdlc/issues/415
- .csdlc/prepared/issues/415

## Execution

- Run each required builder toolchain probe as an individually labeled check with separately captured stdout and stderr.
- Redact bounded raw captures before atomically publishing builder-toolchain.log and remove every raw or partial capture on exit.
- Emit the retained diagnostic best-effort from the remote runner's normal captured-command path while preserving summary and cleanup processing.
- Add deterministic missing-executable, redaction, raw-removal, validation-non-entry, runner-summary, shell, and exact-scope regressions.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_builder_image_validation.sh"
    ],
    "purpose": "Run the focused issue #415 builder diagnostics regression harness.",
    "outcome": "passed",
    "evidence_ref": "builder-diagnostics-focused.log"
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
