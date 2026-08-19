#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="${ROOT_DIR}/.csdlc/evidence/341/local-test"
MATRIX="demos/v0.92/provider-neutral-birthday/proof-matrix.json"
mkdir -p "${OUT}"

cd "${ROOT_DIR}"
SOURCE_REVISION="$(git rev-parse HEAD)"
bash "adl/tools/demo_v092_provider_neutral_birthday.sh" --mode local-proof >"${OUT}/local-proof.log"
cp "${ROOT_DIR}/demos/v0.92/provider-neutral-birthday/acip-trace-local-proof.json" "${OUT}/acip-trace-local-proof.json"
python3 "adl/tools/validate_v092_provider_neutral_proof.py" "${MATRIX}" --require-observatory >"${OUT}/validator-pass.log"
python3 "adl/tools/serve_v092_provider_neutral_observatory_api.py" \
  --matrix "demos/v0.92/provider-neutral-birthday/proof-matrix-observatory.json" \
  --source-revision "${SOURCE_REVISION}" \
  --emit-feed >"${OUT}/runtime-v3-overlay-feed.json"
python3 - "${OUT}/runtime-v3-overlay-feed.json" "${SOURCE_REVISION}" <<'PY'
import json, pathlib, sys
feed=json.loads(pathlib.Path(sys.argv[1]).read_text())
source_revision=sys.argv[2]
assert feed["schema"] == "adl.runtime_v3.observatory_feed.v2"
assert feed["agents"]["total_count"] == 3
assert len(feed["agents"]["sample"]) == 3
assert source_revision != "0123456789abcdef0123456789abcdef01234567"
assert all(a["source_revision"] == source_revision for a in feed["agents"]["sample"])
assert all(a["source_revision"] == source_revision for a in feed["health"]["snapshot"]["agent_admissions"].values())
ordinary=[a for a in feed["agents"]["sample"] if a["role"] != "shepherd"]
assert ordinary and all(a["communication_eligible"] is True for a in ordinary)
assert all(a.get("ssm_access") == "none" for a in ordinary)
assert any(a["role"] == "shepherd" and a.get("ssm_access") == "maintenance_only" for a in feed["agents"]["sample"])
PY

python3 - "${MATRIX}" "${OUT}/missing-negative.json" <<'PY'
import json, pathlib, sys
m=json.loads(pathlib.Path(sys.argv[1]).read_text())
m["negative_cases"]=[c for c in m["negative_cases"] if c.get("case")!="substitution_attempt"]
pathlib.Path(sys.argv[2]).write_text(json.dumps(m, indent=2, sort_keys=True)+"\n")
PY
if python3 "adl/tools/validate_v092_provider_neutral_proof.py" "${OUT}/missing-negative.json" >"${OUT}/missing-negative.stdout" 2>"${OUT}/missing-negative.stderr"; then
  echo "validator accepted missing substitution_attempt" >&2
  exit 1
fi

python3 - "${MATRIX}" "${OUT}/fake-negative-receipt.json" <<'PY'
import json, pathlib, sys
m=json.loads(pathlib.Path(sys.argv[1]).read_text())
m["negative_cases"][0]["receipt_sha256"]="0"*64
pathlib.Path(sys.argv[2]).write_text(json.dumps(m, indent=2, sort_keys=True)+"\n")
PY
if python3 "adl/tools/validate_v092_provider_neutral_proof.py" "${OUT}/fake-negative-receipt.json" >"${OUT}/fake-negative-receipt.stdout" 2>"${OUT}/fake-negative-receipt.stderr"; then
  echo "validator accepted fake negative receipt" >&2
  exit 1
fi

python3 - "${MATRIX}" "${OUT}/fake-positive-receipt.json" <<'PY'
import json, pathlib, sys
m=json.loads(pathlib.Path(sys.argv[1]).read_text())
m["provider_columns"][0]["receipt_sha256"]="0"*64
pathlib.Path(sys.argv[2]).write_text(json.dumps(m, indent=2, sort_keys=True)+"\n")
PY
if python3 "adl/tools/validate_v092_provider_neutral_proof.py" "${OUT}/fake-positive-receipt.json" >"${OUT}/fake-positive-receipt.stdout" 2>"${OUT}/fake-positive-receipt.stderr"; then
  echo "validator accepted fake positive receipt" >&2
  exit 1
fi

python3 - "${MATRIX}" "${OUT}/leaky-provider.json" <<'PY'
import json, pathlib, sys
m=json.loads(pathlib.Path(sys.argv[1]).read_text())
m["provider_columns"][0]["raw_output_recorded"]=True
pathlib.Path(sys.argv[2]).write_text(json.dumps(m, indent=2, sort_keys=True)+"\n")
PY
if python3 "adl/tools/validate_v092_provider_neutral_proof.py" "${OUT}/leaky-provider.json" >"${OUT}/leaky-provider.stdout" 2>"${OUT}/leaky-provider.stderr"; then
  echo "validator accepted raw provider output retention" >&2
  exit 1
fi

python3 - "${MATRIX}" "${OUT}/agent-ssm.json" <<'PY'
import json, pathlib, sys
m=json.loads(pathlib.Path(sys.argv[1]).read_text())
m["observatory"]["agents"][1]["ssm_access"]="maintenance_only"
pathlib.Path(sys.argv[2]).write_text(json.dumps(m, indent=2, sort_keys=True)+"\n")
PY
if python3 "adl/tools/validate_v092_provider_neutral_proof.py" "${OUT}/agent-ssm.json" --require-observatory >"${OUT}/agent-ssm.stdout" 2>"${OUT}/agent-ssm.stderr"; then
  echo "validator accepted ordinary agent SSM access" >&2
  exit 1
fi

echo "issue341 provider-neutral proof tests passed"
