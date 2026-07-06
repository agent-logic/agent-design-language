# ADL HTML Observatory

This demo is the v0.91.7 HTML Observatory integrated proof for #4690.

It adapts the Magic UI Pro AI Agent Template direction into a static,
reviewable Observatory surface without importing account credentials or private
template metadata into the repository. The page consumes the retained CSM
runtime Observatory packet and operator report from the v0.91.7 Soak 2 evidence
root, plus the current CSM runtime administration and AWS linkage evidence:
the retained bounded runtime capture stays the source of truth.

- `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/visibility_packet.json`
- `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/operator_report.md`
- `docs/milestones/v0.91.7/review/runtime/csm_service_4903/service/service_manifest.json`
- `docs/milestones/v0.91.7/review/runtime/CSM_RUNTIME_API_4929.md`
- `docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/aws/cloudwatch_recent_events.redacted.json`

The CSM API panel intentionally presents the standalone `csm` runtime ownership
boundary from #4929. The CloudWatch panel presents the retained live heartbeat
proof from WP-08 #4684. The AWS linkage lane includes #4684 through #4688 so
closed heartbeat, ACIP-SNS, and SSM lanes remain distinct from open full-bridge
and S3 archive work. The communication rail can prepare an operator message
envelope and can check a live loopback CSM `/events` endpoint when an operator
supplies the API base.

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

This proves that the HTML Observatory can render the retained bounded runtime
capture and operator report through a polished investor-facing surface, while
also exposing CSM API, CSM service, CloudWatch heartbeat, and WP-08 linkage
status. It does not claim direct runtime mutation, Unity completion, full AWS
signal bridge completion, S3 ObsMem archive completion, or v0.92 runtime
completion.
