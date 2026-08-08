# Structured Planning Prompt

Template: 1.0.0

Issue: 47

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Classify Rust test argv at the typed VPP boundary, reject ambiguous named substrings, retain intentional broad commands, prove exact nonzero schema selection excludes unrelated integration targets, and update affected active guidance.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Trace VPP lane parsing and define typed exact, broad, and invalid Rust selector postures with fail-closed diagnostics.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement selector validation without changing Cargo or ordinary test behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused fixtures proving nonzero exact schema selection, integration exclusion, broad compatibility, and invalid-shape rejection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Update directly affected active skills/runbooks, run typed/focused validation, and obtain exact-head review.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- A named lane cannot escape its declared Cargo target boundary
- Accepted exact lanes execute a nonzero intended test set
- Intentional broad commands remain truthful and supported
- Invalid selectors fail before expensive or unrelated execution
- Unrelated test semantics remain unchanged

## Risks

- Over-validation incorrectly rejects legitimate broad Cargo commands
- A syntactically exact selector still runs zero tests and creates false proof
- Command parsing diverges from Cargo target syntax
- Guidance retains ambiguous substring examples

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/47/design.md

Digest: 5423a1c6db9a58f90b384cf70126f355b60312af6a523c6214e39dfd26577759

## Diagram

.csdlc/prepared/issues/47/diagram.mmd

Digest: 719049193cfdf0c2d97bbb8cede151e9650f1493c26025791f88694375941dca

## Stop Conditions

- The solution requires modifying or skipping unrelated tests
- Intentional broad validation can no longer be represented truthfully
- Selector intent cannot be determined from typed argv without a new explicit field
- Scope expands into CI scheduling or general command execution

## Handoff

Proceed only after doctor readiness.
