# Structured Planning Prompt

Template: 1.0.0

Issue: 627

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Confirm the v2 command denominator, normalize the v3 command manifest, implement the single-binary command shell and fail-closed live-authority stubs, then run focused denominator/help/fail-closed/no-v2-source validation.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Confirm the installed v2 binary denominator and the operator-confirmed 19-route sprint target.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Author or update the machine-readable v3 command manifest with ownership, status, and child-issue routing.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement the one-binary `csdlc` command shell with visible help and fail-closed stubs for not-yet-implemented live-authority routes.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Add focused denominator, help, fail-closed, and no-v2-source-change validation.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Record handoff notes for #628 through #632 and stop before claiming full v3 behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- v2 remains live authority until #505.
- The final v3 replacement target is one `csdlc` binary.
- Missing command behavior is explicit and fail-closed.
- No hidden fallback to v2, raw gh, v1 wrappers, or shell strings.
- No v2 source changes occur in this issue.

## Risks

- A command name can be listed without behavior or fail-closed proof.
- The manifest can drift from help output.
- A later child issue can implement against a different denominator.
- Tests can prove strings instead of command behavior.
- v2 source can be touched accidentally while building parity.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/627/design.md

Digest: dca1208e4d4b73d2e1848dca19978f06b5e2c377fea89576f8e8c0e95d2610dd

## Diagram

.csdlc/prepared/issues/627/diagram.mmd

Digest: e7bf94eabbac6c71577f94162fabf619a55418214985e2c542c1fdfb39b69ed7

## Stop Conditions

- The denominator cannot be reconciled to the installed v2 binary set.
- A route requires live v3 authority before #505.
- A route implementation would require raw gh or v2 fallback.
- The change needs C-SDLC v2 source edits.
- The one-binary model conflicts with an explicit operator decision.

## Handoff

Proceed only after doctor readiness.
