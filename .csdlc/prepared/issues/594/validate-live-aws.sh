#!/usr/bin/env bash
set -euo pipefail

: "${ADL_LOG_ARCHIVE_BUCKET:?set ADL_LOG_ARCHIVE_BUCKET from the applied Terraform output}"
: "${ADL_LOG_ARCHIVE_OBJECT_KEY:?set ADL_LOG_ARCHIVE_OBJECT_KEY to the exact proof object}"
: "${ADL_EXPECTED_AWS_ACCOUNT_ID:?set ADL_EXPECTED_AWS_ACCOUNT_ID to the approved business account}"
: "${ADL_LOG_ARCHIVE_PUBLISHER_POLICY_ARN:?set ADL_LOG_ARCHIVE_PUBLISHER_POLICY_ARN from Terraform output}"
: "${ADL_LOG_ARCHIVE_PREFIX_ARN:?set ADL_LOG_ARCHIVE_PREFIX_ARN to the exact environment, Polis, and Runtime object prefix ARN}"

evidence_dir=".csdlc/evidence/594"
compressed_path="${evidence_dir}/live-archive-object.json.gz"
object_path="${evidence_dir}/live-archive-object.jsonl"
mkdir -p "${evidence_dir}"

account_id="$(aws --profile agent-logic-admin sts get-caller-identity --query Account --output text)"
test "${account_id}" = "${ADL_EXPECTED_AWS_ACCOUNT_ID}"

aws --profile agent-logic-admin s3api get-public-access-block --bucket "${ADL_LOG_ARCHIVE_BUCKET}" \
  --query 'PublicAccessBlockConfiguration.[BlockPublicAcls,IgnorePublicAcls,BlockPublicPolicy,RestrictPublicBuckets]' \
  --output text | awk '$1=="True" && $2=="True" && $3=="True" && $4=="True" {ok=1} END {exit !ok}'
test "$(aws --profile agent-logic-admin s3api get-bucket-versioning --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --query Status --output text)" = "Enabled"
test "$(aws --profile agent-logic-admin s3api get-bucket-encryption --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --query 'ServerSideEncryptionConfiguration.Rules[0].ApplyServerSideEncryptionByDefault.SSEAlgorithm' --output text)" = "AES256"
test "$(aws --profile agent-logic-admin s3api get-bucket-ownership-controls --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --query 'OwnershipControls.Rules[0].ObjectOwnership' --output text)" = "BucketOwnerEnforced"
lifecycle_json="$(aws --profile agent-logic-admin s3api get-bucket-lifecycle-configuration --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --output json)"
jq -e 'any(.Rules[]; .Status == "Enabled" and .Expiration.Days == 30 and .NoncurrentVersionExpiration.NoncurrentDays == 7 and .AbortIncompleteMultipartUpload.DaysAfterInitiation == 1)' <<<"${lifecycle_json}" >/dev/null
policy_version="$(aws --profile agent-logic-admin iam get-policy --policy-arn "${ADL_LOG_ARCHIVE_PUBLISHER_POLICY_ARN}" --query 'Policy.DefaultVersionId' --output text)"
policy_json="$(aws --profile agent-logic-admin iam get-policy-version --policy-arn "${ADL_LOG_ARCHIVE_PUBLISHER_POLICY_ARN}" --version-id "${policy_version}" --query 'PolicyVersion.Document' --output json)"
jq -e --arg bucket "arn:aws:s3:::${ADL_LOG_ARCHIVE_BUCKET}" --arg prefix "${ADL_LOG_ARCHIVE_PREFIX_ARN}" '
  [.Statement[] | {effect: .Effect, actions: ([.Action] | flatten | sort), resources: ([.Resource] | flatten | sort)}]
  == [
    {effect:"Allow", actions:["s3:GetBucketLocation"], resources:[$bucket]},
    {effect:"Allow", actions:["s3:AbortMultipartUpload","s3:ListMultipartUploadParts","s3:PutObject"], resources:[$prefix]}
  ]
' <<<"${policy_json}" >/dev/null
test "$(aws --profile agent-logic-admin s3api head-object --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --key "${ADL_LOG_ARCHIVE_OBJECT_KEY}" --query ServerSideEncryption --output text)" = "AES256"
aws --profile agent-logic-admin s3api get-object --bucket "${ADL_LOG_ARCHIVE_BUCKET}" --key "${ADL_LOG_ARCHIVE_OBJECT_KEY}" "${compressed_path}" >/dev/null
gzip -dc "${compressed_path}" >"${object_path}"
test -s "${object_path}"
jq -s -e '
  def sensitive_key: test("^(authorization|api[_-]?key|password|secret|token)$"; "i");
  all(.[];
    ([.. | objects | to_entries[] | select(.key | sensitive_key) | .value]
      | all(. == "<redacted>")))
' "${object_path}" >/dev/null

echo "Verified business account, bucket controls, encrypted proof object, and bounded redaction inspection without printing archive content."
