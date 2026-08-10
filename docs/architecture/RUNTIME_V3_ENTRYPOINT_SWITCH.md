# Runtime v3 Entrypoint Switch

## Status

As of v0.91.7, Runtime v3 is selectable through an explicit CLI compatibility
boundary. Runtime v2 remains the default runtime until the cutover proof gate
authorizes a default switch.

## Selection Report Surface

- `adl runtime-v3 select [--runtime v2|v3] [--json]`
- `adl-runtime runtime-v3 select [--runtime v2|v3] [--json]`

These compatibility entrypoints report the requested runtime and the Runtime v3
control API policy. They are not runtime launchers: they invoke neither Runtime
v2 nor Runtime v3 and do not change global defaults. Runtime v3 execution uses
the independent `adl-runtime-kernel serve` command below.

The selector reports `DEFAULT_CHANGED=false` for both Runtime v2 and Runtime v3
selection. `SELECTION_DIFFERS_FROM_DEFAULT=true` is the reversible selection
signal for explicit Runtime v3 use while Runtime v2 remains the default.

## Selection Rules

- No selector: Runtime v2 remains selected.
- `--runtime v3`: the report records an explicit Runtime v3 request.
- `--runtime v2`: the report records an explicit Runtime v2 request.
- `ADL_RUNTIME_SELECTION=v3`: Runtime v3 is selected when `--runtime` is
  omitted.
- Unknown values fail closed.

`--runtime` takes precedence over `ADL_RUNTIME_SELECTION`.

## Runtime v3 Control Policy

Runtime v3 uses an HTTPS control API endpoint on a real DNS name:

```text
https://runtime.dev.agent-logic.ai:20997
```

The independent Runtime v3 kernel launch command reported by this surface is:

```text
adl-runtime-kernel serve --init infra/runtime-v3/runtime-init.toml
```

The kernel terminates TLS through the maintained `axum-server` Rustls adapter.
The init file supplies the certificate-chain and private-key PEM paths; no
private key is checked into the repository. The ready event and Observatory
feed report the port actually bound by the listener rather than assuming
`20997`.

Runtime v3 has one production Axum HTTP/HTTPS/WSS stack in
`adl-runtime-kernel`. The canonical ACIP WebSocket endpoint is owned there and
is not duplicated as a second independently served ACIP OpenAPI authority.

## TLS Certificate Boundary

Runtime v3 does not issue certificates, create a local certificate authority,
install trust anchors, or support certificate-verification bypasses. The Axum
listener receives an externally issued certificate chain, matching private
key, trust-root bundle, and exact DNS identity through the `[api.tls]` fields in
the operator-local init file. The shared Rustls loader validates that identity
before the listener binds.

The certificate must be valid for the exact DNS name in `public_base_url` and
must chain to a root already trusted by ordinary browser, operating-system, and
Unity TLS clients. Production certificates must not be self-signed. AWS
deployments may use an ACM exportable public certificate when Axum terminates
TLS directly, or an ordinary ACM certificate when an AWS-managed ingress
terminates TLS. Direct and local deployments may use an equivalent externally
managed public certificate. Export, deployment, renewal, and private-key
custody remain infrastructure responsibilities outside the Runtime process.

Development uses the same contract as deployment: a real DNS name, external CA
material, normal hostname verification, and no host trust-store mutation. Split
DNS or an explicit test-host mapping may route that DNS name to loopback, but it
must not weaken certificate verification or replace the platform trust store.

Guardian-to-Guardian transport remains Quinn QUIC with Rustls TLS 1.3 and its
separate private mTLS trust domain. Axum remains the single HTTP/HTTPS/WSS stack.
ADL authority certificates are application authorization records and are not
substitutes for X.509 transport identity.

## Non-Covered Surfaces

This issue does not:

- make Runtime v3 the default;
- delete or decommission Runtime v2;
- migrate every Runtime v2 demo command;
- introduce a custom supervisor;
- start the Runtime v3 daemon implicitly.

Runtime v2 decommission remains gated by the aggregate Runtime v3 cutover proof
and an explicit default-switch decision.

## v0.91.7 Decision

#5254 records the final v0.91.7 default-switch decision: Runtime v2 remains the
default runtime, Runtime v3 remains explicit opt-in only, and Runtime v2
decommission is not authorized. See
`docs/architecture/RUNTIME_V3_CUTOVER_DECISION_5254.md`.
