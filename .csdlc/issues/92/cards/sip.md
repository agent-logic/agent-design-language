# Structured Intent Prompt

Template: 1.0.0

Issue: 92

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make Runtime transport security conventional and invisible: one Axum HTTP/WSS stack, Quinn only for Guardian QUIC, and one shared Rustls policy with externally provisioned CA-issued identities.

## Required Outcome

Remove Runtime certificate issuance and host trust mutation, unify server TLS configuration, standardize private mTLS, and make live browser and Unity consumers use ordinary certificate-valid endpoints without bypasses.

## Scope

- Runtime and kernel TLS configuration and loading
- Axum HTTP/HTTPS/WSS listener construction
- Quinn Guardian trust-policy integration
- Local TLS bootstrap removal
- Runtime TLS proofs, API contracts, and consumer documentation

## Authority

- Axum remains the only Runtime HTTP/WSS server stack
- Quinn remains the Guardian QUIC transport and does not gain an HTTP/2 fallback
- Runtime consumes externally provisioned PEM identity material and never acts as a CA or trust-store manager
- Application Guardian authority credentials remain separate from X.509 transport identity
- Issue 84 remains gated until certificate-valid live Unity proof

## Assumptions

- none

## Operator Constraints

- No self-signed Runtime served leaf certificates
- No certificate-verification bypass or leaf-as-root trust
- No Runtime certificate issuance or host trust-store mutation
- Use maintained Axum, Rustls, Quinn, and WebPKI/library APIs rather than custom TLS code
- Use ACM or another external CA where deployment requires certificate issuance
- Never implement tracked changes on main
- Do not merge without explicit operator authorization
