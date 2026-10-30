#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: bash adl/tools/run_wp08_polis_storage_live_proof.sh --out <dir> --expected-account-sha256 <sha256> [options]

Runs the WP-08 Polis durable-storage proof through the standalone csm runtime
binary against the Agent Logic S3 archive bucket from #4688.

Options:
  --out <dir>                       Required proof output directory.
  --expected-account-sha256 <sha>   Required approved Agent Logic account hash.
  --profile <name>                  AWS profile. Default: agent-logic-admin.
  --region <region>                 AWS region. Default: us-west-2.
  --bucket <name>                   Override S3 bucket name. Default derives from account hash.
  --prefix <prefix>                 S3 key prefix. Default: community-memory/.
  --run-id <id>                     Proof run id. Default: wp08-4913-polis-storage.
  --csm-bin <path>                  csm binary. Default: adl/target/debug/csm.
USAGE
}

OUT=""
EXPECTED="${ADL_AWS_POLIS_STORAGE_ACCOUNT_SHA256:-}"
PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${ADL_AWS_REGION:-us-west-2}"
BUCKET=""
PREFIX="community-memory/"
RUN_ID="wp08-4913-polis-storage"
CSM_BIN="${CSM_BIN:-adl/target/debug/csm}"
AWS_BIN="${AWS_BIN:-aws}"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --out) OUT="${2:?--out requires a value}"; shift ;;
    --expected-account-sha256) EXPECTED="${2:?--expected-account-sha256 requires a value}"; shift ;;
    --profile) PROFILE="${2:?--profile requires a value}"; shift ;;
    --region) REGION="${2:?--region requires a value}"; shift ;;
    --bucket) BUCKET="${2:?--bucket requires a value}"; shift ;;
    --prefix) PREFIX="${2:?--prefix requires a value}"; shift ;;
    --run-id) RUN_ID="${2:?--run-id requires a value}"; shift ;;
    --csm-bin) CSM_BIN="${2:?--csm-bin requires a value}"; shift ;;
    --help|-h) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
  esac
  shift
done

if [ -z "$OUT" ] || [ -z "$EXPECTED" ]; then
  usage >&2
  exit 2
fi

mkdir -p "$OUT"
# shellcheck source=adl/tools/csm_binary_availability.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/csm_binary_availability.sh"
CSM_BIN="$(adl_resolve_csm_binary "$CSM_BIN" "$OUT/csm_binary_availability.json")"

ACCOUNT="$("$AWS_BIN" sts get-caller-identity --profile "$PROFILE" --region "$REGION" --query Account --output text)"
ACCOUNT_SHA="$(printf '%s' "$ACCOUNT" | shasum -a 256 | awk '{print $1}')"
ACCOUNT_HASH="$(printf '%s' "$ACCOUNT_SHA" | cut -c1-16)"
if [ "$ACCOUNT_SHA" != "$EXPECTED" ]; then
  echo "AWS profile account hash does not match expected Agent Logic account hash" >&2
  exit 1
fi
echo "PASS account_profile_resolved profile=$PROFILE account_matches_expected=true" >&2

if [ -z "$BUCKET" ]; then
  BUCKET="adl-wp08-obsmem-community-archive-${ACCOUNT_HASH}-${REGION}"
fi

"$CSM_BIN" storage prove-s3 \
  --out "$OUT" \
  --bucket "$BUCKET" \
  --prefix "$PREFIX" \
  --profile "$PROFILE" \
  --region "$REGION" \
  --expected-account-sha256 "$EXPECTED" \
  --run-id "$RUN_ID" \
  --aws-bin "$AWS_BIN" \
  --json >"$OUT/csm_storage_command_result.json"

echo "PASS wp08_polis_storage_live_proof bucket=$BUCKET run_id=$RUN_ID"
