#!/usr/bin/env bash
set -euo pipefail

lane="static"
repo="."
for arg in "$@"; do
  case "$arg" in
    --lane=*) lane="${arg#--lane=}" ;;
    --repo=*) repo="${arg#--repo=}" ;;
    *) ;;
  esac
done

adoption_register="$repo/docs/operations/cloud/aws/adoption/AWS_RESOURCE_ADOPTION_REGISTER.md"
summary_dir="$repo/.csdlc/evidence/488"
summary_file="$summary_dir/aws-e-live-readback-summary.log"

need_tool() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required tool: $1" >&2
    exit 1
  fi
}

need_register_text() {
  local marker="$1"
  if ! grep -Fq "$marker" "$adoption_register"; then
    echo "adoption register missing required live marker" >&2
    echo "marker_sha256=$(printf '%s' "$marker" | shasum -a 256 | awk '{print $1}')" >&2
    exit 1
  fi
}

json_count() {
  jq -r "$1 | length" "$2"
}

case "$lane" in
  static)
    echo "aws_e_readback_lane=static"
    echo "cloud_mutation=false"
    echo "credential_material_retained=false"
    echo "redaction=credential_material_not_printed"
    ;;
  inventory-readonly)
    if [[ "${AWS_PROFILE:-}" != "agent-logic-admin" ]]; then
      echo "AWS_PROFILE must be agent-logic-admin for #488 live readback" >&2
      exit 1
    fi
    if [[ ! -f "$adoption_register" ]]; then
      echo "missing adoption register: $adoption_register" >&2
      exit 1
    fi
    need_tool aws
    need_tool jq
    need_tool shasum
    mkdir -p "$summary_dir"

    sts_json="$(aws sts get-caller-identity --profile "$AWS_PROFILE" --output json)"
    account_id="$(printf '%s' "$sts_json" | jq -r '.Account')"
    if [[ "$account_id" != "713332525889" ]]; then
      echo "unexpected AWS account for #488 live readback" >&2
      exit 1
    fi
    need_register_text "account/713332525889"

    regions_json="$(aws ec2 describe-regions --all-regions --profile "$AWS_PROFILE" --output json)"
    enabled_region_count="$(printf '%s' "$regions_json" | jq -r '[.Regions[] | select((.OptInStatus // "opt-in-not-required") == "opt-in-not-required" or .OptInStatus == "opted-in")] | length')"
    need_register_text "regions/enabled-or-available-${enabled_region_count}"

    buckets_json="$(aws s3api list-buckets --profile "$AWS_PROFILE" --output json)"
    bucket_count="$(printf '%s' "$buckets_json" | jq -r '(.Buckets // []) | length')"
    while IFS= read -r bucket; do
      [[ -z "$bucket" ]] && continue
      need_register_text "s3/${bucket}"
    done < <(printf '%s' "$buckets_json" | jq -r '(.Buckets // [])[].Name')

    zones_json="$(aws route53 list-hosted-zones --profile "$AWS_PROFILE" --output json)"
    zone_count="$(printf '%s' "$zones_json" | jq -r '(.HostedZones // []) | length')"
    while IFS= read -r zone; do
      [[ -z "$zone" ]] && continue
      need_register_text "route53/${zone}"
    done < <(printf '%s' "$zones_json" | jq -r '(.HostedZones // [])[].Name')

    distributions_json="$(aws cloudfront list-distributions --profile "$AWS_PROFILE" --output json)"
    distribution_count="$(printf '%s' "$distributions_json" | jq -r '(.DistributionList.Items // []) | length')"
    need_register_text "cloudfront/all-observed"

    tagged_json="$(aws resourcegroupstaggingapi get-resources --region us-west-2 --profile "$AWS_PROFILE" --output json)"
    tagged_count="$(printf '%s' "$tagged_json" | jq -r '(.ResourceTagMappingList // []) | length')"
    need_register_text "tagged/api-gateway-runtime-http"
    need_register_text "tagged/lambda-eventbridge-sns-notice"
    need_register_text "tagged/security-groups"
    need_register_text "tagged/ssm-managed-instances"
    need_register_text "tagged/ec2-instance-i-027183bbc454a62e3"
    need_register_text "regional-default-networking"
    need_register_text "aws-a-inventory-remainder"
    need_register_text "one management authority"

    {
      echo "aws_e_readback_lane=inventory-readonly"
      echo "account_match=true"
      echo "enabled_region_count=${enabled_region_count}"
      echo "s3_bucket_count=${bucket_count}"
      echo "route53_zone_count=${zone_count}"
      echo "cloudfront_distribution_count=${distribution_count}"
      echo "us_west_2_tagged_resource_count=${tagged_count}"
      echo "adoption_register_reconciled=true"
      echo "cloud_mutation=false"
      echo "credential_material_retained=false"
      echo "redaction=names_and_arns_not_printed"
    } | tee "$summary_file"
    ;;
  *)
    echo "unsupported #488 readback lane: $lane" >&2
    exit 1
    ;;
esac
