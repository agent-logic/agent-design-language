#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MATRIX_PATH="${1:-$ROOT_DIR/docs/milestones/v0.91.7/review/runtime/soak2_feature_list_matrix_4843.json}"
PACKET_PATH="${2:-$ROOT_DIR/docs/milestones/v0.91.7/review/runtime/SOAK2_FEATURE_LIST_MATRIX_4843.md}"

python3 - "$MATRIX_PATH" "$PACKET_PATH" "$ROOT_DIR" <<'PY'
import json
import pathlib
import sys

matrix_path = pathlib.Path(sys.argv[1])
packet_path = pathlib.Path(sys.argv[2])
root = pathlib.Path(sys.argv[3])

required_rows = {
    "tokio_runtime_substrate",
    "agent_lifecycle",
    "aee_path",
    "acip_a2a_path",
    "provider_model_substrate",
    "scheduler",
    "resilience",
    "logging_observability",
    "runtime_aws_signal_bridge",
    "observatory_unity",
    "obsmem_memory_handoff",
    "identity_continuity",
    "capability_envelope",
    "security_cav_boundary",
    "curiosity_constructability_optional",
}
allowed_states = {
    "ready_for_soak2",
    "pending_upstream_pr",
    "blocked_before_soak2",
    "optional_non_claim",
}
required_fields = {
    "id",
    "surface",
    "owner_issues",
    "pre_soak_state",
    "command_or_harness",
    "fixture_setup",
    "expected_evidence",
    "negative_case",
    "blocker_policy",
    "v092_readiness_impact",
}

data = json.loads(matrix_path.read_text(encoding="utf-8"))
packet_text = packet_path.read_text(encoding="utf-8")
if data.get("schema") != "adl.v0917.runtime_soak2_feature_matrix.v1":
    raise SystemExit("unexpected or missing matrix schema")
if data.get("issue") != 4843 or data.get("consumer_issue") != 4682:
    raise SystemExit("matrix issue identity mismatch")

rows = data.get("rows")
if not isinstance(rows, list):
    raise SystemExit("rows must be a list")

seen = set()
for index, row in enumerate(rows, start=1):
    missing = sorted(required_fields - row.keys())
    if missing:
        raise SystemExit(f"row {index} missing fields: {missing}")
    row_id = row["id"]
    if row_id in seen:
        raise SystemExit(f"duplicate row id: {row_id}")
    seen.add(row_id)
    if row["pre_soak_state"] not in allowed_states:
        raise SystemExit(f"{row_id}: invalid pre_soak_state {row['pre_soak_state']!r}")
    if not row["owner_issues"] or not all(isinstance(issue, int) for issue in row["owner_issues"]):
        raise SystemExit(f"{row_id}: owner_issues must be non-empty integer list")
    for field in required_fields - {"owner_issues"}:
        if not isinstance(row[field], str) or not row[field].strip():
            raise SystemExit(f"{row_id}: {field} must be non-empty text")
    if row["pre_soak_state"].startswith("blocked") and "block" not in row["v092_readiness_impact"].lower():
        raise SystemExit(f"{row_id}: blocked row must state v0.92 blocking impact")
    if row["pre_soak_state"] == "optional_non_claim" and "non-claim" not in row["blocker_policy"].lower():
        raise SystemExit(f"{row_id}: optional row must preserve non-claim policy")
    if "docs/milestones/v0.91.7/review/runtime/soak2_4682" in row["expected_evidence"]:
        parent = root / "docs/milestones/v0.91.7/review/runtime"
        if not parent.exists():
            raise SystemExit(f"{row_id}: runtime review directory is missing")
    if row["surface"] not in packet_text:
        raise SystemExit(f"{row_id}: markdown packet is missing surface {row['surface']!r}")
    if row["pre_soak_state"] not in packet_text:
        raise SystemExit(f"{row_id}: markdown packet is missing state {row['pre_soak_state']!r}")
    for issue in row["owner_issues"]:
        issue_ref = f"#{issue}"
        if issue_ref not in packet_text:
            raise SystemExit(f"{row_id}: markdown packet is missing owner issue {issue_ref}")

missing_rows = sorted(required_rows - seen)
extra_rows = sorted(seen - required_rows)
if missing_rows or extra_rows:
    raise SystemExit(f"row set mismatch missing={missing_rows} extra={extra_rows}")

print(f"validated {len(rows)} Soak #2 matrix rows from {matrix_path}")
PY
