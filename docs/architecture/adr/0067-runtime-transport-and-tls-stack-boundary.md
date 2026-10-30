# ADR 0067: Runtime Transport And TLS Stack Boundary

## Status

Status: **Proposed**

## Context

Runtime API, WSS, and distributed links need one consistent certificate and
peer-verification model without custom TLS protocols.

## Decision

Runtime transport uses the existing Rustls ecosystem for TLS and mTLS, with
explicit trust roots, hostname or identity binding, certificate purpose, and
fail-closed verification. Axum serves HTTP and WSS application routes; Quinn
may serve explicit QUIC transport without becoming an HTTP fallback. Production
certificates come from an operator-managed CA service such as ACM or another
public/private CA, never runtime-generated self-signed credentials.

## Consequences

Protocol routing stays small while certificate issuance remains outside Runtime
business logic.

## Alternatives Considered

Custom TLS, permissive verification, runtime certificate generation, and
implicit QUIC-to-HTTP fallback were rejected.

## Source Evidence

- `adl-runtime-kernel/src/tls.rs`
- `adl-runtime/tests/support/tls.rs`
- `adl-runtime/src/distributed/transport.rs`

## Validation Evidence

- `adl-runtime/tests/distributed_transport.rs`
- `adl-runtime/tests/distributed_certificates.rs`
- `.csdlc/evidence/5800/local-tls-contract.log`

## Supersession Relationships

Refines ADR 0017 and the transport consequences of ADR 0054.

## Non-Claims

Does not claim public deployment, automatic certificate issuance, HTTP/3
fallback, or acceptance of self-signed production certificates.

## Approval Boundary

Human review must separately promote this candidate into `docs/adr/`.
