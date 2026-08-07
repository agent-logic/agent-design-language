# ADL HTML Observatory

This demo is the v0.91.7 HTML Observatory integrated proof for #4690.

It adapts the Magic UI Pro AI Agent Template direction, with the Magic UI
Devtool Template used for denser dashboard composition cues, into a reviewable
CSM polis panopticon without importing account credentials or private template
metadata into the repository. The first-class mode is a compact control-room
dashboard over the Runtime v3 `/v1/observatory` and `/v1/ready` browser
surfaces, with retained CSM `/status`, `/health`, `/ready`, `/metrics`, and
`/events` proof available as fallback. Runtime KPIs, agent graph preview, event
tail, CSM API status, CloudWatch linkage, governance proof, and operator
communication status remain visible in the first dashboard viewport. It
auto-refreshes the retained publishable CSM API response artifacts from #4976 as
a runtime mirror, and upgrades to live loopback polling when an operator
supplies the currently running CSM API base. The retained runtime packet remains
the fallback proof surface if the Runtime v3 feed cannot load. The page reads
the configured Runtime v3 Observatory API base and endpoints from
`runtime-v3.config.json`. It also consumes the retained CSM runtime Observatory
packet and operator report from the v0.91.7 Soak 2 evidence root, plus the
current CSM runtime administration and AWS linkage evidence.

The primary desktop dashboard is fixed to the viewport, while narrower browser
windows use page scrolling so controls are never clipped. Event streams and
inspector areas retain bounded internal overflow. The visible shell uses local inline SVG icons, role-specific topology
glyphs for owner, readiness, scheduler, telemetry, event, and checkpoint lanes,
non-overlapping graph nodes with signal-line affordances, a compact table-style
event stream, rail telemetry, an inspector-style CSM API/gauge stack, and a
bottom runtime status bar to match the approved control-room mockup without
importing external template assets.

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

The CSM polis panopticon presents an auto-refreshing agent map, agent roster,
health, readiness, metrics, and operator event stream from the retained CSM API
mirror fallback when Runtime v3 is unavailable. When a loopback API base is
supplied with `runtime=v2` or `csmApiBase`, it polls the running CSM API
directly. For Runtime v3, the default path reads
`demos/html-observatory/runtime-v3.config.json`; query parameters such as
`?runtime=v3&runtimeApiBase=<runtime-api-base>&live=1` remain a troubleshooting
override. The runtime API base must match the configured Runtime v3 control API
host, port, TLS posture, and Observatory allowed-origin policy. The Runtime v3
path consumes the runtime-owned `/v1/observatory` read feed and `/v1/ready`
without bearer credentials. Runtime v3 control mutation remains
signed-command-only through `/v1/control`; the browser has no unsigned
shutdown, mutation, CloudWatch, SNS, or state authority. The CSM API panel intentionally
presents the standalone `csm` runtime ownership boundary from #4929 when the
retained/default mirror is selected. The CloudWatch panel presents the retained
live heartbeat proof from WP-08 #4684. The AWS linkage lane includes #4684
through #4688 so closed heartbeat, ACIP-SNS, and SSM lanes remain distinct from
open full-bridge and S3 archive work. The communication rail can prepare an
ACIP-shaped operator message envelope, mirror the retained #4685 ACIP-SNS proof,
check the Runtime v3 event tail through `/v1/observatory` by default, and check
a live loopback CSM `/events` endpoint when an operator supplies a Runtime v2
API base. Live SNS/SQS mutation remains runtime/tool-owned and is not performed
by the browser surface.

## Run

The supported local identity is one Runtime-generated certificate containing
exactly these SANs:

```text
DNS:localhost
IP:127.0.0.1
IP:::1
```

Both separate listeners use the current certificate and key paths returned by
`adl-runtime-local-tls-bootstrap`. The Runtime remains on
`https://localhost:20997`; the Observatory remains on
`https://localhost:8765`. There is no HTTP fallback and no certificate-warning
bypass.

Create an operator-local bootstrap config outside the repository. The state
root must be absolute, while all paths beneath it remain relative:

```toml
schema = "adl.runtime_v3.local_tls_bootstrap.v1"
mode = "local_self_signed"
state_root = "<absolute-operator-state-root>"
tls_dir = "runtime-tls"
certificate_chain_path = "localhost-chain.pem"
public_certificate_path = "localhost-public.pem"
private_key_path = "localhost-key.pem"
dns_names = ["localhost"]
ip_addresses = ["127.0.0.1", "::1"]
replace = false
```

Generate or validate the stable identity without changing host trust:

```sh
adl-runtime-local-tls-bootstrap \
  --config <absolute-bootstrap-config> \
  --operation bootstrap
```

### macOS trust lifecycle

macOS is the supported host-trust implementation. Installation and removal
require the explicit consent flag and an absolute user-keychain path. The tool
first validates dates, SANs, the Rustls certificate/key pair, and key mode. It
refuses to modify an existing certificate that lacks its own receipt.

```sh
adl-runtime-local-tls-bootstrap \
  --config <absolute-bootstrap-config> \
  --operation trust-install \
  --consent-host-trust \
  --trust-store <absolute-user-login-keychain>
```

Trust verification is a separate, non-mutating operation and exits non-zero
when macOS does not trust the certificate for `localhost`:

```sh
adl-runtime-local-tls-bootstrap \
  --config <absolute-bootstrap-config> \
  --operation trust-verify \
  --trust-store <absolute-user-login-keychain>
```

Reissue stages and validates a new generation, installs and verifies its trust,
then swaps the current manifest. Any pre-swap failure removes candidate trust
and preserves the prior generation and trust entry. The old trust entry is
removed only after the new manifest commits, and only when an exact tool-owned
receipt authorizes that removal.

If old-trust cleanup fails after the new manifest commits, reissue returns a
successful `trusted_cleanup_pending` outcome with the exact old certificate
fingerprint. The new identity remains current; retry cleanup with
`trust-remove --certificate-sha256 <fingerprint>` instead of rotating again.
Cleanup is idempotent when the exact certificate is already absent but its
tool-owned receipt still needs removal.

```sh
adl-runtime-local-tls-bootstrap \
  --config <absolute-bootstrap-config> \
  --operation reissue \
  --consent-host-trust \
  --trust-store <absolute-user-login-keychain>
```

Remove the current issue-created trust entry:

```sh
adl-runtime-local-tls-bootstrap \
  --config <absolute-bootstrap-config> \
  --operation trust-remove \
  --consent-host-trust \
  --trust-store <absolute-user-login-keychain>
```

Pass `--certificate-sha256 <digest>` only to retry cleanup of an older receipt
reported by a reissue. Removal fails closed if the receipt is absent, malformed,
for another keychain, or for another certificate.

### Platform disposition

- macOS: implemented with `security(1)` against an explicitly selected user
  keychain; native live trust remains operator-consent work.
- Linux: blocked. Chrome NSS trust and system curl trust are separate stores;
  this issue does not claim a single reversible native transaction for both.
- Native Windows: blocked. CurrentUser Root import and exact removal have not
  received native execution proof.

These blockers are not instructions to disable verification.

### Runtime and Observatory

Configure Runtime `[api.tls]` with the returned current generation paths and
keep `[observatory].allowed_origins` set to `https://localhost:8765`. Start the
Runtime v3 kernel with its operator-local init file and token:

```sh
ADL_RUNTIME_OBSERVATORY_TOKEN="<operator-local-token>" \
  adl-runtime-kernel serve --init <absolute-operator-runtime-init>
```

Serve the Observatory with the same current certificate and key using the
issue validator below, or another HTTPS server that is explicitly configured
with those exact files. Then open:

```text
https://localhost:8765/demos/html-observatory/
```

The default init file keeps the Runtime v3 listener on `localhost:20997`.
Runtime v3 browser/API access is HTTPS-only. Before launch, provision a
localhost certificate and private key at the `[api.tls]` paths in the init file;
the repository does not retain private keys. Set a 32-to-256-character
operator-local write token for the runtime process in
`ADL_RUNTIME_OBSERVATORY_TOKEN`. Runtime v3 health, readiness, metrics,
Observatory snapshots, and the Observatory WSS feed are public read surfaces
and require no token.

Runtime v3 uses versioned operator probes:

```text
GET https://localhost:20997/v1/health
GET https://localhost:20997/v1/ready
GET https://localhost:20997/v1/metrics
GET https://localhost:20997/v1/observatory
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
execution. To enable WSS writes for the current browser tab, set the same token
without putting it in the URL or repository, then reconnect:

```js
sessionStorage.setItem("adl.runtimeV3.observatoryToken", "<operator-local-token>");
```

The token elevates only that WSS connection for writes; `/v1/control` remains a
signed-command endpoint and signature verification plus canonical ingress
policy still apply. The shared localhost certificate must be trusted by the
browser for both `https://localhost:8765` and `https://localhost:20997`. The
kernel terminates its own TLS connection; a local API Gateway or sidecar is not
required.

The default init file also configures a high-cardinality Runtime v3 agent
population with `count = 10000` and a bounded sample. The Observatory shows the
true total while rendering only the sample, so live polis-scale demos do not
create 10,000 DOM nodes.

For an externally reachable polis, copy that init file to an operator-local
path and configure the runtime host interface, public route, certificate paths,
and allowed origins there. This packaged browser dashboard only sends Runtime v3
WSS/auth traffic to the trusted localhost API base. Native clients may still
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

Deterministic TLS contract and negative coverage:

```sh
cargo test --locked --manifest-path adl-runtime/Cargo.toml --test local_tls
```

The repository-native browser validator requires Playwright `1.60.0` exactly,
a real Chrome channel, the current public certificate/private key, and an
explicit isolated Runtime candidate command. Install the pinned package and
browser under operator-approved storage outside the repository, then expose the
module entrypoint through `ADL_PLAYWRIGHT_MODULE`.

Use alternate ports when `8765` or `20997` already have running services. The
validator refuses occupied listeners and only terminates the child Runtime it
started itself:

```sh
ADL_PLAYWRIGHT_MODULE=<absolute-playwright-1.60.0-entrypoint> \
ADL_V092_TLS_CERT=<absolute-current-public-certificate> \
ADL_V092_TLS_KEY=<absolute-current-private-key> \
ADL_V092_RUNTIME_COMMAND_JSON='["<runtime-binary>","serve","--init","<isolated-runtime-init>"]' \
node adl/tools/validate_v092_browser_trusted_observatory.mjs \
  --browser chrome \
  --require-trusted-tls \
  --runtime-url https://localhost:<alternate-runtime-port> \
  --observatory-url https://localhost:<alternate-observatory-port> \
  --evidence <absolute-redacted-evidence-path>
```

The validator launches the real HTTPS static listener and supplied Runtime
candidate, rejects browser interstitials plus console/network TLS failures,
checks HTML and Runtime health/readiness/feed in Chrome, then independently
runs `curl --cacert` over the same endpoints. It records only exact head,
certificate digest, localhost listener URLs, statuses, and platform
dispositions; private keys, tokens, trust exports, and command output are not
retained.

Print the required native-platform dispositions without launching services:

```sh
node adl/tools/validate_v092_browser_trusted_observatory.mjs \
  --require-native-platform-evidence macos,linux,windows
```

## Claim Boundary

The validation lane includes a real TLS client against a running Runtime v3
endpoint and separately retains the JavaScript mock as static
rendering-contract coverage. This proves that the HTML Observatory can render an auto-refreshing CSM
panopticon over retained publishable runtime API responses, and can upgrade to a
live loopback CSM panopticon when the running CSM API base is supplied. It can
also consume the public Runtime v3 `/v1/observatory` read feed when
Runtime v3 is available at its configured local API base. It renders the retained
bounded runtime capture through a polished investor-facing operator UI, while exposing
CSM API, CSM service, CloudWatch heartbeat, ACIP-SNS projection proof, Runtime
v3 status, and WP-08 linkage status. Its Operator Channel can submit
pre-signed commands to `/v1/control`, while Runtime v3 retains signature and
policy authority. It does not claim browser-owned AWS publish authority, Unity
completion, Runtime v2 decommission, full AWS signal
bridge completion, S3 ObsMem archive completion, or v0.92 runtime completion.
