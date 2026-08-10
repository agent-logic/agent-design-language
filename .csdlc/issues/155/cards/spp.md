# Structured Planning Prompt

Template: 1.0.0

Issue: 155

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Resolve provenance and use-rights dispositions against CORP-01's frozen denominator, cite authoritative sources, exclude unresolved assets from transfer/release, and route legal trademark judgments to counsel.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Load terminal CORP-01's stable asset denominator and partition it into source, dependency, model, dataset, media, contributor, domain, and brand classes.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Collect machine-readable manifests, repository history, provider terms, licenses, and other authoritative source references for each applicable asset.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Author the provenance matrix with source reference, evidence digest, use-rights disposition, reviewer role, and remediation route for every asset.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Record restricted or unresolved assets as excluded from transfer and release gates, never as implicitly accepted residual risk.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Record trademark/domain conclusions as bounded factual observations and route any legal judgment to counsel with an explicit pending or blocked disposition.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run denominator, authoritative-source, license-manifest, exclusion, and forbidden-legal-claim checks; stop on missing provenance or unverifiable terms.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Complete exact-head review and publish only the redacted matrix, disposition register, source-verification receipts, and boundary runbook.",
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

- Issue CORP-03 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/155/design.md

Digest: b6f9fb3fad2fef6f0b39d591ebf71d809380f7f86074c0651b9f2406ddd37d56

## Diagram

.csdlc/prepared/issues/155/diagram.mmd

Digest: e80e8032d8b59df4e5659aab476d4e076b8401f4e346972e94d4fc9cf2e2ed2d

## Stop Conditions

- A critical asset has no provenance path
- Terms cannot be verified from an authoritative source
- A legal conclusion exceeds the documented authority boundary
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
