#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)
wrapper="$ROOT/adl/tools/run_issue268_six_hour_spot_qualification.sh"
spot="$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh"
soak="$ROOT/adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs"
validator="$ROOT/adl/tools/validate_v092_runtime_guardian_lifecycle.sh"
uts_plan="$ROOT/adl/tools/issue268_six_resident_uts_plan.json"
uts_plan_validator="$ROOT/adl/tools/validate_issue268_six_resident_uts_plan.py"

for file in "$wrapper" "$spot" "$soak" "$validator" "$uts_plan" "$uts_plan_validator"; do
  [[ -f "$file" ]] || { echo "missing #268 file: $file" >&2; exit 1; }
done

python3 - "$wrapper" "$spot" "$soak" "$validator" <<'PY'
import pathlib, sys
w,s,r,v=(pathlib.Path(p).read_text() for p in sys.argv[1:])
required_wrapper=("authorized-usd20-20260817","estimated_max_cost_microusd\": 20000000","timeout_seconds\": 25200","--max-spot-retries","--runtime-continuity-volume-id","remaining_task_instances","ADL_ISSUE268_REPORT_BEGIN","r7i.2xlarge","#269")
for marker in required_wrapper:
    if marker not in w: raise SystemExit(f"wrapper marker missing: {marker}")
for marker in ("SIX_HOUR_MINIMUM_SECONDS: u64 = 21_600","SIX_HOUR_MAX_OVERSHOOT_SECONDS: u64 = 600","tokio::time::timeout_at","six_hour_qualification"):
    if marker not in r: raise SystemExit(f"suite marker missing: {marker}")
if '--max-spot-retries "$MAX_SPOT_RETRIES"' not in s: raise SystemExit("Spot retry forwarding missing")
if 'overshoot > 600' not in v: raise SystemExit("six-hour receipt validation missing")
print("PASS: issue268 contract markers")
PY

python3 "$uts_plan_validator" >/dev/null
python3 "$ROOT/adl/tools/test_run_issue268_six_resident_uts_cycle.py" >/dev/null
python3 "$ROOT/adl/tools/test_run_issue268_continuity_uts_qualification.py" >/dev/null
python3 "$ROOT/adl/tools/test_materialize_issue268_ollama_plan.py" >/dev/null
bash "$ROOT/adl/tools/test_run_issue268_remote_resident_qualification.sh" >/dev/null

if ADL_ISSUE268_AUTHORIZATION=wrong \
  ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID=vol-12345678 \
  ADL_AWS_RUNTIME_CONTINUITY_VOLUME_NAME=adl-issue268-runtime \
  ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID_SHA256="$(python3 -c 'import hashlib; print(hashlib.sha256(b"vol-12345678").hexdigest())')" \
  "$wrapper" authorized-launch >/dev/null 2>&1; then
  echo "invalid authorization unexpectedly reached launch" >&2
  exit 1
fi

unit_log=$(mktemp "$ROOT/.adl/issue268-unit.XXXXXX")
test_root=$(mktemp -d "$ROOT/.adl/issue268-wrapper.XXXXXX")
trap 'rm -f "$unit_log"; rm -rf "$test_root"' EXIT
cargo test --locked --manifest-path "$ROOT/adl-runtime/Cargo.toml" \
  --bin adl-runtime-lifecycle-soak \
  tests::accepts_the_fixed_six_hour_qualification_without_duration_input \
  -- --exact --nocapture >"$unit_log" 2>&1
grep -F 'running 1 test' "$unit_log" >/dev/null
grep -F 'test result: ok. 1 passed; 0 failed; 0 ignored;' "$unit_log" >/dev/null

revision=$(git -C "$ROOT" rev-parse HEAD)
mkdir -p "$test_root/artifacts/attempt-0"
cat >"$test_root/owner" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"${ADL_ISSUE268_FAKE_OWNER_LOG:?}"
case "$1" in
  run)
    [[ " $* " == *" --run "* ]] || exit 98
    exit 0
    ;;
  status)
    if [[ "${ADL_ISSUE268_FAKE_MANAGER_STATE:-dead}" == active ]]; then
      printf 'status=running run_id=issue268-six-hour-r7i-20260819-24\n'
      exit 0
    fi
    printf 'status=incomplete run_id=issue268-six-hour-r7i-20260819-24 action=inspect_logs_or_cleanup\n'
    exit 1
    ;;
  cleanup) exit 0 ;;
  *) echo "unexpected owner mutation" >&2; exit 99 ;;
esac
EOF
cat >"$test_root/aws" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"${ADL_ISSUE268_FAKE_AWS_LOG:?}"
if [[ "$*" == *"describe-instances"* ]]; then
  count_file="${ADL_ISSUE268_FAKE_AWS_COUNT:?}"
  count=$(cat "$count_file" 2>/dev/null || printf 0)
  if [[ "$count" == 0 ]]; then
    printf '1\n' >"$count_file"
    printf '["i-test-owned"]\n'
  else
    printf '[]\n'
  fi
else
  printf '{}\n'
fi
EOF
chmod +x "$test_root/owner" "$test_root/aws"

ADL_ISSUE268_EVIDENCE_ROOT="$test_root" \
ADL_ISSUE268_OWNER="$test_root/owner" \
ADL_ISSUE268_FAKE_OWNER_LOG="$test_root/owner.log" \
ADL_ISSUE268_AUTHORIZATION=authorized-usd20-20260817 \
ADL_ISSUE268_ESTIMATED_HOURLY_COST_USD=0.1763 \
ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID=vol-12345678 \
ADL_AWS_RUNTIME_CONTINUITY_VOLUME_NAME=adl-issue268-runtime \
ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID_SHA256="$(python3 -c 'import hashlib; print(hashlib.sha256(b"vol-12345678").hexdigest())')" \
  "$wrapper" authorized-launch >/dev/null
python3 - "$test_root/portable-request.json" <<'PY'
import json, pathlib, sys
request=json.loads(pathlib.Path(sys.argv[1]).read_text())
assert request["request_id"] == "issue268-six-hour-r7i-20260819-24"
assert "HOME" in request["command_profile"]["environment_allowlist"]
assert "ADL_RUN_ID" in request["command_profile"]["environment_allowlist"]
assert request["command_profile"]["argv"] == ["bash", "adl/tools/run_issue268_remote_resident_qualification.sh"]
assert request["resource_budget"] == {
    "cpu_cores": 8,
    "memory_mib": 65536,
    "timeout_seconds": 25200,
    "estimated_max_cost_microusd": 20000000,
}
assert request["fallback"] == "disabled"
assert "ADL_ISSUE268_S3_SOURCE_RECEIPT" in request["command_profile"]["environment_allowlist"]
PY
grep -F -- '--runtime-continuity-volume-id vol-12345678' "$test_root/owner.log" >/dev/null
grep -F -- '--runtime-continuity-volume-name adl-issue268-runtime' "$test_root/owner.log" >/dev/null
grep -F -- 'run --run ' "$test_root/owner.log" >/dev/null

if ADL_ISSUE268_EVIDENCE_ROOT="$test_root" \
  ADL_ISSUE268_OWNER="$test_root/owner" \
  ADL_ISSUE268_AWS_CLI="$test_root/aws" \
  ADL_ISSUE268_FAKE_OWNER_LOG="$test_root/owner.log" \
  ADL_ISSUE268_FAKE_MANAGER_STATE=active \
  ADL_ISSUE268_FAKE_AWS_LOG="$test_root/aws.log" \
  ADL_ISSUE268_FAKE_AWS_COUNT="$test_root/aws.count" \
    "$wrapper" validate >/dev/null 2>&1; then
  echo "active manager validation unexpectedly succeeded" >&2
  exit 1
fi
grep -F 'status --profile agent-logic-admin --region us-west-2' "$test_root/owner.log" >/dev/null
[[ ! -s "$test_root/aws.log" ]]
if grep -F 'cleanup ' "$test_root/owner.log" >/dev/null; then
  echo "active manager validation invoked cleanup" >&2
  exit 1
fi
: >"$test_root/owner.log"

if ADL_ISSUE268_EVIDENCE_ROOT="$test_root" \
  ADL_ISSUE268_OWNER="$test_root/owner" \
  ADL_ISSUE268_AWS_CLI="$test_root/aws" \
  ADL_ISSUE268_FAKE_OWNER_LOG="$test_root/owner.log" \
  ADL_ISSUE268_FAKE_AWS_LOG="$test_root/aws.log" \
  ADL_ISSUE268_FAKE_AWS_COUNT="$test_root/aws.count" \
    "$wrapper" validate >/dev/null 2>&1; then
  echo "manager-death recovery unexpectedly claimed receipt validation" >&2
  exit 1
fi
grep -F -- '--region us-west-2 ec2 terminate-instances --instance-ids i-test-owned' "$test_root/aws.log" >/dev/null
grep -F -- '--region us-west-2 ec2 wait instance-terminated --instance-ids i-test-owned' "$test_root/aws.log" >/dev/null
[[ $(grep -c 'Name=tag:adl:issue,Values=268' "$test_root/aws.log") == 2 ]]
[[ $(grep -c 'Name=tag:adl:run_id,Values=issue268-six-hour-r7i-20260819-24' "$test_root/aws.log") == 2 ]]
: >"$test_root/aws.log"

cat >"$test_root/summary.json" <<'EOF'
{
  "issue": 268,
  "run_id": "issue268-six-hour-r7i-20260819-24",
  "status": "passed",
  "attempts": [{"purchase_option": "spot", "status": "launched"}],
  "expected_max_cost_usd": 20.0,
  "cleanup": {"termination_attempted": true, "final_instance_state": "terminated"}
}
EOF
cat >"$test_root/artifacts/attempt-0/command-stdout.log" <<EOF
ADL_ISSUE268_REPORT_BEGIN
{"schema":"adl.runtime_v3.lifecycle_soak.v1","suite":"six_hour_qualification","revision":"$revision","minimum_exposure_seconds":21600,"measured_exposure_seconds":21607,"overshoot_seconds":7,"maximum_overshoot_seconds":600,"runtime_v3_soak":{"nested":{"status":"pass"}},"failure":null}
ADL_ISSUE268_REPORT_END
EOF
ADL_ISSUE268_EVIDENCE_ROOT="$test_root" \
ADL_ISSUE268_OWNER="$test_root/owner" \
ADL_ISSUE268_AWS_CLI="$test_root/aws" \
ADL_ISSUE268_FAKE_OWNER_LOG="$test_root/owner.log" \
ADL_ISSUE268_FAKE_AWS_LOG="$test_root/aws.log" \
ADL_ISSUE268_FAKE_AWS_COUNT="$test_root/aws.count" \
  "$wrapper" validate >/dev/null
python3 - "$test_root/validation.json" <<'PY'
import json,sys
d=json.load(open(sys.argv[1])); assert d["status"]=="pass" and d["overshoot_seconds"]==7
PY
[[ $(grep -c 'Name=tag:adl:issue,Values=268' "$test_root/aws.log") == 1 ]]
[[ $(grep -c 'Name=tag:adl:run_id,Values=issue268-six-hour-r7i-20260819-24' "$test_root/aws.log") == 1 ]]

grep -Fq 'if [[ -n "$RUNTIME_CONTINUITY_VOLUME_ID" ]]' "$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh"
grep -Fq 'Runtime continuity volume requires an explicit colocated subnet' "$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh"

changed=$(git -C "$ROOT" diff --name-only HEAD -- && git -C "$ROOT" ls-files --others --exclude-standard)
python3 - "$changed" <<'PY'
import sys
allowed={
"adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs",
"adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
"adl/tools/run_aws_spot_remote_validation_lane.sh",
"adl/tools/run_aws_spot_builder_image_validation.sh",
"adl/tools/test_run_aws_spot_builder_image_validation.sh",
"tools/aws_remote_validation/src/aws_remote_validation.rs",
"tools/aws_remote_validation/scripts/remote_validation_runner.sh",
"adl/tools/run_issue268_six_hour_spot_qualification.sh",
"adl/tools/test_run_issue268_six_hour_spot_qualification.sh",
"adl/tools/issue268_six_resident_uts_plan.json",
"adl/tools/validate_issue268_six_resident_uts_plan.py",
"adl/tools/run_issue268_six_resident_uts_cycle.py",
"adl/tools/test_run_issue268_six_resident_uts_cycle.py",
"adl/tools/uts_benchmark_runner.py",
"adl/src/provider_adapter.rs",
"adl/src/provider_communication.rs",
"adl/src/agent_comms/dispatch/coding.inc",
"adl/tools/run_issue268_continuity_uts_qualification.py",
"adl/tools/test_run_issue268_continuity_uts_qualification.py",
"adl/tools/benchmark/uts_benchmark_panel.py",
"adl/tools/benchmark/uts_benchmark_tasks.py",
"adl/tools/materialize_issue268_ollama_plan.py",
"adl/tools/test_materialize_issue268_ollama_plan.py",
"adl/tools/run_issue268_remote_resident_qualification.sh",
"adl/tools/test_run_issue268_remote_resident_qualification.sh",
"adl/tools/install_issue268_runtime_volume.py",
"adl/tools/test_install_issue268_runtime_volume.py",
}
for path in filter(None,sys.argv[1].splitlines()):
    if path.startswith((".csdlc/issues/268/",".csdlc/prepared/issues/268/",".csdlc/evidence/268/")) or path==".csdlc/locks/268.lock": continue
    if path not in allowed: raise SystemExit(f"out-of-scope #268 path: {path}")
print("PASS: issue268 exact scope")
PY

echo "PASS: issue268 six-hour Spot qualification contracts"
