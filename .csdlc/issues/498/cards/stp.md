# Structured Task Prompt

Template: 1.0.0

Issue: 498

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one diligence acceptance decision; prerequisite and counsel receipts are inputs to that decision.

## Deliverables

- Exact corporate diligence index
- CORP-A through CORP-C prerequisite census with blocker dispositions
- Counsel-boundary receipt register containing only public or redacted references
- Corporate diligence acceptance record
- Validation evidence for diligence index, prerequisite census, counsel boundary, acceptance readback, redaction, and diff hygiene
- Truthful SRP/SOR records and exact-head pre-publication review

## Acceptance

1. AC-1: Every CORP-A-C blocker has a disposition.
2. AC-2: Counsel-controlled judgments are recorded only as bounded receipts.
3. AC-3: Corporate acceptance binds the exact diligence index.
4. AC-4: CORP-A #482, CORP-B #483, and CORP-C #497 are live merged into main and ancestral to the #498 execution base before acceptance is recorded.
5. AC-5: No private advice, credentials, account identifiers, recovery factors, or secret material is committed.

## Dependencies

- CORP-A #482 live merged and ancestral to #498 execution base
- CORP-B #483 live merged and ancestral to #498 execution base
- CORP-C #497 live merged and ancestral to #498 execution base; current execution must fail closed while #497 remains open or non-ancestral
- Sprint 4 umbrella #532 remains open until both #497 and #498 are merged and ancestral

## Inputs

- AGENTS.md
- GitHub issue #498
- GitHub issue #532
- GitHub issue #497
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#CORP-D
- .csdlc/prepared/issues/498/design.md
- .csdlc/prepared/issues/498/diagram.mmd

## Non Goals

- Replacing counsel
- Inferring legal conclusions
- Repairing or completing CORP-C #497
- Publishing private diligence material
- Mutating provider, billing, credentials, DNS, certificates, CI, Terraform, deployment, or production state
- Closing Sprint 4 umbrella #532
