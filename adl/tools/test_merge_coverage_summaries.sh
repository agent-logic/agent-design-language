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
agent_cmd_alias='{"branches":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"mcdc":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"functions":{"count":51,"covered":48,"percent":94.11764705882352},"instantiations":{"count":246,"covered":58,"percent":23.577235772357724},"lines":{"count":708,"covered":660,"percent":93.22033898305084},"regions":{"count":1117,"covered":1044,"notcovered":73,"percent":93.46463742166517}}'
agent_cmd_canonical='{"branches":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"mcdc":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"functions":{"count":51,"covered":44,"percent":86.27450980392157},"instantiations":{"count":90,"covered":61,"percent":67.77777777777779},"lines":{"count":708,"covered":609,"percent":86.01694915254238},"regions":{"count":1117,"covered":970,"notcovered":147,"percent":86.83974932855864}}'

cat > "$TMP/workspace.json" <<JSON
{"type":"llvm.coverage.json.export","version":"2.0.1","data":[{"files":[
  {"filename":"/repo/other/src/ignored.rs","summary":$metric_summary},
  {"filename":"/repo/adl/src/z.rs","summary":$metric_summary},
  {"filename":"/repo/adl/src/aws_remote_validation.rs","summary":$metric_summary},
  {"filename":"/repo/adl/src/bin/../aws_remote_validation.rs","summary":$metric_summary},
  {"filename":"/repo/adl/src/bin/../cli/agent_cmd.rs","summary":$agent_cmd_alias},
  {"filename":"/repo/adl/src/cli/agent_cmd.rs","summary":$agent_cmd_canonical},
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
    "/adl/src/cli/agent_cmd.rs",
    "/adl/src/z.rs",
]
if filenames != expected:
    raise SystemExit(f"unexpected ownership or sort result: {filenames!r}")
expected_totals = {
    "branches": {"count": 20, "covered": 15, "notcovered": 5, "percent": 75.0},
    "functions": {"count": 66, "covered": 58, "percent": (58 * 100.0) / 66},
    "instantiations": {"count": 251, "covered": 66, "percent": (66 * 100.0) / 251},
    "lines": {"count": 758, "covered": 700, "percent": (700 * 100.0) / 758},
    "mcdc": {"count": 8, "covered": 4, "notcovered": 4, "percent": 50.0},
    "regions": {"count": 1147, "covered": 1069, "notcovered": 78, "percent": (1069 * 100.0) / 1147},
}
agent_cmd = next(entry for entry in data[0]["files"] if entry["filename"] == "/adl/src/cli/agent_cmd.rs")
instantiations = agent_cmd["summary"]["instantiations"]
if instantiations["count"] != max(246, 90):
    raise SystemExit(f"canonical alias count difference was not preserved conservatively: {instantiations!r}")
if instantiations != {"count": 246, "covered": 61, "notcovered": 185, "percent": (61 * 100.0) / 246}:
    raise SystemExit(f"canonical aliases were not conservatively coalesced: {instantiations!r}")
for metric_name, metric in agent_cmd["summary"].items():
    if metric["covered"] > metric["count"]:
        raise SystemExit(f"coalesced {metric_name} has covered > count: {metric!r}")
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

cat > "$TMP/conflicting-alias.json" <<JSON
{"data":[{"files":[
  {"filename":"/repo/adl/src/aws_remote_validation.rs","summary":$metric_summary},
  {"filename":"/repo/adl/src/bin/../aws_remote_validation.rs","summary":$metric_without_mcdc}
],"totals":{}}]}
JSON
expect_failure conflicting-alias "$TMP/conflicting-alias.json" "$TMP/runtime.json"
grep -F "conflicting metric schema for canonical alias: /adl/src/aws_remote_validation.rs" "$TMP/conflicting-alias.stderr"

cat > "$TMP/non-summary-conflict.json" <<JSON
{"data":[{"files":[
  {"filename":"/repo/adl/src/aws_remote_validation.rs","kind":"first","summary":$metric_summary},
  {"filename":"/repo/adl/src/bin/../aws_remote_validation.rs","kind":"second","summary":$metric_summary}
],"totals":{}}]}
JSON
expect_failure non-summary-conflict "$TMP/non-summary-conflict.json" "$TMP/runtime.json"
grep -F "conflicting non-summary fields for canonical alias: /adl/src/aws_remote_validation.rs" "$TMP/non-summary-conflict.stderr"

cat > "$TMP/malformed-covered.json" <<JSON
{"data":[{"files":[
  {"filename":"/repo/adl/src/aws_remote_validation.rs","summary":$metric_summary},
  {"filename":"/repo/adl/src/bin/../aws_remote_validation.rs","summary":{"branches":{"count":4,"covered":5},"mcdc":{"count":2,"covered":1},"functions":{"count":3,"covered":2},"instantiations":{"count":1,"covered":1},"lines":{"count":10,"covered":8},"regions":{"count":6,"covered":5}}}
],"totals":{}}]}
JSON
expect_failure malformed-covered "$TMP/malformed-covered.json" "$TMP/runtime.json"
grep -F "must have integer 0 <= covered <= count" "$TMP/malformed-covered.stderr"

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
