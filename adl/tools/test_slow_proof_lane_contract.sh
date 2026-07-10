#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
FAMILY_CONFIG="$ADL_DIR/config/slow_proof_families.v0.91.6.json"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

python3 - "$FAMILY_CONFIG" <<'PY'
import json
import sys
from pathlib import Path

config = json.loads(Path(sys.argv[1]).read_text())
if config.get("schema_version") != "adl.slow_proof_families.v1":
    raise SystemExit("unsupported slow-proof family config schema")
if config.get("umbrella_feature") != "slow-proof-tests":
    raise SystemExit("slow-proof umbrella feature drifted")
families = config.get("families", [])
if not families:
    raise SystemExit("slow-proof family config must declare at least one family")
seen_ids = set()
seen_features = set()
seen_samples = set()
for family in families:
    family_id = family.get("id")
    feature = family.get("feature")
    samples = family.get("sample_tests", [])
    if not family_id or family_id in seen_ids:
        raise SystemExit(f"invalid or duplicate slow-proof family id: {family_id!r}")
    if not feature or feature in seen_features:
        raise SystemExit(f"invalid or duplicate slow-proof feature: {feature!r}")
    if not feature.startswith("slow-proof-"):
        raise SystemExit(f"slow-proof feature must use slow-proof-* prefix: {feature}")
    if not samples:
        raise SystemExit(f"slow-proof family must carry sample tests: {family_id}")
    for sample in samples:
        if not sample.startswith("runtime_v2_"):
            raise SystemExit(f"slow-proof sample must stay runtime_v2-scoped: {sample}")
        if sample in seen_samples:
            raise SystemExit(f"duplicate slow-proof sample test: {sample}")
        seen_samples.add(sample)
    seen_ids.add(family_id)
    seen_features.add(feature)
PY

python3 - "$ROOT_DIR/adl/src/runtime_v2/tests.rs" "$FAMILY_CONFIG" <<'PY'
import json
import re
import sys
from pathlib import Path

tests_rs = Path(sys.argv[1]).read_text()
config = json.loads(Path(sys.argv[2]).read_text())

required_gates = {
    "governed_learning_substrate": "slow-proof-runtime",
    "intelligence_metric_architecture": "slow-proof-runtime",
    "memory_identity_architecture": "slow-proof-runtime",
    "observatory_flagship": "slow-proof-observatory",
}
for module, feature in required_gates.items():
    pattern = (
        r'#\[cfg\(any\(feature = "slow-proof-tests", feature = "'
        + re.escape(feature)
        + r'"\)\)\]\s*mod\s+'
        + re.escape(module)
        + r';'
    )
    if not re.search(pattern, tests_rs):
        raise SystemExit(f"slow runtime_v2 module is not gated by {feature}: {module}")

samples_by_family = {
    family["id"]: set(family.get("sample_tests", []))
    for family in config.get("families", [])
}
required_samples = {
    "runtime": {
        "runtime_v2_memory_identity_architecture_contract_is_stable",
        "runtime_v2_intelligence_metric_architecture_contract_is_stable",
        "runtime_v2_governed_learning_substrate_contract_is_stable",
    },
    "observatory": {
        "runtime_v2_observatory_flagship_review_surfaces_are_stable_and_serializable",
    },
}
for family, samples in required_samples.items():
    missing = samples - samples_by_family.get(family, set())
    if missing:
        raise SystemExit(f"slow-proof family {family} is missing samples: {sorted(missing)}")
PY

python3 - "$FAMILY_CONFIG" >"$tmpdir/families.tsv" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text())
for family in payload["families"]:
    print(f"{family['id']}\t{family['feature']}")
PY

while IFS=$'\t' read -r family feature; do
  [ -n "$family" ] || continue
  plan="$tmpdir/${family}.json"
  bash "$ROOT_DIR/adl/tools/run_slow_proof_family.sh" --family "$family" --json >"$plan"
  python3 - "$plan" "$family" "$feature" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text())
family = sys.argv[2]
feature = sys.argv[3]
if payload["id"] != family:
    raise SystemExit(f"wrong family id in plan: {payload['id']} != {family}")
if payload["feature"] != feature:
    raise SystemExit(f"wrong feature in plan: {payload['feature']} != {feature}")
expected_list = ["cargo", "nextest", "list", "--lib", "--features", feature, "runtime_v2_"]
expected_run = [
    "cargo",
    "nextest",
    "run",
    "--lib",
    "--features",
    feature,
    "runtime_v2_",
    "--status-level",
    "all",
    "--final-status-level",
    "slow",
]
if payload["list_command"] != expected_list:
    raise SystemExit(f"slow-proof list command drifted for {family}: {payload['list_command']}")
if payload["run_command"] != expected_run:
    raise SystemExit(f"slow-proof run command drifted for {family}: {payload['run_command']}")
PY
done <"$tmpdir/families.tsv"

echo "PASS test_slow_proof_lane_contract"
