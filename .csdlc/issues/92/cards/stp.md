# Structured Task Prompt

Template: 1.0.0

Issue: 92

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Standardize Runtime TLS/mTLS and remove the obsolete local PKI subsystem without replacing Quinn or redesigning Guardian authorization.

## Deliverables

- Shared Runtime Rustls identity/trust policy and loader
- Unified Axum HTTP/WSS listener TLS construction
- Quinn reuse of shared trust policy
- Removed self-signed bootstrap and host trust commands
- Corrected Runtime init, OpenAPI, operational proof, browser, and Unity surfaces
- Focused positive and negative certificate-chain and mTLS tests

## Acceptance

1. AC-1: All Runtime HTTP/HTTPS/WSS listeners use one shared Axum/Rustls construction path
2. AC-2: Quinn Guardian transport reuses shared trust-policy types while retaining TLS 1.3 mTLS
3. AC-3: Runtime product code cannot generate self-signed certificates or mutate host trust stores
4. AC-4: Public TLS fails closed on self-signed leaf, wrong SAN, invalid validity, unsuitable usage, unknown CA, or incomplete chain
5. AC-5: mTLS uses standard Rustls/WebPKI verification and exposes peer identity only after successful handshake
6. AC-6: Runtime and kernel use one reviewed Rustls patch level and crypto-provider policy
7. AC-7: OpenAPI and documentation truthfully distinguish public server TLS, signed command authorization, private mTLS, and application authority
8. AC-8: Operational, browser, and Unity proof surfaces contain no leaf-as-root trust or verification bypass
9. AC-9: Focused tests use test-CA-issued leaves and cover positive and required negative cases
10. AC-10: Live Unity validation remains gated until an ordinary certificate-valid Axum WSS endpoint succeeds

## Dependencies

- Current canonical Agent Logic main
- Maintained Axum/axum-server, Rustls, Quinn, and WebPKI APIs
- Externally provisioned CA-issued certificate material for live consumer proof

## Inputs

- AGENTS.md
- agent-logic/agent-design-language#92
- Runtime TLS/mTLS implementation and proof review dated 2026-08-09
- Vector TLS settings/incoming/reload architecture
- Tokio-Rustls server example
- Rustls platform verifier guidance

## Non Goals

- Replacing Quinn with HTTP/2
- Adding automatic QUIC fallback
- Implementing ACME, a CA, a trust store, or cryptographic verification algorithms inside ADL
- Requiring browser or Unity client certificates
- Replacing application Guardian authorization with transport TLS
