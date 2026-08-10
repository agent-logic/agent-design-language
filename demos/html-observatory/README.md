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

## Run Locally

Issue #83 is a private local Runtime v3 product proof. Public DNS, ACM, S3,
CloudFront, and public Runtime ingress are deferred to #122 after the distributed
Runtime is complete.

Use two loopback-only HTTPS listeners with an operator-trusted development
certificate whose SAN includes `localhost`. Keep the certificate and all private
keys outside the repository. Configure the Runtime init with:

- `api.address` bound to `127.0.0.1`;
- `api.public_base_url` and `api.tls.server_name` set to the same trusted local
  identity;
- certificate, private-key, and trust-root paths under the operator-local state
  root;
- the exact Observatory HTTPS origin in `observatory.allowed_origins`;
- distinct control, operation, and continuity keys.

Start the Runtime through its required Guardian lease and explicit init file,
then serve this repository root from the second loopback HTTPS listener. Open:

```text
https://localhost:<observatory-port>/demos/html-observatory/?runtime=v3&runtimeApiBase=https%3A%2F%2Flocalhost%3A<runtime-port>&live=1
```

The production binary admits the resident Shepherd through the production
Shepherd adapter before publishing the live roster. Startup fails closed if
that admission fails. The browser should show `Shepherd - running`, not a
fixture or retained owner-agent projection.

Runtime v3 uses only versioned routes:

```text
GET /v1/health
GET /v1/ready
GET /v1/metrics
GET /v1/observatory
GET /v1/observatory/ws
POST /v1/control
```

Public reads do not grant write authority. Layer 8 chat loads an operator
Ed25519 seed into memory for the current page only and submits a complete signed
control command. Runtime verifies the principal, capability, runtime identity,
recipient, correlation, content bounds, and command policy. The browser renders
only the bounded public response or refusal.

## Validate

Install Playwright `1.60.0` and its Chromium build under operator-approved
storage outside the repository. Start the exact Runtime and Observatory
candidate first, then run:

```sh
NODE_PATH=<playwright-node-modules> \
PLAYWRIGHT_BROWSERS_PATH=<playwright-browser-storage> \
ADL_OBSERVATORY_URL=https://localhost:<observatory-port>/demos/html-observatory/ \
ADL_RUNTIME_API_BASE=https://localhost:<runtime-port> \
ADL_OPERATOR_KEY_FILE=<operator-ed25519-seed-file> \
ADL_OBSERVATORY_EVIDENCE_DIR=<absolute-fastwork-evidence-directory> \
node adl/tools/validate_v092_html_observatory_live.mjs
```

The browser context keeps certificate verification enabled. The validator
requires the real Shepherd roster, signed selected-agent delivery, a real
`400 invalid_request` refusal, stopped-state authority removal, reconnect
deduplication, secret absence, and a clean console/network result.

Focused deterministic validation is:

```sh
bash adl/tools/test_html_observatory.sh
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test assembly
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test control
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract
```

## Claim Boundary

When the live validator completes successfully, its retained evidence establishes a real loopback-only Runtime v3, production-admitted
Shepherd roster, governed one-to-one Layer 8 message, bounded response/refusal,
and reconnect behavior in a real browser. It does not establish durable
conversation history, rooms, Unity integration, distributed Runtime completion,
AWS deployment, public exposure, or a birthday event. Those remain separate
downstream issues.
