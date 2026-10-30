# v0.87 Demo Program

Historical boundary: the editor start, monolith steering/checkpoint, and shared-ObsMem shell routes named below were retired in #1223. Their original assertions and artifacts remain historical obligations, not current passing proof. The independent `adl-runtime` trace-truth route and current editor adapter/assets are retained.

This records the historical `v0.87` substrate demo program. The aggregate and
reviewer-package wrapper below are retired after the monolithic tooling route
was sunset. Individual child demos have separate owner and qualification
requirements; this page does not establish a current full-suite pass.

Run from repository root.

## Historical full review path

Former command (unavailable): `bash adl/tools/demo_v087_suite.sh`.

The historical aggregate populated artifact roots under:

- `artifacts/v087/trace_v1/`
- `artifacts/v087/provider_portability/`
- `artifacts/v087/shared_obsmem/`
- `artifacts/v087/skills/`
- `artifacts/v087/control_plane/`
- `artifacts/v087/reviewer_package/`

## Per-demo entrypoints

### D1 Trace v1 substrate truth

The ADL wrapper is retired; the [exact Runtime source scenario](https://github.com/agent-logic/agent-logic-runtime/blob/7d0db620ad79c50ff5d8fabf0fb543183583d05c/adl/tools/demo_v087_trace_truth.sh) retains its original command and mock-backed workflow. The command below is historical ADL provenance, not current ADL execution or a new Runtime acceptance claim.

```bash
bash adl/tools/demo_v087_trace_truth.sh
```

Primary proof surface:
- `artifacts/v087/trace_v1/runs/v0-6-hitl-no-pause-demo/logs/trace_v1.json`

### D2 Provider portability substrate

```bash
bash adl/tools/demo_v087_provider_portability.sh
```

Primary proof surface:
- `artifacts/v087/provider_portability/provider_substrate_manifest.v1.json`

### D3 Shared ObsMem foundation coherence

```bash
# Historical retired command: bash adl/tools/demo_v087_shared_obsmem.sh
```

Primary proof surface:
- `artifacts/v087/shared_obsmem/demo-f-obsmem-retrieval/obsmem_retrieval_result.json`

### D4 Operational skills substrate

```bash
bash adl/tools/demo_v087_operational_skills.sh
```

Primary proof surface:
- `artifacts/v087/skills/skills_inventory.txt`

### D5 Control-plane / PR tooling substrate

```bash
bash adl/tools/demo_v087_control_plane.sh
```

Primary proof surface:
- `artifacts/v087/control_plane/runs/v0-4-demo-deterministic-replay/run_summary.json`

### D6 Historical reviewer-facing substrate package

Former command (unavailable): `bash adl/tools/demo_v087_reviewer_package.sh`.
The original good/bad review contract assertions and source remain in Git and
retirement custody. The retained review format and fixtures are unchanged;
this is not a replacement validator or a claim of current acceptance.

Primary proof surface:
- `artifacts/v087/reviewer_package/reviewer_package_manifest.txt`

## Truth boundaries

- These demos are repo-local and bounded.
- They use fixture-backed or mock-provider-backed commands where that keeps the
  proof deterministic and runnable without network dependence.
- They prove the `v0.87` substrate surfaces that actually exist now; they do
  not claim later identity, society, routing, or governance capabilities.
