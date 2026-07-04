#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT_ROOT="$ROOT_DIR/docs/milestones/v0.91.7/review/runtime/soak2_4682"

python3 - "$ARTIFACT_ROOT" <<'PY'
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
status_path = root / "soak2_execution_status_4682.json"
blocker_path = root / "blocker_register.json"
evidence_path = root / "evidence_index.json"

for path in (status_path, blocker_path, evidence_path, root / "README.md"):
    if not path.exists():
        raise SystemExit(f"missing required artifact: {path}")

status = json.loads(status_path.read_text())
blockers = json.loads(blocker_path.read_text())
evidence = json.loads(evidence_path.read_text())

if status.get("schema") != "adl.v0917.runtime_soak2.execution_status.v1":
    raise SystemExit("unexpected execution status schema")
if status.get("issue") != 4682 or status.get("umbrella_issue") != 4634:
    raise SystemExit("issue identity mismatch")
if status.get("status") != "blocked_before_full_soak":
    raise SystemExit("Soak 2 status must remain blocked_before_full_soak")
if status.get("v092_runtime_coherence_claim") != "blocked":
    raise SystemExit("v0.92 runtime coherence claim must be blocked")

required_rows = {
    "tokio_runtime_substrate",
    "agent_lifecycle",
    "resilience",
    "logging_observability",
    "soak2_matrix",
    "module_diet_map",
}
rows = {row.get("id"): row for row in status.get("row_results", [])}
row_ids = set(rows)
missing = sorted(required_rows - row_ids)
if missing:
    raise SystemExit(f"missing required row results: {missing}")
allowed_blocked_status_rows = {"blocked", "prerequisite_proven"}
for row_id, row in rows.items():
    state = row.get("state")
    if state not in allowed_blocked_status_rows:
        raise SystemExit(f"row {row_id} has invalid blocked-attempt state: {state}")
    if state in {"integrated_proven", "complete", "completed", "passed"}:
        raise SystemExit(f"row {row_id} overclaims completion during blocked attempt")
if rows["logging_observability"].get("state") != "prerequisite_proven":
    raise SystemExit("logging_observability must remain prerequisite_proven, not integrated_proven")
for row_id in required_rows - {"logging_observability"}:
    if rows[row_id].get("state") != "blocked":
        raise SystemExit(f"row {row_id} must remain blocked")

pr_index = {item["pr"]: item for item in status.get("upstream_pr_evidence", [])}
expected_prs = {
    4868: {
        "issue": 4681,
        "commit": "0b8821ec1de112d3b84fa64e87d2a6fb9fb63a02",
        "is_draft": False,
        "projection_status": "checks_pending",
        "failed_checks": [],
        "pending_checks": ["adl-ci"],
    },
    4869: {
        "issue": 4783,
        "commit": "cc87f735758ff78bc5a3360fe19d45c4b025552e",
        "is_draft": False,
        "projection_status": "checks_pending",
        "failed_checks": [],
        "pending_checks": ["adl-coverage"],
    },
    4870: {
        "issue": 4843,
        "commit": "2d15a0273d04d58467f1a477c9923cc6f6834b89",
        "is_draft": False,
        "projection_status": "ready_to_merge_or_review",
        "failed_checks": [],
        "pending_checks": [],
    },
    4871: {
        "issue": 4784,
        "commit": "573307379d3e487c05ccd974eb5b29942128d8db",
        "is_draft": False,
        "projection_status": "ready_to_merge_or_review",
        "failed_checks": [],
        "pending_checks": [],
    },
    4872: {
        "issue": 4683,
        "commit": "6464203cc8fe33d0f448dbf5973c73c8f5750f0e",
        "is_draft": False,
        "projection_status": "ready_to_merge_or_review",
        "failed_checks": [],
        "pending_checks": [],
    },
}
for pr, expected in expected_prs.items():
    actual = pr_index.get(pr)
    if actual is None:
        raise SystemExit(f"missing upstream PR evidence for #{pr}")
    for key, expected_value in expected.items():
        if actual.get(key) != expected_value:
            raise SystemExit(
                f"PR #{pr} {key} mismatch: expected {expected_value!r}, got {actual.get(key)!r}"
            )

matrix = status.get("matrix_source", {})
if (
    matrix.get("pr") != 4870
    or matrix.get("state") != "ready_green"
    or matrix.get("main_contains_matrix") is not False
):
    raise SystemExit("matrix source must point to ready-green PR #4870 and not main")

blocker_by_id = {item.get("id"): item for item in blockers.get("blockers", [])}
expected_blockers = {
    "soak2-matrix-not-on-main": (4843, 4870, "sequencing_blocker"),
    "runtime-path-checks-pending": (4681, 4868, "checks_pending_blocker"),
    "resilience-middleware-checks-pending": (4783, 4869, "checks_pending_blocker"),
    "resilience-failure-injection-not-on-main": (4784, 4871, "sequencing_blocker"),
    "runtime-diet-map-not-on-main": (4683, 4872, "sequencing_blocker"),
}
for blocker_id, (owner_issue, owner_pr, classification) in expected_blockers.items():
    blocker = blocker_by_id.get(blocker_id)
    if blocker is None:
        raise SystemExit(f"missing blocker: {blocker_id}")
    if (
        blocker.get("owner_issue") != owner_issue
        or blocker.get("owner_pr") != owner_pr
        or blocker.get("classification") != classification
    ):
        raise SystemExit(f"blocker {blocker_id} owner/classification drift")

refs = set(evidence.get("artifacts", []))
for required in (
    "README.md",
    "soak2_execution_status_4682.json",
    "blocker_register.json",
):
    if required not in refs:
        raise SystemExit(f"missing evidence index ref: {required}")

print("PASS validate_v0917_soak2_4682_status")
PY
