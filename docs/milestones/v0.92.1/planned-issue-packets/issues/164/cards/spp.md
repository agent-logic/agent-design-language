# Structured Planning Prompt

Template: 1.0.0

Issue: 164

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Construct the production single-binary shell and mechanically bind its parser, generated help, output envelopes, in-process filters/templates, dependency policy, completions, installation identity, and provenance.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Promote only the approved V3-02 slice and freeze the package/lib/binary identities plus dependency-policy inputs.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the complete Clap command graph and parser-only direct-flag/typed-input conflict rules without repository or adapter initialization.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement human/JSON output separation, schema discriminants, typed diagnostics, and the approved in-process jq/template engines.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Generate help, docs, completions, installer checks, and immutable release provenance from the same command metadata.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run parser, golden help, jq compatibility/rejection, output-channel, cargo-deny, install, and provenance tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Retain exact artifacts and stop on hidden global state, help drift, extra binary targets, or unapproved dependencies.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Remove install and completion scratch output, verify exactly one operational executable remains, and leave no parser test process or generated untracked artifact.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-03 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/164/design.md

Digest: 5fc0db35c4b95fa5a443296460493cd0bd9a96080188dafb0109fb1f1bcc3f51

## Diagram

.csdlc/prepared/issues/164/diagram.mmd

Digest: 50b3388cc5ef3552cb6d8ca29d566941914c1b280f2dbe39e095761cb12297d1

## Stop Conditions

- A command requires hidden global state, generated docs diverge from Clap, or more than one operational binary becomes necessary.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
