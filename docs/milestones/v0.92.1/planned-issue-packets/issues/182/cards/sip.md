# Structured Intent Prompt

Template: 1.0.0

Issue: 182

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prove deterministic ACIP identity, authority, ordering, causation, duplicate, denial, and replay conformance before live distribution.

## Required Outcome

canonical vectors ordering causation denial duplicate and replay digest is produced at an exact revision and independently reproducible.

## Scope

- Canonical envelopes and encodings, identity and authority bindings, sequence and term rules, duplicate and replay behavior, negative vectors, deterministic receipts, and cross-polis denial.

## Authority

- Issue DRT-02 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
