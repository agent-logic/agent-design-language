#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: bash adl/tools/run_wp08_acip_sns_live_proof.sh --out <dir> [options]

Proves WP-08 #4685 live ACIP-to-SNS publication by creating or reusing a
bounded SNS topic in the Agent Logic AWS account, running the repo-owned Rust
proof command, and writing a redacted summary.

Options:
  --out <dir>             Required proof output directory.
  --profile <name>        AWS profile. Default: agent-logic-admin.
  --region <region>       AWS region. Default: us-west-2.
  --run-id <id>           Run id suffix. Default: wp08-4685-<utc>.
  --topic-name <name>     SNS topic name. Default: adl-v0917-wp08-acip-sns-4685.
  --proof-bin <path>      csm proof command or legacy proof binary.
                          Default: ADL_ACIP_SNS_PROOF_BIN or adl/target/debug/csm.
  --expected-account-sha256 <hash>
                          Required approved Agent Logic account SHA-256.
                          Defaults to ADL_AWS_ACIP_SNS_ACCOUNT_SHA256.
  --cleanup               Delete the SNS topic after verification.
  --help                  Show this help.
USAGE
}

OUT=""
PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${ADL_AWS_REGION:-us-west-2}"
RUN_ID="wp08-4685-$(date -u +%Y%m%dT%H%M%SZ)"
TOPIC_NAME="${ADL_AWS_SNS_TOPIC_NAME:-adl-v0917-wp08-acip-sns-4685}"
if [ -n "${ADL_ACIP_SNS_PROOF_BIN:-}" ]; then
  PROOF_BIN="$ADL_ACIP_SNS_PROOF_BIN"
  PROOF_BIN_EXPLICIT=1
else
  PROOF_BIN="adl/target/debug/csm"
  PROOF_BIN_EXPLICIT=0
fi
EXPECTED_ACCOUNT_SHA256="${ADL_AWS_ACIP_SNS_ACCOUNT_SHA256:-}"
CLEANUP=0
TOPIC_ARN=""
CLEANUP_DONE=0

cleanup_topic() {
  if [ "$CLEANUP" -eq 1 ] && [ "$CLEANUP_DONE" -eq 0 ] && [ -n "$TOPIC_ARN" ]; then
    "$AWS_BIN" sns delete-topic \
      --profile "$PROFILE" \
      --region "$REGION" \
      --topic-arn "$TOPIC_ARN" >/dev/null 2>&1 || true
    CLEANUP_DONE=1
  fi
}
trap cleanup_topic EXIT

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
    --topic-name)
      TOPIC_NAME="${2:?--topic-name requires a value}"
      shift
      ;;
    --proof-bin)
      PROOF_BIN="${2:?--proof-bin requires a value}"
      PROOF_BIN_EXPLICIT=1
      shift
      ;;
    --expected-account-sha256)
      EXPECTED_ACCOUNT_SHA256="${2:?--expected-account-sha256 requires a value}"
      shift
      ;;
    --cleanup)
      CLEANUP=1
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

AWS_BIN="${AWS_BIN:-aws}"
if ! command -v "$AWS_BIN" >/dev/null 2>&1; then
  echo "aws CLI not found; set AWS_BIN or install aws CLI" >&2
  exit 2
fi

mkdir -p "$OUT"
# shellcheck source=adl/tools/csm_binary_availability.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/csm_binary_availability.sh"
if [ "$PROOF_BIN_EXPLICIT" -eq 1 ] && [ "$(basename "$PROOF_BIN")" != "csm" ]; then
  if [ ! -x "$PROOF_BIN" ]; then
    echo "explicit ACIP/SNS proof binary is not executable: $PROOF_BIN" >&2
    exit 2
  fi
else
  PROOF_BIN="$(adl_resolve_csm_binary "$PROOF_BIN" "$OUT/csm_binary_availability.json")"
fi
SUMMARY="$OUT/acip_sns_summary.json"
RESOURCE_SUMMARY="$OUT/sns_resource_summary.json"
rm -f "$SUMMARY" "$RESOURCE_SUMMARY"

ACCOUNT="$("$AWS_BIN" sts get-caller-identity --profile "$PROFILE" --query Account --output text)"
ACCOUNT_SHA256="$(printf '%s' "$ACCOUNT" | shasum -a 256 | awk '{print $1}')"
ACCOUNT_HASH="$(printf '%s' "$ACCOUNT_SHA256" | cut -c1-16)"
if [ -z "$EXPECTED_ACCOUNT_SHA256" ]; then
  echo "expected Agent Logic account hash is required; set ADL_AWS_ACIP_SNS_ACCOUNT_SHA256 or pass --expected-account-sha256" >&2
  exit 2
fi
if [ "$ACCOUNT_SHA256" != "$EXPECTED_ACCOUNT_SHA256" ]; then
  echo "AWS profile did not resolve to the approved Agent Logic account hash" >&2
  exit 2
fi
printf 'PASS account_profile_resolved profile=%s account_matches_expected=true\n' "$PROFILE" >&2

TOPIC_ARN="$("$AWS_BIN" sns create-topic \
  --profile "$PROFILE" \
  --region "$REGION" \
  --name "$TOPIC_NAME" \
  --tags Key=adl:milestone,Value=v0.91.7 Key=adl:issue,Value=4685 Key=adl:purpose,Value=wp08-acip-sns-proof \
  --query TopicArn \
  --output text)"

python3 - "$RESOURCE_SUMMARY" "$RUN_ID" "$REGION" "$PROFILE" "$ACCOUNT_HASH" "$TOPIC_ARN" "$TOPIC_NAME" "$CLEANUP" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

path, run_id, region, profile, account_hash, topic_arn, topic_name, cleanup = sys.argv[1:]
summary = {
    "schema": "adl.wp08.acip_sns_resource.v1",
    "issue": 4685,
    "run_id": run_id,
    "aws_profile": profile,
    "aws_region": region,
    "aws_account_hash": account_hash,
    "sns": {
        "topic_name": topic_name,
        "topic_arn_hash": hashlib.sha256(topic_arn.encode()).hexdigest()[:16],
        "cleanup_requested": cleanup == "1",
    },
    "redaction": {
        "raw_account_id_recorded": False,
        "raw_topic_arn_recorded": False,
        "credentials_recorded": False,
    },
}
Path(path).write_text(json.dumps(summary, indent=2) + "\n")
PY

if [ "$(basename "$PROOF_BIN")" = "run_wp08_acip_sns_live_proof" ]; then
  env \
    ADL_AWS_SIGNAL_MODE=live \
    ADL_AWS_SIGNAL_APPROVED=true \
    ADL_AWS_REGION="$REGION" \
    ADL_AWS_PROFILE="$PROFILE" \
    AWS_PROFILE="$PROFILE" \
    ADL_AWS_SNS_TOPIC_ARN="$TOPIC_ARN" \
    ADL_AWS_ACCOUNT_HASH="$ACCOUNT_HASH" \
    ADL_AWS_ACCOUNT_SHA256="$ACCOUNT_SHA256" \
    "$PROOF_BIN" --out "$OUT" --run-id "$RUN_ID" >/tmp/wp08-acip-sns-proof-output.json
else
  env \
    ADL_AWS_SIGNAL_MODE=live \
    ADL_AWS_SIGNAL_APPROVED=true \
    ADL_AWS_REGION="$REGION" \
    ADL_AWS_PROFILE="$PROFILE" \
    AWS_PROFILE="$PROFILE" \
    ADL_AWS_SNS_TOPIC_ARN="$TOPIC_ARN" \
    ADL_AWS_ACCOUNT_HASH="$ACCOUNT_HASH" \
    ADL_AWS_ACCOUNT_SHA256="$ACCOUNT_SHA256" \
    "$PROOF_BIN" aws-signal acip-sns-proof --out "$OUT" --run-id "$RUN_ID" >/tmp/wp08-acip-sns-proof-output.json
fi

python3 adl/tools/validate_wp08_acip_sns_live_proof.py "$SUMMARY" "$RESOURCE_SUMMARY"

if [ "$CLEANUP" -eq 1 ]; then
  "$AWS_BIN" sns delete-topic \
    --profile "$PROFILE" \
    --region "$REGION" \
    --topic-arn "$TOPIC_ARN" >/dev/null
  CLEANUP_DONE=1
fi

cat "$SUMMARY"
