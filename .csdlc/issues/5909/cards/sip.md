# Structured Intent Prompt

Template: 1.0.0

Issue: 5909

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Correct the four security and proof defects merged by PR #107 before dependent distributed authority work advances.

## Required Outcome

Bind mutation authorization to the ledger-owned exact applied index, hard-bound and atomically enforce lineage and serialized state capacity, derive exact negative evidence from executed issue-owned Rust cases, and enforce distinct LeaseGrant and Activate transitions.

## Scope

- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_lease.rs
- .csdlc/evidence/5909

## Authority

- Issue 5909 exclusively owns the two product paths and issue-local proof
- Issue 5878 alone owns final distributed module registration
- No umbrella, manifest, lockfile, or sibling distributed paths

## Assumptions

- none

## Operator Constraints

- PR #107 is terminal and must not be updated
- Publish a new corrective PR against current origin/main
- Fresh independent exact-head review is required
- Do not merge
