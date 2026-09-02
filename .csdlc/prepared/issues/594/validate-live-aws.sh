#!/usr/bin/env bash
set -euo pipefail

: "${ADL_LOG_ARCHIVE_BUCKET:?set ADL_LOG_ARCHIVE_BUCKET from the applied Terraform output}"
: "${ADL_LOG_ARCHIVE_OBJECT_KEY:?set ADL_LOG_ARCHIVE_OBJECT_KEY to the exact proof object}"
: "${ADL_EXPECTED_AWS_ACCOUNT_ID:?set ADL_EXPECTED_AWS_ACCOUNT_ID to the approved business account}"

evidence_dir=".csdlc/evidence/594"
object_path="${evidence_dir}/live-archive-object.jsonl"
mkdir -p "${evidence_dir}"

account_id="$(aws --profile agent-logic-admin sts get-caller-identity --query Account --output text)"
test "${account_id}" = "${ADL_EXPECTED_AWS_ACCOUNT_ID}"

aws --profile agent-logic-admin s3api get-public-access-block --bucket "${ADL_LOG_ARCHIVE_BUCKET}" \
  --query 'PublicAccessBlockConfiguration.[BlockPublicAcls,IgnorePublicAcls,BlockPublicPolicy,RestrictPublicBuckets]' \
  --output text | awk '$1=="True" && $2=="True" && $3=="True" && $4=="True" {ok=1} END {exit !ok}'
test "$(aws --profile agent-logic-admin s3api get-bucket-versioning --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --query Status --output text)" = "Enabled"
test "$(aws --profile agent-logic-admin s3api get-bucket-encryption --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --query 'ServerSideEncryptionConfiguration.Rules[0].ApplyServerSideEncryptionByDefault.SSEAlgorithm' --output text)" = "AES256"
aws --profile agent-logic-admin s3api get-bucket-lifecycle-configuration --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --output json >/dev/null
test "$(aws --profile agent-logic-admin s3api head-object --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --key "${ADL_LOG_ARCHIVE_OBJECT_KEY}" --query ServerSideEncryption --output text)" = "AES256"
aws --profile agent-logic-admin s3api get-object --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --key "${ADL_LOG_ARCHIVE_OBJECT_KEY}" "${object_path}" >/dev/null
test -s "${object_path}"
if rg -i 'authorization|api[_-]?key|password|secret|token' "${object_path}" | rg -v '"<redacted>"' >/dev/null; then
  echo "archive object contains an unredacted sensitive-field marker" >&2
  exit 1
fi

echo "Verified business account, bucket controls, encrypted proof object, and bounded redaction inspection without printing archive content."
