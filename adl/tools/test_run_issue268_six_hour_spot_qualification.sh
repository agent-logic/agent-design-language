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
import pathlib, subprocess, sys, tempfile
w,s,r,v=(pathlib.Path(p).read_text() for p in sys.argv[1:])
required_wrapper=("authorized-on-demand-usd20-20260820","estimated_max_cost_microusd\": 20000000","timeout_seconds\": 25200","--on-demand-only","--runtime-continuity-volume-id","remaining_task_instances","ADL_ISSUE268_REPORT_BEGIN","r7i.2xlarge","#269")
for marker in required_wrapper:
    if marker not in w: raise SystemExit(f"wrapper marker missing: {marker}")
for marker in ("SIX_HOUR_MINIMUM_SECONDS: u64 = 21_600","SIX_HOUR_MAX_OVERSHOOT_SECONDS: u64 = 600","tokio::time::timeout_at","six_hour_qualification"):
    if marker not in r: raise SystemExit(f"suite marker missing: {marker}")
if '--max-spot-retries "$MAX_SPOT_RETRIES"' not in s: raise SystemExit("Spot retry forwarding missing")
if 'overshoot > 600' not in v: raise SystemExit("six-hour receipt validation missing")
if 'public_url_pattern = re.compile' not in v: raise SystemExit("structural Runtime URL adaptation missing")
if 'https://runtime.dev.agent-logic.ai:20997' in v: raise SystemExit("Guardian must not depend on public DNS")
prefix = 'python3 - "$repo_root/infra/runtime-v3/runtime-init.toml" "$init_template" "$api_port"'
if v.count(prefix) != 1: raise SystemExit("Runtime init localization block is missing or ambiguous")
snippet = v.split(prefix, 1)[1].split("<<'PY'\n", 1)[1].split("\nPY\n", 1)[0]
with tempfile.TemporaryDirectory() as tmp:
    tmp = pathlib.Path(tmp)
    source = tmp / "source.toml"
    output = tmp / "output.toml"
    repo_root = pathlib.Path(sys.argv[1]).resolve().parents[2]
    snippet = snippet.replace(
        'repo_root = source_path.parents[2]',
        f'repo_root = pathlib.Path({str(repo_root)!r})',
        1,
    )
    canonical = (repo_root / "infra/runtime-v3/runtime-init.toml").read_text()
    source.write_text(canonical)
    snippet_args = [sys.executable, "-", str(source), str(output), "34567", "34568", str(tmp / "state")]
    result = subprocess.run(snippet_args, input=snippet, text=True, capture_output=True)
    if result.returncode != 0: raise SystemExit(f"Runtime init localization failed: {result.stderr}")
    localized = output.read_text()
    if 'address = "127.0.0.1:34567"' not in localized or 'public_base_url = "https://localhost:34567"' not in localized:
        raise SystemExit("Runtime init localization produced the wrong local endpoint")
    source.write_text(canonical.replace('public_base_url = "https://runtime.dev.agent-logic.ai:20997"\n', '', 1))
    if subprocess.run(snippet_args, input=snippet, text=True, capture_output=True).returncode == 0:
        raise SystemExit("missing public URL unexpectedly localized")
    source.write_text(canonical + 'public_base_url = "https://duplicate.invalid:20997"\n')
    if subprocess.run(snippet_args, input=snippet, text=True, capture_output=True).returncode == 0:
        raise SystemExit("duplicate public URLs unexpectedly localized")
print("PASS: issue268 contract markers")
PY

python3 "$uts_plan_validator" >/dev/null
python3 "$ROOT/adl/tools/test_run_issue268_six_resident_uts_cycle.py" >/dev/null
python3 "$ROOT/adl/tools/test_run_issue268_continuity_uts_qualification.py" >/dev/null
python3 "$ROOT/adl/tools/test_materialize_issue268_ollama_plan.py" >/dev/null
bash "$ROOT/adl/tools/test_run_issue268_remote_resident_qualification.sh" >/dev/null

unit_log=$(mktemp "$ROOT/.adl/issue268-unit.XXXXXX")
test_root=$(mktemp -d "$ROOT/.adl/issue268-wrapper.XXXXXX")
trap 'rm -f "$unit_log"; rm -rf "$test_root"' EXIT
wrong_auth_root="$test_root/wrong-authorization"
if ADL_ISSUE268_EVIDENCE_ROOT="$wrong_auth_root" \
  ADL_ISSUE268_AUTHORIZATION=wrong \
  ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID=vol-12345678 \
  ADL_AWS_RUNTIME_CONTINUITY_VOLUME_NAME=adl-issue268-runtime \
  ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID_SHA256="$(python3 -c 'import hashlib; print(hashlib.sha256(b"vol-12345678").hexdigest())')" \
  "$wrapper" authorized-launch >/dev/null 2>&1; then
  echo "invalid authorization unexpectedly reached launch" >&2
  exit 1
fi
[[ ! -e "$wrong_auth_root" ]] || {
  echo "invalid authorization created isolated run evidence" >&2
  exit 1
}

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
    printf 'existing_instance=%s runtime_root=%s runtime_volume=%s\n' \
      "${ADL_AWS_EXISTING_INSTANCE_ID:-}" "${ADL_AWS_PRE_MOUNTED_RUNTIME_ROOT:-}" \
      "${ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID:-}" >>"${ADL_ISSUE268_FAKE_OWNER_LOG:?}"
    exit 0
    ;;
  status)
    if [[ "${ADL_ISSUE268_FAKE_MANAGER_STATE:-dead}" == active ]]; then
      printf 'status=running run_id=issue268-six-hour-r7i-20260821-58\n'
      exit 0
    fi
    printf 'status=incomplete run_id=issue268-six-hour-r7i-20260821-58 action=inspect_logs_or_cleanup\n'
    exit 1
    ;;
  cleanup) exit 0 ;;
  preflight)
    [[ " $* " == *" --check-account "* ]] || exit 97
    exit 0
    ;;
  *) echo "unexpected owner mutation" >&2; exit 99 ;;
esac
EOF
cat >"$test_root/aws" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"${ADL_ISSUE268_FAKE_AWS_LOG:?}"
if [[ "$*" == *"describe-subnets"* ]]; then
  if [[ "$*" == *"VpcId"* ]]; then
    printf 'vpc-12345678\n'
  else
    printf 'us-west-2a\n'
  fi
elif [[ "$*" == *"wait stack-create-complete"* && "${ADL_ISSUE268_FAKE_CREATE_WAIT_FAIL:-0}" == 1 ]]; then
  exit 42
elif [[ "$*" == *"describe-stacks"* && "$*" == *"StackStatus"* ]]; then
  printf '%s\n' "${ADL_ISSUE268_FAKE_STACK_STATUS:-}"
elif [[ "$*" == *"describe-stacks"* && "$*" == *"InstanceId"* ]]; then
  printf 'i-12345678\n'
elif [[ "$*" == *"describe-stacks"* && "$*" == *"RuntimeVolumeId"* ]]; then
  printf 'vol-12345678\n'
elif [[ "$*" == *"describe-instances"* ]]; then
  if [[ "${ADL_ISSUE268_FAKE_ACTIVE_INSTANCES:-0}" == 1 ]]; then
    printf '["i-test-owned"]\n'
    exit 0
  fi
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

if ADL_ISSUE268_EVIDENCE_ROOT="$test_root" \
  ADL_ISSUE268_OWNER="$test_root/owner" \
  ADL_ISSUE268_FAKE_OWNER_LOG="$test_root/owner.log" \
  ADL_ISSUE268_ESTIMATED_HOURLY_COST_USD=0 \
  ADL_ISSUE268_SUBNET_ID=subnet-12345678 \
  ADL_ISSUE268_AVAILABILITY_ZONE=us-west-2a \
  ADL_ISSUE268_RUNTIME_SNAPSHOT_ID=snap-12345678 \
  ADL_ISSUE268_AWS_CLI="$test_root/aws" \
  ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID=vol-12345678 \
  ADL_AWS_RUNTIME_CONTINUITY_VOLUME_NAME=adl-issue268-runtime \
  ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID_SHA256="$(python3 -c 'import hashlib; print(hashlib.sha256(b"vol-12345678").hexdigest())')" \
    "$wrapper" preflight >/dev/null 2>&1; then
  echo "zero On-Demand price unexpectedly passed preflight" >&2
  exit 1
fi
[[ ! -s "$test_root/owner.log" ]]

ADL_ISSUE268_EVIDENCE_ROOT="$test_root" \
ADL_ISSUE268_OWNER="$test_root/owner" \
ADL_ISSUE268_FAKE_OWNER_LOG="$test_root/owner.log" \
ADL_ISSUE268_AUTHORIZATION=authorized-on-demand-usd20-20260820 \
ADL_ISSUE268_ESTIMATED_HOURLY_COST_USD=0.5292 \
ADL_ISSUE268_SUBNET_ID=subnet-12345678 \
ADL_ISSUE268_AVAILABILITY_ZONE=us-west-2a \
ADL_ISSUE268_RUNTIME_SNAPSHOT_ID=snap-12345678 \
ADL_ISSUE268_AWS_CLI="$test_root/aws" \
ADL_ISSUE268_FAKE_AWS_LOG="$test_root/aws.log" \
ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID=vol-12345678 \
ADL_AWS_RUNTIME_CONTINUITY_VOLUME_NAME=adl-issue268-runtime \
ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID_SHA256="$(python3 -c 'import hashlib; print(hashlib.sha256(b"vol-12345678").hexdigest())')" \
  "$wrapper" authorized-launch >/dev/null
python3 - "$test_root/portable-request.json" <<'PY'
import json, pathlib, sys
request=json.loads(pathlib.Path(sys.argv[1]).read_text())
assert request["request_id"] == "issue268-six-hour-r7i-20260821-58"
assert "HOME" in request["command_profile"]["environment_allowlist"]
assert "ADL_RUN_ID" in request["command_profile"]["environment_allowlist"]
assert "ADL_ISSUE414_SIGNING_KEY_HEX" in request["command_profile"]["environment_allowlist"]
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
grep -F -- '--runtime-continuity-volume-name issue268-six-hour-r7i-20260821-58-runtime' "$test_root/owner.log" >/dev/null
grep -F -- 'preflight --check-account --profile agent-logic-admin --region us-west-2' "$test_root/owner.log" >/dev/null
grep -F -- 'run --run ' "$test_root/owner.log" >/dev/null
grep -F -- '--on-demand-only' "$test_root/owner.log" >/dev/null
grep -F -- 'existing_instance=i-12345678 runtime_root=/opt/adl-runtime runtime_volume=vol-12345678' "$test_root/owner.log" >/dev/null
python3 - "$test_root/aws.log" <<'PY'
import pathlib, sys
lines=pathlib.Path(sys.argv[1]).read_text().splitlines()
need=("cloudformation validate-template", "cloudformation create-stack", "cloudformation wait stack-create-complete", "cloudformation describe-stacks", "cloudformation delete-stack", "cloudformation wait stack-delete-complete")
positions=[]
for marker in need:
    positions.append(next(i for i,line in enumerate(lines) if marker in line))
assert positions == sorted(positions), (need, positions)
assert sum("cloudformation create-stack" in line for line in lines) == 1
assert any("ParameterKey=VpcId,ParameterValue=vpc-12345678" in line for line in lines)
PY

failure_root=$(mktemp -d "$ROOT/.adl/issue268-wrapper-create-failure.XXXXXX")
failure_owner_log="$failure_root/owner.log"
failure_aws_log="$failure_root/aws.log"
if ADL_ISSUE268_EVIDENCE_ROOT="$failure_root" \
  ADL_ISSUE268_OWNER="$test_root/owner" \
  ADL_ISSUE268_FAKE_OWNER_LOG="$failure_owner_log" \
  ADL_ISSUE268_AUTHORIZATION=authorized-on-demand-usd20-20260820 \
  ADL_ISSUE268_ESTIMATED_HOURLY_COST_USD=0.5292 \
  ADL_ISSUE268_SUBNET_ID=subnet-12345678 \
  ADL_ISSUE268_AVAILABILITY_ZONE=us-west-2a \
  ADL_ISSUE268_RUNTIME_SNAPSHOT_ID=snap-12345678 \
  ADL_ISSUE268_AWS_CLI="$test_root/aws" \
  ADL_ISSUE268_FAKE_AWS_LOG="$failure_aws_log" \
  ADL_ISSUE268_FAKE_CREATE_WAIT_FAIL=1 \
    "$wrapper" authorized-launch >/dev/null 2>&1; then
  echo "CloudFormation create-wait failure unexpectedly passed" >&2
  exit 1
fi
python3 - "$failure_aws_log" <<'PY'
import pathlib, sys
lines=pathlib.Path(sys.argv[1]).read_text().splitlines()
create_wait=next(i for i,line in enumerate(lines) if "cloudformation wait stack-create-complete" in line)
delete=next(i for i,line in enumerate(lines) if "cloudformation delete-stack" in line)
delete_wait=next(i for i,line in enumerate(lines) if "cloudformation wait stack-delete-complete" in line)
assert create_wait < delete < delete_wait
assert not any("cloudformation describe-stacks" in line for line in lines)
PY
[[ ! -s "$failure_owner_log" ]]
rm -rf "$failure_root"

if ADL_ISSUE268_EVIDENCE_ROOT="$test_root" \
  ADL_ISSUE268_OWNER="$test_root/owner" \
  ADL_ISSUE268_AWS_CLI="$test_root/aws" \
  ADL_ISSUE268_FAKE_OWNER_LOG="$test_root/owner.log" \
  ADL_ISSUE268_FAKE_MANAGER_STATE=active \
  ADL_ISSUE268_FAKE_AWS_LOG="$test_root/aws.log" \
  ADL_ISSUE268_FAKE_ACTIVE_INSTANCES=1 \
    "$wrapper" validate >/dev/null 2>&1; then
  echo "active manager validation unexpectedly succeeded" >&2
  exit 1
fi
grep -F 'cloudformation describe-stacks --stack-name adl-issue268-runtime-58' "$test_root/aws.log" >/dev/null
[[ ! -s "$test_root/owner.log" ]]
if grep -F 'cleanup ' "$test_root/owner.log" >/dev/null; then
  echo "active manager validation invoked cleanup" >&2
  exit 1
fi
: >"$test_root/owner.log"
: >"$test_root/aws.log"

if ADL_ISSUE268_EVIDENCE_ROOT="$test_root" \
  ADL_ISSUE268_OWNER="$test_root/owner" \
  ADL_ISSUE268_AWS_CLI="$test_root/aws" \
  ADL_ISSUE268_FAKE_OWNER_LOG="$test_root/owner.log" \
  ADL_ISSUE268_FAKE_AWS_LOG="$test_root/aws.log" \
  ADL_ISSUE268_FAKE_STACK_STATUS=CREATE_COMPLETE \
    "$wrapper" validate >/dev/null 2>&1; then
  echo "active CloudFormation stack validation unexpectedly succeeded" >&2
  exit 1
fi
grep -F 'cloudformation describe-stacks --stack-name adl-issue268-runtime-58' "$test_root/aws.log" >/dev/null
[[ ! -s "$test_root/owner.log" ]]
if grep -F 'terminate-instances' "$test_root/aws.log" >/dev/null; then
  echo "active CloudFormation validation terminated an instance" >&2
  exit 1
fi
: >"$test_root/aws.log"

if ADL_ISSUE268_EVIDENCE_ROOT="$test_root" \
  ADL_ISSUE268_OWNER="$test_root/owner" \
  ADL_ISSUE268_AWS_CLI="$test_root/aws" \
  ADL_ISSUE268_FAKE_OWNER_LOG="$test_root/owner.log" \
  ADL_ISSUE268_FAKE_AWS_LOG="$test_root/aws.log" \
  ADL_ISSUE268_FAKE_ACTIVE_INSTANCES=1 \
    "$wrapper" validate >/dev/null 2>"$test_root/validate.err"; then
  echo "manager-death recovery unexpectedly claimed receipt validation" >&2
  exit 1
fi
grep -F 'task-owned instance is active without terminal summary' "$test_root/validate.err" >/dev/null
if grep -F 'terminate-instances' "$test_root/aws.log" >/dev/null; then
  echo "active instance validation terminated an instance" >&2
  exit 1
fi
[[ $(grep -c 'Name=tag:adl:issue,Values=268' "$test_root/aws.log") == 1 ]]
[[ $(grep -c 'Name=tag:adl:run_id,Values=issue268-six-hour-r7i-20260821-58' "$test_root/aws.log") == 1 ]]
: >"$test_root/aws.log"

cat >"$test_root/summary.json" <<'EOF'
{
  "issue": 268,
  "run_id": "issue268-six-hour-r7i-20260821-58",
  "status": "passed",
  "attempts": [{"purchase_option": "on_demand", "status": "launched"}],
  "expected_max_cost_usd": 20.0,
  "cleanup": {"termination_attempted": true, "final_instance_state": "terminated"}
}
EOF
cat >"$test_root/artifacts/attempt-0/command-stdout.log" <<EOF
ADL_ISSUE268_REPORT_BEGIN
{"schema":"adl.runtime_v3.lifecycle_soak.v1","suite":"six_hour_qualification","revision":"$revision","minimum_exposure_seconds":21600,"measured_exposure_seconds":21607,"overshoot_seconds":7,"maximum_overshoot_seconds":600,"runtime_v3_soak":{"nested":{"status":"pass"}},"failure":null}
ADL_ISSUE268_REPORT_END
EOF
printf '1\n' >"$test_root/aws.count"
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
[[ $(grep -c 'Name=tag:adl:run_id,Values=issue268-six-hour-r7i-20260821-58' "$test_root/aws.log") == 1 ]]

grep -Fq 'ADL_ISSUE414_SIGNING_KEY_HEX",' "$wrapper"
grep -Fq 'od -An -N32 -tx1 /dev/urandom' "$ROOT/tools/aws_remote_validation/scripts/remote_validation_runner.sh"
grep -Fq 'export ADL_ISSUE414_SIGNING_KEY_HEX' "$ROOT/tools/aws_remote_validation/scripts/remote_validation_runner.sh"
grep -Fq 'ADL_SPOT_RETAINED_RUNTIME_ROOT="$ADL_RUNTIME_CONTINUITY_ROOT/state/$ADL_RUN_ID"' "$ROOT/tools/aws_remote_validation/scripts/remote_validation_runner.sh"
grep -Fq 'REMOTE_BIN=${ADL_AWS_REMOTE_VALIDATION_BIN:-$ROOT/.adl/bin/adl-aws-remote-validation-tool}' "$ROOT/adl/tools/run_issue268_six_hour_spot_qualification.sh"
grep -Fq '.provenance/adl-aws-remote-validation-tool.sha256' "$ROOT/adl/tools/run_issue268_six_hour_spot_qualification.sh"
grep -Fq 'command -v sha256sum' "$ROOT/adl/tools/install_vector_component.sh"
[[ $(rg -c 'shasum -a 256' "$ROOT/adl/tools/install_vector_component.sh") == 1 ]]
grep -Fq 'ADL_RUNTIME_GUARDIAN_TARGET_ROOT="$(dirname "${CARGO_TARGET_DIR:?CARGO_TARGET_DIR is required}")"' "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"

grep -Fq 'if [[ -n "$RUNTIME_CONTINUITY_VOLUME_ID" ]]' "$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh"
grep -Fq 'Runtime continuity volume requires an explicit colocated subnet' "$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh"

changed=$(git -C "$ROOT" diff --name-only HEAD -- && git -C "$ROOT" ls-files --others --exclude-standard)
python3 - "$changed" <<'PY'
import sys
allowed={
"adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs",
"adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
"adl/tools/run_aws_spot_remote_validation_lane.sh",
"adl/tools/aws_spot_artifact_finalize.py",
"adl/tools/test_aws_spot_artifact_finalize.sh",
"adl/tools/test_run_aws_spot_remote_validation_lane.sh",
"adl/tools/run_aws_spot_builder_image_validation.sh",
"adl/tools/test_run_aws_spot_builder_image_validation.sh",
"tools/aws_remote_validation/src/aws_remote_validation.rs",
"tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs",
"tools/aws_remote_validation/tests/portable_adapter.rs",
"tools/aws_remote_validation/scripts/remote_validation_runner.sh",
"adl/tools/run_issue268_six_hour_spot_qualification.sh",
"adl/tools/install_aws_remote_validation_tool.sh",
"adl/tools/test_run_issue268_six_hour_spot_qualification.sh",
"adl/tools/issue268_runtime_qualification.cloudformation.yaml",
"adl/tools/test_issue268_runtime_qualification_cloudformation.py",
"adl/tools/issue268_six_resident_uts_plan.json",
"adl/tools/issue268_runtime_uts_task_panel.json",
"adl/tools/validate_issue268_six_resident_uts_plan.py",
"adl/tools/run_issue268_six_resident_uts_cycle.py",
"adl/tools/test_run_issue268_six_resident_uts_cycle.py",
"adl/tools/uts_benchmark_runner.py",
"adl/tools/test_uts_benchmark_runner_contracts.sh",
"adl/src/provider_adapter.rs",
"adl/src/provider_communication.rs",
"adl/src/resident_tool_execution.rs",
"adl/src/long_lived_agent.rs",
"adl/src/agent_comms/dispatch/coding.inc",
"adl/tools/run_issue268_continuity_uts_qualification.py",
"adl/tools/test_run_issue268_continuity_uts_qualification.py",
"adl/tools/benchmark/uts_benchmark_panel.py",
"adl/tools/benchmark/uts_benchmark_tasks.py",
"adl/tools/materialize_issue268_ollama_plan.py",
"adl/tools/test_materialize_issue268_ollama_plan.py",
"adl/tools/warm_issue268_ollama_models.py",
"adl/tools/test_warm_issue268_ollama_models.py",
"adl/tools/run_issue268_remote_resident_qualification.sh",
"adl/tools/test_run_issue268_remote_resident_qualification.sh",
"adl/tools/install_issue268_runtime_volume.py",
"adl/tools/test_install_issue268_runtime_volume.py",
"adl/tools/install_vector_component.sh",
}
for path in filter(None,sys.argv[1].splitlines()):
    if path.startswith((".csdlc/issues/268/",".csdlc/prepared/issues/268/",".csdlc/evidence/268/")) or path==".csdlc/locks/268.lock": continue
    if path not in allowed: raise SystemExit(f"out-of-scope #268 path: {path}")
print("PASS: issue268 exact scope")
PY

echo "PASS: issue268 six-hour On-Demand qualification contracts"
