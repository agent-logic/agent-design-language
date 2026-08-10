# Structured Intent Prompt

Template: 1.0.0

Issue: 186

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Validate that the single Observatory presents a coherent, causal, authority-aware, redacted distributed evidence surface.

## Required Outcome

coherent authority cut causal trace redaction stale-read denial and singleton ownership is produced at an exact revision and independently reproducible.

## Scope

- Quorum-leased singleton ownership, coherent authority cuts, node and agent correlation, causation, terms, commit indexes, state revisions, stale-read denial, redaction, partition and recovery visibility.

## Authority

- Issue DRT-06 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
