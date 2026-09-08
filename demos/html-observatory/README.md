# ADL HTML Observatory

This demo is the v0.91.7 HTML Observatory integrated proof for #4690.

The Observatory is a read-only control room over a live Runtime v3 polis. It
consumes the runtime-owned `/v1/observatory` feed over an authenticated
WebSocket plus `/v1/ready`, and falls back to retained evidence when the live
feed is unavailable. The browser holds no mutation authority: reads are public,
writes require operator login, and runtime mutation remains signed-command-only
through `/v1/control`.

## Surfaces

A persistent left rail selects one surface at a time. Each is a distinct
information contract rather than a view of the same data.

- **Overview** — runtime readiness, agent count, event total and host CPU as
  live stat cards; the polis topology graph; the live event stream; and the
  Inspector (see below).
- **Chat** — the Layer 8 channel. The conversation leads the surface; operator
  access, the multi-agent room, the attention inbox and signed control collapse
  into disclosures. Login state is shown explicitly, since it is the difference
  between a read-only view and being able to send.
- **Agents** — a directory of who is in the polis. One card per agent carrying
  identity, health, availability, freshness against the runtime's own window,
  admission path (dynamically admitted versus resident component), declared
  capabilities, and Layer 8 reachability.
- **Modules** — every runtime subsystem reporting its lifecycle state, read
  from `health.snapshot.components`.
- **Events** — the ordered runtime event log. Repeated heartbeats collapse into
  a single counted row so subsystem transitions stay visible.
- **Infrastructure** — the public endpoint (domain from the feed, external IP
  resolved live over DNS-over-HTTPS), retained AWS Systems Manager evidence
  labelled with its capture age, and the CloudWatch heartbeat.
- **Logs** — the durable JSONL runtime log, filterable by text, level or
  component. A secondary in-content navigation reaches the retained operator
  report at `#evidence`.

### Inspector

The Overview sidebar carries three tabs, all live:

- **Integrity** — continuity checkpoint (generation, accepted-through,
  integrity and topology digests), trusted time authority, resource weather
  measured against its staleness budget, queue backpressure, governance posture
  and degradation counters.
- **Agent** — the detail half of a master/detail pair with the Agents surface.
- **Activity** — agent admissions and departures, lifecycle transitions, and
  conversation turns derived by diffing successive snapshots.

## Data sources and honesty rules

The feed schema is pinned to `adl.runtime_v3.observatory_feed.v3`, the only
version carrying `polis_identity`. Polis identity is projected from
runtime-published values and validated; it is never derived from the endpoint,
the URL or the browser location. If the runtime does not publish it, the
Observatory shows `Unavailable` rather than substituting a connection label.

A feed schema this client cannot read is reported as an explicit error rather
than dropped, because a silently discarded frame is indistinguishable from an
outage.

When the live feed drops, a banner states that the values on screen are the
last received snapshot. Retained evidence is labelled as retained, with its
capture age, so it cannot be mistaken for live data.

The operator write token is held in memory for the page lifetime only and is
never written to any browser storage (see issue #679).

- `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/visibility_packet.json`
- `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/operator_report.md`
- `docs/milestones/v0.91.7/review/runtime/csm_service_4903/service/service_manifest.json`
- `docs/milestones/v0.91.7/review/runtime/CSM_RUNTIME_API_4929.md`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/status.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/health.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/ready.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/metrics.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/events.json`
- `docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/aws/cloudwatch_recent_events.redacted.json`
- `docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/acip_sns_summary.json`
- `docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/sns_resource_summary.json`

Runtime v3 endpoints and the pinned feed schema are read from
`demos/html-observatory/runtime-v3.config.json`. Query parameters such as
`?runtime=v3&runtimeApiBase=<runtime-api-base>&live=1` remain a troubleshooting
override. The runtime API base must match the configured Runtime v3 control API
host, port, TLS posture, and Observatory allowed-origin policy.

The retained AWS lanes cover #4684 through #4688, keeping closed heartbeat,
ACIP-SNS and SSM proof distinct from open full-bridge and S3 archive work. Live
SNS/SQS mutation remains runtime- and tool-owned; the browser never performs it.

### Serve from the repository root

Retained evidence is referenced as `../../../docs/...`, so the server root must
be the repository root, not the demo folder:

```bash
python3 -m http.server 8000
# then open /demos/html-observatory/?runtime=v3&runtimeApiBase=<base>&live=1
```

Serving the demo folder directly returns 404 for every retained artifact, which
degrades Infrastructure, the published mirror and the operator report.

The Logs surface is a bounded projection of the selected polis's public
Observatory event feed. The browser never fetches or publishes the Runtime's
durable master log; that operator-only artifact remains outside the static site.

## Run

Runtime v3 and the Observatory use real DNS names and externally issued
certificate material. The canonical example endpoints are:

```text
https://runtime.dev.agent-logic.ai:20997
https://observatory.dev.agent-logic.ai
```

The Runtime process does not generate certificates, install trust anchors, or
modify host trust stores. Configure the Axum/Rustls listener with an external
CA-issued full chain and matching private key through `[api.tls]`. AWS may use
an ACM exportable public certificate for direct Axum termination, or terminate
an ordinary ACM certificate at an AWS-managed ingress. Other environments may
use an equivalent externally managed public certificate.

The certificate SAN must contain the exact endpoint DNS name. Browser, curl,
Node, and Unity clients must validate it through their ordinary platform trust
path. There is no HTTP fallback, leaf-as-root trust, certificate-warning bypass,
or repository-managed trust installation. Split DNS or a test-host mapping may
route the real DNS name to loopback without changing that trust contract.

### Runtime and Observatory

Configure Runtime `[api.tls]` with the externally provisioned full-chain,
private-key, and trust-root paths plus the exact certificate DNS name in
`server_name`. Keep `[observatory].allowed_origins` set to the exact HTTPS
Observatory origin. Start the
Runtime v3 kernel with its operator-local init file and token:

```sh
ADL_RUNTIME_OBSERVATORY_TOKEN="<operator-local-token>" \
  adl-runtime-kernel serve --init <absolute-operator-runtime-init>
```

Serve the Observatory through an HTTPS endpoint with its own valid external
certificate, then open:

```text
https://observatory.dev.agent-logic.ai/demos/html-observatory/
```

The default init file binds the Runtime v3 listener locally while advertising
`runtime.dev.agent-logic.ai`. Runtime v3 browser/API access is HTTPS-only.
Before launch, provision the externally issued full chain and private key at
the `[api.tls]` paths in the init file; the repository does not retain private
keys. Set a 32-to-256-character
operator-local write token for the runtime process in
`ADL_RUNTIME_OBSERVATORY_TOKEN`. Runtime v3 health, readiness, metrics,
Observatory snapshots, and the Observatory WSS feed are public read surfaces
and require no token.

Runtime v3 uses versioned operator probes:

```text
GET https://runtime.dev.agent-logic.ai:20997/v1/health
GET https://runtime.dev.agent-logic.ai:20997/v1/ready
GET https://runtime.dev.agent-logic.ai:20997/v1/metrics
GET https://runtime.dev.agent-logic.ai:20997/v1/observatory
```

The HTML Observatory reads its Runtime v3 browser API base and endpoints from
`demos/html-observatory/runtime-v3.config.json`. Keep that file on the same
static host as `index.html`; if it cannot be loaded, the browser falls back to
the versioned defaults listed above.

Do not use unversioned `/health` or `/ready` paths for Runtime v3 overnight
monitoring. `/v1/ready` is the watcher-ready signal: it returns `200` with
`ready: true` only when the runtime is observability-ready and weather
freshness is not stale; it returns `503` with `degraded_reasons` such as
`weather_stale` when the runtime is reachable but should not be reported as
fully ready.

Operator login is required only before the browser sends WSS-authenticated ACIP
work. Signed Runtime v3 control envelopes can be submitted through `/v1/control`
without putting a bearer token in the browser URL; Runtime v3 still verifies the
signature, principal, capability, runtime identity, and command policy before
execution.

The Observatory write token is intentionally tab-local and memory-only. Enter
it through the Operator Channel login control after the page loads. The browser
does not write the token to `localStorage`, `sessionStorage`, URLs, checked-in
configuration, exported proof, or retained evidence. Reloading the page,
closing the tab, or logging out clears write authority; reconnecting as an
operator requires entering the token again.

The token elevates only the active WSS connection for writes; `/v1/control`
remains a signed-command endpoint and signature verification plus canonical
ingress policy still apply. Transport security is ordinary server TLS, not
listener-side mTLS. The kernel terminates its Axum/Rustls connection directly
unless an operator intentionally uses an AWS-managed TLS ingress.

The default init file also configures a high-cardinality Runtime v3 agent
population with `count = 10000` and a bounded sample. The Observatory shows the
true total while rendering only the sample, so live polis-scale demos do not
create 10,000 DOM nodes.

For an externally reachable polis, copy that init file to an operator-local
path and configure the runtime host interface, public route, certificate paths,
and allowed origins there. The checked-in browser client restricts Runtime v3
to the configured `runtime.dev.agent-logic.ai` HTTPS API base. Native clients may still
read the runtime-owned Observatory feed without an Origin header when the
operator's deployment policy allows it.

The Runtime v3 browser path consumes the public runtime-owned read feed at
`/v1/observatory`, the watcher readiness surface at `/v1/ready`, and the public
read stream at `/v1/observatory/ws`. The Operator Channel can submit a complete
pre-signed `adl.runtime.control_command.v1` envelope to `/v1/control`, and can
log in for WSS writes when authenticated socket control is available. The
browser never creates or stores the signing key; Runtime v3 verifies the
signature, principal, capability, runtime identity, and command policy before
execution. Logging out reconnects the public read stream without write
authority.

The browser-served dashboard only receives CORS permission when its origin is
listed in `[observatory].allowed_origins`. If the Runtime v3 API is reachable by
curl but the browser refuses the cross-origin fetch, the dashboard stays on the
retained mirror and reports the live fetch failure instead of claiming a live
Runtime v3 path.

Opening `index.html` directly may show the fallback shell in browsers that block
local `fetch()` for files. The retained proof is the local-server path plus the
validator below.

## Validate

The repository-native browser validator requires Playwright `1.60.0` exactly,
a real Chrome channel, an externally issued full-chain/private-key pair for the
exact Runtime and Observatory DNS names, and an explicit isolated Runtime
candidate command. Install the pinned package and browser under
operator-approved storage outside the repository, then expose the module
entrypoint through `ADL_PLAYWRIGHT_MODULE`.

Use alternate ports when `8765` or `20997` already have running services. The
validator refuses occupied listeners and only terminates the child Runtime it
started itself:

```sh
ADL_PLAYWRIGHT_MODULE=<absolute-playwright-1.60.0-entrypoint> \
ADL_V092_TLS_CERT=<absolute-external-full-chain> \
ADL_V092_TLS_KEY=<absolute-current-private-key> \
ADL_V092_RUNTIME_COMMAND_JSON='["<guardian-binary>","--init","<isolated-runtime-init>"]' \
node adl/tools/validate_v092_browser_trusted_observatory.mjs \
  --browser chrome \
  --require-trusted-tls \
  --runtime-url https://runtime.dev.agent-logic.ai:<alternate-runtime-port> \
  --observatory-url https://observatory.dev.agent-logic.ai:<alternate-observatory-port> \
  --evidence <absolute-redacted-evidence-path>
```

The validator uses ordinary platform trust only. It does not pass a custom CA
to curl, Node, or Chrome and does not install trust. It rejects localhost and IP
endpoint identities, browser interstitials, and console/network TLS failures.
This document does not claim a live browser proof until that command completes
against the deployed real-DNS endpoints.

Print the required native-platform dispositions without launching services:

```sh
node adl/tools/validate_v092_browser_trusted_observatory.mjs \
  --require-native-platform-evidence macos,linux,windows
```

## Claim Boundary

The retained validation proves static rendering and contract behavior. It does
not currently prove an ordinary platform-trusted browser or WSS exchange against
the real-DNS Runtime endpoint. That live proof remains gated on the browser
client update described above. The retained evidence proves that the HTML
Observatory can render an auto-refreshing CSM
panopticon over retained publishable runtime API responses, and can upgrade to a
live loopback CSM panopticon when the running CSM API base is supplied. It can
also consume the public Runtime v3 `/v1/observatory` read feed under its bounded
historical local contract. It renders the retained
bounded runtime capture through a polished investor-facing operator UI, while exposing
CSM API, CSM service, CloudWatch heartbeat, ACIP-SNS projection proof, Runtime
v3 status, and WP-08 linkage status. Its Operator Channel can submit
pre-signed commands to `/v1/control`, while Runtime v3 retains signature and
policy authority. It does not claim browser-owned AWS publish authority, Unity
completion, Runtime v2 decommission, full AWS signal
bridge completion, S3 ObsMem archive completion, or v0.92 runtime completion.
