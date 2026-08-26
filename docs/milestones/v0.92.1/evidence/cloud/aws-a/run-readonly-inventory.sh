#!/usr/bin/env bash
set -euo pipefail

AWS_PROFILE_NAME="${AWS_PROFILE_NAME:-agent-logic-admin}"
OUTPUT_DIR="${OUTPUT_DIR:-docs/milestones/v0.92.1/evidence/cloud/aws-a/readbacks}"
mkdir -p "${OUTPUT_DIR}/regions"

aws_readonly() {
  aws --profile "${AWS_PROFILE_NAME}" "$@"
}

record_json() {
  local name="$1"
  shift
  if "$@" > "${OUTPUT_DIR}/${name}.json" 2> "${OUTPUT_DIR}/${name}.stderr"; then
    if [ ! -s "${OUTPUT_DIR}/${name}.stderr" ]; then
      rm -f "${OUTPUT_DIR}/${name}.stderr"
    fi
  else
    local code=$?
    {
      echo "{"
      echo "  \"status\": \"read_failed\","
      echo "  \"exit_code\": ${code},"
      echo "  \"surface\": \"${name}\""
      echo "}"
    } > "${OUTPUT_DIR}/${name}.json"
  fi
}

record_json account-identity aws_readonly sts get-caller-identity
record_json regions aws_readonly ec2 describe-regions --all-regions
record_json s3-buckets aws_readonly s3api list-buckets
record_json route53-hosted-zones aws_readonly route53 list-hosted-zones
record_json cloudfront-distributions aws_readonly cloudfront list-distributions
record_json global-tagged-resources aws_readonly resourcegroupstaggingapi get-resources --resources-per-page 100

REGIONS=()
while IFS= read -r region; do
  REGIONS+=("${region}")
done < <(jq -r '.Regions[] | select(.OptInStatus == null or .OptInStatus == "opt-in-not-required" or .OptInStatus == "opted-in") | .RegionName' "${OUTPUT_DIR}/regions.json" | sort)

{
  echo "# AWS-A read-only inventory command manifest"
  echo
  echo "- profile: ${AWS_PROFILE_NAME}"
  echo "- output_dir: ${OUTPUT_DIR}"
  echo "- region_count: ${#REGIONS[@]}"
  echo "- commands:"
  echo "  - aws sts get-caller-identity"
  echo "  - aws ec2 describe-regions --all-regions"
  echo "  - aws s3api list-buckets"
  echo "  - aws route53 list-hosted-zones"
  echo "  - aws cloudfront list-distributions"
  echo "  - aws resourcegroupstaggingapi get-resources --resources-per-page 100"
} > "${OUTPUT_DIR}/command-manifest.md"

for region in "${REGIONS[@]}"; do
  safe_region="${region//[^a-zA-Z0-9-]/_}"
  record_json "regions/${safe_region}-ec2-instances" aws_readonly ec2 describe-instances --region "${region}"
  record_json "regions/${safe_region}-ec2-volumes" aws_readonly ec2 describe-volumes --region "${region}"
  record_json "regions/${safe_region}-vpcs" aws_readonly ec2 describe-vpcs --region "${region}"
  record_json "regions/${safe_region}-subnets" aws_readonly ec2 describe-subnets --region "${region}"
  record_json "regions/${safe_region}-security-groups" aws_readonly ec2 describe-security-groups --region "${region}"
  record_json "regions/${safe_region}-load-balancers" aws_readonly elbv2 describe-load-balancers --region "${region}"
  record_json "regions/${safe_region}-acm-certificates" aws_readonly acm list-certificates --region "${region}"
  record_json "regions/${safe_region}-cloudformation-stacks" aws_readonly cloudformation list-stacks --region "${region}" --stack-status-filter CREATE_COMPLETE UPDATE_COMPLETE UPDATE_ROLLBACK_COMPLETE IMPORT_COMPLETE
  {
    echo "  - aws ec2 describe-instances --region ${region}"
    echo "  - aws ec2 describe-volumes --region ${region}"
    echo "  - aws ec2 describe-vpcs --region ${region}"
    echo "  - aws ec2 describe-subnets --region ${region}"
    echo "  - aws ec2 describe-security-groups --region ${region}"
    echo "  - aws elbv2 describe-load-balancers --region ${region}"
    echo "  - aws acm list-certificates --region ${region}"
    echo "  - aws cloudformation list-stacks --region ${region} --stack-status-filter CREATE_COMPLETE UPDATE_COMPLETE UPDATE_ROLLBACK_COMPLETE IMPORT_COMPLETE"
  } >> "${OUTPUT_DIR}/command-manifest.md"
done

{
  echo
  echo "All commands above are read-only list/describe/get calls. Failures are recorded as read_failed JSON surfaces and do not trigger mutation."
} >> "${OUTPUT_DIR}/command-manifest.md"
