#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"
EVIDENCE_DIR="${ROOT}/docs/milestones/v0.92.1/evidence/cloud/aws-a"
INVENTORY_DIR="${ROOT}/docs/operations/cloud/aws/inventory"

test -d "${EVIDENCE_DIR}"
test -d "${INVENTORY_DIR}"
test -f "${EVIDENCE_DIR}/readbacks/account-identity.json"
test -f "${EVIDENCE_DIR}/readbacks/regions.json"
test -f "${EVIDENCE_DIR}/readbacks/command-manifest.md"
test -f "${INVENTORY_DIR}/AWS_RESOURCE_OWNERSHIP_INVENTORY.md"

if rg -n --pcre2 '(?i)(aws_secret_access_key|aws_session_token|aws_access_key_id|secret access key|BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY)' "${EVIDENCE_DIR}" "${INVENTORY_DIR}"; then
  echo "credential-like material found in AWS-A evidence or inventory" >&2
  exit 1
fi

if rg -n --pcre2 '(?i)\b(create|delete|terminate|modify|put-|attach|detach|authorize|revoke|update|apply|import)\b' "${EVIDENCE_DIR}/readbacks/command-manifest.md"; then
  echo "mutation-like command found in AWS-A command manifest" >&2
  exit 1
fi

rg -n 'frozen-unknown|owned|externally-owned|not-observed' "${INVENTORY_DIR}/AWS_RESOURCE_OWNERSHIP_INVENTORY.md" >/dev/null

inventory="${INVENTORY_DIR}/AWS_RESOURCE_OWNERSHIP_INVENTORY.md"
readbacks="${EVIDENCE_DIR}/readbacks"

check_inventory_contains() {
  local value="$1"
  local source="$2"
  if [[ -z "${value}" || "${value}" == "null" ]]; then
    return 0
  fi
  if ! rg -F -- "${value}" "${inventory}" >/dev/null; then
    echo "inventory missing discovered resource from ${source}: ${value}" >&2
    exit 1
  fi
}

while IFS= read -r value; do check_inventory_contains "${value}" "s3-buckets.json"; done < <(jq -r '(.Buckets // [])[].Name' "${readbacks}/s3-buckets.json")
while IFS= read -r value; do check_inventory_contains "${value}" "route53-hosted-zones.json"; done < <(jq -r '(.HostedZones // [])[] | (.Name // .Id)' "${readbacks}/route53-hosted-zones.json")
while IFS= read -r value; do check_inventory_contains "${value}" "cloudfront-distributions.json"; done < <(jq -r '(.DistributionList.Items // [])[] | (.Id // .ARN // .DomainName)' "${readbacks}/cloudfront-distributions.json")
while IFS= read -r value; do check_inventory_contains "${value}" "global-tagged-resources.json"; done < <(jq -r '(.ResourceTagMappingList // [])[].ResourceARN' "${readbacks}/global-tagged-resources.json")

for file in "${readbacks}"/regions/*.json; do
  base="$(basename "${file}")"
  case "${base}" in
    *-ec2-instances.json)
      jq_filter='[.Reservations[]?.Instances[]?][].InstanceId'
      ;;
    *-ec2-volumes.json)
      jq_filter='(.Volumes // [])[].VolumeId'
      ;;
    *-vpcs.json)
      jq_filter='(.Vpcs // [])[].VpcId'
      ;;
    *-subnets.json)
      jq_filter='(.Subnets // [])[].SubnetId'
      ;;
    *-security-groups.json)
      jq_filter='(.SecurityGroups // [])[].GroupId'
      ;;
    *-load-balancers.json)
      jq_filter='(.LoadBalancers // [])[] | (.LoadBalancerName // .LoadBalancerArn)'
      ;;
    *-acm-certificates.json)
      jq_filter='(.CertificateSummaryList // [])[] | (.DomainName // .CertificateArn)'
      ;;
    *-cloudformation-stacks.json)
      jq_filter='(.StackSummaries // [])[] | (.StackName // .StackId)'
      ;;
    *)
      echo "unrecognized regional readback file: ${base}" >&2
      exit 1
      ;;
  esac
  while IFS= read -r value; do check_inventory_contains "${value}" "${base}"; done < <(jq -r "${jq_filter}" "${file}")
done

expected_region_count="$(jq -r '[.Regions[] | select(.OptInStatus == null or .OptInStatus == "opt-in-not-required" or .OptInStatus == "opted-in")] | length' "${readbacks}/regions.json")"
if ! rg -F -- "Enabled/available region denominator: ${expected_region_count}" "${inventory}" >/dev/null; then
  echo "inventory missing current enabled/available region denominator: ${expected_region_count}" >&2
  exit 1
fi
