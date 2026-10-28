# v0.93.1 ADR Plan

## Metadata

Planning template family 1.1.0; authoring issue #922. Scope split approved; Sprint 1 preparation opened by the operator on 2026-09-25. Split execution and release are not authorized. Named execution owners and resource limits remain to be assigned.

## Status

ADR candidates only; this document accepts no architecture decision.

## Candidates

| Decision surface | Owner/result | Evidence needed |
|---|---|---|
| Public/private product boundaries | RD-02 | Source ownership, license, contracts and rollback |
| Existing Runtime launch compatibility | CF-01/CF-02 | Versioned interface inventory and installed consumer proof |
| Immutable templates and evidence identity | CT-01/CT-02/CT-04 | Field schemas, pinning, upgrade and rollback behavior |
| Unknown-request recovery | #1149/CF-04 | Durable identity, reconciliation and budget reservation behavior |
| Launch data and access boundary | CF-01/CF-04 | Audience, authorized source/data classes, deletion and privacy proof |

## Review rule

Reuse existing accepted ADRs where applicable; create a new decision only when architecture changes. Link rejected alternatives and exact evidence without exposing private payloads.

## Exit Criteria

Every material changed boundary has a reviewed decision; no ADR substitutes for behavior proof.
