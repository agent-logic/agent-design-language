#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)
PRIMARY_ROOT=$(dirname "$(git -C "$ROOT" rev-parse --path-format=absolute --git-common-dir)")
MODE=${1:-}
RUN_ID=issue268-six-hour-20260817
EVIDENCE_ROOT=${ADL_ISSUE268_EVIDENCE_ROOT:-$ROOT/.csdlc/evidence/268/aws}
REQUEST="$EVIDENCE_ROOT/portable-request.json"
SUMMARY="$EVIDENCE_ROOT/summary.json"
ARTIFACTS="$EVIDENCE_ROOT/artifacts"
CANCEL="$EVIDENCE_ROOT/cancel"
LAUNCH_CLAIM="$EVIDENCE_ROOT/launch-claimed.json"
REMOTE_BIN=${ADL_AWS_REMOTE_VALIDATION_BIN:-$PRIMARY_ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation}
PORTABLE_BIN=${ADL_REMOTE_VALIDATION_BIN:-$PRIMARY_ROOT/tools/remote_validation/target/debug/adl-remote-validation}
OWNER=${ADL_ISSUE268_OWNER:-$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh}
AWS_CLI=${ADL_ISSUE268_AWS_CLI:-aws}
PROFILE=agent-logic-admin
REGION=us-west-2
EXCLUDED_ISSUE=269

usage() {
  echo "usage: $0 preflight|authorized-launch|terminal-status|validate" >&2
  exit 64
}

[[ "$MODE" =~ ^(preflight|authorized-launch|terminal-status|validate)$ ]] || usage
[[ "$EXCLUDED_ISSUE" == 269 ]] || { echo "issue268: #269 boundary drifted" >&2; exit 65; }
python3 - "$ROOT" "$EVIDENCE_ROOT" <<'PY'
import os, pathlib, sys
root=pathlib.Path(sys.argv[1]).resolve(); candidate=pathlib.Path(sys.argv[2]).resolve()
allowed=(root/".csdlc/evidence/268", root/".adl")
if not any(candidate == base or base in candidate.parents for base in allowed):
    raise SystemExit("issue268: evidence root escaped governed repository paths")
PY
[[ -x "$OWNER" ]] || { echo "issue268: Spot owner wrapper missing" >&2; exit 69; }
[[ -x "$REMOTE_BIN" ]] || { echo "issue268: tools AWS owner binary missing" >&2; exit 69; }
[[ -x "$PORTABLE_BIN" ]] || { echo "issue268: portable validation binary missing" >&2; exit 69; }

mkdir -p "$EVIDENCE_ROOT" "$ARTIFACTS"
REVISION=$(git -C "$ROOT" rev-parse HEAD)
[[ "$REVISION" =~ ^[0-9a-f]{40}$ ]] || { echo "issue268: immutable revision required" >&2; exit 65; }
BRANCH=$(git -C "$ROOT" symbolic-ref --quiet --short HEAD || true)
[[ "$BRANCH" == "codex/268-six-hour-spot-qualification" ]] || {
  echo "issue268: wrong execution branch" >&2
  exit 65
}

python3 - "$REQUEST" "$REVISION" "$CANCEL" <<'PY'
import hashlib, json, os, pathlib, sys
request_path, revision, cancel = sys.argv[1:]
profile = {
    "argv": ["bash", "adl/tools/validate_v092_runtime_guardian_lifecycle.sh", "--suite", "six_hour_qualification"],
    "working_directory": ".",
    "environment_allowlist": ["PATH", "CARGO_HOME", "RUSTUP_HOME", "CARGO_TARGET_DIR", "ADL_RUNTIME_VECTOR_BIN"],
}
digest = hashlib.sha256(json.dumps(profile, separators=(",", ":")).encode()).hexdigest()
request = {
    "schema": "adl.remote_validation.request.v1",
    "request_id": "issue268-six-hour-20260817",
    "checkout": ".",
    "revision": revision,
    "source_ref": "refs/heads/codex/268-six-hour-spot-qualification",
    "command_profile": profile,
    "command_profile_digest": digest,
    "adapter": "aws",
    "requested_platform": "linux",
    "resource_budget": {
        "cpu_cores": 4,
        "memory_mib": 8192,
        "timeout_seconds": 25200,
        "estimated_max_cost_microusd": 20000000,
    },
    "artifact_policy": {
        "paths": ["attempt-0/command-stdout.log"],
        "required": True,
        "max_total_bytes": 134217728,
    },
    "cancellation_file": os.path.relpath(cancel, pathlib.Path(request_path).parents[4]),
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
  --bin "$REMOTE_BIN" --instance-types c7i.2xlarge
  --max-spot-retries 0 --out "$SUMMARY" --artifact-dir "$ARTIFACTS" --json
)

case "$MODE" in
  preflight)
    "$OWNER" preflight --check-account "${common[@]}"
    ;;
  authorized-launch)
    [[ "${ADL_ISSUE268_AUTHORIZATION:-}" == "authorized-usd20-20260817" ]] || {
      echo "issue268: exact operator authorization marker missing" >&2
      exit 77
    }
    HOURLY=${ADL_ISSUE268_ESTIMATED_HOURLY_COST_USD:-}
    [[ "$HOURLY" =~ ^[0-9]+([.][0-9]+)?$ ]] || {
      echo "issue268: current estimated hourly Spot cost required" >&2
      exit 77
    }
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
    "$OWNER" launch "${common[@]}" --estimated-hourly-cost-usd "$HOURLY"
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
      owner_status_rc=0
      owner_status=$("$OWNER" status --profile "$PROFILE" --region "$REGION" --issue 268 --run-id "$RUN_ID" --out "$SUMMARY" --artifact-dir "$ARTIFACTS" --bin "$REMOTE_BIN" --json 2>&1) || owner_status_rc=$?
      if [[ "$owner_status" == *"status=running"* ]]; then
        echo "issue268: qualification manager is active; refusing validation cleanup" >&2
        exit 75
      fi
      if [[ $owner_status_rc -eq 0 ]]; then
        echo "issue268: owner reported a terminal state without the required summary" >&2
      fi
    else
      "$OWNER" cleanup --profile "$PROFILE" --region "$REGION" --issue 268 --run-id "$RUN_ID" --out "$SUMMARY" --artifact-dir "$ARTIFACTS" --bin "$REMOTE_BIN" --json
    fi
    ZERO_JSON=$("$AWS_CLI" --profile "$PROFILE" --region "$REGION" ec2 describe-instances \
      --filters "Name=tag:adl:issue,Values=268" "Name=tag:adl:run_id,Values=$RUN_ID" "Name=instance-state-name,Values=pending,running,stopping,stopped,shutting-down" \
      --query 'Reservations[].Instances[].InstanceId' --output json)
    if [[ "$ZERO_JSON" != "[]" ]]; then
      python3 - "$ZERO_JSON" <<'PY' >"$EVIDENCE_ROOT/residual-instance-ids.txt"
import json, sys
for value in json.loads(sys.argv[1]): print(value)
PY
      residual_ids=()
      while IFS= read -r residual_id; do
        residual_ids+=("$residual_id")
      done <"$EVIDENCE_ROOT/residual-instance-ids.txt"
      "$AWS_CLI" --profile "$PROFILE" --region "$REGION" ec2 terminate-instances --instance-ids "${residual_ids[@]}" >/dev/null
      "$AWS_CLI" --profile "$PROFILE" --region "$REGION" ec2 wait instance-terminated --instance-ids "${residual_ids[@]}"
      ZERO_JSON=$("$AWS_CLI" --profile "$PROFILE" --region "$REGION" ec2 describe-instances \
        --filters "Name=tag:adl:issue,Values=268" "Name=tag:adl:run_id,Values=$RUN_ID" "Name=instance-state-name,Values=pending,running,stopping,stopped,shutting-down" \
        --query 'Reservations[].Instances[].InstanceId' --output json)
      : >"$EVIDENCE_ROOT/residual-instance-ids.txt"
    fi
    [[ -s "$SUMMARY" ]] || {
      echo "issue268: cleanup recovery completed but terminal run summary is absent" >&2
      exit 75
    }
    python3 - "$SUMMARY" "$ARTIFACTS" "$ZERO_JSON" "$EVIDENCE_ROOT/validation.json" "$REVISION" <<'PY'
import hashlib, json, os, pathlib, re, sys
summary_path, artifacts, zero_json, output, revision = sys.argv[1:]
d=json.load(open(summary_path))
if d.get("issue") != 268 or d.get("run_id") != "issue268-six-hour-20260817": raise SystemExit("issue268: summary identity mismatch")
if d.get("status") != "passed": raise SystemExit(f"issue268: paid qualification did not pass: {d.get('status')}")
attempts=d.get("attempts") or []
if len(attempts) != 1 or attempts[0].get("purchase_option") != "spot": raise SystemExit("issue268: not exactly one Spot attempt")
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
 "attempt_count":1,"purchase_option":"spot","remaining_task_instances":0,
 "summary_sha256":hashlib.sha256(pathlib.Path(summary_path).read_bytes()).hexdigest(),
 "command_stdout_sha256":hashlib.sha256(logs[0].read_bytes()).hexdigest(),
}
p=pathlib.Path(output); tmp=p.with_suffix(".tmp"); tmp.write_text(json.dumps(receipt,indent=2,sort_keys=True)+"\n"); os.replace(tmp,p)
print(json.dumps(receipt,sort_keys=True))
PY
    ;;
esac
