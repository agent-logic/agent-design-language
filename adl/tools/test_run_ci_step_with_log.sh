#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/run_ci_step_with_log.sh"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

success_root="$tmp_dir/success"
bash "$SCRIPT" --name "Coverage run and summary (json)" --log-root "$success_root" -- \
  bash -c 'echo success-out; echo success-err >&2'

success_meta="$(find "$success_root" -name metadata.json -print -quit)"
if [ ! -s "$success_meta" ]; then
  echo "expected success metadata.json" >&2
  exit 1
fi
python3 - "$success_meta" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1]))
assert data["schema"] == "adl.ci.step_log.v1"
assert data["step_name"] == "Coverage run and summary (json)"
assert data["exit_code"] == 0
assert data["stdout_path"].endswith("stdout.log")
assert data["stderr_path"].endswith("stderr.log")
assert data["combined_path"].endswith("combined.log")
PY

if ! grep -R "success-out" "$success_root" >/dev/null 2>&1; then
  echo "expected stdout capture" >&2
  exit 1
fi
if ! grep -R "success-err" "$success_root" >/dev/null 2>&1; then
  echo "expected stderr capture" >&2
  exit 1
fi

failure_root="$tmp_dir/failure"
set +e
bash "$SCRIPT" --name "failing step" --log-root "$failure_root" -- \
  bash -c 'echo fail-out; echo fail-err >&2; exit 37'
status=$?
set -e
if [ "$status" -ne 37 ]; then
  echo "expected wrapped command exit code 37, got $status" >&2
  exit 1
fi

failure_meta="$(find "$failure_root" -name metadata.json -print -quit)"
if [ ! -s "$failure_meta" ]; then
  echo "expected failure metadata.json" >&2
  exit 1
fi
python3 - "$failure_meta" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1]))
assert data["exit_code"] == 37
PY

if ! grep -R "fail-err" "$failure_root" >/dev/null 2>&1; then
  echo "expected failing stderr capture" >&2
  exit 1
fi

secret_root="$tmp_dir/secret"
bash "$SCRIPT" --name "secret step" --log-root "$secret_root" -- \
  bash -c 'echo no-secret-output' -- --token super-secret-value
secret_meta="$(find "$secret_root" -name metadata.json -print -quit)"
if grep -R "super-secret-value" "$secret_root" >/dev/null 2>&1; then
  echo "secret-looking command argument leaked into retained logs" >&2
  exit 1
fi
python3 - "$secret_meta" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1]))
command = data["command"]
assert "<redacted>" in command
assert "super-secret-value" not in command
PY

if bash "$SCRIPT" --name missing-command --log-root "$tmp_dir/missing" >/dev/null 2>&1; then
  echo "expected missing command invocation to fail" >&2
  exit 1
fi

echo "PASS test_run_ci_step_with_log"
