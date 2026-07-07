# ADL HTML Observatory

This demo is the v0.91.7 HTML Observatory integrated proof for #4690.

It adapts the Magic UI Pro AI Agent Template direction, with the Magic UI
Devtool Template used for denser dashboard composition cues, into a reviewable
CSM polis panopticon without importing account credentials or private template
metadata into the repository. The first-class mode is a compact control-room
dashboard over `/status`, `/health`, `/ready`, `/metrics`, and `/events`, with
runtime KPIs, agent graph preview, event tail, CSM API status, CloudWatch
linkage, governance proof, and operator communication status visible in the
first dashboard viewport. It auto-refreshes the retained publishable CSM API
response artifacts from #4976 as a runtime mirror, and upgrades to live loopback
polling when an operator supplies the currently running CSM API base. The
retained runtime packet remains the fallback proof surface if the CSM API mirror
cannot load. The page also consumes the retained CSM runtime Observatory packet
and operator report from the v0.91.7 Soak 2 evidence root, plus the current CSM
runtime administration and AWS linkage evidence.

The primary dashboard is intentionally fixed to the viewport: the page itself
does not scroll, while the event stream and inspector areas own bounded internal
overflow. The visible shell uses local inline SVG icons, role-specific topology
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

The CSM polis panopticon presents an auto-refreshing agent map, agent roster,
health, readiness, metrics, and operator event stream from the retained CSM API
mirror by default. When a loopback API base is supplied, it polls the running
CSM API directly. The CSM API panel intentionally presents the standalone `csm`
runtime ownership boundary from #4929. The CloudWatch panel presents the
retained live heartbeat proof from WP-08 #4684. The AWS linkage lane includes
#4684 through #4688 so closed heartbeat, ACIP-SNS, and SSM lanes remain distinct
from open full-bridge and S3 archive work. The communication rail can prepare an
operator message envelope and can check a live loopback CSM `/events` endpoint
when an operator supplies the API base.

## Run

From the repository root:

```sh
python3 -m http.server 8765
```

Then open:

```text
http://127.0.0.1:8765/demos/v0.91.7/html-observatory/
```

Opening `index.html` directly may show the fallback shell in browsers that block
local `fetch()` for files. The retained proof is the local-server path plus the
validator below.

## Validate

```sh
bash adl/tools/test_v0917_html_observatory_integrated_proof.sh
```

## Claim Boundary

This proves that the HTML Observatory can render an auto-refreshing CSM
panopticon over retained publishable runtime API responses, and can upgrade to a
live loopback CSM panopticon when the running API base is supplied. It also
renders the retained bounded runtime capture through a polished investor-facing
operator UI, while exposing CSM API, CSM service, CloudWatch heartbeat, and WP-08
linkage status. It does not claim direct runtime mutation, public/remote API
exposure, Unity completion, full AWS signal bridge completion, S3 ObsMem archive
completion, or v0.92 runtime completion.
