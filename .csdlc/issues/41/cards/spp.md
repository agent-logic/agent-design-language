# Structured Planning Prompt

Template: 1.0.0

Issue: 41

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the issue-read-only failure taxonomy and redaction boundary, add a contextual Octocrab classifier, prove the real CLI with deterministic loopback responses, run focused validation, resolve exact-head review, and publish only in the later execution session.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Confirm the issue-read-only failure taxonomy, safe diagnostic vocabulary, exit codes, closed rate-limit allowlist, and redaction invariant.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Extract the existing GitHub GET into fetch_issue_value returning structured octocrab::Error; keep shared read_issue_packet generic, and map only explicit IssueRead errors before the common normalizer.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Extend the existing loopback fixture and prove real CLI JSON, exits, classification, non-read compatibility, successful-read compatibility, and redaction.",
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
  },
  {
    "id": "S4",
    "action": "Run focused validation and exact-head independent review, resolve findings, and publish a qualified closing PR.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Successful issue-read packets remain unchanged
- Only issue-read failures gain contextual classification
- Repository identity and issue number are the only remote context emitted
- Non-404 failures cannot be mislabeled as not-found
- Raw remote errors and credential material never reach output
- All proof is deterministic and loopback-local

## Risks

- GitHub uses 404 for inaccessible private objects as well as absent objects
- A broad 403 matcher could misclassify authorization as rate limiting
- Using Octocrab Display or Debug could leak response content
- CLI-only assertions could miss a library-level compatibility regression

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/41/design.md

Digest: f6ae66c2ba91d72c737b6ec5e9de23063c70460e857c004a6d2ff3f70ae6cca7

## Diagram

.csdlc/prepared/issues/41/diagram.mmd

Digest: 168b37bbb533d68dce62abfc84f9377a5edeb188847a409fd5b8f29df1100f47

## Stop Conditions

- The change would require exposing raw GitHub response content
- Reliable rate-limit classification cannot be derived from structured Octocrab fields
- Scope must widen beyond issue reads and the four declared Rust files
- Successful issue-read packet compatibility cannot be preserved

## Handoff

Proceed only after doctor readiness.
