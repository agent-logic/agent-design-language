# Issue #550 Design — Trusted Observatory Origin in CSM Launch Config

## Problem

Merged PR #543 proves Runtime support for exact additional origins, but the
follow-up CSM launch configuration that supplies the browser-trusted
Observatory origin was not part of that merge. Wuji therefore needed manual
configuration to combine `http://localhost:8000` with
`https://wuji.dev.csm.agent-logic.ai:8765`.

The live browser failure also exposed two closely coupled startup assumptions:
the HTML Observatory still trusted the stale `runtime.dev.agent-logic.ai`
hostname, while the working Wuji Runtime API is
`https://wuji.dev.csm.agent-logic.ai:20997`; and the Runtime `/v1/health`
endpoint did not carry the same exact-origin CORS behavior as the other
Observatory reads.

## Design

`CSMctl` accepts one optional `ADL_CSM_OBSERVATORY_PUBLIC_ORIGIN`. It must be an
exact HTTPS origin consisting only of a DNS host and optional port. CSMctl
combines it with the independently opted-in localhost:8000 origin and writes one
deterministic `additional_allowed_origins` array into the generated Runtime init
configuration.

The HTML Observatory reads its trusted Runtime v3 API base and trusted host from
`runtime-v3.config.json`. The checked-in Wuji development config points to
`https://wuji.dev.csm.agent-logic.ai:20997` and rejects arbitrary hosts, paths,
queries, credentials, and non-HTTPS values. The Runtime health endpoint uses the
same configured-origin CORS policy as `/v1/ready` and `/v1/observatory`.

Validation fails before generated configuration replacement for non-HTTPS,
wildcard, path, query, credential, malformed-port, out-of-range-port, or
duplicate canonical origins. No TLS key, bind address, continuity, certificate,
or router authority moves into this issue.

The implementation test executes configuration generation for the empty,
localhost-only, public-only, and combined cases plus the invalid-input matrix.
HTML Observatory tests prove the configured Wuji host is accepted while unsafe
or arbitrary hosts are rejected. Runtime control tests prove allowed and
forbidden `/v1/health` origins. Static source assertions remain secondary
evidence only.

## Boundary with HOT-01

Issue #550 supplies correct boot-time CSM input and the static HTML config
authority needed for the current Wuji connection. Sprint 1 HOT-01 issue #510
owns atomic config-file hot reload after boot. #550 neither implements nor
claims dynamic reload. Polis name/domain display is a separate Runtime v3
follow-on and is not silently folded into this emergency connection repair.

## Live proof

The retained local proof checks both Observatory pages and Runtime CORS:

- `http://localhost:8000` returns HTTP 200;
- `https://wuji.dev.csm.agent-logic.ai:8765` returns HTTP 200 with the existing
  Let's Encrypt certificate;
- Runtime health, readiness, and Observatory reads return HTTP 200 with trusted
  TLS; and
- Runtime returns the exact matching CORS origin for both callers.
