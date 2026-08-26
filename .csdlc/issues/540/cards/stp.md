# Structured Task Prompt

Template: 1.0.0

Issue: 540

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Test and, only if necessary, minimally repair additional_allowed_origins handling for http://localhost:8000.

## Deliverables

- Focused CORS/configuration regression tests for http://localhost:8000.
- Any smallest Runtime kernel implementation fix required by those tests.
- Truthful validation and review-ready issue record.

## Acceptance

1. AC-1: Runtime v3 accepts Origin: http://localhost:8000 on relevant Observatory/browser CORS routes when the origin is explicitly configured.
2. AC-2: Runtime v3 rejects or omits CORS allowance for Origin: http://localhost:8000 when the origin is not configured.
3. AC-3: The canonical Observatory origin https://localhost:8765 remains supported.
4. AC-4: No ADL Runtime, Observatory, shell fallback, static server, or test helper binds to port 8000.
5. AC-5: No public API, authentication, signed-command, production ingress, or wildcard CORS behavior changes.

## Dependencies

- none

## Inputs

- agent-logic/agent-design-language#540
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/control.rs

## Non Goals

- Do not move the Observatory from port 8765.
- Do not use port 8000 as an ADL-owned listener.
- Do not add fallback shell behavior or a broad proxy.
- Do not change CORS to wildcard/default-open behavior.
- Do not broaden into production CloudFront/API Gateway/load-balancer work.
