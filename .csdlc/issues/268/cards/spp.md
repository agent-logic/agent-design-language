# Structured Planning Prompt

Template: 1.0.0

Issue: 268

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Prepare and review the fixed six-hour and Spot-control design, bind, implement and locally prove it, run one authorized paid attempt, retain cleanup proof, obtain exact-head review, publish, merge, finish, and clean.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Validate dependencies, AWS account/quota/price/residue, and obtain fresh design approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind and implement the fixed six-hour suite plus fail-closed issue-owned wrapper and tests.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run local proof and one authorized six-hour Spot attempt, retaining exact evidence and zero-instance cleanup proof.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Record exact-head review, publish, merge, finish, validate cache/ancestry, and clean the worktree.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- The workload exposure is at least 21,600 monotonic seconds and final-cycle overshoot is measured and capped at 600 seconds
- One run identity maps to at most one instance attempt
- Thresholds cannot be weakened after launch
- Cleanup targets only exact run-tag-owned resources
- Issue #269 remains untouched and non-gating

## Risks

- Spot interruption or capacity loss
- A production fault receipt fails during the long denominator
- Cleanup or independent zero-instance readback fails
- Evidence volume exceeds its bounded retention limit

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/268/design.md

Digest: 51903b24a8f56c962864f8aed232e7a10bb82b0f6a6ae15a0db42ae87a976159

## Diagram

.csdlc/prepared/issues/268/diagram.mmd

Digest: b113de31c6a95217ed837f2aee2efe3386f09af4ee3c62667cc2ade0718f1af7

## Stop Conditions

- Wrong AWS account or insufficient quota
- Resolved estimated cost exceeds USD 20
- Source or builder image is mutable or mismatched
- Existing run-owned resource residue
- A second attempt would be required
- Cleanup cannot prove zero remaining task-owned instances

## Handoff

Proceed only after doctor readiness.
