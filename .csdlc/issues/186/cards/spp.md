# Structured Planning Prompt

Template: 1.0.0

Issue: 186

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Run one portable production-path matrix across macOS, Linux, arm64, x86_64, hosted models, local models, reconnects, restarts, and Observatory reattachment, with exact model and artifact identity.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the supported OS, architecture, provider, model, transport, restart, and reconnect denominator with one exact expected production command and resource budget per cell.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build or obtain exact-revision artifacts for macOS arm64 and Linux x86_64 and verify artifact digests, toolchain provenance, and absence of machine-local path assumptions.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Execute governed hosted-model and local-model work through the production Runtime path on each required platform, binding provider model identity and source revision in producer receipts.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Exercise disconnect, reconnect, voter restart, agent identity continuity, Shepherd continuity, Observatory reattachment, and deterministic replay, comparing terms, indexes, and state digests.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Reproduce each supported cell from tracked commands without hand repair and validate exact matrix coverage, artifact/model digests, receipts, and cleanup.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Obtain independent exact-head portability review and publish the truthful matrix, explicitly retaining unsupported cells and provider limitations as non-passing dispositions.",
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

- Issue DRT-06 owns only its declared repository paths and named external operation/evidence boundary.
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
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/186/design.md

Digest: 0094a845fca2e44aca0c1449c22658c483a6074c18f70e0bd0b3bb3ddda7141f

## Diagram

.csdlc/prepared/issues/186/diagram.mmd

Digest: 953f92d2ec83e0ec04e72b5acb89bf63cf533e23acc60d03e48ec593db89b536

## Stop Conditions

- Two Observatory owners overlap
- A displayed row lacks authority or revision context
- A stale read appears current
- Sensitive data is exposed
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
