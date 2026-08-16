# Structured Task Prompt

Template: 1.0.0

Issue: 358

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Minimal #350/#356 extension only; no #274 state machine.

## Deliverables

- adl-runtime/src/distributed/authority_protocol.rs
- adl-runtime/src/distributed/serving_authority.rs
- adl-runtime/tests/distributed_observatory_authority_projection.rs
- .csdlc/issues/358
- .csdlc/evidence/358

## Acceptance

1. AC-1: Canonical artifact authenticates acquire/renew/transfer/revoke and exact predecessor shape.
2. AC-2: Invalid/unknown/missing/unexpected/self predecessor intent fails before projection creation.
3. AC-3: Projection exposes read-only action/predecessor plus deadline/finalization seconds,nanos,uncertainty.
4. AC-4: Exact tuple boundaries and durable restart are proven without caller time authority.
5. AC-5: Private construction and redaction remain intact.
6. AC-6: Focused tests, Clippy, review, CI, finish and ancestry pass before #274.

## Dependencies

- #350 terminal and ancestral
- #356 terminal and ancestral
- Blocks #274

## Inputs

- issue #358
- terminal #350 authority_protocol.rs and serving_authority.rs
- terminal #356 accessors and focused test

## Non Goals

- #274 state machine
- #273/#272/#203/#205/#275 changes
- UI/listener/transport/cloud/provider
