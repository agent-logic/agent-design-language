# Structured Planning Prompt

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Recover the retained runner into issue-owned paths, reconcile it with the current Shepherd contract and business-account controls, add executable failure/cleanup/redaction tests, prove a read-only preflight, obtain exact-head review, and leave paid execution gated on separate exact-run authorization.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind exact historical runner/evidence inputs and current Shepherd/AWS authority boundaries without importing legacy lifecycle state.",
    "acceptance_ids": [
      "AC-1",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Recover, rename, and simplify the runner under issue-#345 ownership while preserving explicit preflight, run, cleanup, owner lock, deadlines, and no-fallback semantics.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add executable deterministic positive and negative runner tests plus evidence redaction and cleanup contracts without AWS mutation.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run the read-only business-account preflight and reconcile any operator-provisioned resource drift without creating or broadening resources.",
    "acceptance_ids": [
      "AC-2",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Obtain exact-head implementation review and prepare a truthful publication handoff with the paid lane still separately gated.",
    "acceptance_ids": [
      "AC-6",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S6",
    "action": "Only after explicit exact-run operator authorization, run one bounded On-Demand GPU proof, retain redacted proof/cost/cleanup evidence, and re-review the final evidence head.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-9"
    ],
    "status": "completed"
  }
]

## Invariants

- The approved Agent Logic business account is verified before mutation.
- No IAM or security-group creation/broadening and no public ingress occurs.
- No paid launch occurs without exact-run authorization, revision, budget, run ID, and deadline.
- At most one On-Demand instance is attempted and there is no fallback or retry.
- Immutable artifacts and current source are digest-bound before model execution.
- All temporary compute is owner-tagged, deadline-bound, and cleaned on every path.
- Public evidence is redacted and cannot expose credentials, identifiers, prompts, responses, private paths, or environment dumps.
- AWS remains optional and does not change Runtime/local Shepherd availability.

## Risks

- Recovered shell logic may encode stale account, AMI, IAM, artifact, quota, or pricing assumptions.
- A launch/lock ordering gap could leave unowned paid compute.
- Cleanup could target another run or fail after the operator session exits.
- S3 or source drift could produce a model result that is not bound to the reviewed revision.
- Raw AWS responses or model data could leak into retained evidence.
- A long bootstrap could exceed the declared budget before proof begins.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/345/design.md

Digest: 18204494877650a5a4e6c5cefce8aae663dae778e9ab250b0e7d00ed75e97d82

## Diagram

.csdlc/prepared/issues/345/diagram.mmd

Digest: a05bc537227bc698c64e1e2adcdd95b26ef9c903e95a7a9e1d5ba427f1ee4037

## Stop Conditions

- The configured AWS profile does not resolve to the approved business account.
- The instance profile, policies, security group, immutable artifact manifest, quota, price, or permanent deadline reaper is missing or ambiguous.
- A required change would create or broaden IAM/security-group authority.
- Exact adapter/source/artifact identity cannot be proven.
- A paid launch lacks explicit exact-run operator authorization or exceeds its cost/deadline ceiling.
- Cleanup ownership is ambiguous or any temporary compute remains.
- Evidence cannot be redacted without losing the proving denominator.
- Fresh review reports an unresolved actionable finding.

## Handoff

Proceed only after doctor readiness.
