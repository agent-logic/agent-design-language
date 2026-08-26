# Structured Task Prompt

Template: 1.0.0

Issue: 550

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Exactly the omitted CSM trusted-origin wiring, config-owned HTML Runtime API host trust, the missing CORS behavior on the Runtime health endpoint consumed by the Observatory, executable proof, and bounded live validation.

## Deliverables

- Validated ADL_CSM_OBSERVATORY_PUBLIC_ORIGIN input
- Deterministic combined additional_allowed_origins generation
- Executable valid and invalid CSM configuration tests
- Config-owned HTML Observatory trusted Runtime API host
- Exact configured-origin CORS behavior on /v1/health
- Live browser connection proof from localhost:8000 and the trusted port-8765 Observatory
- Fresh exact-head review and truthful PR

## Acceptance

1. AC-1: The optional public origin is an exact HTTPS DNS host with an optional port in 1..65535.
2. AC-2: Paths, queries, fragments, credentials, wildcards, malformed ports, out-of-range ports, and canonical duplicates fail before config replacement.
3. AC-3: Generated init covers empty, localhost-only, public-only, and combined origin sets deterministically.
4. AC-4: /v1/health returns exact configured-origin CORS headers and rejects unconfigured browser origins consistently with /v1/ready and /v1/observatory.
5. AC-5: The HTML Observatory default Runtime API base and trusted host come from runtime-v3.config.json, accept the configured Wuji HTTPS host, and reject arbitrary hosts.
6. AC-6: TLS key, certificate, bind, continuity, and unrelated CSM authority remain unchanged.
7. AC-7: Executable focused tests, shell syntax, diff hygiene, Runtime CORS/WSS tests, HTML Observatory security tests, and live Wuji browser/TLS/CORS probes pass.
8. AC-8: Fresh exact-head review has zero actionable findings and publication truth closes only issue #550.

## Dependencies

- Merged issue #540 and PR #543
- HOT-01 #510 remains separately owned for dynamic reload

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/550
- CSMctl
- docs/tooling/CSMctl.conf.example
- adl-runtime/tests/runtime_api_wss.rs
- demos/html-observatory/runtime-v3.config.json
- demos/html-observatory/app.js
- .csdlc/prepared/issues/550/design.md

## Non Goals

- Dynamic origin hot reload
- Router, DNS, ACM, Caddy, or certificate issuance changes
- Runtime kernel origin-validation redesign
- Self-signed certificate support
- Rewriting or reopening #540
