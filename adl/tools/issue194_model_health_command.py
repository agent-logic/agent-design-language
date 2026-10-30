#!/usr/bin/env python3
"""Generate the remote model-health command for ADL issue #194.

The generated command is intentionally self-contained because it runs through
the shepherd maintenance plane (SSM) on private voters. It must not depend on
Internet egress or hosted model fallbacks.
"""

from __future__ import annotations

import json
import pathlib
import shlex
import sys


BUCKET = "adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2"
MANIFEST_KEY = "shepherd/gemma4-12b/ollama-0.31.1/artifact-manifest.json"


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: issue194_model_health_command.py <out-json>")
    out = pathlib.Path(sys.argv[1])
    script = f"""set -euo pipefail
export AWS_DEFAULT_REGION=us-west-2
export ADL_ISSUE194_ROOT=/var/lib/adl/issue194
export ADL_ISSUE194_BUNDLE="${{ADL_ISSUE194_ROOT}}/model-health"
export ADL_ISSUE194_MANIFEST="${{ADL_ISSUE194_BUNDLE}}/artifact-manifest.json"
export ADL_ISSUE194_MODEL_STORE="${{ADL_ISSUE194_BUNDLE}}/model-store"
export ADL_ISSUE194_RUNTIME="${{ADL_ISSUE194_BUNDLE}}/runtime"
export ADL_ISSUE194_HOME="${{ADL_ISSUE194_BUNDLE}}/home"
export OLLAMA_MODELS="${{ADL_ISSUE194_MODEL_STORE}}/model-store"
export OLLAMA_HOST=127.0.0.1:11434
mkdir -p "${{ADL_ISSUE194_BUNDLE}}" "${{ADL_ISSUE194_MODEL_STORE}}" "${{ADL_ISSUE194_RUNTIME}}" "${{ADL_ISSUE194_HOME}}"
command -v aws
command -v python3
command -v curl
if command -v zstd >/dev/null 2>&1; then
  ADL_ISSUE194_TAR_ARGS=(--use-compress-program zstd)
elif tar --help 2>/dev/null | grep -q -- '--zstd'; then
  ADL_ISSUE194_TAR_ARGS=(--zstd)
else
  echo "missing zstd-capable tar runtime" >&2
  exit 41
fi
timeout 60s aws s3 cp --region us-west-2 s3://{BUCKET}/{MANIFEST_KEY} "${{ADL_ISSUE194_MANIFEST}}" >/dev/null
python3 - <<'PY'
import json
import pathlib

manifest_path = pathlib.Path("/var/lib/adl/issue194/model-health/artifact-manifest.json")
manifest = json.loads(manifest_path.read_text())
runtime = [entry for entry in manifest["artifacts"] if entry["kind"] == "ollama_runtime"]
model = [entry for entry in manifest["artifacts"] if entry["kind"] == "ollama_model_store"]
print(json.dumps({{
    "manifest_model_identity": manifest["model_identity"],
    "manifest_model_digest_sha256": manifest["model_digest_sha256"],
    "ollama_runtime_artifacts": len(runtime),
    "model_store_artifacts": len(model),
    "model_store_bytes": sum(entry["size_bytes"] for entry in model),
}}, sort_keys=True))
PY
python3 - <<'PY' > "${{ADL_ISSUE194_BUNDLE}}/download-model-store.sh"
import json
import pathlib
import shlex

manifest = json.loads(pathlib.Path("/var/lib/adl/issue194/model-health/artifact-manifest.json").read_text())
bucket = manifest["bucket"]
for entry in manifest["artifacts"]:
    if entry["kind"] != "ollama_model_store":
        continue
    dest = pathlib.Path("/var/lib/adl/issue194/model-health/model-store") / entry["relative_path"]
    print(f"mkdir -p {{shlex.quote(str(dest.parent))}}")
    print(f"timeout 900s aws s3 cp --region us-west-2 {{shlex.quote('s3://' + bucket + '/' + entry['key'])}} {{shlex.quote(str(dest))}} >/dev/null")
    print(f"echo {{shlex.quote(entry['sha256'])}}  {{shlex.quote(str(dest))}} | sha256sum -c - >/dev/null")
PY
sh "${{ADL_ISSUE194_BUNDLE}}/download-model-store.sh"
python3 - <<'PY' > "${{ADL_ISSUE194_BUNDLE}}/download-runtime.sh"
import json
import pathlib
import shlex

manifest = json.loads(pathlib.Path("/var/lib/adl/issue194/model-health/artifact-manifest.json").read_text())
bucket = manifest["bucket"]
runtime = next(entry for entry in manifest["artifacts"] if entry["kind"] == "ollama_runtime")
dest = pathlib.Path("/var/lib/adl/issue194/model-health/runtime") / runtime["relative_path"]
print(f"mkdir -p {{shlex.quote(str(dest.parent))}}")
print(f"timeout 900s aws s3 cp --region us-west-2 {{shlex.quote('s3://' + bucket + '/' + runtime['key'])}} {{shlex.quote(str(dest))}} >/dev/null")
print(f"echo {{shlex.quote(runtime['sha256'])}}  {{shlex.quote(str(dest))}} | sha256sum -c - >/dev/null")
print(f"echo {{shlex.quote(str(dest))}}")
PY
ADL_ISSUE194_RUNTIME_TAR="$(sh "${{ADL_ISSUE194_BUNDLE}}/download-runtime.sh" | tail -1)"
tar "${{ADL_ISSUE194_TAR_ARGS[@]}}" -xf "${{ADL_ISSUE194_RUNTIME_TAR}}" -C "${{ADL_ISSUE194_RUNTIME}}"
ADL_ISSUE194_OLLAMA_BIN="$(find "${{ADL_ISSUE194_RUNTIME}}" -type f -name ollama -perm -111 | head -1)"
test -n "${{ADL_ISSUE194_OLLAMA_BIN}}"
env HOME="${{ADL_ISSUE194_HOME}}" "${{ADL_ISSUE194_OLLAMA_BIN}}" --version
pkill -f "${{ADL_ISSUE194_OLLAMA_BIN}} serve" >/dev/null 2>&1 || true
nohup env HOME="${{ADL_ISSUE194_HOME}}" "${{ADL_ISSUE194_OLLAMA_BIN}}" serve > "${{ADL_ISSUE194_BUNDLE}}/ollama.log" 2>&1 &
ADL_ISSUE194_OLLAMA_PID=$!
for attempt in $(seq 1 90); do
  if curl -fsS http://127.0.0.1:11434/api/tags > "${{ADL_ISSUE194_BUNDLE}}/tags-before.json" 2>/dev/null; then
    break
  fi
  sleep 2
done
if ! curl -fsS http://127.0.0.1:11434/api/tags > "${{ADL_ISSUE194_BUNDLE}}/tags-before.json"; then
  echo "ollama serve did not open loopback before restart; log tail follows" >&2
  tail -120 "${{ADL_ISSUE194_BUNDLE}}/ollama.log" >&2 || true
  exit 42
fi
python3 - <<'PY'
import json
import pathlib

tags = json.loads(pathlib.Path("/var/lib/adl/issue194/model-health/tags-before.json").read_text())
names = sorted(model.get("name") or model.get("model") for model in tags.get("models", []))
if "gemma4:12b" not in names:
    raise SystemExit(f"gemma4:12b not listed locally: {{names}}")
print(json.dumps({{"local_ollama_models": names}}, sort_keys=True))
PY
curl -fsS http://127.0.0.1:11434/api/generate \
  -H 'Content-Type: application/json' \
  -d '{{"model":"gemma4:12b","prompt":"Return exactly: adl-issue-194-private-model-ok","stream":false,"options":{{"num_predict":8,"temperature":0}}}}' \
  > "${{ADL_ISSUE194_BUNDLE}}/generate-before.json"
kill "${{ADL_ISSUE194_OLLAMA_PID}}"
wait "${{ADL_ISSUE194_OLLAMA_PID}}" >/dev/null 2>&1 || true
nohup env HOME="${{ADL_ISSUE194_HOME}}" "${{ADL_ISSUE194_OLLAMA_BIN}}" serve >> "${{ADL_ISSUE194_BUNDLE}}/ollama.log" 2>&1 &
ADL_ISSUE194_OLLAMA_PID=$!
for attempt in $(seq 1 90); do
  if curl -fsS http://127.0.0.1:11434/api/tags > "${{ADL_ISSUE194_BUNDLE}}/tags-after.json" 2>/dev/null; then
    break
  fi
  sleep 2
done
if ! curl -fsS http://127.0.0.1:11434/api/tags > "${{ADL_ISSUE194_BUNDLE}}/tags-after.json"; then
  echo "ollama serve did not reopen loopback after restart; log tail follows" >&2
  tail -120 "${{ADL_ISSUE194_BUNDLE}}/ollama.log" >&2 || true
  exit 43
fi
kill "${{ADL_ISSUE194_OLLAMA_PID}}"
wait "${{ADL_ISSUE194_OLLAMA_PID}}" >/dev/null 2>&1 || true
python3 - <<'PY'
import json
import pathlib

root = pathlib.Path("/var/lib/adl/issue194/model-health")
before = json.loads((root / "tags-before.json").read_text())
after = json.loads((root / "tags-after.json").read_text())
generation = json.loads((root / "generate-before.json").read_text())
def names(payload):
    return sorted(model.get("name") or model.get("model") for model in payload.get("models", []))
if "gemma4:12b" not in names(after):
    raise SystemExit("gemma4:12b missing after restart")
print(json.dumps({{
    "status": "passed",
    "model": "gemma4:12b",
    "runtime_surface": "ollama_http_loopback",
    "pre_restart_model_count": len(names(before)),
    "post_restart_model_count": len(names(after)),
    "generation_response_chars": len(generation.get("response", "")),
}}, sort_keys=True))
PY
"""
    out.write_text(json.dumps({"commands": ["bash -lc " + shlex.quote(script)]}, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
