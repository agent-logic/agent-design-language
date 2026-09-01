#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

ledger="${1:-docs/milestones/v0.92.1/evidence/cloud/aws-g/aws-g-cloudformation-retirement-ledger.md}"

require_file() {
  test -f "$1" || { echo "missing required file: $1" >&2; exit 1; }
}

require_text() {
  local path="$1"
  local text="$2"
  grep -Fq -- "$text" "$path" || { echo "missing text in $path: $text" >&2; exit 1; }
}

require_no_text() {
  local path="$1"
  local text="$2"
  if grep -Fq -- "$text" "$path"; then
    echo "forbidden text in $path: $text" >&2
    exit 1
  fi
}

require_file "adl/tools/issue194_private_network.cloudformation.json"
require_file "adl/tools/issue268_runtime_qualification.cloudformation.yaml"
require_file "$ledger"

require_text "$ledger" "adl/tools/issue194_private_network.cloudformation.json"
require_text "$ledger" "adl/tools/issue268_runtime_qualification.cloudformation.yaml"
require_text "$ledger" "AWS-F #489"
require_text "$ledger" "XCL-01 #495"
require_text "$ledger" "69ba35e066d1389a9f194659acb066a7dca82a40"
require_text "$ledger" "c78c60f5a45a87a96159d4910a831b69b62b042c"
require_text "$ledger" "CloudFormation rollback authority remains retained"
require_text "$ledger" "No template deletion is authorized by #496"
require_text "$ledger" "Live stack retirement is not claimed"
require_text "$ledger" "consumer-census"
require_text "$ledger" "retained-evidence"
require_text "$ledger" "terraform-replacement"
require_text "$ledger" "future-deletion-authority"
require_text "$ledger" "| Reference | Disposition | Evidence |"

require_no_text "$ledger" "BEGIN PRIVATE KEY"
require_no_text "$ledger" "AKIA"
require_no_text "$ledger" "aws_secret_access_key"

template_refs="$(
  git grep -n -e "issue194_private_network.cloudformation.json" -e "issue268_runtime_qualification.cloudformation.yaml" -- \
    ':!docs/milestones/v0.92.1/evidence/cloud/aws-g/aws-g-cloudformation-retirement-ledger.md' \
    ':!.csdlc/issues/496/**' \
    ':!.csdlc/prepared/issues/496/**' \
    ':!.csdlc/requests/496-*' \
    ':!.t2/**'
)"

if test -z "$template_refs"; then
  echo "no current repo references to the CloudFormation templates were found" >&2
  exit 1
fi

while IFS= read -r ref; do
  path="${ref%%:*}"
  require_text "$ledger" "$path"
  if ! grep -F -- "$path" "$ledger" | grep -Eq 'rollback|source-denominator|terraform-replacement|retained-evidence|follow-on'; then
    echo "reference is listed without an allowed disposition: $path" >&2
    exit 1
  fi
done <<< "$template_refs"

echo "aws-g CloudFormation retirement validation passed"
