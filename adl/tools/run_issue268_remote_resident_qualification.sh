#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)
RUN_ID=${ADL_RUN_ID:?ADL_RUN_ID is required}
EVIDENCE_ROOT=${ADL_ISSUE268_REMOTE_EVIDENCE_ROOT:-$ROOT/.adl/issue268-remote}
VOLUME_ROOT=${ADL_RUNTIME_CONTINUITY_ROOT:?ADL_RUNTIME_CONTINUITY_ROOT is required}
CONTINUITY_BIN=${ADL_ISSUE268_CONTINUITY_BIN:-$VOLUME_ROOT/install/current/bin/adl_resident_shepherd_continuity}
RUNTIME_ROOT=${ADL_ISSUE268_RETAINED_RUNTIME_ROOT:-$VOLUME_ROOT/state}
BUILD_CACHE_ROOT=${ADL_ISSUE268_BUILD_CACHE_ROOT:-${ADL_CACHE_VOLUME_MOUNT_PATH:?ADL_CACHE_VOLUME_MOUNT_PATH is required}}
AGENT_SPEC_DIR=${ADL_ISSUE268_AGENT_SPEC_DIR:-$EVIDENCE_ROOT/agent-specs}
VOLUME_IDENTITY=${ADL_ISSUE268_RUNTIME_VOLUME_IDENTITY_SHA256:-${ADL_RUNTIME_CONTINUITY_VOLUME_ID_SHA256:?ADL_RUNTIME_CONTINUITY_VOLUME_ID_SHA256 is required}}
MATERIALIZER=${ADL_ISSUE268_MATERIALIZER:-$ROOT/adl/tools/materialize_issue268_ollama_plan.py}
ORCHESTRATOR=${ADL_ISSUE268_CONTINUITY_UTS_RUNNER:-$ROOT/adl/tools/run_issue268_continuity_uts_qualification.py}
GUARDIAN=${ADL_ISSUE268_GUARDIAN_RUNNER:-$ROOT/adl/tools/validate_v092_runtime_guardian_lifecycle.sh}
SOURCE_RECEIPT=${ADL_ISSUE268_S3_SOURCE_RECEIPT:-$ROOT/.csdlc/evidence/268/aws/$RUN_ID/s3-source-receipt.json}
INSTALLER=${ADL_ISSUE268_VOLUME_INSTALLER:-$ROOT/adl/tools/install_issue268_runtime_volume.py}
ISSUE414_REVIEWED_SHA=${ADL_ISSUE268_414_REVIEWED_SHA:-6b1a4ee66f838e612d58c7b396851a610470190f}
CONTINUITY_BIN_SHA256=${ADL_ISSUE268_CONTINUITY_BIN_SHA256:-}

[[ "$VOLUME_IDENTITY" =~ ^[0-9a-f]{64}$ ]] || { echo "issue268: exact retained-volume SHA-256 required" >&2; exit 65; }
[[ "$ISSUE414_REVIEWED_SHA" =~ ^[0-9a-f]{40}$ ]] || { echo "issue268: exact #414 reviewed SHA required" >&2; exit 65; }
[[ -f "$SOURCE_RECEIPT" && -f "$INSTALLER" ]] || { echo "issue268: immutable S3/EBS installation inputs missing" >&2; exit 69; }
python3 - "$VOLUME_ROOT" "$BUILD_CACHE_ROOT" <<'PY'
import pathlib, sys
volume = pathlib.Path(sys.argv[1]).resolve()
cache = pathlib.Path(sys.argv[2]).resolve()
if volume == cache or volume in cache.parents or cache in volume.parents:
    raise SystemExit("issue268: build cache must remain separate from retained Runtime volume")
PY
mkdir -p "$EVIDENCE_ROOT" "$RUNTIME_ROOT" "$BUILD_CACHE_ROOT"
SOURCE_REVISION=$(git -C "$ROOT" rev-parse HEAD)
install_json=$(python3 "$INSTALLER" \
  --volume-root "$VOLUME_ROOT" \
  --build-cache "$BUILD_CACHE_ROOT" \
  --source-root "$ROOT" \
  --source-revision "$SOURCE_REVISION" \
  --source-receipt "$SOURCE_RECEIPT" \
  --reviewed-git-sha "$ISSUE414_REVIEWED_SHA" \
  --volume-identity-sha256 "$VOLUME_IDENTITY")
installed_paths_file="$EVIDENCE_ROOT/installed-paths.txt"
python3 - "$install_json" >"$installed_paths_file" <<'PY'
import json,sys
d=json.loads(sys.argv[1])
print(d["ollama_binary"])
print(d["ollama_models"])
print(d["continuity_binary"])
print(d["continuity_binary_sha256"])
PY
OLLAMA_BIN=$(sed -n '1p' "$installed_paths_file")
OLLAMA_MODELS=$(sed -n '2p' "$installed_paths_file")
CONTINUITY_BIN=$(sed -n '3p' "$installed_paths_file")
installed_continuity_sha=$(sed -n '4p' "$installed_paths_file")
[[ -x "$OLLAMA_BIN" && -x "$CONTINUITY_BIN" ]] || { echo "issue268: installed Runtime binaries missing" >&2; exit 69; }
if [[ -n "$CONTINUITY_BIN_SHA256" && "$installed_continuity_sha" != "$CONTINUITY_BIN_SHA256" ]]; then
  echo "issue268: installed continuity binary provenance mismatch" >&2
  exit 65
fi
export OLLAMA_MODELS OLLAMA_HOST=http://127.0.0.1:11434
OLLAMA_LOG="$EVIDENCE_ROOT/ollama.log"
"$OLLAMA_BIN" serve >"$OLLAMA_LOG" 2>&1 &
OLLAMA_PID=$!
cleanup_ollama() { kill "$OLLAMA_PID" >/dev/null 2>&1 || true; wait "$OLLAMA_PID" 2>/dev/null || true; }
trap cleanup_ollama EXIT
for _ in $(seq 1 90); do
  if curl -fsS "$OLLAMA_HOST/api/tags" >/dev/null 2>&1; then break; fi
  sleep 2
done
curl -fsS "$OLLAMA_HOST/api/tags" >/dev/null || { echo "issue268: installed Ollama failed to open loopback" >&2; exit 70; }
materialized="$EVIDENCE_ROOT/materialized-plan.json"
python3 "$MATERIALIZER" --output "$materialized" --agent-spec-dir "$AGENT_SPEC_DIR" >/dev/null
python3 "$ORCHESTRATOR" \
  --continuity-bin "$CONTINUITY_BIN" \
  --runtime-root "$RUNTIME_ROOT" \
  --build-cache-root "$BUILD_CACHE_ROOT" \
  --agent-spec-dir "$AGENT_SPEC_DIR" \
  --runtime-volume-identity-sha256 "$VOLUME_IDENTITY" \
  --state "$EVIDENCE_ROOT/uts-state.json" \
  --evidence-dir "$EVIDENCE_ROOT/continuity-uts" \
  --plan "$materialized"

receipt="$EVIDENCE_ROOT/continuity-uts/qualification-receipt.json"
python3 - "$receipt" <<'PY'
import json, pathlib, sys
value=json.loads(pathlib.Path(sys.argv[1]).read_text())
population=value.get("signed_population_sha256", "")
residents=value.get("residents") or []
if (value.get("status") != "passed" or value.get("resident_count") != 6
    or value.get("continuation_verified") is not True
    or value.get("replay_denied") is not True
    or len(population) != 64
    or len(residents) != 6
    or len({row.get("agent_id") for row in residents}) != 6):
    raise SystemExit("issue268: continuity/UTS receipt is not proving")
print("ADL_ISSUE268_CONTINUITY_UTS_BEGIN")
print(json.dumps(value, sort_keys=True))
print("ADL_ISSUE268_CONTINUITY_UTS_END")
PY

# The six-hour guardian begins only after the resident population has proven
# useful work across a closed-admission dehydration/restore cycle.
bash "$GUARDIAN" --suite six_hour_qualification
