#!/usr/bin/env bash
set -euo pipefail

lane="${1:-}"
if [[ "$lane" == "--lane" ]]; then
  lane="${2:-}"
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../../.." && pwd)"
cd "$repo_root"

fail() {
  printf 'aws-c-bootstrap-readback: FAIL: %s\n' "$*" >&2
  exit 1
}

case "$lane" in
  terraform-static)
    terraform -chdir=infra/aws/bootstrap fmt -check -recursive
    terraform -chdir=infra/aws/bootstrap init -backend=false
    terraform -chdir=infra/aws/bootstrap validate
    ;;
  aws-readback)
    : "${AWS_PROFILE:?AWS_PROFILE must name the approved Agent Logic business profile}"
    terraform -chdir=infra/aws/bootstrap output -json >/dev/null
    aws sts get-caller-identity --output json >/dev/null
    bucket="$(terraform -chdir=infra/aws/bootstrap output -raw state_bucket_name)"
    table="$(terraform -chdir=infra/aws/bootstrap output -raw lock_table_name)"
    aws s3api get-bucket-versioning --bucket "$bucket" --output json
    aws s3api get-bucket-encryption --bucket "$bucket" --output json
    aws dynamodb describe-table --table-name "$table" --output json
    ;;
  *)
    fail "usage: $0 --lane terraform-static|aws-readback"
    ;;
esac

printf 'aws-c-bootstrap-readback: PASS %s\n' "$lane"
