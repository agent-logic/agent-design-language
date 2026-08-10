# Structured Intent Prompt

Template: 1.0.0

Issue: 184

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prove continuity, fencing, healing, and halt behavior across Wuji and two private AWS availability zones.

## Required Outcome

two private AZs authenticated transport independently materialized snapshots Wuji isolation AWS-only quorum continuity fencing healing halt and per-phase cleanup is produced at an exact revision and independently reproducible.

## Scope

- One Wuji voter, two private AWS voters in separate AZs, authenticated private transport, independent snapshots, isolation, AWS-only quorum, asymmetric partition, healing, stale-owner fencing, and cleanup.

## Authority

- Issue DRT-04 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
