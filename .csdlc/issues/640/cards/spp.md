# Structured Planning Prompt

Template: 1.0.0

Issue: 640

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add provider-neutral configuration for a non-empty unique Shepherd set, replace native placeholder reasoning with the governed provider executor, implement preload and isolated lifetime recovery, derive all readiness projections from one snapshot, and prove it with focused tests plus bounded Wuji acceptance.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Extend and validate the non-empty uniquely named resident Shepherd set and thread authoritative identity/provider/model/preload values into Runtime bootstrap.",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Wire each Shepherd reasoning path through its configured provider and the existing governed operation boundary.",
    "acceptance_ids": [
      "AC-2",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement startup preload, truthful per-Shepherd readiness states, and idempotent lifetime recovery using generous bounded probes and backoff.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Derive /v1/ready, blocking_reasons, roster/detail, and Observatory snapshot/feed from one non-secret Shepherd health snapshot and update OpenAPI.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run focused deterministic validation and bounded Wuji restart/preload/inference acceptance, then resolve independent exact-head review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- At least one resident Shepherd exists and each configured canonical identity is unique
- Runtime configuration is authoritative for Shepherd identity and provider selection
- No provider credential is serialized or logged
- One health snapshot keeps Runtime readiness, blocking reasons, roster/detail, and Observatory projections consistent
- Transient inference failure is isolated from global Runtime and non-Shepherd availability
- Lifetime recovery uses bounded configurable probes and backoff; no short timeout terminates the Runtime
- Canonical name and lifecycle continuity delivered by #617 remain unchanged

## Risks

- Provider-specific preload behavior leaks into the Runtime contract
- A cold model exceeds a small startup or inference budget and creates false failure
- Recovery duplicates a configured Shepherd identity or provider task
- API health claims ready before the model can infer
- Provider failure globally blocks Runtime readiness or makes /v1/ready disagree with Observatory
- Execution accidentally stacks or duplicates unmerged #617 changes

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/640/design.md

Digest: f73bbe055c9d064099191075b3e9c1e38eb3dee8693df8ed48569325290f1959

## Diagram

.csdlc/prepared/issues/640/diagram.mmd

Digest: 81aeb9544c88940ec0fcef0ed2963e68ee37fd481c16f1199bd28c5f6ef0efaa

## Stop Conditions

- Execution base does not contain merged #617/#636 state
- Provider credentials would need to enter tracked config or API output
- The design requires terminating or globally blocking the Runtime on temporary provider/model failure
- A validation lane selects zero tests
- Scope expands into general dynamic-agent lifecycle redesign
- Wuji acceptance would overwrite another session's live configuration without coordination

## Handoff

Proceed only after doctor readiness.
