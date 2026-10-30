#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

SOURCE="$ROOT/docs/milestones/v0.91.7/review/runtime/wp08_local_polis_ssm_4687/local_polis_ssm_summary.json"
READINESS="$ROOT/docs/milestones/v0.91.7/review/security/wp12_ssm_readiness_4657.json"
GATE="$ROOT/docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json"

python3 "$ROOT/adl/tools/validate_wp12_ssm_readiness_4657.py" \
  --source-summary "$SOURCE" \
  --readiness-summary "$READINESS" \
  --gate "$GATE" >/dev/null

cp "$READINESS" "$TMP/readiness_bad.json"
python3 - "$TMP/readiness_bad.json" <<'PY'
import json
import sys
path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))
data["access_evidence"]["account_hash_verified"] = False
json.dump(data, open(path, "w", encoding="utf-8"), indent=2, sort_keys=True)
PY

if python3 "$ROOT/adl/tools/validate_wp12_ssm_readiness_4657.py" \
  --source-summary "$SOURCE" \
  --readiness-summary "$TMP/readiness_bad.json" \
  --gate "$GATE" >/dev/null 2>"$TMP/bad.err"
then
  echo "expected account_hash_verified=false to fail" >&2
  exit 1
fi
grep -F "account_hash_verified must be true" "$TMP/bad.err" >/dev/null

cp "$GATE" "$TMP/gate_bad.json"
python3 - "$TMP/gate_bad.json" <<'PY'
import json
import sys
path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))
for row in data["requirements"]:
    if row["id"] == "ssm_and_local_polis_secret_readiness":
        row["state"] = "child_issue_open"
json.dump(data, open(path, "w", encoding="utf-8"), indent=2, sort_keys=True)
PY

if python3 "$ROOT/adl/tools/validate_wp12_ssm_readiness_4657.py" \
  --source-summary "$SOURCE" \
  --readiness-summary "$READINESS" \
  --gate "$TMP/gate_bad.json" >/dev/null 2>"$TMP/gate_bad.err"
then
  echo "expected stale gate row to fail" >&2
  exit 1
fi
grep -F "SSM gate row must be integrated_proven" "$TMP/gate_bad.err" >/dev/null

cp "$READINESS" "$TMP/readiness_observable_bad.json"
python3 - "$TMP/readiness_observable_bad.json" <<'PY'
import json
import sys
path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))
data["observable_status_evidence"]["redacted_stream_hashes_retained"] = False
json.dump(data, open(path, "w", encoding="utf-8"), indent=2, sort_keys=True)
PY

if python3 "$ROOT/adl/tools/validate_wp12_ssm_readiness_4657.py" \
  --source-summary "$SOURCE" \
  --readiness-summary "$TMP/readiness_observable_bad.json" \
  --gate "$GATE" >/dev/null 2>"$TMP/observable_bad.err"
then
  echo "expected redacted_stream_hashes_retained=false to fail" >&2
  exit 1
fi
grep -F "redacted_stream_hashes_retained must be true" "$TMP/observable_bad.err" >/dev/null

cp "$READINESS" "$TMP/readiness_gate_update_bad.json"
python3 - "$TMP/readiness_gate_update_bad.json" <<'PY'
import json
import sys
path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))
data["gate_update"]["v092_disposition"] = "blocks_secret_runtime_claims"
json.dump(data, open(path, "w", encoding="utf-8"), indent=2, sort_keys=True)
PY

if python3 "$ROOT/adl/tools/validate_wp12_ssm_readiness_4657.py" \
  --source-summary "$SOURCE" \
  --readiness-summary "$TMP/readiness_gate_update_bad.json" \
  --gate "$GATE" >/dev/null 2>"$TMP/gate_update_bad.err"
then
  echo "expected stale gate_update disposition to fail" >&2
  exit 1
fi
grep -F "gate_update.v092_disposition must support SSM operations claims" "$TMP/gate_update_bad.err" >/dev/null

echo "PASS test_validate_wp12_ssm_readiness_4657"
