#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/merge_coverage_summaries.py"
TEMP_BASE="${ADL_TEST_TMP_ROOT:-${TMPDIR:-$ROOT_DIR/.adl/tmp}}"
mkdir -p "$TEMP_BASE"
TMP="$(mktemp -d "$TEMP_BASE/merge-coverage-summaries.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT

metric_summary='{"branches":{"count":4,"covered":3},"mcdc":{"count":2,"covered":1},"functions":{"count":3,"covered":2},"instantiations":{"count":1,"covered":1},"lines":{"count":10,"covered":8},"regions":{"count":6,"covered":5}}'
metric_without_mcdc='{"branches":{"count":4,"covered":3},"functions":{"count":3,"covered":2},"instantiations":{"count":1,"covered":1},"lines":{"count":10,"covered":8},"regions":{"count":6,"covered":5}}'

cat > "$TMP/workspace.json" <<JSON
{"type":"llvm.coverage.json.export","version":"2.0.1","data":[{"files":[
  {"filename":"/repo/other/src/ignored.rs","summary":$metric_summary},
  {"filename":"/repo/adl/src/z.rs","summary":$metric_summary},
  {"filename":"/repo/adl/src/bin/../aws_remote_validation.rs","summary":$metric_summary},
  {"filename":"adl/src/a.rs","summary":$metric_summary},
  {"filename":"/repo/adl-runtime/src/dependency.rs","summary":$metric_summary}
],"totals":{"lines":{"count":999,"covered":999}}}]}
JSON
cat > "$TMP/runtime.json" <<JSON
{"type":"llvm.coverage.json.export","version":"2.0.1","data":[{"files":[
  {"filename":"/repo/adl/src/dependency.rs","summary":$metric_summary},
  {"filename":"adl-runtime/src/b.rs","summary":$metric_without_mcdc},
  {"filename":"/repo/adl-runtime/src/a.rs","summary":$metric_summary}
],"totals":{"lines":{"count":999,"covered":0}}}]}
JSON

output="$TMP/merged.json"
python3 "$SCRIPT" --workspace "$TMP/workspace.json" --adl-runtime "$TMP/runtime.json" --output "$output"
python3 - "$output" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as stream:
    document = json.load(stream)
data = document["data"]
if len(data) != 1:
    raise SystemExit("merged summary must have one data document")
filenames = [entry["filename"] for entry in data[0]["files"]]
expected = [
    "/adl-runtime/src/a.rs",
    "/adl-runtime/src/b.rs",
    "/adl/src/a.rs",
    "/adl/src/aws_remote_validation.rs",
    "/adl/src/z.rs",
]
if filenames != expected:
    raise SystemExit(f"unexpected ownership or sort result: {filenames!r}")
expected_totals = {
    "branches": {"count": 20, "covered": 15, "notcovered": 5, "percent": 75.0},
    "functions": {"count": 15, "covered": 10, "percent": 200 / 3},
    "instantiations": {"count": 5, "covered": 5, "percent": 100.0},
    "lines": {"count": 50, "covered": 40, "percent": 80.0},
    "mcdc": {"count": 8, "covered": 4, "notcovered": 4, "percent": 50.0},
    "regions": {"count": 30, "covered": 25, "notcovered": 5, "percent": 250 / 3},
}
if data[0]["totals"] != expected_totals:
    raise SystemExit(f"totals were not recomputed from retained files: {data[0]['totals']!r}")
payload = open(path, "rb").read()
if not payload.endswith(b"\n") or payload.endswith(b"\n\n"):
    raise SystemExit("merged JSON must have exactly one trailing newline")
PY

cp "$output" "$TMP/first.json"
python3 "$SCRIPT" --workspace "$TMP/workspace.json" --adl-runtime "$TMP/runtime.json" --output "$output"
cmp "$TMP/first.json" "$output"

expect_failure() {
  local label="$1"
  local workspace_path="$2"
  local runtime_path="$3"
  printf 'preserve-on-%s\n' "$label" > "$output"
  cp "$output" "$TMP/expected-output"
  if python3 "$SCRIPT" --workspace "$workspace_path" --adl-runtime "$runtime_path" --output "$output" >"$TMP/$label.stdout" 2>"$TMP/$label.stderr"; then
    echo "expected merge failure for $label" >&2
    exit 1
  fi
  cmp "$TMP/expected-output" "$output"
  if find "$TMP" -maxdepth 1 -name '.merged.json.*' -print -quit | grep . >/dev/null; then
    echo "atomic merge left a temporary file after $label failure" >&2
    exit 1
  fi
}

cat > "$TMP/duplicate.json" <<JSON
{"data":[{"files":[
  {"filename":"adl/src/a.rs","summary":$metric_summary},
  {"filename":"adl/src/a.rs","summary":$metric_summary}
],"totals":{}}]}
JSON
expect_failure duplicate "$TMP/duplicate.json" "$TMP/runtime.json"

cat > "$TMP/malformed.json" <<'JSON'
{"data":[{"files":[{"filename":"adl/src/a.rs","summary":{}}],"totals":{}}]}
JSON
expect_failure malformed "$TMP/malformed.json" "$TMP/runtime.json"

cat > "$TMP/multiple-data.json" <<'JSON'
{"data":[{},{}]}
JSON
expect_failure multiple-data "$TMP/multiple-data.json" "$TMP/runtime.json"

cat > "$TMP/empty-ownership.json" <<JSON
{"data":[{"files":[{"filename":"other/src/a.rs","summary":$metric_summary}],"totals":{}}]}
JSON
expect_failure empty-ownership "$TMP/empty-ownership.json" "$TMP/runtime.json"

expect_failure missing "$TMP/does-not-exist.json" "$TMP/runtime.json"

cat > "$TMP/owned-root-escape.json" <<JSON
{"data":[{"files":[{"filename":"/repo/adl/src/../../outside.rs","summary":$metric_summary}],"totals":{}}]}
JSON
expect_failure owned-root-escape "$TMP/owned-root-escape.json" "$TMP/runtime.json"

cat > "$TMP/relative-root-escape.json" <<JSON
{"data":[{"files":[{"filename":"../adl/src/x.rs","summary":$metric_summary}],"totals":{}}]}
JSON
expect_failure relative-root-escape "$TMP/relative-root-escape.json" "$TMP/runtime.json"

cat > "$TMP/absolute-prefix-escape.json" <<JSON
{"data":[{"files":[{"filename":"/repo/../adl/src/x.rs","summary":$metric_summary}],"totals":{}}]}
JSON
expect_failure absolute-prefix-escape "$TMP/absolute-prefix-escape.json" "$TMP/runtime.json"

cat > "$TMP/relative-prefix-escape.json" <<JSON
{"data":[{"files":[{"filename":"repo/../adl/src/x.rs","summary":$metric_summary}],"totals":{}}]}
JSON
expect_failure relative-prefix-escape "$TMP/relative-prefix-escape.json" "$TMP/runtime.json"

cat > "$TMP/normalized-outside-owned-root.json" <<JSON
{"data":[{"files":[{"filename":"/repo/adl/src/../outside.rs","summary":$metric_summary}],"totals":{}}]}
JSON
expect_failure normalized-outside-owned-root "$TMP/normalized-outside-owned-root.json" "$TMP/runtime.json"

echo "PASS test_merge_coverage_summaries"
