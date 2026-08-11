# Structured Intent Prompt

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Give the Guardian-owned distributed Runtime an authenticated bounded bridge to the separate kernel's live continuity coordinator.

## Required Outcome

The production Guardian client can quiesce and export a real signed kernel checkpoint, stage and validate it in an isolated target, resume or discard idempotently, and reconcile crashes without public routes, caller paths, synthetic snapshots, or partial residue.

## Scope

- Loopback-only mutual-TLS kernel continuity listener and validated configuration
- Opaque production Guardian continuity client with distinct control identity
- Real live CheckpointCoordinator quiesce and signed checkpoint bundle export
- Bounded isolated target staging, validation, resume, and discard operations
- Durable replay, result, deadline, cancellation, crash, restart, cleanup, and path reconciliation
- Exact focused proof that the public Runtime and Observatory surfaces expose no continuity operation

## Authority

- Only the configured distinct Guardian client certificate and signed canonical request authorize internal continuity dispatch
- The internal listener is loopback-only and is absent from the public Axum routes and OpenAPI
- Caller paths, bearer tokens, agent keys, voter keys, Shepherd keys, and public control identities never authorize continuity
- The bridge produces kernel effect receipts only; it creates no distributed policy, ownership, activation, serving, or cloud authority
- Normal builds expose only opaque client and bundle/possession handles; deterministic fakes are cfg(test)-only

## Assumptions

- none

## Operator Constraints

- Do not bind or edit product source until #191 / PR #197 is externally reviewed, merged, and ancestral
- Keep #208 limited to the local Guardian-kernel continuity bridge; #204 owns distributed orchestration
- Resolve all review findings through a subagent and obtain fresh exact-head review before publication
- Open a ready PR for visibility but never merge before operator review and authorization
- No public continuity route, no AWS use, and no lifecycle closeout
