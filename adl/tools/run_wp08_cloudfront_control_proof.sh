#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: bash adl/tools/run_wp08_cloudfront_control_proof.sh --out <dir> [options]

Runs WP-08 #4915 live read-only CloudFront/cloud-control proof through the
standalone csm runtime binary.

Options:
  --out <dir>             Required proof output directory.
  --profile <name>        AWS profile. Default: agent-logic-admin.
  --region <region>       AWS region/account-check region. Default: us-west-2.
  --run-id <id>           Run id. Default: wp08-4915-<utc>.
  --csm-bin <path>        csm binary. Default: ADL_CSM_BIN or adl/target/debug/csm.
  --expected-account-sha256 <hash>
                          Required approved Agent Logic account SHA-256.
                          Defaults to ADL_AWS_CLOUD_CONTROL_ACCOUNT_SHA256.
  --distribution-id <id>  Optional exact CloudFront distribution id to observe.
  --negative-distribution-id <id>
                          Nonexistent distribution id for live negative case.
                          Default: E0000000000000.
  --help                  Show this help.
USAGE
}

OUT=""
PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${ADL_AWS_REGION:-us-west-2}"
RUN_ID="wp08-4915-$(date -u +%Y%m%dT%H%M%SZ)"
CSM_BIN="${ADL_CSM_BIN:-adl/target/debug/csm}"
EXPECTED_ACCOUNT_SHA256="${ADL_AWS_CLOUD_CONTROL_ACCOUNT_SHA256:-}"
DISTRIBUTION_ID="${ADL_AWS_CLOUDFRONT_DISTRIBUTION_ID:-}"
NEGATIVE_DISTRIBUTION_ID="${ADL_AWS_CLOUDFRONT_NEGATIVE_DISTRIBUTION_ID:-E0000000000000}"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --out)
      OUT="${2:?--out requires a value}"
      shift
      ;;
    --profile)
      PROFILE="${2:?--profile requires a value}"
      shift
      ;;
    --region)
      REGION="${2:?--region requires a value}"
      shift
      ;;
    --run-id)
      RUN_ID="${2:?--run-id requires a value}"
      shift
      ;;
    --csm-bin)
      CSM_BIN="${2:?--csm-bin requires a value}"
      shift
      ;;
    --expected-account-sha256)
      EXPECTED_ACCOUNT_SHA256="${2:?--expected-account-sha256 requires a value}"
      shift
      ;;
    --distribution-id)
      DISTRIBUTION_ID="${2:?--distribution-id requires a value}"
      shift
      ;;
    --negative-distribution-id)
      NEGATIVE_DISTRIBUTION_ID="${2:?--negative-distribution-id requires a value}"
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

if [ -z "$OUT" ]; then
  echo "--out is required" >&2
  usage >&2
  exit 2
fi
if [ -z "$EXPECTED_ACCOUNT_SHA256" ]; then
  echo "expected Agent Logic account hash is required; set ADL_AWS_CLOUD_CONTROL_ACCOUNT_SHA256 or pass --expected-account-sha256" >&2
  exit 2
fi
if [ ! -x "$CSM_BIN" ]; then
  echo "csm binary not executable: $CSM_BIN" >&2
  exit 2
fi

mkdir -p "$OUT"
ARGS=(
  cloud-control cloudfront-status
  --out "$OUT"
  --run-id "$RUN_ID"
  --profile "$PROFILE"
  --region "$REGION"
  --expected-account-sha256 "$EXPECTED_ACCOUNT_SHA256"
  --negative-distribution-id "$NEGATIVE_DISTRIBUTION_ID"
)
if [ -n "$DISTRIBUTION_ID" ]; then
  ARGS+=(--distribution-id "$DISTRIBUTION_ID")
fi

ADL_AWS_PROFILE="$PROFILE" \
AWS_PROFILE="$PROFILE" \
ADL_AWS_REGION="$REGION" \
"$CSM_BIN" "${ARGS[@]}" >"$OUT/csm_cloudfront_command_result.json"

python3 adl/tools/validate_wp08_cloudfront_control_proof.py \
  "$OUT/cloudfront_status_summary.json" >/dev/null

printf 'PASS wp08_cloudfront_control_proof out=%s run_id=%s\n' "$OUT" "$RUN_ID"
