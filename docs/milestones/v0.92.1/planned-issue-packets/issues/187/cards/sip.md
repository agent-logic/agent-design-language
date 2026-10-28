# Structured Intent Prompt

Template: 1.0.0

Issue: 187

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Complete bounded local and hybrid soak, resource, deterministic replay, cleanup, and final qualification synthesis.

## Required Outcome

two-hour local and four-hour hybrid soak independent replay exact commands terms indexes receipts source and model digests plus cleanup after every failed phase is produced at an exact revision and independently reproducible.

## Scope

- Two-hour local soak, four-hour hybrid soak, workload and fault schedule, CPU/memory/disk/network/cost bounds, exact commands and terms, committed indexes, source/model digests, independent replay, cleanup after success and every failure, and residual-risk synthesis.

## Authority

- Issue DRT-07 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
