#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)
wrapper="$ROOT/adl/tools/run_issue268_six_hour_spot_qualification.sh"
spot="$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh"
soak="$ROOT/adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs"
validator="$ROOT/adl/tools/validate_v092_runtime_guardian_lifecycle.sh"

for file in "$wrapper" "$spot" "$soak" "$validator"; do
  [[ -f "$file" ]] || { echo "missing #268 file: $file" >&2; exit 1; }
done

python3 - "$wrapper" "$spot" "$soak" "$validator" <<'PY'
import pathlib, sys
w,s,r,v=(pathlib.Path(p).read_text() for p in sys.argv[1:])
required_wrapper=("authorized-usd20-20260817","estimated_max_cost_microusd\": 20000000","timeout_seconds\": 25200","--max-spot-retries","remaining_task_instances","ADL_ISSUE268_REPORT_BEGIN","#269")
for marker in required_wrapper:
    if marker not in w: raise SystemExit(f"wrapper marker missing: {marker}")
for marker in ("SIX_HOUR_MINIMUM_SECONDS: u64 = 21_600","SIX_HOUR_MAX_OVERSHOOT_SECONDS: u64 = 600","tokio::time::timeout_at","six_hour_qualification"):
    if marker not in r: raise SystemExit(f"suite marker missing: {marker}")
if '--max-spot-retries "$MAX_SPOT_RETRIES"' not in s: raise SystemExit("Spot retry forwarding missing")
if 'overshoot > 600' not in v: raise SystemExit("six-hour receipt validation missing")
print("PASS: issue268 contract markers")
PY

if ADL_ISSUE268_AUTHORIZATION=wrong "$wrapper" authorized-launch >/dev/null 2>&1; then
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
cat >"$test_root/launch-claimed.json" <<EOF
{"revision":"$revision","run_id":"issue268-six-hour-20260817","schema":"adl.issue268.launch_claim.v1"}
EOF
cat >"$test_root/owner" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"${ADL_ISSUE268_FAKE_OWNER_LOG:?}"
case "$1" in
  status)
    if [[ "${ADL_ISSUE268_FAKE_MANAGER_STATE:-dead}" == active ]]; then
      printf 'status=running run_id=issue268-six-hour-20260817\n'
      exit 0
    fi
    printf 'status=incomplete run_id=issue268-six-hour-20260817 action=inspect_logs_or_cleanup\n'
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
  "$wrapper" authorized-launch >/dev/null

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
[[ $(grep -c 'Name=tag:adl:run_id,Values=issue268-six-hour-20260817' "$test_root/aws.log") == 2 ]]
: >"$test_root/aws.log"

cat >"$test_root/summary.json" <<'EOF'
{
  "issue": 268,
  "run_id": "issue268-six-hour-20260817",
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
[[ $(grep -c 'Name=tag:adl:run_id,Values=issue268-six-hour-20260817' "$test_root/aws.log") == 1 ]]

changed=$(git -C "$ROOT" diff --name-only HEAD -- && git -C "$ROOT" ls-files --others --exclude-standard)
python3 - "$changed" <<'PY'
import sys
allowed={
"adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs",
"adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
"adl/tools/run_aws_spot_remote_validation_lane.sh",
"adl/tools/run_issue268_six_hour_spot_qualification.sh",
"adl/tools/test_run_issue268_six_hour_spot_qualification.sh",
}
for path in filter(None,sys.argv[1].splitlines()):
    if path.startswith((".csdlc/issues/268/",".csdlc/prepared/issues/268/",".csdlc/evidence/268/")) or path==".csdlc/locks/268.lock": continue
    if path not in allowed: raise SystemExit(f"out-of-scope #268 path: {path}")
print("PASS: issue268 exact scope")
PY

echo "PASS: issue268 six-hour Spot qualification contracts"
