# Structured Task Prompt

Template: 1.0.0

Issue: 506

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly acceptance of one distributed-qualification contract; conformance checks are evidence inputs.

## Deliverables

- adl-runtime/tests/distributed_contract/validate_drt_a.sh
- docs/milestones/v0.92.1/evidence/runtime/drt-a/qualification-contract.json

## Acceptance

1. AC-1: Requirements 181 and 182 are mapped to the DRT-A contract denominator.
2. AC-2: Identity and authority behavior are deterministic and non-synthetic.
3. AC-3: Duplicate denial and replay receipts are exact and cannot change authority.
4. AC-4: Negative scenarios fail closed for stale, duplicate, reordered, malformed, unsigned, wrong-domain, cross-Polis, and authority-mutation inputs.

## Dependencies

- #181
- #182

## Inputs

- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml
- docs/milestones/v0.92.1/planned-issue-packets/prepared/issues/181/design.md
- docs/milestones/v0.92.1/planned-issue-packets/prepared/issues/182/design.md
- adl-runtime/src/acip.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_lease.rs

## Non Goals

- Paid AWS execution
- Observatory redesign
- DRT-B six-resident qualification
- DRT-C final distributed Runtime qualification
- DRT-D GCP portability qualification
- provider credential proof
- public cloud exposure
- paid GCP execution
