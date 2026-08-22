#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)
MODE=${1:-}
RUN_ID=issue268-six-hour-r7i-20260821-56
EVIDENCE_ROOT=${ADL_ISSUE268_EVIDENCE_ROOT:-$ROOT/.csdlc/evidence/268/aws/$RUN_ID}
REQUEST="$EVIDENCE_ROOT/portable-request.json"
SUMMARY="$EVIDENCE_ROOT/summary.json"
ARTIFACTS="$EVIDENCE_ROOT/artifacts"
CANCEL="$EVIDENCE_ROOT/cancel"
LAUNCH_CLAIM="$EVIDENCE_ROOT/launch-claimed.json"
REMOTE_BIN=${ADL_AWS_REMOTE_VALIDATION_BIN:-$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation}
PORTABLE_BIN=${ADL_REMOTE_VALIDATION_BIN:-$ROOT/tools/remote_validation/target/debug/adl-remote-validation}
OWNER=${ADL_ISSUE268_OWNER:-$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh}
UTS_PLAN_VALIDATOR="$ROOT/adl/tools/validate_issue268_six_resident_uts_plan.py"
AWS_CLI=${ADL_ISSUE268_AWS_CLI:-aws}
PROFILE=agent-logic-admin
REGION=us-west-2
EXCLUDED_ISSUE=269
REMOTE_QUALIFICATION=adl/tools/run_issue268_remote_resident_qualification.sh
RUNTIME_VOLUME_ID=${ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID:-}
RUNTIME_VOLUME_NAME=${ADL_AWS_RUNTIME_CONTINUITY_VOLUME_NAME:-}
RUNTIME_VOLUME_ID_SHA256=${ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID_SHA256:-}
HOURLY=${ADL_ISSUE268_ESTIMATED_HOURLY_COST_USD:-}
CFN_TEMPLATE="$ROOT/adl/tools/issue268_runtime_qualification.cloudformation.yaml"
CFN_STACK_NAME="adl-issue268-runtime-56"
CFN_SUBNET_ID=${ADL_ISSUE268_SUBNET_ID:-${ADL_AWS_REMOTE_VALIDATION_SUBNET_ID:-}}
CFN_AVAILABILITY_ZONE=${ADL_ISSUE268_AVAILABILITY_ZONE:-}
CFN_RUNTIME_SNAPSHOT_ID=${ADL_ISSUE268_RUNTIME_SNAPSHOT_ID:-}
CFN_BOOTSTRAP_BUCKET=${ADL_ISSUE268_BOOTSTRAP_BUCKET:-adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2}
CFN_BOOTSTRAP_PREFIX=${ADL_ISSUE268_BOOTSTRAP_PREFIX:-shepherd/}

usage() {
  echo "usage: $0 preflight|authorized-launch|terminal-status|validate" >&2
  exit 64
}

[[ "$MODE" =~ ^(preflight|authorized-launch|terminal-status|validate)$ ]] || usage
[[ "$EXCLUDED_ISSUE" == 269 ]] || { echo "issue268: #269 boundary drifted" >&2; exit 65; }
if [[ "$MODE" == authorized-launch ]]; then
  [[ "${ADL_ISSUE268_AUTHORIZATION:-}" == "authorized-on-demand-usd20-20260820" ]] || {
    echo "issue268: exact operator authorization marker missing" >&2
    exit 77
  }
fi
python3 - "$ROOT" "$EVIDENCE_ROOT" <<'PY'
import os, pathlib, sys
root=pathlib.Path(sys.argv[1]).resolve(); candidate=pathlib.Path(sys.argv[2]).resolve()
allowed=(root/".csdlc/evidence/268", root/".adl")
if not any(candidate == base or base in candidate.parents for base in allowed):
    raise SystemExit("issue268: evidence root escaped governed repository paths")
PY
[[ -x "$OWNER" ]] || { echo "issue268: AWS owner wrapper missing" >&2; exit 69; }
[[ -x "$REMOTE_BIN" ]] || { echo "issue268: tools AWS owner binary missing" >&2; exit 69; }
[[ -x "$PORTABLE_BIN" ]] || { echo "issue268: portable validation binary missing" >&2; exit 69; }
[[ -f "$UTS_PLAN_VALIDATOR" ]] || { echo "issue268: six-resident UTS plan validator missing" >&2; exit 69; }
[[ -f "$ROOT/$REMOTE_QUALIFICATION" ]] || { echo "issue268: coupled remote qualification wrapper missing" >&2; exit 69; }
python3 "$UTS_PLAN_VALIDATOR" >/dev/null

if [[ "$MODE" == preflight || "$MODE" == authorized-launch ]]; then
  [[ -f "$CFN_TEMPLATE" \
      && "$CFN_SUBNET_ID" =~ ^subnet-[0-9a-f]{8,17}$ \
      && "$CFN_RUNTIME_SNAPSHOT_ID" =~ ^snap-[0-9a-f]{8,17}$ \
      && -n "$CFN_AVAILABILITY_ZONE" ]] || {
    echo "issue268: exact CloudFormation subnet, availability zone, and Runtime snapshot are required" >&2
    exit 77
  }
  [[ "$HOURLY" =~ ^[0-9]+([.][0-9]+)?$ ]] || {
    echo "issue268: current estimated hourly On-Demand cost required" >&2
    exit 77
  }
  python3 - "$HOURLY" <<'PY' || exit 77
import sys
if float(sys.argv[1]) <= 0:
    raise SystemExit("issue268: estimated hourly On-Demand cost must be greater than zero")
PY
fi

mkdir -p "$EVIDENCE_ROOT" "$ARTIFACTS"
REVISION=$(git -C "$ROOT" rev-parse HEAD)
[[ "$REVISION" =~ ^[0-9a-f]{40}$ ]] || { echo "issue268: immutable revision required" >&2; exit 65; }
BRANCH=$(git -C "$ROOT" symbolic-ref --quiet --short HEAD || true)
[[ "$BRANCH" == "codex/268-six-hour-spot-qualification" ]] || {
  echo "issue268: wrong execution branch" >&2
  exit 65
}

python3 - "$REQUEST" "$REVISION" "$CANCEL" "$RUN_ID" "$ROOT" <<'PY'
import hashlib, json, os, pathlib, sys
request_path, revision, cancel, run_id, root = sys.argv[1:]
profile = {
    "argv": ["bash", "adl/tools/run_issue268_remote_resident_qualification.sh"],
    "working_directory": ".",
    "environment_allowlist": [
        "PATH", "HOME", "CARGO_HOME", "RUSTUP_HOME", "CARGO_TARGET_DIR", "ADL_RUN_ID", "ADL_RUNTIME_VECTOR_BIN",
        "ADL_RUNTIME_CONTINUITY_ROOT", "ADL_RUNTIME_CONTINUITY_VOLUME_ID_SHA256",
        "ADL_CACHE_VOLUME_MOUNT_PATH", "ADL_RETAINED_VOLUME_ROLE", "ADL_REGION",
        "ADL_ISSUE268_REMOTE_EVIDENCE_ROOT", "ADL_ISSUE268_CONTINUITY_BIN",
        "ADL_ISSUE268_RETAINED_RUNTIME_ROOT", "ADL_ISSUE268_BUILD_CACHE_ROOT",
        "ADL_ISSUE268_AGENT_SPEC_DIR", "ADL_ISSUE268_RUNTIME_VOLUME_IDENTITY_SHA256",
        "ADL_ISSUE268_S3_SOURCE_RECEIPT", "ADL_ISSUE268_414_REVIEWED_SHA",
        "ADL_ISSUE268_CONTINUITY_BIN_SHA256", "ADL_ISSUE414_SIGNING_KEY_HEX",
        "ADL_ISSUE268_CUSTODY_ENV_FILE",
    ],
}
digest = hashlib.sha256(json.dumps(profile, separators=(",", ":")).encode()).hexdigest()
request = {
    "schema": "adl.remote_validation.request.v1",
    "request_id": run_id,
    "checkout": ".",
    "revision": revision,
    "source_ref": "refs/heads/codex/268-six-hour-spot-qualification",
    "command_profile": profile,
    "command_profile_digest": digest,
    "adapter": "aws",
    "requested_platform": "linux",
    "resource_budget": {
        "cpu_cores": 8,
        "memory_mib": 65536,
        "timeout_seconds": 25200,
        "estimated_max_cost_microusd": 20000000,
    },
    "artifact_policy": {
        "paths": ["attempt-0/command-stdout.log"],
        "required": True,
        "max_total_bytes": 134217728,
    },
    "cancellation_file": os.path.relpath(cancel, root),
    "fallback": "disabled",
}
path = pathlib.Path(request_path)
temporary = path.with_suffix(".tmp")
temporary.write_text(json.dumps(request, indent=2, sort_keys=True) + "\n")
os.replace(temporary, path)
PY

"$PORTABLE_BIN" validate-request "$REQUEST" >/dev/null

common=(
  --profile "$PROFILE" --region "$REGION" --issue 268 --run-id "$RUN_ID"
  --portable-request "$REQUEST" --portable-runner "$PORTABLE_BIN"
  --bin "$REMOTE_BIN" --instance-types r7i.2xlarge
  --max-spot-retries 0 --out "$SUMMARY" --artifact-dir "$ARTIFACTS" --json
  --on-demand-only
  --estimated-hourly-cost-usd "$HOURLY"
)

profile_args=()
if [[ "$PROFILE" != env && "$PROFILE" != environment ]]; then
  profile_args=(--profile "$PROFILE")
fi

validate_cloudformation_inputs() {
  "$OWNER" preflight --check-account --profile "$PROFILE" --region "$REGION" >/dev/null
  "$AWS_CLI" "${profile_args[@]}" --region "$REGION" cloudformation validate-template \
    --template-body "file://$CFN_TEMPLATE" >/dev/null
  local subnet_az
  subnet_az=$("$AWS_CLI" "${profile_args[@]}" --region "$REGION" ec2 describe-subnets \
    --subnet-ids "$CFN_SUBNET_ID" --query 'Subnets[0].AvailabilityZone' --output text)
  CFN_VPC_ID=$("$AWS_CLI" "${profile_args[@]}" --region "$REGION" ec2 describe-subnets \
    --subnet-ids "$CFN_SUBNET_ID" --query 'Subnets[0].VpcId' --output text)
  [[ "$subnet_az" == "$CFN_AVAILABILITY_ZONE" && "$CFN_VPC_ID" =~ ^vpc-[0-9a-f]{8,17}$ ]] || {
    echo "issue268: CloudFormation subnet/AZ/VPC contract failed" >&2
    return 1
  }
}

delete_cloudformation_stack() {
  "$AWS_CLI" "${profile_args[@]}" --region "$REGION" cloudformation delete-stack \
    --stack-name "$CFN_STACK_NAME" >/dev/null 2>&1 || return 1
  "$AWS_CLI" "${profile_args[@]}" --region "$REGION" cloudformation wait stack-delete-complete \
    --stack-name "$CFN_STACK_NAME" >/dev/null 2>&1
}

CFN_STACK_OWNED=false
cleanup_cloudformation_on_exit() {
  local status=${1:-$?}
  trap - EXIT INT TERM
  if [[ "$CFN_STACK_OWNED" == true ]]; then
    if ! delete_cloudformation_stack; then
      echo "issue268: CloudFormation stack cleanup failed" >&2
      status=70
    fi
  fi
  exit "$status"
}

case "$MODE" in
  preflight)
    validate_cloudformation_inputs
    python3 - "$RUN_ID" "$REVISION" "$CFN_STACK_NAME" "$HOURLY" <<'PY'
import hashlib, json, sys
run_id, revision, stack_name, hourly = sys.argv[1:]
print(json.dumps({
    "schema":"adl.issue268.cloudformation_preflight.v1",
    "status":"ready",
    "run_id":run_id,
    "revision":revision,
    "stack_name_sha256":hashlib.sha256(stack_name.encode()).hexdigest(),
    "instance_type":"r7i.2xlarge",
    "purchase_option":"on_demand",
    "max_attempts":1,
    "fallback":"disabled",
    "timeout_seconds":25200,
    "expected_max_cost_usd":20.0,
    "estimated_hourly_cost_usd":float(hourly),
    "aws_resources_created":False,
},sort_keys=True))
PY
    ;;
  authorized-launch)
    if [[ -e "$LAUNCH_CLAIM" ]]; then
      python3 - "$LAUNCH_CLAIM" "$RUN_ID" "$REVISION" <<'PY'
import json, sys
d=json.load(open(sys.argv[1]))
if d != {"revision":sys.argv[3],"run_id":sys.argv[2],"schema":"adl.issue268.launch_claim.v1"}:
    raise SystemExit("issue268: existing launch claim mismatch")
print(json.dumps({"schema":"adl.issue268.launch.v1","status":"existing_run_claim_resolved","run_id":sys.argv[2]},sort_keys=True))
PY
      exit 0
    fi
    (set -o noclobber; printf '{"revision":"%s","run_id":"%s","schema":"adl.issue268.launch_claim.v1"}\n' "$REVISION" "$RUN_ID" >"$LAUNCH_CLAIM") 2>/dev/null || {
      echo "issue268: another launch invocation owns the one-attempt claim" >&2
      exit 75
    }
    validate_cloudformation_inputs
    CFN_STACK_OWNED=true
    trap 'cleanup_cloudformation_on_exit $?' EXIT
    trap 'cleanup_cloudformation_on_exit 130' INT
    trap 'cleanup_cloudformation_on_exit 143' TERM
    "$AWS_CLI" "${profile_args[@]}" --region "$REGION" cloudformation create-stack \
      --stack-name "$CFN_STACK_NAME" \
      --template-body "file://$CFN_TEMPLATE" \
      --capabilities CAPABILITY_IAM \
      --parameters \
        "ParameterKey=RunId,ParameterValue=$RUN_ID" \
        "ParameterKey=AvailabilityZone,ParameterValue=$CFN_AVAILABILITY_ZONE" \
        "ParameterKey=SubnetId,ParameterValue=$CFN_SUBNET_ID" \
        "ParameterKey=VpcId,ParameterValue=$CFN_VPC_ID" \
        "ParameterKey=RuntimeSnapshotId,ParameterValue=$CFN_RUNTIME_SNAPSHOT_ID" \
        "ParameterKey=BootstrapBucket,ParameterValue=$CFN_BOOTSTRAP_BUCKET" \
        "ParameterKey=BootstrapPrefix,ParameterValue=$CFN_BOOTSTRAP_PREFIX" \
      --tags "Key=adl:issue,Value=268" "Key=adl:run_id,Value=$RUN_ID" >/dev/null
    "$AWS_CLI" "${profile_args[@]}" --region "$REGION" cloudformation wait stack-create-complete \
      --stack-name "$CFN_STACK_NAME"
    RUNTIME_INSTANCE_ID=$("$AWS_CLI" "${profile_args[@]}" --region "$REGION" cloudformation describe-stacks \
      --stack-name "$CFN_STACK_NAME" --query 'Stacks[0].Outputs[?OutputKey==`InstanceId`].OutputValue|[0]' --output text)
    RUNTIME_VOLUME_ID=$("$AWS_CLI" "${profile_args[@]}" --region "$REGION" cloudformation describe-stacks \
      --stack-name "$CFN_STACK_NAME" --query 'Stacks[0].Outputs[?OutputKey==`RuntimeVolumeId`].OutputValue|[0]' --output text)
    [[ "$RUNTIME_INSTANCE_ID" =~ ^i-[0-9a-f]{8,17}$ && "$RUNTIME_VOLUME_ID" =~ ^vol-[0-9a-f]{8,17}$ ]] || {
      echo "issue268: CloudFormation outputs were incomplete" >&2
      exit 70
    }
    RUNTIME_VOLUME_ID_SHA256=$(python3 -c 'import hashlib,sys; print(hashlib.sha256(sys.argv[1].encode()).hexdigest())' "$RUNTIME_VOLUME_ID")
    RUNTIME_VOLUME_NAME="$RUN_ID-runtime"
    python3 - "$EVIDENCE_ROOT/cloudformation.json" "$CFN_STACK_NAME" "$RUNTIME_INSTANCE_ID" "$RUNTIME_VOLUME_ID" <<'PY'
import hashlib, json, os, pathlib, sys
path, stack, instance, volume = sys.argv[1:]
payload={"schema":"adl.issue268.cloudformation.v1","status":"created","stack_name_sha256":hashlib.sha256(stack.encode()).hexdigest(),"instance_id_sha256":hashlib.sha256(instance.encode()).hexdigest(),"runtime_volume_id_sha256":hashlib.sha256(volume.encode()).hexdigest()}
p=pathlib.Path(path); t=p.with_suffix('.tmp'); t.write_text(json.dumps(payload,sort_keys=True)+'\n'); os.replace(t,p)
PY
    set +e
    ADL_AWS_EXISTING_INSTANCE_ID="$RUNTIME_INSTANCE_ID" \
    ADL_AWS_PRE_MOUNTED_RUNTIME_ROOT=/opt/adl-runtime \
    ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID="$RUNTIME_VOLUME_ID" \
    ADL_AWS_RUNTIME_CONTINUITY_VOLUME_NAME="$RUNTIME_VOLUME_NAME" \
    ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID_SHA256="$RUNTIME_VOLUME_ID_SHA256" \
    ADL_AWS_REMOTE_VALIDATION_SUBNET_ID="$CFN_SUBNET_ID" \
      "$OWNER" run --run "${common[@]}" \
        --runtime-continuity-volume-id "$RUNTIME_VOLUME_ID" \
        --runtime-continuity-volume-name "$RUNTIME_VOLUME_NAME" \
        --runtime-continuity-volume-id-sha256 "$RUNTIME_VOLUME_ID_SHA256"
    owner_status=$?
    set -e
    delete_cloudformation_stack || {
      echo "issue268: CloudFormation stack cleanup failed" >&2
      exit 70
    }
    CFN_STACK_OWNED=false
    trap - EXIT INT TERM
    exit "$owner_status"
    ;;
  terminal-status)
    [[ -s "$SUMMARY" ]] || { echo "issue268: run summary absent" >&2; exit 75; }
    "$OWNER" status --profile "$PROFILE" --region "$REGION" --issue 268 --run-id "$RUN_ID" --out "$SUMMARY" --artifact-dir "$ARTIFACTS" --bin "$REMOTE_BIN" --json
    python3 - "$SUMMARY" <<'PY'
import json, sys
d=json.load(open(sys.argv[1])); status=d.get("status")
if status not in {"passed","failed","timed_out","cancelled","provider_unavailable","cleanup_incomplete"}:
    raise SystemExit(f"issue268: run is not terminal: {status}")
print(json.dumps({"schema":"adl.issue268.terminal.v1","status":status},sort_keys=True))
PY
    ;;
  validate)
    if [[ ! -s "$SUMMARY" ]]; then
      stack_status=$("$AWS_CLI" --profile "$PROFILE" --region "$REGION" cloudformation describe-stacks \
        --stack-name "$CFN_STACK_NAME" --query 'Stacks[0].StackStatus' --output text 2>/dev/null || true)
      if [[ "$stack_status" =~ ^(CREATE|UPDATE|IMPORT|ROLLBACK)_IN_PROGRESS$ || "$stack_status" == "CREATE_COMPLETE" ]]; then
        echo "issue268: CloudFormation runtime stack is active without terminal summary; refusing validation cleanup" >&2
        exit 75
      fi
      active_instance_json=$("$AWS_CLI" --profile "$PROFILE" --region "$REGION" ec2 describe-instances \
        --filters "Name=tag:adl:issue,Values=268" "Name=tag:adl:run_id,Values=$RUN_ID" "Name=instance-state-name,Values=pending,running,stopping,stopped,shutting-down" \
        --query 'Reservations[].Instances[].InstanceId' --output json)
      if [[ "$active_instance_json" != "[]" ]]; then
        echo "issue268: task-owned instance is active without terminal summary; refusing validation cleanup" >&2
        exit 75
      fi
      owner_status_rc=0
      owner_status=$("$OWNER" status --profile "$PROFILE" --region "$REGION" --issue 268 --run-id "$RUN_ID" --out "$SUMMARY" --artifact-dir "$ARTIFACTS" --bin "$REMOTE_BIN" --json 2>&1) || owner_status_rc=$?
      if [[ "$owner_status" == *"status=running"* ]]; then
        echo "issue268: qualification manager is active; refusing validation cleanup" >&2
        exit 75
      fi
      if [[ $owner_status_rc -eq 0 ]]; then
        echo "issue268: owner reported a terminal state without the required summary" >&2
      fi
    fi
    ZERO_JSON=$("$AWS_CLI" --profile "$PROFILE" --region "$REGION" ec2 describe-instances \
      --filters "Name=tag:adl:issue,Values=268" "Name=tag:adl:run_id,Values=$RUN_ID" "Name=instance-state-name,Values=pending,running,stopping,stopped,shutting-down" \
      --query 'Reservations[].Instances[].InstanceId' --output json)
    [[ "$ZERO_JSON" == "[]" ]] || {
      echo "issue268: validation is read-only and refuses while task-owned compute remains" >&2
      exit 75
    }
    [[ -s "$SUMMARY" ]] || {
      echo "issue268: cleanup recovery completed but terminal run summary is absent" >&2
      exit 75
    }
python3 - "$SUMMARY" "$ARTIFACTS" "$ZERO_JSON" "$EVIDENCE_ROOT/validation.json" "$REVISION" "$RUN_ID" <<'PY'
import hashlib, json, os, pathlib, re, sys
summary_path, artifacts, zero_json, output, revision, run_id = sys.argv[1:]
d=json.load(open(summary_path))
if d.get("issue") != 268 or d.get("run_id") != run_id: raise SystemExit("issue268: summary identity mismatch")
if d.get("status") != "passed": raise SystemExit(f"issue268: paid qualification did not pass: {d.get('status')}")
attempts=d.get("attempts") or []
if len(attempts) != 1 or attempts[0].get("purchase_option") != "on_demand": raise SystemExit("issue268: not exactly one On-Demand attempt")
if d.get("expected_max_cost_usd") != 20.0: raise SystemExit("issue268: USD 20 ceiling missing")
cleanup=d.get("cleanup") or {}
if cleanup.get("termination_attempted") is not True or cleanup.get("final_instance_state") != "terminated": raise SystemExit("issue268: owner cleanup incomplete")
remaining=json.loads(zero_json)
if remaining: raise SystemExit("issue268: task-owned instance remains")
logs=list(pathlib.Path(artifacts).rglob("command-stdout.log"))
if len(logs) != 1: raise SystemExit("issue268: exact command stdout evidence missing")
text=logs[0].read_text(errors="replace")
begin="ADL_ISSUE268_REPORT_BEGIN"; end="ADL_ISSUE268_REPORT_END"
if text.count(begin) != 1 or text.count(end) != 1: raise SystemExit("issue268: exact six-hour report markers missing")
report=json.loads(text.split(begin,1)[1].split(end,1)[0].strip())
if report.get("revision") != revision or report.get("suite") != "six_hour_qualification": raise SystemExit("issue268: report revision/suite mismatch")
measured=int(report.get("measured_exposure_seconds",0)); over=int(report.get("overshoot_seconds",-1))
if measured < 21600 or over != measured-21600 or over > 600: raise SystemExit("issue268: elapsed denominator/overshoot failed")
receipt={
 "schema":"adl.issue268.validation.v1","status":"pass","revision":revision,
 "minimum_exposure_seconds":21600,"measured_exposure_seconds":measured,"overshoot_seconds":over,
 "attempt_count":1,"purchase_option":"on_demand","remaining_task_instances":0,
 "summary_sha256":hashlib.sha256(pathlib.Path(summary_path).read_bytes()).hexdigest(),
 "command_stdout_sha256":hashlib.sha256(logs[0].read_bytes()).hexdigest(),
}
p=pathlib.Path(output); tmp=p.with_suffix(".tmp"); tmp.write_text(json.dumps(receipt,indent=2,sort_keys=True)+"\n"); os.replace(tmp,p)
print(json.dumps(receipt,sort_keys=True))
PY
    ;;
esac
