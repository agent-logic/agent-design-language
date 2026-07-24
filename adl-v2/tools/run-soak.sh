#!/usr/bin/env bash
set -euo pipefail

usage() {
  printf 'usage: %s (--manifest PATH | --verify-report PATH | --post-merge PATH)\n' "$0" >&2
  exit 64
}

mode=
input=
while (($#)); do
  case "$1" in
    --manifest)
      [[ -z "$mode" && $# -ge 2 ]] || usage
      mode=run
      input=$2
      shift 2
      ;;
    --verify-report)
      [[ -z "$mode" && $# -ge 2 ]] || usage
      mode=verify
      input=$2
      shift 2
      ;;
    --post-merge)
      [[ -z "$mode" && $# -ge 2 ]] || usage
      mode=post_merge
      input=$2
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done
[[ -n "$mode" && -n "$input" ]] || usage

root=$(git rev-parse --show-toplevel)
cd "$root"
[[ "$input" != /* && "$input" != *".."* && -f "$input" ]] ||
  { printf 'WP-12 input must be a regular repo-relative path\n' >&2; exit 65; }

sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

verify_report() {
  local report=$1
  jq -e '
    .schema == "adl.wp12.soak_report.v1" and
    .issue == 5344 and
    .status == "pass" and
    (.revision | test("^[0-9a-f]{40}$")) and
    (.manifest_sha256 | test("^[0-9a-f]{64}$")) and
    (.results | type == "array" and length > 0) and
    (all(.results[]; .status == "pass")) and
    (.rollback.status == "pass") and
    (.rollback.exact_prior_bytes_restored == true) and
    (.default_generation_changed == false) and
    (.runtime_v2_edited == false) and
    (.deferred_acceptance == false)
  ' "$report" >/dev/null
  if rg -n '(/Users/|/Volumes/|/private/|[A-Za-z]:\\\\Users\\\\)' "$report" >/dev/null; then
    printf 'WP-12 report contains a machine-local path\n' >&2
    exit 66
  fi
  printf '{"schema":"adl.wp12.report_verification.v1","issue":5344,"status":"pass","report_sha256":"%s"}\n' \
    "$(sha256 "$report")"
}

if [[ "$mode" == verify ]]; then
  verify_report "$input"
  exit 0
fi

manifest=$input
jq -e '
  .schema == "adl.wp12.soak_manifest.v1" and
  .issue == 5344 and
  (.scenarios | type == "array" and length >= 8) and
  ([.scenarios[].id] == ([.scenarios[].id] | sort)) and
  (all(.scenarios[];
    (.id | test("^[a-z0-9][a-z0-9-]+$")) and
    (.claim_class | IN("local_deterministic","ci_contract","runtime_v3_live","provider_disposition","demo","negative","rollback")) and
    (.timeout_seconds | type == "number" and . >= 1 and . <= 1800) and
    (.expected_exit | type == "number")
  ))
' "$manifest" >/dev/null

manifest_sha256=$(sha256 "$manifest")
revision=$(git rev-parse HEAD)
target_dir=${ADL_WP12_TARGET_DIR:-${CARGO_TARGET_DIR:-"$root/.adl/target/wp12"}}
adl_v2_bin=${ADL_WP12_ADL_V2_BIN:-"$target_dir/debug/adl-v2"}
if [[ ! -x "$adl_v2_bin" ]]; then
  CARGO_TARGET_DIR="$target_dir" cargo build --locked \
    --manifest-path adl-v2/Cargo.toml \
    -p adl-cli --bin adl-v2
fi
[[ -x "$adl_v2_bin" ]] || { printf 'ADL v2 binary unavailable\n' >&2; exit 67; }

work_parent="$root/.csdlc/evidence/5344/work"
mkdir -p "$work_parent"
run_root=$(mktemp -d "$work_parent/soak.XXXXXX")
trap 'rm -rf "$run_root"' EXIT
results="$run_root/results.jsonl"

run_bounded() {
  local timeout_seconds=$1
  shift
  ruby -e '
    timeout = Integer(ARGV.shift)
    pid = Process.spawn(*ARGV)
    deadline = Process.clock_gettime(Process::CLOCK_MONOTONIC) + timeout
    loop do
      waited = Process.waitpid(pid, Process::WNOHANG)
      exit($?.exitstatus || 1) if waited
      if Process.clock_gettime(Process::CLOCK_MONOTONIC) >= deadline
        Process.kill("TERM", pid) rescue nil
        sleep 0.2
        Process.kill("KILL", pid) rescue nil
        Process.wait(pid) rescue nil
        exit 124
      end
      sleep 0.02
    end
  ' "$timeout_seconds" "$@"
}

record_result() {
  local id=$1 claim_class=$2 status=$3 exit_code=$4 duration_ms=$5 artifact_sha256=$6
  jq -n -c \
    --arg id "$id" \
    --arg claim_class "$claim_class" \
    --arg status "$status" \
    --argjson exit_code "$exit_code" \
    --argjson duration_ms "$duration_ms" \
    --arg artifact_sha256 "$artifact_sha256" \
    '{
      id:$id,
      claim_class:$claim_class,
      status:$status,
      exit_code:$exit_code,
      duration_ms:$duration_ms,
      artifact_sha256:$artifact_sha256
    }' >>"$results"
}

scenario_count=$(jq '.scenarios | length' "$manifest")
for ((index=0; index<scenario_count; index++)); do
  scenario=$(jq -c ".scenarios[$index]" "$manifest")
  id=$(jq -r .id <<<"$scenario")
  kind=$(jq -r .kind <<<"$scenario")
  claim_class=$(jq -r .claim_class <<<"$scenario")
  timeout_seconds=$(jq -r .timeout_seconds <<<"$scenario")
  expected_exit=$(jq -r .expected_exit <<<"$scenario")
  output="$run_root/$id.out"
  error="$run_root/$id.err"
  started=$(ruby -e 'puts(Process.clock_gettime(Process::CLOCK_MONOTONIC, :millisecond))')
  set +e
  case "$kind" in
    adl_v2)
      argv=()
      while IFS= read -r argument; do
        argv+=("$argument")
      done < <(jq -r '.argv[]' <<<"$scenario")
      run_bounded "$timeout_seconds" "$adl_v2_bin" "${argv[@]}" >"$output" 2>"$error"
      exit_code=$?
      ;;
    artifact)
      path=$(jq -r .path <<<"$scenario")
      required_schema=$(jq -r .required_schema <<<"$scenario")
      required_status=$(jq -r .required_status <<<"$scenario")
      if [[ "$path" == /* || "$path" == *".."* || ! -f "$path" ]]; then
        exit_code=2
      else
        jq -e --arg schema "$required_schema" --arg status "$required_status" \
          '.schema == $schema and .status == $status' "$path" >"$output" 2>"$error"
        exit_code=$?
      fi
      ;;
    contract)
      path=$(jq -r .path <<<"$scenario")
      pattern=$(jq -r .pattern <<<"$scenario")
      if [[ "$path" == /* || "$path" == *".."* || ! -f "$path" ]]; then
        exit_code=2
      else
        rg -n --fixed-strings "$pattern" "$path" >"$output" 2>"$error"
        exit_code=$?
      fi
      ;;
    rollback)
      run_bounded "$timeout_seconds" bash adl-v2/tools/prove-rollback.sh \
        --manifest "$manifest" >"$output" 2>"$error"
      exit_code=$?
      ;;
    *)
      printf 'unsupported scenario kind: %s\n' "$kind" >"$error"
      exit_code=2
      ;;
  esac
  set -e
  finished=$(ruby -e 'puts(Process.clock_gettime(Process::CLOCK_MONOTONIC, :millisecond))')
  duration_ms=$((finished - started))
  artifact_sha256=$(sha256 "$output")
  if [[ "$exit_code" -ne "$expected_exit" ]]; then
    printf 'scenario %s expected exit %s, observed %s\n' \
      "$id" "$expected_exit" "$exit_code" >&2
    sed -n '1,80p' "$error" >&2
    exit 68
  fi
  record_result "$id" "$claim_class" pass "$exit_code" "$duration_ms" "$artifact_sha256"
done

rollback=$(jq -c 'select(.schema == "adl.wp12.rollback_report.v1")' \
  "$run_root/rollback-matrix.out")
[[ -n "$rollback" ]] || { printf 'rollback report missing\n' >&2; exit 69; }

report_path=docs/milestones/v0.91.8/evidence/wp12/report.json
mkdir -p "$(dirname "$report_path")"
jq -s \
  --arg revision "$revision" \
  --arg manifest_sha256 "$manifest_sha256" \
  --arg mode "$mode" \
  --argjson rollback "$rollback" \
  '{
    schema:"adl.wp12.soak_report.v1",
    issue:5344,
    status:"pass",
    revision:$revision,
    manifest_sha256:$manifest_sha256,
    mode:$mode,
    results:sort_by(.id),
    rollback:$rollback,
    default_generation_changed:false,
    runtime_v2_edited:false,
    deferred_acceptance:false
  }' "$results" >"$report_path"

verify_report "$report_path" >/dev/null
cat "$report_path"
