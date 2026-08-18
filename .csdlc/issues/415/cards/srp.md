# Structured Review Prompt

Template: 1.0.0

Issue: 415

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/415
.csdlc/prepared/issues/415
.csdlc/evidence/415
adl/tools/run_aws_spot_builder_image_validation.sh
adl/tools/test_run_aws_spot_builder_image_validation.sh
tools/aws_remote_validation/scripts/remote_validation_runner.sh

## Prompts

- Does every required tool/check produce an individually attributable retained diagnostic?
- Can early exit 127 preserve exact redacted output and identify the missing executable?
- Are success compatibility and exact cleanup semantics unchanged?
- Are AWS, #268, and #269 strictly outside execution scope?

## Findings

[
  {
    "id": "415-r1-p2-path-leak",
    "severity": "p2",
    "summary": "Failure summary emits the machine-local retained-log path without redaction.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "415-r1-p2-canned-missing-tool",
    "severity": "p2",
    "summary": "Missing-tool fixture keys only on the label instead of dynamically executing the received probe command.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "415-r1-p2-unreadable-diagnostic",
    "severity": "p2",
    "summary": "Runner compatibility proof covers an absent diagnostic but not a present unreadable diagnostic.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "415-r1-p3-evidence-binding",
    "severity": "p3",
    "summary": "Focused evidence lacks an explicit assertion denominator and exact-revision binding is not visible beside it.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No P0 or P1 findings; fixes require a new exact-head review.

## Review Result

Revision: Some("git-blake3:e9fb5687180cae2803f240022177b95d51217c3b:fc1888f29402591a0f9df30e1004ee284f52df942bc74480f1bd90733e91ee3d")

Reviewer: Some("fresh-session:cdbaa5ed-0414-4785-9db8-e9032b5e68a8")

Result: changes_required
