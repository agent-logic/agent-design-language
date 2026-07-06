#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: bash adl/tools/setup_wp08_s3_obsmem_archive_policy.sh --out <dir> --expected-account-sha256 <sha256> [options]

Creates or repairs the WP-08 S3 ObsMem community-memory archive bucket policy
and writes a redacted live configuration summary.

Options:
  --out <dir>                       Required proof output directory.
  --expected-account-sha256 <sha>   Required approved Agent Logic account hash.
  --profile <name>                  AWS profile. Default: agent-logic-admin.
  --region <region>                 AWS region. Default: us-west-2.
  --bucket <name>                   Override bucket name. Default derives from account hash.
USAGE
}

OUT=""
EXPECTED="${ADL_AWS_S3_OBSMEM_ARCHIVE_ACCOUNT_SHA256:-}"
PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${ADL_AWS_REGION:-us-west-2}"
BUCKET=""
AWS_BIN="${AWS_BIN:-aws}"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --out) OUT="${2:?--out requires a value}"; shift ;;
    --expected-account-sha256) EXPECTED="${2:?--expected-account-sha256 requires a value}"; shift ;;
    --profile) PROFILE="${2:?--profile requires a value}"; shift ;;
    --region) REGION="${2:?--region requires a value}"; shift ;;
    --bucket) BUCKET="${2:?--bucket requires a value}"; shift ;;
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

if ! "$AWS_BIN" s3api head-bucket --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" >/dev/null 2>&1; then
  "$AWS_BIN" s3api create-bucket \
    --profile "$PROFILE" \
    --region "$REGION" \
    --bucket "$BUCKET" \
    --create-bucket-configuration "LocationConstraint=$REGION" \
    --object-lock-enabled-for-bucket >/dev/null
fi

"$AWS_BIN" s3api put-public-access-block \
  --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" \
  --public-access-block-configuration BlockPublicAcls=true,IgnorePublicAcls=true,BlockPublicPolicy=true,RestrictPublicBuckets=true
"$AWS_BIN" s3api put-bucket-encryption \
  --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" \
  --server-side-encryption-configuration '{"Rules":[{"ApplyServerSideEncryptionByDefault":{"SSEAlgorithm":"AES256"},"BucketKeyEnabled":true}]}'
"$AWS_BIN" s3api put-bucket-versioning \
  --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" \
  --versioning-configuration Status=Enabled
"$AWS_BIN" s3api put-object-lock-configuration \
  --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" \
  --object-lock-configuration '{"ObjectLockEnabled":"Enabled","Rule":{"DefaultRetention":{"Mode":"GOVERNANCE","Days":365}}}'

LIFECYCLE="$OUT/lifecycle-request.json"
cat >"$LIFECYCLE" <<'JSON'
{
  "Rules": [
    {
      "ID": "adl-wp08-community-memory-archive-tiering",
      "Status": "Enabled",
      "Filter": {"Prefix": "community-memory/"},
      "Transitions": [
        {"Days": 90, "StorageClass": "GLACIER_IR"},
        {"Days": 365, "StorageClass": "DEEP_ARCHIVE"}
      ],
      "NoncurrentVersionTransitions": [
        {"NoncurrentDays": 30, "StorageClass": "GLACIER_IR"},
        {"NoncurrentDays": 180, "StorageClass": "DEEP_ARCHIVE"}
      ],
      "AbortIncompleteMultipartUpload": {"DaysAfterInitiation": 7}
    }
  ]
}
JSON
"$AWS_BIN" s3api put-bucket-lifecycle-configuration \
  --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" \
  --lifecycle-configuration "file://$LIFECYCLE" >/dev/null

"$AWS_BIN" s3api get-public-access-block --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" --output json >"$OUT/public_access_block.live.json"
"$AWS_BIN" s3api get-bucket-encryption --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" --output json >"$OUT/encryption.live.json"
"$AWS_BIN" s3api get-bucket-versioning --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" --output json >"$OUT/versioning.live.json"
"$AWS_BIN" s3api get-object-lock-configuration --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" --output json >"$OUT/object_lock.live.json"
"$AWS_BIN" s3api get-bucket-lifecycle-configuration --profile "$PROFILE" --region "$REGION" --bucket "$BUCKET" --output json >"$OUT/lifecycle.live.json"

python3 - "$OUT" "$PROFILE" "$REGION" "$ACCOUNT_HASH" "$BUCKET" <<'PY'
import json, sys, datetime, hashlib
from pathlib import Path
out=Path(sys.argv[1])
profile, region, account_hash, bucket = sys.argv[2:]
def load(name): return json.loads((out/name).read_text())
summary={
  "schema":"adl.wp08.s3_obsmem_archive_policy.v1",
  "issue":4688,
  "status":"passed",
  "checked_at_utc":datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
  "aws_profile":profile,
  "aws_region":region,
  "aws_account_hash":account_hash,
  "aws_account_matches_expected":True,
  "bucket_name":bucket,
  "bucket_name_hash":hashlib.sha256(bucket.encode()).hexdigest()[:16],
  "policy":{
    "prefix":"community-memory/",
    "durability_class":"s3_standard_or_colder_vendor_durability_11_nines_per_object_non_12_nines_claim",
    "versioning_required":True,
    "object_lock_default_mode":"GOVERNANCE",
    "object_lock_default_days":365,
    "public_access_block_required":True,
    "encryption":"SSE-S3",
    "lifecycle_transitions":["GLACIER_IR_after_90_days","DEEP_ARCHIVE_after_365_days"],
    "noncurrent_transitions":["GLACIER_IR_after_30_noncurrent_days","DEEP_ARCHIVE_after_180_noncurrent_days"],
    "abort_incomplete_multipart_days":7
  },
  "live_configuration":{
    "public_access_block":load("public_access_block.live.json"),
    "encryption":load("encryption.live.json"),
    "versioning":load("versioning.live.json"),
    "object_lock":load("object_lock.live.json"),
    "lifecycle":load("lifecycle.live.json")
  },
  "non_claims":[
    "This policy does not claim mathematical 12-nines durability from a single-region S3 bucket.",
    "This policy does not grant public access.",
    "Write/read/restore object proof is owned by #4913."
  ],
  "redaction":{"raw_account_id_retained":False,"aws_credentials_retained":False}
}
(out/"archive_policy_summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True)+"\n")
PY

echo "PASS wp08_s3_obsmem_archive_policy bucket=$BUCKET"
