#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: bash adl/tools/run_wp08_aws_signal_integration_live_proof.sh --out <dir> --expected-account-sha256 <sha256> [options]

Runs the WP-08 integrated AWS signal proof by executing the live CloudWatch
heartbeat proof and the live ACIP-to-SNS proof under one approved Agent Logic
account guard, then writes one redacted integration summary.

Options:
  --out <dir>                       Required proof output directory.
  --expected-account-sha256 <sha>   Required approved Agent Logic account SHA-256.
  --profile <name>                  AWS profile. Default: agent-logic-admin.
  --region <region>                 AWS region. Default: us-west-2.
  --run-id <id>                     Run id. Default: wp08-4686-<utc>.
  --csm-bin <path>                  csm binary for heartbeat proof.
  --acip-proof-bin <path>           csm proof command or legacy ACIP/SNS proof binary.
  --cleanup                         Ask child proofs to clean up disposable streams/topics where supported.
USAGE
}

OUT=""
EXPECTED="${ADL_AWS_SIGNAL_INTEGRATION_ACCOUNT_SHA256:-${ADL_AWS_ACIP_SNS_ACCOUNT_SHA256:-}}"
PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${ADL_AWS_REGION:-us-west-2}"
RUN_ID="wp08-4686-$(date -u +%Y%m%dT%H%M%SZ)"
CSM_BIN="${ADL_CSM_BIN:-adl/target/debug/csm}"
if [ -n "${ADL_ACIP_SNS_PROOF_BIN:-}" ]; then
  ACIP_PROOF_BIN="$ADL_ACIP_SNS_PROOF_BIN"
  ACIP_PROOF_BIN_EXPLICIT=1
else
  ACIP_PROOF_BIN="adl/target/debug/csm"
  ACIP_PROOF_BIN_EXPLICIT=0
fi
CLEANUP=0
AWS_BIN="${AWS_BIN:-aws}"
HEARTBEAT_SCRIPT="${ADL_WP08_HEARTBEAT_PROOF_SCRIPT:-adl/tools/run_wp08_heartbeat_live_proof.sh}"
ACIP_SCRIPT="${ADL_WP08_ACIP_SNS_PROOF_SCRIPT:-adl/tools/run_wp08_acip_sns_live_proof.sh}"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --out) OUT="${2:?--out requires a value}"; shift ;;
    --expected-account-sha256) EXPECTED="${2:?--expected-account-sha256 requires a value}"; shift ;;
    --profile) PROFILE="${2:?--profile requires a value}"; shift ;;
    --region) REGION="${2:?--region requires a value}"; shift ;;
    --run-id) RUN_ID="${2:?--run-id requires a value}"; shift ;;
    --csm-bin) CSM_BIN="${2:?--csm-bin requires a value}"; shift ;;
    --acip-proof-bin) ACIP_PROOF_BIN="${2:?--acip-proof-bin requires a value}"; ACIP_PROOF_BIN_EXPLICIT=1; shift ;;
    --cleanup) CLEANUP=1 ;;
    --help|-h) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
  esac
  shift
done

if [ -z "$OUT" ] || [ -z "$EXPECTED" ]; then
  usage >&2
  exit 2
fi
if ! command -v "$AWS_BIN" >/dev/null 2>&1; then
  echo "aws CLI not found; set AWS_BIN or install aws CLI" >&2
  exit 2
fi

ACCOUNT="$("$AWS_BIN" sts get-caller-identity --profile "$PROFILE" --region "$REGION" --query Account --output text)"
ACCOUNT_SHA="$(printf '%s' "$ACCOUNT" | shasum -a 256 | awk '{print $1}')"
ACCOUNT_HASH="$(printf '%s' "$ACCOUNT_SHA" | cut -c1-16)"
if [ "$ACCOUNT_SHA" != "$EXPECTED" ]; then
  echo "AWS profile account hash does not match expected Agent Logic account hash" >&2
  exit 1
fi
echo "PASS account_profile_resolved profile=$PROFILE account_matches_expected=true" >&2

mkdir -p "$OUT"
# shellcheck source=adl/tools/csm_binary_availability.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/csm_binary_availability.sh"
CSM_BIN="$(adl_resolve_csm_binary "$CSM_BIN" "$OUT/csm_binary_availability_csm.json")"
if [ "$ACIP_PROOF_BIN_EXPLICIT" -eq 1 ] && [ "$(basename "$ACIP_PROOF_BIN")" != "csm" ]; then
  if [ ! -x "$ACIP_PROOF_BIN" ]; then
    echo "explicit ACIP/SNS proof binary is not executable: $ACIP_PROOF_BIN" >&2
    exit 2
  fi
else
  ACIP_PROOF_BIN="$(adl_resolve_csm_binary "$ACIP_PROOF_BIN" "$OUT/csm_binary_availability_acip.json")"
fi
SUMMARY="$OUT/aws_signal_integration_summary.json"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
HEARTBEAT_OUT="$WORK/heartbeat"
ACIP_OUT="$WORK/acip_sns"
mkdir -p "$HEARTBEAT_OUT" "$ACIP_OUT"

if [ "$CLEANUP" -eq 1 ]; then
  AWS_BIN="$AWS_BIN" \
  ADL_AWS_PROFILE="$PROFILE" \
  AWS_PROFILE="$PROFILE" \
  ADL_AWS_REGION="$REGION" \
  bash "$HEARTBEAT_SCRIPT" \
    --out "$HEARTBEAT_OUT" \
    --profile "$PROFILE" \
    --region "$REGION" \
    --run-id "$RUN_ID-heartbeat" \
    --csm-bin "$CSM_BIN" \
    --cleanup \
    >"$WORK/heartbeat.stdout"

  AWS_BIN="$AWS_BIN" \
  ADL_AWS_PROFILE="$PROFILE" \
  AWS_PROFILE="$PROFILE" \
  ADL_AWS_REGION="$REGION" \
  ADL_AWS_ACIP_SNS_ACCOUNT_SHA256="$EXPECTED" \
  bash "$ACIP_SCRIPT" \
    --out "$ACIP_OUT" \
    --profile "$PROFILE" \
    --region "$REGION" \
    --run-id "$RUN_ID-acip-sns" \
    --proof-bin "$ACIP_PROOF_BIN" \
    --expected-account-sha256 "$EXPECTED" \
    --cleanup \
    >"$WORK/acip_sns.stdout"
else
  AWS_BIN="$AWS_BIN" \
  ADL_AWS_PROFILE="$PROFILE" \
  AWS_PROFILE="$PROFILE" \
  ADL_AWS_REGION="$REGION" \
  bash "$HEARTBEAT_SCRIPT" \
    --out "$HEARTBEAT_OUT" \
    --profile "$PROFILE" \
    --region "$REGION" \
    --run-id "$RUN_ID-heartbeat" \
    --csm-bin "$CSM_BIN" \
    >"$WORK/heartbeat.stdout"

  AWS_BIN="$AWS_BIN" \
  ADL_AWS_PROFILE="$PROFILE" \
  AWS_PROFILE="$PROFILE" \
  ADL_AWS_REGION="$REGION" \
  ADL_AWS_ACIP_SNS_ACCOUNT_SHA256="$EXPECTED" \
  bash "$ACIP_SCRIPT" \
    --out "$ACIP_OUT" \
    --profile "$PROFILE" \
    --region "$REGION" \
    --run-id "$RUN_ID-acip-sns" \
    --proof-bin "$ACIP_PROOF_BIN" \
    --expected-account-sha256 "$EXPECTED" \
    >"$WORK/acip_sns.stdout"
fi

python3 - "$SUMMARY" "$RUN_ID" "$PROFILE" "$REGION" "$ACCOUNT_HASH" "$HEARTBEAT_OUT/live_heartbeat_summary.json" "$ACIP_OUT/acip_sns_summary.json" "$ACIP_OUT/sns_resource_summary.json" <<'PY'
import json, sys, datetime, re
from pathlib import Path

summary_path, run_id, profile, region, account_hash, heartbeat_path, acip_path, resource_path = sys.argv[1:]
heartbeat = json.loads(Path(heartbeat_path).read_text())
acip = json.loads(Path(acip_path).read_text())
resource = json.loads(Path(resource_path).read_text())

def fail(msg):
    raise SystemExit(msg)

if heartbeat.get("status") != "passed":
    fail("heartbeat child proof did not pass")
if acip.get("status") != "passed":
    fail("acip child proof did not pass")
if heartbeat.get("aws_account_hash") != account_hash or acip.get("aws_account_hash") != account_hash:
    fail("child proof account hash mismatch")

summary = {
    "schema": "adl.wp08.aws_signal_integration.v1",
    "issue": 4686,
    "status": "passed",
    "checked_at_utc": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
    "run_id": run_id,
    "aws_profile": profile,
    "aws_region": region,
    "aws_account_hash": account_hash,
    "aws_account_matches_expected": True,
    "integrated_paths": {
        "heartbeat_cloudwatch": {
            "source_issue": 4684,
            "status": heartbeat.get("status"),
            "log_group": heartbeat.get("cloudwatch", {}).get("log_group"),
            "log_stream": heartbeat.get("cloudwatch", {}).get("log_stream"),
            "retention_days": heartbeat.get("cloudwatch", {}).get("retention_days"),
            "event_count": heartbeat.get("cloudwatch", {}).get("event_count"),
            "signal_kind": heartbeat.get("heartbeat", {}).get("signal_kind"),
            "transport_mode": heartbeat.get("heartbeat", {}).get("transport_mode"),
            "target_kind": heartbeat.get("heartbeat", {}).get("target_kind"),
        },
        "acip_sns": {
            "source_issue": 4685,
            "status": acip.get("status"),
            "topic_name": acip.get("sns", {}).get("topic_name"),
            "topic_arn_hash": acip.get("sns", {}).get("topic_arn_hash"),
            "message_id": acip.get("sns", {}).get("message_id"),
            "signal_kind": acip.get("acip_projection", {}).get("signal_kind"),
            "route_class": acip.get("acip_projection", {}).get("route_class"),
            "projection_level": acip.get("acip_projection", {}).get("projection_level"),
            "resource_cleanup_requested": resource.get("sns", {}).get("cleanup_requested"),
        },
    },
    "negative_cases": {
        "heartbeat_missing_approval": "covered_by_runtime_aws_signal_tests",
        "heartbeat_unsupported_target": "covered_by_runtime_aws_signal_tests",
        "acip_missing_profile": acip.get("negative_case_policy", {}).get("missing_profile"),
        "acip_missing_topic": acip.get("negative_case_policy", {}).get("missing_topic"),
        "acip_projection_denied": acip.get("negative_case_policy", {}).get("malformed_or_denied_projection"),
        "sns_unavailable_or_access_denied": acip.get("negative_case_policy", {}).get("sns_unavailable_or_access_denied"),
        "account_mismatch": "covered_by_wrapper_contract_test",
    },
    "durability": {
        "cloudwatch_retention_days": heartbeat.get("cloudwatch", {}).get("retention_days"),
        "sns_message_id_retained": bool(acip.get("sns", {}).get("message_id")),
        "child_proof_retention": "transient_child_outputs_distilled_into_integrated_summary",
    },
    "redaction": {
        "raw_account_id_recorded": False,
        "full_account_digest_recorded": False,
        "credentials_recorded": False,
        "raw_topic_arn_recorded": False,
        "raw_private_acip_content_recorded": False,
    },
}
text = json.dumps(summary, indent=2, sort_keys=True)
if re.search(r"\b\d{12}\b", text):
    fail("integrated summary contains raw account id")
if re.search(r"\b[0-9a-f]{64}\b", text):
    fail("integrated summary contains full digest")
Path(summary_path).write_text(text + "\n")
PY

python3 adl/tools/validate_wp08_aws_signal_integration_live_proof.py "$SUMMARY"
cat "$SUMMARY"
