# v0.93.1 Decisions

## Metadata

Planning template family 1.1.0; authoring issue #922. Scope split approved; Sprint 1 preparation opened by the operator on 2026-09-25. Split execution and release are not authorized. Named execution owners and resource limits remain to be assigned.

## Purpose

Preserve the approved allocation and pending implementation decisions.

## How To Use

Scope approval is separate from opening, deployment and release authority.

## Decision Log

| Decision | Disposition |
|---|---|
| Split original v0.93 | Approved: launch in v0.93.1; remaining platform work in v0.93.2 |
| Repository split | First, before feature work; retain RD-11 acceptance |
| All templates and foundations | Approved for v0.93.1, CT-01–CT-10 |
| Runtime v4 | v0.93.2; remove blanket CF-02 → RV-08 edge |
| Launch Runtime | Pin and qualify the existing compatible Runtime; fix only demonstrated launch blockers |
| #915 residuals | #1148/#1149/#1150 remain required before Beta 1 launch; historical failure is not PASS |

## Open Questions

CF-01 resolves audience, environment, capacity, budget, privacy/data classes and support owner. RD-02 resolves repository names, ownership, licenses and access. Named task owners and sprint capacity are not yet assigned.

## Exit Criteria

Before opening, every unresolved decision has an accountable owner and every launch-critical choice is resolved before its dependent execution.
