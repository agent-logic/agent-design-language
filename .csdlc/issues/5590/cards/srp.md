# Structured Review Prompt

Template: 1.0.0

Issue: 5590

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

Review exact #5590 preparation truth across all six typed cards, design, current Runtime v3 diagram, security matrix, preparation-only claim, future disjoint scope, AC-1 through AC-8, S1 through S6, and every positive and negative validation lane. Reject product edits, weak local access, HTTP, hard-coded IPs, fake port discovery, unauthenticated WebSockets, secret leakage, sidecars, custom OTel, unbounded guardian behavior, Runtime v2 coupling, AWS, partial acceptance, or premature readiness.

## Prompts

- Does one init model and one Axum/rustls router truthfully cover local and remote access without hard-coded addresses or HTTP?
- Do HTTP and WebSocket Observatory paths share authentication, origin, authority, frame, redaction, and live-state contracts?
- Does discovery report the actual listener and configured public HTTPS base for default, non-default, and ephemeral ports?
- Does the external guardian distinguish intentional stop, invalid config, bounded retry, pressure serialization, and checkpoint restore without sidecars?
- Does Vector own collection/export while Runtime stderr, health, control, and shutdown survive collector absence?
- Is rollback explicit, reviewed, evidence-preserving, and free of Runtime v2 source edits, automatic cutover, AWS, or deployment claims?
- Do S1 through S6 and all lanes cover AC-1 through AC-8 with no deferred or fixture-only parity credit?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
