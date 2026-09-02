#!/usr/bin/env bash
set -euo pipefail

: "${ADL_LOG_ARCHIVE_BUCKET:?set ADL_LOG_ARCHIVE_BUCKET from the applied Terraform output}"
: "${ADL_LOG_ARCHIVE_OBJECT_KEY:?set ADL_LOG_ARCHIVE_OBJECT_KEY to the exact proof object}"

account_id="$(aws --profile agent-logic-admin sts get-caller-identity --query Account --output text)"
test -n "${account_id}"

aws --profile agent-logic-admin s3api head-bucket \
  --bucket "${ADL_LOG_ARCHIVE_BUCKET}"
aws --profile agent-logic-admin s3api head-object \
  --bucket "${ADL_LOG_ARCHIVE_BUCKET}" \
  --key "${ADL_LOG_ARCHIVE_OBJECT_KEY}"

echo "Live archive object metadata is reachable in the verified business profile; content retrieval and redaction inspection require the issue-owned evidence path."
