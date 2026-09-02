# Structured Planning Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After confirming #514 terminal truth, add a local-only non-authoritative provider shadow path with deterministic comparison inputs, fail-closed fallback semantics, and redacted evidence.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Verify PROV-A/#514 terminal dependency truth and preserve the explicit non-authoritative authority boundary before binding execution.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the local-model shadow path so authority and shadow execution are represented by distinct types, state, and result channels.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Define exact deterministic comparison inputs and rules for authority-versus-shadow observations.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Prove shadow failures preserve authoritative results and retain only redacted evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Shadow output cannot acquire authority
- Authoritative results remain the returned source of truth
- Comparison inputs and rules are exact and deterministic
- Shadow failures do not alter authoritative result, lifecycle state, provider profile state, or production routing
- Evidence excludes credentials, private payloads, prompts, and unredacted local paths

## Risks

- Shadow-result data could accidentally flow into authority result handling
- Comparison fixtures could drift from declared inputs
- Fallback could mask authoritative failures or overwrite authoritative success
- Evidence could leak provider prompts, payloads, credentials, or local machine paths

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/515/design.md

Digest: e6bce2ed1224a495cf01affe51a1bd651979a7981d5b0da50700f8a93191f970

## Diagram

.csdlc/prepared/issues/515/diagram.mmd

Digest: 1639b745f2068d0919b5b82714992d451b50eb5d1d6039fa42188a09c14852a3

## Stop Conditions

- Shadow output can mutate authority
- Comparison inputs drift
- A live paid provider or cloud call is required without explicit authorization
- Evidence cannot be redacted

## Handoff

Proceed only after doctor readiness.
