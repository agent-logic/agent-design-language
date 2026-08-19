#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
COMMON_GIT_DIR="$(git -C "$ROOT" rev-parse --path-format=absolute --git-common-dir 2>/dev/null || true)"
PRIMARY_ROOT="${COMMON_GIT_DIR:+$(dirname "$COMMON_GIT_DIR")}"
PROCESS_BIN="${ADL_PROCESS_BIN:-${PRIMARY_ROOT:-$ROOT}/adl/target/debug/adl}"
SCRIPT_PATH="$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh"
ORIGINAL_ARGS=("$@")

ACTION="plan"
if [[ $# -gt 0 ]]; then
  case "$1" in
    preflight|launch|run|status|logs|ssh|stop|cleanup)
      ACTION="$1"
      shift
      ;;
  esac
fi

PROFILE="${AWS_PROFILE:-agent-logic-admin}"
REGION="${AWS_REGION:-us-west-2}"
ISSUE="5191"
RUN_ID="adl-wp-5191-aws-spot-$(date -u +%Y%m%d%H%M%S)"
COMMAND=""
COMMAND_EXPLICIT=false
GIT_REF=""
GIT_REF_EXPLICIT=false
PORTABLE_REQUEST=""
PORTABLE_RUNNER="${ADL_REMOTE_VALIDATION_BIN:-}"
PORTABLE_MAX_COST_USD=""
PORTABLE_EXPECTED_REVISION=""
PORTABLE_CPU_CORES=""
PORTABLE_MEMORY_MIB=""
PORTABLE_CANCELLATION_FILE=""
PORTABLE_FALLBACK=""
PORTABLE_PROFILE_DIGEST=""
PORTABLE_ARTIFACT_POLICY_JSON=""
SOURCE_COMMIT=""
REPO_URL="https://github.com/agent-logic/agent-design-language.git"
OUT_PATH=""
ARTIFACT_DIR=""
EXPECTED_PROOF="$ROOT/docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry11_agentlogic_hotcache.json"
AWS_CLI="${ADL_AWS_CLI:-aws}"
LANE_BIN="${ADL_AWS_REMOTE_VALIDATION_BIN:-}"
RUN=false
CHECK_ACCOUNT=false
JSON=false
PRINT_COMMAND=false
FOLLOW=false
INSTANCE_TYPES=()
CACHE_VOLUME_NAME="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_NAME:-adl-aws-remote-validation-cache-volume}"
CACHE_VOLUME_SIZE_GIB="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_SIZE_GIB:-}"
CACHE_VOLUME_TYPE="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_TYPE:-}"
CACHE_VOLUME_IOPS="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_IOPS:-}"
CACHE_VOLUME_THROUGHPUT_MBPS="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_THROUGHPUT_MBPS:-}"
CACHE_VOLUME_SIZE_GIB_EXPLICIT=false
CACHE_VOLUME_TYPE_EXPLICIT=false
CACHE_VOLUME_IOPS_EXPLICIT=false
CACHE_VOLUME_THROUGHPUT_EXPLICIT=false
[[ "${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_SIZE_GIB+x}" == x ]] && CACHE_VOLUME_SIZE_GIB_EXPLICIT=true
[[ "${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_TYPE+x}" == x ]] && CACHE_VOLUME_TYPE_EXPLICIT=true
[[ "${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_IOPS+x}" == x ]] && CACHE_VOLUME_IOPS_EXPLICIT=true
[[ "${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_THROUGHPUT_MBPS+x}" == x ]] && CACHE_VOLUME_THROUGHPUT_EXPLICIT=true
CACHE_VOLUME_DEVICE_NAME="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_DEVICE_NAME:-/dev/sdf}"
CACHE_VOLUME_MOUNT_PATH="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_MOUNT_PATH:-/mnt/adl-cache}"
SSH_KEY_NAME="${ADL_AWS_REMOTE_VALIDATION_SSH_KEY_NAME:-adl-wp06-spot-ssh-debug-20260704}"
SSH_PRIVATE_KEY_PATH="${ADL_AWS_REMOTE_VALIDATION_SSH_PRIVATE_KEY_PATH:-$HOME/.ssh/adl-4603-ssh-debug-20260701.pem}"
SSH_USER="${ADL_AWS_REMOTE_VALIDATION_SSH_USER:-ec2-user}"
SSH_ALLOWED_CIDR="${ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR:-}"
SSH_BIN="${ADL_SSH_BIN:-ssh}"
BUILDER_IMAGE="${ADL_AWS_SPOT_BUILDER_IMAGE:-}"
BUILDER_IMAGE_REPOSITORY="${ADL_AWS_SPOT_BUILDER_IMAGE_REPOSITORY:-adl-builder}"
BUILDER_IMAGE_TAG="${ADL_AWS_SPOT_BUILDER_IMAGE_TAG:-v0.91.7-fixed}"
EXPECTED_ARCHITECTURE="${ADL_AWS_SPOT_EXPECTED_ARCHITECTURE:-x86_64}"
MIN_CACHE_FREE_GIB="${ADL_AWS_SPOT_MIN_CACHE_FREE_GIB:-10}"
ESTIMATED_HOURLY_COST_USD="${ADL_AWS_SPOT_ESTIMATED_HOURLY_COST_USD:-}"
MAX_RUN_SECONDS=""
MAX_SPOT_RETRIES="${ADL_AWS_SPOT_MAX_RETRIES:-2}"
AMI_ID="${ADL_AWS_REMOTE_VALIDATION_AMI_ID:-}"
SUBNET_ID="${ADL_AWS_REMOTE_VALIDATION_SUBNET_ID:-}"
EXPECTED_CACHE_VOLUME_ID_SHA256="${ADL_AWS_REMOTE_VALIDATION_CACHE_VOLUME_ID_SHA256:-}"
RETAINED_CACHE_VOLUME_ID=""
RUNTIME_CONTINUITY_VOLUME_ID="${ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID:-}"
RUNTIME_CONTINUITY_VOLUME_NAME="${ADL_AWS_RUNTIME_CONTINUITY_VOLUME_NAME:-}"
RUNTIME_CONTINUITY_VOLUME_ID_SHA256="${ADL_AWS_RUNTIME_CONTINUITY_VOLUME_ID_SHA256:-}"

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_aws_spot_remote_validation_lane.sh preflight [options]
  adl/tools/run_aws_spot_remote_validation_lane.sh [launch|run] --command <shell-command> [options]
  adl/tools/run_aws_spot_remote_validation_lane.sh status|logs|ssh|stop|cleanup --run-id <id> [options]

Options:
  --run                         Launch the AWS Spot remote validation lane.
  --check-account               Verify profile account against retained Agent Logic proof only.
  --print-command               Print the underlying adl-aws-remote-validation command.
  --profile <name>              AWS profile. Defaults to agent-logic-admin. Use env for OIDC/env credentials.
  --region <region>             AWS region. Defaults to us-west-2.
  --issue <number>              Issue recorded in the summary. Defaults to 5191.
  --run-id <id>                 Stable run id for artifacts.
  --command <shell-command>     Remote validation command to run.
  --portable-request <path>     Portable request JSON; mutually exclusive with command/ref overrides.
  --portable-runner <path>      adl-remote-validation binary for portable requests.
  --git-ref <ref>               Remote git ref. Defaults to current branch/ref.
  --repo-url <url>              Remote ADL repository URL.
  --out <path>                  Summary JSON path. Defaults under .adl/tmp.
  --artifact-dir <dir>          Artifact root. Defaults beside --out.
  --instance-type <type>        Add an allowed EC2 instance type.
  --instance-types <list>       Add comma-separated allowed EC2 instance types.
  --cache-volume-name <name>    Warm EBS cache volume name. Defaults to retained WP-06 cache.
  --cache-volume-size-gib <gib> Cache volume size when created. Defaults to 500.
  --cache-volume-type <type>    Cache volume type. Defaults to gp3.
  --cache-volume-iops <iops>    Cache volume IOPS. Defaults to 3000.
  --cache-volume-throughput-mbps <mbps>
                                Cache volume throughput. Defaults to 125.
  --cache-volume-device-name <device>
                                EC2 device name for attach. Defaults to /dev/sdf.
  --cache-volume-mount-path <path>
                                Remote mount path. Defaults to /mnt/adl-cache.
                                The retained warm EBS cache is forwarded by
                                default; it is not by itself proof that a
                                builder image was used.
  --runtime-continuity-volume-id <id>
                                Use this pre-provisioned retained volume for
                                Runtime continuity, never as build cache.
  --runtime-continuity-volume-name <name>
                                Exact Name tag for the Runtime volume.
  --runtime-continuity-volume-id-sha256 <hash>
                                Expected redacted identity for that volume.
  --ssh-key-name <name>          EC2 key pair for live remote-tail logging.
                                Defaults to retained Agent Logic debug key.
  --ssh-private-key-path <path>  Private key for live remote-tail logging.
  --ssh-user <user>              SSH user. Defaults to ec2-user.
  --ssh-allowed-cidr <cidr>      SSH source CIDR. Defaults to auto-detected operator IP.
  --builder-image <uri@digest>   Immutable builder image. Defaults to resolving
                                adl-builder:v0.91.7-fixed in Agent Logic ECR.
  --builder-image-repository <name>
                                ECR repository used for default digest resolution.
  --builder-image-tag <tag>      ECR tag resolved once to an immutable digest.
  --expected-architecture <arch> Expected image/runtime architecture. Defaults x86_64.
  --min-cache-free-gib <gib>     Required warm-cache headroom. Defaults 10.
  --estimated-hourly-cost-usd <usd>
                                Override the pre-run Spot hourly price estimate.
  --max-run-seconds <seconds>   Remote validation command timeout in seconds.
  --max-spot-retries <count>    Maximum additional Spot instance attempts. Defaults to 2.
  --ami-id <id>                 Explicit AMI. Defaults to the current AL2023 SSM image.
  --subnet-id <id>              Explicit subnet. Defaults to retained hot-cache proof topology.
  --expected-cache-volume-id-sha256 <hash>
                                Expected retained EBS identity hash.
  --expected-proof <summary>    Retained Agent Logic proof summary used for account-hash comparison.
  --bin <path>                  adl-aws-remote-validation binary path.
  --json                        Pass --json to the underlying binary.
  --follow                      Follow logs until interrupted (logs action only).
  -h, --help                    Show this help.

Without --run the wrapper performs account checking only when --check-account is
present, then prints a dry-run plan. It never launches EC2 unless --run is set.
The `launch` action is the explicit asynchronous paid path; `run --run` is the
synchronous paid path. Status, logs, SSH, stop, and cleanup reuse --run-id.

Live runs always resolve or require an immutable builder-image digest, verify
the image toolchain and architecture, and execute the requested validation
inside that image. Rust validation tools are never installed on the host.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --run)
      RUN=true
      shift
      ;;
    --check-account)
      CHECK_ACCOUNT=true
      shift
      ;;
    --print-command)
      PRINT_COMMAND=true
      shift
      ;;
    --profile)
      PROFILE="${2:-}"
      shift 2
      ;;
    --region)
      REGION="${2:-}"
      shift 2
      ;;
    --issue)
      ISSUE="${2:-}"
      shift 2
      ;;
    --run-id)
      RUN_ID="${2:-}"
      shift 2
      ;;
    --command)
      COMMAND="${2:-}"
      COMMAND_EXPLICIT=true
      shift 2
      ;;
    --portable-request)
      PORTABLE_REQUEST="${2:-}"
      shift 2
      ;;
    --portable-runner)
      PORTABLE_RUNNER="${2:-}"
      shift 2
      ;;
    --git-ref)
      GIT_REF="${2:-}"
      GIT_REF_EXPLICIT=true
      shift 2
      ;;
    --repo-url)
      REPO_URL="${2:-}"
      shift 2
      ;;
    --out)
      OUT_PATH="${2:-}"
      shift 2
      ;;
    --artifact-dir)
      ARTIFACT_DIR="${2:-}"
      shift 2
      ;;
    --instance-type)
      INSTANCE_TYPES+=("${2:-}")
      shift 2
      ;;
    --instance-types)
      IFS=',' read -r -a requested_instance_types <<<"${2:-}"
      for requested_instance_type in "${requested_instance_types[@]}"; do
        if [[ -n "$requested_instance_type" ]]; then
          INSTANCE_TYPES+=("$requested_instance_type")
        fi
      done
      shift 2
      ;;
    --cache-volume-name)
      CACHE_VOLUME_NAME="${2:-}"
      shift 2
      ;;
    --cache-volume-size-gib)
      CACHE_VOLUME_SIZE_GIB="${2:-}"
      CACHE_VOLUME_SIZE_GIB_EXPLICIT=true
      shift 2
      ;;
    --cache-volume-type)
      CACHE_VOLUME_TYPE="${2:-}"
      CACHE_VOLUME_TYPE_EXPLICIT=true
      shift 2
      ;;
    --cache-volume-iops)
      CACHE_VOLUME_IOPS="${2:-}"
      CACHE_VOLUME_IOPS_EXPLICIT=true
      shift 2
      ;;
    --cache-volume-throughput-mbps)
      CACHE_VOLUME_THROUGHPUT_MBPS="${2:-}"
      CACHE_VOLUME_THROUGHPUT_EXPLICIT=true
      shift 2
      ;;
    --cache-volume-device-name)
      CACHE_VOLUME_DEVICE_NAME="${2:-}"
      shift 2
      ;;
    --cache-volume-mount-path)
      CACHE_VOLUME_MOUNT_PATH="${2:-}"
      shift 2
      ;;
    --runtime-continuity-volume-id)
      RUNTIME_CONTINUITY_VOLUME_ID="${2:-}"
      shift 2
      ;;
    --runtime-continuity-volume-name)
      RUNTIME_CONTINUITY_VOLUME_NAME="${2:-}"
      shift 2
      ;;
    --runtime-continuity-volume-id-sha256)
      RUNTIME_CONTINUITY_VOLUME_ID_SHA256="${2:-}"
      shift 2
      ;;
    --ssh-key-name)
      SSH_KEY_NAME="${2:-}"
      shift 2
      ;;
    --ssh-private-key-path)
      SSH_PRIVATE_KEY_PATH="${2:-}"
      shift 2
      ;;
    --ssh-user)
      SSH_USER="${2:-}"
      shift 2
      ;;
    --ssh-allowed-cidr)
      SSH_ALLOWED_CIDR="${2:-}"
      shift 2
      ;;
    --builder-image)
      BUILDER_IMAGE="${2:-}"
      shift 2
      ;;
    --builder-image-repository)
      BUILDER_IMAGE_REPOSITORY="${2:-}"
      shift 2
      ;;
    --builder-image-tag)
      BUILDER_IMAGE_TAG="${2:-}"
      shift 2
      ;;
    --expected-architecture)
      EXPECTED_ARCHITECTURE="${2:-}"
      shift 2
      ;;
    --min-cache-free-gib)
      MIN_CACHE_FREE_GIB="${2:-}"
      shift 2
      ;;
    --estimated-hourly-cost-usd)
      ESTIMATED_HOURLY_COST_USD="${2:-}"
      shift 2
      ;;
    --max-run-seconds)
      MAX_RUN_SECONDS="${2:-}"
      shift 2
      ;;
    --max-spot-retries)
      MAX_SPOT_RETRIES="${2:-}"
      shift 2
      ;;
    --ami-id)
      AMI_ID="${2:-}"
      shift 2
      ;;
    --subnet-id)
      SUBNET_ID="${2:-}"
      shift 2
      ;;
    --expected-cache-volume-id-sha256)
      EXPECTED_CACHE_VOLUME_ID_SHA256="${2:-}"
      shift 2
      ;;
    --expected-proof)
      EXPECTED_PROOF="${2:-}"
      shift 2
      ;;
    --bin)
      LANE_BIN="${2:-}"
      shift 2
      ;;
    --json)
      JSON=true
      shift
      ;;
    --follow)
      FOLLOW=true
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "run_aws_spot_remote_validation_lane: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -n "$PORTABLE_REQUEST" ]]; then
  if [[ "$COMMAND_EXPLICIT" == true || "$GIT_REF_EXPLICIT" == true ]]; then
    echo "run_aws_spot_remote_validation_lane: portable request conflicts with command/ref overrides" >&2
    exit 2
  fi
  if [[ ! -x "$PORTABLE_RUNNER" ]]; then
    echo "run_aws_spot_remote_validation_lane: portable runner is missing or not executable" >&2
    exit 2
  fi
  PORTABLE_PLAN="$($PORTABLE_RUNNER adapter-plan aws "$PORTABLE_REQUEST")" || {
    echo "run_aws_spot_remote_validation_lane: portable request was rejected" >&2
    exit 2
  }
  COMMAND="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["shell_command"])')"
  GIT_REF="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["source_ref"])')"
  PORTABLE_EXPECTED_REVISION="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["revision"])')"
  MAX_RUN_SECONDS="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["resource_budget"]["timeout_seconds"])')"
  PORTABLE_MAX_COST_USD="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; value=json.load(sys.stdin)["resource_budget"].get("estimated_max_cost_microusd"); print("" if value is None else format(value / 1000000, ".6f"))')"
  PORTABLE_CPU_CORES="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["resource_budget"]["cpu_cores"])')"
  PORTABLE_MEMORY_MIB="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["resource_budget"]["memory_mib"])')"
  PORTABLE_CANCELLATION_FILE="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("cancellation_file") or "")')"
  PORTABLE_FALLBACK="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["fallback"])')"
  PORTABLE_PROFILE_DIGEST="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["command_profile_digest"])')"
  PORTABLE_ARTIFACT_POLICY_JSON="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.dumps(json.load(sys.stdin)["artifact_policy"], separators=(",", ":")))')"
fi

if [[ -n "$PORTABLE_MAX_COST_USD" && -z "$ESTIMATED_HOURLY_COST_USD" && "$RUN" != true && "$ACTION" != "preflight" ]]; then
  echo "run_aws_spot_remote_validation_lane: portable AWS planning requires --estimated-hourly-cost-usd" >&2
  exit 2
fi

if [[ "$ACTION" == "launch" ]]; then
  RUN=true
elif [[ "$ACTION" == "preflight" ]]; then
  CHECK_ACCOUNT=true
elif [[ "$ACTION" == "run" && "$RUN" != true ]]; then
  echo "run_aws_spot_remote_validation_lane: the run action requires explicit --run" >&2
  exit 2
fi

if [[ ${#INSTANCE_TYPES[@]} -eq 0 ]]; then
  INSTANCE_TYPES=("m7a.2xlarge" "c7a.2xlarge" "c7i.2xlarge")
fi

if [[ -z "$PROFILE" ]]; then
  echo "run_aws_spot_remote_validation_lane: --profile must not be empty" >&2
  exit 2
fi

if [[ -z "$CACHE_VOLUME_NAME" ]]; then
  echo "run_aws_spot_remote_validation_lane: cache volume name must not be empty" >&2
  exit 2
fi

if [[ ! "$MIN_CACHE_FREE_GIB" =~ ^[0-9]+$ ]] || [[ "$MIN_CACHE_FREE_GIB" -lt 1 ]]; then
  echo "run_aws_spot_remote_validation_lane: --min-cache-free-gib must be a positive integer" >&2
  exit 2
fi
if [[ -n "$ESTIMATED_HOURLY_COST_USD" ]] && [[ ! "$ESTIMATED_HOURLY_COST_USD" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
  echo "run_aws_spot_remote_validation_lane: --estimated-hourly-cost-usd must be numeric" >&2
  exit 2
fi
if [[ ! "$MAX_SPOT_RETRIES" =~ ^[0-9]+$ ]]; then
  echo "run_aws_spot_remote_validation_lane: --max-spot-retries must be a non-negative integer" >&2
  exit 2
fi

if [[ -z "$GIT_REF" ]]; then
  GIT_REF="$(git -C "$ROOT" symbolic-ref --quiet --short HEAD 2>/dev/null || git -C "$ROOT" rev-parse HEAD)"
fi
SOURCE_COMMIT="$(git -C "$ROOT" rev-parse "${GIT_REF}^{commit}" 2>/dev/null || true)"
if [[ "$RUN" == true && ! "$SOURCE_COMMIT" =~ ^[0-9a-f]{40}$ ]]; then
  echo "run_aws_spot_remote_validation_lane: --git-ref must resolve to a committed source revision" >&2
  exit 2
fi
if [[ -n "$PORTABLE_EXPECTED_REVISION" && "$SOURCE_COMMIT" != "$PORTABLE_EXPECTED_REVISION" ]]; then
  echo "run_aws_spot_remote_validation_lane: portable source ref does not resolve to requested revision" >&2
  exit 2
fi
if [[ -n "$PORTABLE_CANCELLATION_FILE" && -e "$ROOT/$PORTABLE_CANCELLATION_FILE" ]]; then
  echo "run_aws_spot_remote_validation_lane: portable cancellation requested before provider use" >&2
  exit 130
fi

if [[ -z "$OUT_PATH" ]]; then
  OUT_PATH="$ROOT/.adl/tmp/aws-spot-remote-validation/$RUN_ID/summary.json"
fi

if [[ -z "$ARTIFACT_DIR" ]]; then
  ARTIFACT_DIR="$(dirname "$OUT_PATH")/artifacts"
fi

if [[ -z "$LANE_BIN" ]]; then
  if [[ -x "$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation" ]]; then
    LANE_BIN="$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation"
  else
    LANE_BIN="$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation"
  fi
fi

if [[ ! -x "$LANE_BIN" ]]; then
  echo "run_aws_spot_remote_validation_lane: dedicated remote-validation binary is missing: $LANE_BIN" >&2
  echo "build it with: cargo build --locked --manifest-path tools/aws_remote_validation/Cargo.toml --bin adl-aws-remote-validation" >&2
  exit 2
fi
if ! "$LANE_BIN" --help 2>&1 | grep -F -- "--spot-only" >/dev/null; then
  echo "run_aws_spot_remote_validation_lane: selected binary does not implement the required Spot contract: $LANE_BIN" >&2
  exit 2
fi

check_account() {
  local identity_json
  local aws_profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    aws_profile_args=(--profile "$PROFILE")
  fi
  if ! identity_json="$("$AWS_CLI" sts get-caller-identity "${aws_profile_args[@]}" --output json)"; then
    return 1
  fi
  local account_status=0
  ADL_AWS_IDENTITY_JSON="$identity_json" python3 - "$EXPECTED_PROOF" "$PROFILE" <<'PY' || account_status="$?"
import hashlib
import json
import os
import sys

proof_path, profile = sys.argv[1:3]
identity = json.loads(os.environ["ADL_AWS_IDENTITY_JSON"])
proof = json.load(open(proof_path, encoding="utf-8"))
account = identity.get("Account")
if not account:
    raise SystemExit("run_aws_spot_remote_validation_lane: AWS profile did not return an account")
expected = (proof.get("account_identity") or {}).get("account_id_sha256")
if not expected:
    raise SystemExit("run_aws_spot_remote_validation_lane: retained proof has no account hash")
observed = hashlib.sha256(account.encode("utf-8")).hexdigest()
if observed != expected:
    raise SystemExit(
        "run_aws_spot_remote_validation_lane: AWS profile account does not match retained Agent Logic proof"
    )
arn_present = bool(identity.get("Arn"))
user_id_present = bool(identity.get("UserId"))
print(
    f"PASS account_profile_resolved profile={profile} "
    f"account_matches_retained_proof=true arn_present={str(arn_present).lower()} "
    f"user_id_present={str(user_id_present).lower()}"
)
PY
  return "$account_status"
}

resolve_builder_image() {
  if [[ -n "$BUILDER_IMAGE" ]]; then
    [[ "$BUILDER_IMAGE" =~ @sha256:[0-9a-f]{64}$ ]] || {
      echo "run_aws_spot_remote_validation_lane: --builder-image must use an immutable sha256 digest" >&2
      return 2
    }
    return 0
  fi
  local profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  local account digest
  account="$("$AWS_CLI" sts get-caller-identity "${profile_args[@]}" --query Account --output text)"
  digest="$("$AWS_CLI" ecr describe-images "${profile_args[@]}" --region "$REGION" \
    --repository-name "$BUILDER_IMAGE_REPOSITORY" \
    --image-ids "imageTag=$BUILDER_IMAGE_TAG" \
    --query 'imageDetails[0].imageDigest' --output text)"
  if [[ ! "$account" =~ ^[0-9]{12}$ ]] || [[ ! "$digest" =~ ^sha256:[0-9a-f]{64}$ ]]; then
    echo "run_aws_spot_remote_validation_lane: failed to resolve immutable Agent Logic builder image" >&2
    return 1
  fi
  BUILDER_IMAGE="$account.dkr.ecr.$REGION.amazonaws.com/$BUILDER_IMAGE_REPOSITORY@$digest"
}

resolve_spot_hourly_cost() {
  if [[ -n "$ESTIMATED_HOURLY_COST_USD" ]]; then
    return 0
  fi
  local profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  local price_json
  price_json="$("$AWS_CLI" ec2 describe-spot-price-history \
    "${profile_args[@]}" --region "$REGION" --instance-types "${INSTANCE_TYPES[@]}" \
    --product-descriptions Linux/UNIX --max-items 20 --output json)"
  ESTIMATED_HOURLY_COST_USD="$(python3 -c 'import json,sys; values=[float(x["SpotPrice"]) for x in json.load(sys.stdin).get("SpotPriceHistory",[])]; print(max(values) if values else "")' <<<"$price_json")"
  if [[ ! "$ESTIMATED_HOURLY_COST_USD" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
    echo "run_aws_spot_remote_validation_lane: failed to resolve Spot hourly price" >&2
    return 1
  fi
}

validate_portable_capacity_and_cost() {
  [[ -n "$PORTABLE_REQUEST" ]] || return 0
  [[ "$PORTABLE_MAX_COST_USD" =~ ^[0-9]+([.][0-9]+)?$ ]] || {
    echo "run_aws_spot_remote_validation_lane: portable AWS request requires a nonzero cost ceiling" >&2
    return 2
  }
  python3 - "$PORTABLE_MAX_COST_USD" "$ESTIMATED_HOURLY_COST_USD" "$MAX_RUN_SECONDS" <<'PY'
import sys
ceiling, hourly, seconds = float(sys.argv[1]), float(sys.argv[2]), int(sys.argv[3])
projected = hourly * seconds / 3600.0
if ceiling <= 0:
    raise SystemExit("run_aws_spot_remote_validation_lane: portable AWS cost ceiling must be greater than zero")
if projected > ceiling:
    raise SystemExit(
        f"run_aws_spot_remote_validation_lane: projected cost ${projected:.6f} exceeds portable ceiling ${ceiling:.6f}"
    )
PY
  local profile_args=() instance_type capacity
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  for instance_type in "${INSTANCE_TYPES[@]}"; do
    capacity="$("$AWS_CLI" ec2 describe-instance-types "${profile_args[@]}" --region "$REGION" \
      --instance-types "$instance_type" --query 'InstanceTypes[0].[VCpuInfo.DefaultVCpus,MemoryInfo.SizeInMiB]' --output text)"
    python3 - "$instance_type" "$capacity" "$PORTABLE_CPU_CORES" "$PORTABLE_MEMORY_MIB" <<'PY'
import sys
instance_type, capacity, required_cpu, required_memory = sys.argv[1:]
parts = capacity.split()
if len(parts) != 2 or int(parts[0]) < int(required_cpu) or int(parts[1]) < int(required_memory):
    raise SystemExit(
        f"run_aws_spot_remote_validation_lane: instance type {instance_type} does not satisfy portable CPU/memory request"
    )
PY
  done
}

resolve_and_verify_retained_topology() {
  local proof_topology proof_volume_id proof_subnet_id proof_volume_hash
  local proof_volume_size proof_volume_type proof_volume_iops proof_volume_throughput
  proof_topology="$(python3 - "$EXPECTED_PROOF" <<'PY'
import hashlib
import json
import sys

proof = json.load(open(sys.argv[1], encoding="utf-8"))
volume = proof.get("cache_volume") or {}
surface = proof.get("launch_surface") or {}
volume_id = volume.get("volume_id", "")
subnet_id = surface.get("subnet_id", "")
if not volume_id or not subnet_id:
    raise SystemExit("retained proof is missing cache volume or subnet identity")
print(
    volume_id,
    subnet_id,
    hashlib.sha256(volume_id.encode()).hexdigest(),
    volume.get("size_gib", ""),
    volume.get("volume_type", ""),
    volume.get("iops", ""),
    volume.get("throughput_mbps", ""),
)
PY
)"
  read -r proof_volume_id proof_subnet_id proof_volume_hash proof_volume_size proof_volume_type proof_volume_iops proof_volume_throughput <<<"$proof_topology"
  RETAINED_CACHE_VOLUME_ID="$proof_volume_id"
  if [[ -z "$SUBNET_ID" ]]; then
    SUBNET_ID="$proof_subnet_id"
  fi
  if [[ "$CACHE_VOLUME_SIZE_GIB_EXPLICIT" != true && -n "$proof_volume_size" && "$proof_volume_size" != "None" ]]; then
    CACHE_VOLUME_SIZE_GIB="$proof_volume_size"
  fi
  if [[ "$CACHE_VOLUME_TYPE_EXPLICIT" != true && -n "$proof_volume_type" && "$proof_volume_type" != "None" ]]; then
    CACHE_VOLUME_TYPE="$proof_volume_type"
  fi
  if [[ "$CACHE_VOLUME_IOPS_EXPLICIT" != true && -n "$proof_volume_iops" && "$proof_volume_iops" != "None" ]]; then
    CACHE_VOLUME_IOPS="$proof_volume_iops"
  fi
  if [[ "$CACHE_VOLUME_THROUGHPUT_EXPLICIT" != true && -n "$proof_volume_throughput" && "$proof_volume_throughput" != "None" ]]; then
    CACHE_VOLUME_THROUGHPUT_MBPS="$proof_volume_throughput"
  fi
  CACHE_VOLUME_SIZE_GIB="${CACHE_VOLUME_SIZE_GIB:-500}"
  CACHE_VOLUME_TYPE="${CACHE_VOLUME_TYPE:-gp3}"
  CACHE_VOLUME_IOPS="${CACHE_VOLUME_IOPS:-3000}"
  CACHE_VOLUME_THROUGHPUT_MBPS="${CACHE_VOLUME_THROUGHPUT_MBPS:-125}"
  if [[ -z "$EXPECTED_CACHE_VOLUME_ID_SHA256" ]]; then
    EXPECTED_CACHE_VOLUME_ID_SHA256="$proof_volume_hash"
  fi
  [[ "$EXPECTED_CACHE_VOLUME_ID_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
    echo "run_aws_spot_remote_validation_lane: expected cache volume identity hash is invalid" >&2
    return 1
  }

  local profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  local volume_state volume_name volume_az subnet_az volume_hash matching_volume_count
  local volume_size volume_type volume_iops volume_throughput
  volume_hash="$(python3 -c 'import hashlib,sys; print(hashlib.sha256(sys.argv[1].encode()).hexdigest())' "$proof_volume_id")"
  [[ "$volume_hash" == "$EXPECTED_CACHE_VOLUME_ID_SHA256" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained proof cache identity mismatch" >&2
    return 1
  }
  volume_state="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].State' --output text)"
  volume_name="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].Tags[?Key==`Name`].Value|[0]' --output text)"
  volume_az="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].AvailabilityZone' --output text)"
  volume_size="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].Size' --output text)"
  volume_type="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].VolumeType' --output text)"
  volume_iops="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].Iops' --output text)"
  volume_throughput="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" \
    --volume-ids "$proof_volume_id" --query 'Volumes[0].Throughput' --output text)"
  subnet_az="$("$AWS_CLI" ec2 describe-subnets "${profile_args[@]}" --region "$REGION" \
    --subnet-ids "$SUBNET_ID" --query 'Subnets[0].AvailabilityZone' --output text)"
  [[ "$volume_state" == "available" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained cache volume is not exclusively available" >&2
    return 1
  }
  [[ "$volume_name" == "$CACHE_VOLUME_NAME" && -n "$volume_az" && "$volume_az" == "$subnet_az" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained cache volume and subnet topology mismatch" >&2
    return 1
  }
  matching_volume_count="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" \
    --filters "Name=tag:Name,Values=$CACHE_VOLUME_NAME" "Name=availability-zone,Values=$volume_az" \
    --query 'length(Volumes)' --output text)"
  [[ "$matching_volume_count" == "1" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained cache identity is ambiguous in the selected availability zone" >&2
    return 1
  }
  if [[ "$CACHE_VOLUME_SIZE_GIB_EXPLICIT" != true ]]; then
    CACHE_VOLUME_SIZE_GIB="$volume_size"
  fi
  if [[ "$CACHE_VOLUME_TYPE_EXPLICIT" != true ]]; then
    CACHE_VOLUME_TYPE="$volume_type"
  fi
  if [[ "$CACHE_VOLUME_IOPS_EXPLICIT" != true ]]; then
    CACHE_VOLUME_IOPS="$volume_iops"
  fi
  if [[ "$CACHE_VOLUME_THROUGHPUT_EXPLICIT" != true ]]; then
    CACHE_VOLUME_THROUGHPUT_MBPS="$volume_throughput"
  fi
  [[ "$volume_size" == "$CACHE_VOLUME_SIZE_GIB" && "$volume_type" == "$CACHE_VOLUME_TYPE" \
      && "$volume_iops" == "$CACHE_VOLUME_IOPS" && "$volume_throughput" == "$CACHE_VOLUME_THROUGHPUT_MBPS" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained cache volume shape mismatch" >&2
    return 1
  }
  if [[ -z "$AMI_ID" ]]; then
    AMI_ID="$("$AWS_CLI" ssm get-parameter "${profile_args[@]}" --region "$REGION" \
      --name /aws/service/ami-amazon-linux-latest/al2023-ami-kernel-default-x86_64 \
      --query 'Parameter.Value' --output text)"
  fi
  [[ "$AMI_ID" =~ ^ami-[0-9a-f]{8,17}$ && "$SUBNET_ID" =~ ^subnet-[0-9a-f]{8,17}$ ]] || {
    echo "run_aws_spot_remote_validation_lane: AMI or subnet resolution failed" >&2
    return 1
  }
}

select_runtime_continuity_volume() {
  [[ -n "$RUNTIME_CONTINUITY_VOLUME_ID" || -n "$RUNTIME_CONTINUITY_VOLUME_NAME" \
      || -n "$RUNTIME_CONTINUITY_VOLUME_ID_SHA256" ]] || return 0
  [[ "$RUNTIME_CONTINUITY_VOLUME_ID" =~ ^vol-[0-9a-f]{8,17}$ \
      && -n "$RUNTIME_CONTINUITY_VOLUME_NAME" \
      && "$RUNTIME_CONTINUITY_VOLUME_ID_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
    echo "run_aws_spot_remote_validation_lane: Runtime continuity volume requires exact id, name, and sha256" >&2
    return 1
  }
  local actual_hash profile_args=() state name az subnet_az count
  actual_hash="$(python3 -c 'import hashlib,sys; print(hashlib.sha256(sys.argv[1].encode()).hexdigest())' "$RUNTIME_CONTINUITY_VOLUME_ID")"
  [[ "$actual_hash" == "$RUNTIME_CONTINUITY_VOLUME_ID_SHA256" ]] || {
    echo "run_aws_spot_remote_validation_lane: Runtime continuity volume identity mismatch" >&2
    return 1
  }
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  state="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" --volume-ids "$RUNTIME_CONTINUITY_VOLUME_ID" --query 'Volumes[0].State' --output text)"
  name="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" --volume-ids "$RUNTIME_CONTINUITY_VOLUME_ID" --query 'Volumes[0].Tags[?Key==`Name`].Value|[0]' --output text)"
  az="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" --volume-ids "$RUNTIME_CONTINUITY_VOLUME_ID" --query 'Volumes[0].AvailabilityZone' --output text)"
  subnet_az="$("$AWS_CLI" ec2 describe-subnets "${profile_args[@]}" --region "$REGION" --subnet-ids "$SUBNET_ID" --query 'Subnets[0].AvailabilityZone' --output text)"
  count="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" --filters "Name=tag:Name,Values=$RUNTIME_CONTINUITY_VOLUME_NAME" "Name=availability-zone,Values=$az" --query 'length(Volumes)' --output text)"
  [[ "$state" == available && "$name" == "$RUNTIME_CONTINUITY_VOLUME_NAME" \
      && "$az" == "$subnet_az" && "$count" == 1 ]] || {
    echo "run_aws_spot_remote_validation_lane: Runtime continuity volume is not exclusive, exact, and colocated" >&2
    return 1
  }
  RETAINED_CACHE_VOLUME_ID="$RUNTIME_CONTINUITY_VOLUME_ID"
  CACHE_VOLUME_NAME="$RUNTIME_CONTINUITY_VOLUME_NAME"
  EXPECTED_CACHE_VOLUME_ID_SHA256="$RUNTIME_CONTINUITY_VOLUME_ID_SHA256"
  CACHE_VOLUME_MOUNT_PATH="/mnt/adl-runtime-continuity"
  CACHE_VOLUME_DEVICE_NAME="/dev/sdg"
  CACHE_VOLUME_SIZE_GIB="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" --volume-ids "$RUNTIME_CONTINUITY_VOLUME_ID" --query 'Volumes[0].Size' --output text)"
  CACHE_VOLUME_TYPE="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" --volume-ids "$RUNTIME_CONTINUITY_VOLUME_ID" --query 'Volumes[0].VolumeType' --output text)"
  CACHE_VOLUME_IOPS="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" --volume-ids "$RUNTIME_CONTINUITY_VOLUME_ID" --query 'Volumes[0].Iops' --output text)"
  CACHE_VOLUME_THROUGHPUT_MBPS="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" --volume-ids "$RUNTIME_CONTINUITY_VOLUME_ID" --query 'Volumes[0].Throughput' --output text)"
}

shell_quote() {
  printf '%q' "$1"
}

verify_ssh_recovery_key() {
  local key_mode
  [[ -n "$SSH_KEY_NAME" && -f "$SSH_PRIVATE_KEY_PATH" ]] || {
    echo "run_aws_spot_remote_validation_lane: SSH recovery key is not configured" >&2
    return 1
  }
  key_mode="$(stat -c '%a' "$SSH_PRIVATE_KEY_PATH" 2>/dev/null || stat -f '%Lp' "$SSH_PRIVATE_KEY_PATH")"
  [[ "$key_mode" == "600" || "$key_mode" == "400" ]] || {
    echo "run_aws_spot_remote_validation_lane: SSH private key permissions must be 600 or 400" >&2
    return 1
  }
  ssh-keygen -y -P '' -f "$SSH_PRIVATE_KEY_PATH" >/dev/null 2>&1 || {
    echo "run_aws_spot_remote_validation_lane: SSH private key is not passphraseless" >&2
    return 1
  }
}

redact_stream() {
  sed -E \
    -e 's/[0-9]{12}/<aws-account-id-redacted>/g' \
    -e 's#arn:aws[^[:space:],\"]*#<aws-arn-redacted>#g' \
    -e 's/i-[0-9a-f]{8,17}/<ec2-instance-id-redacted>/g' \
    -e 's/vol-[0-9a-f]{8,17}/<ebs-volume-id-redacted>/g' \
    -e 's/(vpc|subnet|sg|sir)-[0-9a-f]{8,17}/<aws-resource-id-redacted>/g' \
    -e 's/([0-9]{1,3}\.){3}[0-9]{1,3}/<ip-address-redacted>/g' \
    -e 's#/(Users|Volumes|private|tmp)/[^[:space:],"]*#<machine-path-redacted>#g'
}

manager_is_active() {
  local pid_file="$ARTIFACT_DIR/manager.pid"
  [[ -f "$pid_file" ]] || return 1
  [[ -x "$PROCESS_BIN" ]] || return 1
  "$PROCESS_BIN" process status --pid-file "$pid_file" --json 2>/dev/null \
    | python3 -c 'import json,sys; data=json.load(sys.stdin); raise SystemExit(0 if data.get("running") or data.get("status") == "running" else 1)'
}

private_command_status_path() {
  if [[ -f "$ARTIFACT_DIR/.private/command-status.log" ]]; then
    printf '%s\n' "$ARTIFACT_DIR/.private/command-status.log"
  elif find "$ARTIFACT_DIR" -maxdepth 2 -path '*/attempt-*/command-status.log' -type f -print -quit 2>/dev/null | grep -q .; then
    find "$ARTIFACT_DIR" -maxdepth 2 -path '*/attempt-*/command-status.log' -type f -print 2>/dev/null | sort | tail -n 1
  else
    printf '%s\n' "$ARTIFACT_DIR/command-status.log"
  fi
}

run_status_action() {
  if manager_is_active; then
    printf 'status=running run_id=%s\n' "$RUN_ID"
  elif [[ -f "$ARTIFACT_DIR/wrapper-final-summary.json" ]]; then
    cat "$ARTIFACT_DIR/wrapper-final-summary.json"
  elif [[ -d "$ARTIFACT_DIR" ]]; then
    printf 'status=incomplete run_id=%s action=inspect_logs_or_cleanup\n' "$RUN_ID"
    return 1
  else
    printf 'status=not_found run_id=%s\n' "$RUN_ID"
    return 1
  fi
}

run_logs_action() {
  local files=()
  for path in "$ARTIFACT_DIR/manager.stderr.log" "$ARTIFACT_DIR/remote-tail.log" "$ARTIFACT_DIR/command-status.log" "$ARTIFACT_DIR/manager.stdout.log"; do
    [[ -f "$path" ]] && files+=("$path")
  done
  while IFS= read -r path; do
    [[ -f "$path" ]] && files+=("$path")
  done < <(find "$ARTIFACT_DIR" -maxdepth 2 -path '*/attempt-*/*' -type f \
    \( -name 'command-status.log' -o -name 'remote-tail.log' \) -print 2>/dev/null | sort)
  if [[ ${#files[@]} -eq 0 ]]; then
    echo "run_aws_spot_remote_validation_lane: no logs found for run id $RUN_ID" >&2
    return 1
  fi
  if [[ "$FOLLOW" == true ]]; then
    tail -n 80 -F "${files[@]}" | redact_stream
  else
    tail -n 120 "${files[@]}" | redact_stream
  fi
}

run_ssh_action() {
  local status_path public_ip
  status_path="$(private_command_status_path)"
  [[ -f "$status_path" ]] || {
    echo "run_aws_spot_remote_validation_lane: SSH control state is not available" >&2
    return 1
  }
  public_ip="$(sed -nE 's/.*public_ip=([0-9.]+).*/\1/p' "$status_path" | tail -n 1)"
  [[ "$public_ip" =~ ^([0-9]{1,3}[.]){3}[0-9]{1,3}$ ]] || {
    echo "run_aws_spot_remote_validation_lane: active SSH endpoint is not available" >&2
    return 1
  }
  verify_ssh_recovery_key
  exec "$SSH_BIN" -o BatchMode=yes -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
    -o ServerAliveInterval=5 -o ServerAliveCountMax=2 \
    -i "$SSH_PRIVATE_KEY_PATH" "$SSH_USER@$public_ip"
}

run_stop_action() {
  check_account
  local status_path instance_id observed_run_id
  status_path="$(private_command_status_path)"
  [[ -f "$status_path" ]] || {
    echo "run_aws_spot_remote_validation_lane: no private instance control state for $RUN_ID" >&2
    return 1
  }
  instance_id="$(sed -nE 's/.*instance_id=(i-[0-9a-f]+).*/\1/p' "$status_path" | tail -n 1)"
  [[ "$instance_id" =~ ^i-[0-9a-f]{8,17}$ ]] || {
    echo "run_aws_spot_remote_validation_lane: invalid instance control state" >&2
    return 1
  }
  local profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  observed_run_id="$("$AWS_CLI" ec2 describe-instances "${profile_args[@]}" --region "$REGION" \
    --instance-ids "$instance_id" --query 'Reservations[0].Instances[0].Tags[?Key==`adl:run_id`].Value|[0]' --output text)"
  [[ "$observed_run_id" == "$RUN_ID" ]] || {
    echo "run_aws_spot_remote_validation_lane: instance run-id tag mismatch; refusing termination" >&2
    return 1
  }
  "$AWS_CLI" ec2 terminate-instances "${profile_args[@]}" --region "$REGION" --instance-ids "$instance_id" >/dev/null
  "$AWS_CLI" ec2 wait instance-terminated "${profile_args[@]}" --region "$REGION" --instance-ids "$instance_id"
  printf 'status=terminated run_id=%s retained_cache_preserved=true\n' "$RUN_ID"
}

run_cleanup_action() {
  if manager_is_active; then
    run_stop_action
  fi
  local profile_args=()
  if [[ "$PROFILE" != "env" && "$PROFILE" != "environment" ]]; then
    profile_args=(--profile "$PROFILE")
  fi
  check_account
  local volume_state
  volume_state="$("$AWS_CLI" ec2 describe-volumes "${profile_args[@]}" --region "$REGION" \
    --filters "Name=tag:Name,Values=$CACHE_VOLUME_NAME" \
    --query 'Volumes[0].State' --output text)"
  [[ "$volume_state" == "available" || "$volume_state" == "in-use" ]] || {
    echo "run_aws_spot_remote_validation_lane: retained cache volume is missing or unhealthy" >&2
    return 1
  }
  printf 'status=clean retained_cache_preserved=true cache_state=%s run_id=%s\n' "$volume_state" "$RUN_ID"
}

case "$ACTION" in
  status) run_status_action; exit $? ;;
  logs) run_logs_action; exit $? ;;
  ssh) run_ssh_action ;;
  stop) run_stop_action; exit $? ;;
  cleanup) run_cleanup_action; exit $? ;;
esac

if [[ "$CHECK_ACCOUNT" == true || "$RUN" == true ]]; then
  check_account
fi

DIRECT_HOST_RUNTIME=false
VALIDATION_ENVIRONMENT=immutable_builder
if [[ "$COMMAND" == "bash adl/tools/run_issue268_remote_resident_qualification.sh" ]]; then
  DIRECT_HOST_RUNTIME=true
  VALIDATION_ENVIRONMENT=direct_host_runtime
fi

if [[ "$RUN" == true || "$ACTION" == "preflight" ]]; then
  if [[ "$DIRECT_HOST_RUNTIME" != true ]]; then
    resolve_builder_image
  fi
  resolve_spot_hourly_cost
  validate_portable_capacity_and_cost
  resolve_and_verify_retained_topology
  select_runtime_continuity_volume
  verify_ssh_recovery_key
fi

if [[ "$ACTION" == "preflight" ]]; then
  python3 - "$BUILDER_IMAGE" "$SOURCE_COMMIT" "$EXPECTED_CACHE_VOLUME_ID_SHA256" "$AMI_ID" "$SUBNET_ID" "$ESTIMATED_HOURLY_COST_USD" <<'PY'
import hashlib
import json
import sys

image, commit, cache_hash, ami, subnet, hourly = sys.argv[1:]
payload = {
    "schema": "adl.aws_spot_preflight.v1",
    "status": "ready",
    "account_matches_retained_proof": True,
    "source_commit": commit,
    "builder_image_digest_sha256": hashlib.sha256(image.rsplit("@", 1)[-1].encode()).hexdigest(),
    "builder_image_immutable": "@sha256:" in image,
    "retained_cache_volume_id_sha256": cache_hash,
    "retained_cache_available": True,
    "ami_id_sha256": hashlib.sha256(ami.encode()).hexdigest(),
    "subnet_id_sha256": hashlib.sha256(subnet.encode()).hexdigest(),
    "ssh_recovery_configured": True,
    "estimated_hourly_cost_usd": float(hourly),
    "aws_resources_created": False,
}
print(json.dumps(payload, indent=2, sort_keys=True))
PY
  exit 0
fi

if [[ -z "$COMMAND" ]]; then
  if [[ "$RUN" == true ]]; then
    echo "run_aws_spot_remote_validation_lane: --command is required when --run is set" >&2
    exit 2
  fi
fi

cmd=(
  "$LANE_BIN"
  run
  --issue "$ISSUE"
  --run-id "$RUN_ID"
  --profile "$PROFILE"
  --region "$REGION"
  --repo-url "$REPO_URL"
  --git-ref "$GIT_REF"
  --out "$OUT_PATH"
  --artifact-dir "$ARTIFACT_DIR"
  --spot-only
  --max-spot-retries "$MAX_SPOT_RETRIES"
  --cache-volume-id "$RETAINED_CACHE_VOLUME_ID"
  --cache-volume-name "$CACHE_VOLUME_NAME"
  --cache-volume-size-gib "$CACHE_VOLUME_SIZE_GIB"
  --cache-volume-type "$CACHE_VOLUME_TYPE"
  --cache-volume-iops "$CACHE_VOLUME_IOPS"
  --cache-volume-throughput-mbps "$CACHE_VOLUME_THROUGHPUT_MBPS"
  --cache-volume-device-name "$CACHE_VOLUME_DEVICE_NAME"
  --cache-volume-mount-path "$CACHE_VOLUME_MOUNT_PATH"
  --ami-id "$AMI_ID"
  --subnet-id "$SUBNET_ID"
)

if [[ -n "$SSH_KEY_NAME" ]]; then
  cmd+=(--ssh-key-name "$SSH_KEY_NAME")
  cmd+=(--ssh-private-key-path "$SSH_PRIVATE_KEY_PATH")
  cmd+=(--ssh-user "$SSH_USER")
  if [[ -n "$SSH_ALLOWED_CIDR" ]]; then
    cmd+=(--ssh-allowed-cidr "$SSH_ALLOWED_CIDR")
  fi
fi

if [[ -n "$COMMAND" ]]; then
  if [[ "$RUN" == true ]]; then
    if [[ "$DIRECT_HOST_RUNTIME" == true ]]; then
      remote_command="$COMMAND"
    else
      remote_command="bash adl/tools/run_aws_spot_builder_image_validation.sh"
      remote_command+=" --image $(shell_quote "$BUILDER_IMAGE")"
      remote_command+=" --expected-ref $(shell_quote "$SOURCE_COMMIT")"
      remote_command+=" --expected-architecture $(shell_quote "$EXPECTED_ARCHITECTURE")"
      remote_command+=" --min-cache-free-gib $(shell_quote "$MIN_CACHE_FREE_GIB")"
      remote_command+=" --command $(shell_quote "$COMMAND")"
    fi
    cmd+=(--command "$remote_command")
  else
    cmd+=(--command "$COMMAND")
  fi
fi

if [[ -n "$MAX_RUN_SECONDS" ]]; then
  cmd+=(--command-timeout-seconds "$MAX_RUN_SECONDS")
fi

if [[ -n "$PORTABLE_MAX_COST_USD" ]]; then
  cmd+=(
    --expected-max-cost-usd "$PORTABLE_MAX_COST_USD"
    --estimated-hourly-cost-usd "$ESTIMATED_HOURLY_COST_USD"
    --total-run-timeout-seconds "$MAX_RUN_SECONDS"
    --spot-only
  )
fi
if [[ -n "$PORTABLE_CANCELLATION_FILE" ]]; then
  cmd+=(--cancellation-file "$ROOT/$PORTABLE_CANCELLATION_FILE")
fi

for instance_type in ${INSTANCE_TYPES[@]+"${INSTANCE_TYPES[@]}"}; do
  cmd+=(--instance-type "$instance_type")
done

if [[ "$JSON" == true ]]; then
  cmd+=(--json)
fi

if [[ "$PRINT_COMMAND" == true ]]; then
  printf '%q ' "${cmd[@]}" | redact_stream
  printf '\n'
fi

if [[ "$RUN" != true ]]; then
  echo "DRY-RUN aws_spot_remote_validation profile=$PROFILE region=$REGION git_ref=$GIT_REF source_commit_resolved=$([[ "$SOURCE_COMMIT" =~ ^[0-9a-f]{40}$ ]] && printf true || printf false) out=$OUT_PATH artifact_dir=$ARTIFACT_DIR cache_volume=$CACHE_VOLUME_NAME cache_mount=$CACHE_VOLUME_MOUNT_PATH ssh_tail_enabled=$([[ -n "$SSH_KEY_NAME" ]] && printf true || printf false) builder_image_mode=immutable_digest"
  echo "DRY-RUN no EC2 resources launched; pass --run to execute"
  exit 0
fi

if [[ ! -x "$LANE_BIN" ]]; then
  echo "run_aws_spot_remote_validation_lane: binary not executable: $LANE_BIN" >&2
  exit 2
fi

execute_run() {
  mkdir -p "$(dirname "$OUT_PATH")" "$ARTIFACT_DIR"
  mkdir -p "$ARTIFACT_DIR/.private"
  chmod 700 "$ARTIFACT_DIR/.private"
  local runner_stdout="$ARTIFACT_DIR/runner.stdout.log"
  local runner_stderr="$ARTIFACT_DIR/runner.stderr.log"
  local runner_status finalize_status wrapper_summary started_unix_ms finished_unix_ms
  local retained_volume_role="build_cache"
  if [[ -n "$RUNTIME_CONTINUITY_VOLUME_ID" ]]; then
    retained_volume_role="runtime_continuity"
  fi

  started_unix_ms="$(python3 -c 'import time; print(time.time_ns() // 1000000)')"

  set +e
  # Retain stdout and stderr separately without relying on /dev/fd process
  # substitution, which is unavailable on some bounded runners.
  ADL_SSH_KNOWN_HOSTS_FILE="$ARTIFACT_DIR/.private/ssh-known-hosts" \
    "${cmd[@]}" >"$runner_stdout" 2>"$runner_stderr"
  runner_status="$?"
  set -e
  if [[ -z "$PORTABLE_REQUEST" ]]; then
    redact_stream <"$runner_stdout"
  fi
  redact_stream <"$runner_stderr" >&2

  wrapper_summary="$ARTIFACT_DIR/wrapper-final-summary.json"
  finalize_status=0
  python3 "$ROOT/adl/tools/aws_spot_artifact_finalize.py" \
    --summary "$OUT_PATH" \
    --artifact-dir "$ARTIFACT_DIR" \
    --wrapper-summary "$wrapper_summary" \
    --expected-source-commit "$SOURCE_COMMIT" \
    --expected-image "$BUILDER_IMAGE" \
    --expected-cache-volume-id-sha256 "$EXPECTED_CACHE_VOLUME_ID_SHA256" \
    --expected-retained-volume-role "$retained_volume_role" \
    --validation-environment "$VALIDATION_ENVIRONMENT" \
    --estimated-hourly-cost-usd "$ESTIMATED_HOURLY_COST_USD" \
    --runner-exit-code "$runner_status" \
    >"$ARTIFACT_DIR/finalize.out" 2>"$ARTIFACT_DIR/finalize.err" || finalize_status="$?"
  finished_unix_ms="$(python3 -c 'import time; print(time.time_ns() // 1000000)')"

  redact_stream <"$ARTIFACT_DIR/finalize.err" >&2
  printf 'aws_spot_remote_validation_wrapper_summary=%s\n' "$wrapper_summary" | redact_stream >&2
  if [[ -n "$PORTABLE_REQUEST" ]]; then
    local portable_artifact_root="$ARTIFACT_DIR/portable-artifacts"
    local execution_receipt="$ARTIFACT_DIR/portable-execution.json"
    local portable_result="$ARTIFACT_DIR/portable-result.json"
    mkdir -p "$portable_artifact_root"
    python3 - "$PORTABLE_REQUEST" "$wrapper_summary" "$ARTIFACT_DIR" "$portable_artifact_root" <<'PY'
import json
import shutil
import sys
from pathlib import Path

request_path, wrapper_path, artifact_dir, portable_root = map(Path, sys.argv[1:])
request = json.loads(request_path.read_text(encoding="utf-8"))
paths = request["artifact_policy"]["paths"]
for index, relative in enumerate(paths):
    destination = portable_root / relative
    source = artifact_dir / relative
    if not source.is_file() and index == 0:
        source = wrapper_path
    if source.is_file():
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, destination)
PY
    local portable_redaction_status=0
    python3 "$ROOT/adl/tools/aws_spot_artifact_redaction_verify.py" \
      "$portable_artifact_root" || portable_redaction_status="$?"
    python3 - "$execution_receipt" "$PORTABLE_REQUEST" "$SOURCE_COMMIT" \
      "$started_unix_ms" "$finished_unix_ms" "$runner_status" "$finalize_status" \
      "$portable_redaction_status" "$OUT_PATH" <<'PY'
import json
import sys
path, request_path, revision, started, finished, runner_status, finalize_status, redaction_status, summary_path = sys.argv[1:]
request = json.load(open(request_path, encoding="utf-8"))
summary = json.load(open(summary_path, encoding="utf-8"))
runner_status, finalize_status = int(runner_status), int(finalize_status)
passed = runner_status == 0 and finalize_status == 0
resilience = summary.get("resilience") or {}
cleanup_complete = resilience.get("cleanup_complete") is True
failure_reason = str(summary.get("failure_reason") or "").lower()
fault_class = str(resilience.get("fault_class") or "unknown")
if passed:
    outcome = "passed"
elif not cleanup_complete:
    outcome = "cleanup_incomplete"
elif "cancellation" in failure_reason or "cancelled" in failure_reason:
    outcome = "cancelled"
elif "deadline" in failure_reason or "timed out" in failure_reason:
    outcome = "timed_out"
elif fault_class in {
    "quota_blocked",
    "capacity_unavailable",
    "transient_network",
    "ssm_unavailable",
    "spot_interrupted",
  }:
    outcome = "provider_unavailable"
else:
    outcome = "failed"
fallback_allowed = (
    outcome == "provider_unavailable"
    and cleanup_complete
    and request["fallback"] != "disabled"
)
payload = {
    "schema": "adl.remote_validation.adapter_execution.v1",
    "adapter": "aws",
    "platform": {"os": "linux", "architecture": "x86_64", "native": True, "qualification": "live"},
    "revision": revision,
    "started_unix_ms": int(started),
    "finished_unix_ms": int(finished),
    "exit_code": 0 if passed else (runner_status or finalize_status),
    "outcome": outcome,
    "redaction_passed": int(redaction_status) == 0,
    "cleanup": {"attempted": True, "complete": cleanup_complete, "detail": None},
    "fallback": {
        "policy": request["fallback"],
        "offered": fallback_allowed,
        "ran": False,
        "local_profile_digest": None,
    },
}
with open(path, "w", encoding="utf-8") as handle:
    json.dump(payload, handle, separators=(",", ":"))
PY
    "$PORTABLE_RUNNER" canonical-result "$PORTABLE_REQUEST" "$execution_receipt" "$portable_artifact_root" >"$portable_result"
    cat "$portable_result"
  fi
  if [[ "$runner_status" -ne 0 ]]; then
    printf '%s\n' "$runner_status" >"$ARTIFACT_DIR/manager.exit-code"
    return "$runner_status"
  fi
  printf '%s\n' "$finalize_status" >"$ARTIFACT_DIR/manager.exit-code"
  return "$finalize_status"
}

if [[ "$ACTION" == "launch" ]]; then
  mkdir -p "$(dirname "$OUT_PATH")" "$ARTIFACT_DIR"
  launch_args=("${ORIGINAL_ARGS[@]}")
  if [[ "${launch_args[0]:-}" == "launch" ]]; then
    launch_args[0]="run"
  else
    launch_args=("run" "${launch_args[@]}")
  fi
  launch_args+=("--run")
  python3 - "$ARTIFACT_DIR/launch-state.json" "$RUN_ID" <<'PY'
import json
import sys
from pathlib import Path

Path(sys.argv[1]).write_text(json.dumps({
    "schema": "adl.aws_spot_launch_state.v1",
    "status": "launching",
    "run_id": sys.argv[2],
}, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
  nohup "${BASH:-bash}" "$SCRIPT_PATH" "${launch_args[@]}" \
    >"$ARTIFACT_DIR/manager.stdout.log" \
    2>"$ARTIFACT_DIR/manager.stderr.log" \
    </dev/null &
  manager_pid="$!"
  printf '%s\n' "$manager_pid" >"$ARTIFACT_DIR/manager.pid"
  python3 - "$ARTIFACT_DIR/launch-state.json" "$RUN_ID" "$manager_pid" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
payload = {
    "schema": "adl.aws_spot_launch_state.v1",
    "status": "launched",
    "run_id": sys.argv[2],
    "manager_pid": int(sys.argv[3]),
}
path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
  printf 'status=launched run_id=%s pid=%s\n' "$RUN_ID" "$manager_pid"
  printf 'next_status=bash adl/tools/run_aws_spot_remote_validation_lane.sh status --run-id %q\n' "$RUN_ID"
  exit 0
fi

execute_run
