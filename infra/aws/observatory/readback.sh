#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" != "--execute" ]]; then
  echo "readback is inert; rerun with --execute only after explicit operator authorization" >&2
  exit 2
fi
if [[ "${AWS_PROFILE:-}" != "agent-logic-admin" ]]; then
  echo "AWS_PROFILE must be agent-logic-admin" >&2
  exit 2
fi

distribution_id="${2:-}"
site_bucket="${3:-}"
log_bucket="${4:-}"
hosted_zone_id="${5:-}"
observatory_fqdn="${6:-}"
if [[ -z "${distribution_id}" || -z "${site_bucket}" || -z "${log_bucket}" || \
      -z "${hosted_zone_id}" || -z "${observatory_fqdn}" ]]; then
  echo "usage: AWS_PROFILE=agent-logic-admin ./readback.sh --execute DISTRIBUTION_ID SITE_BUCKET LOG_BUCKET HOSTED_ZONE_ID OBSERVATORY_FQDN" >&2
  exit 2
fi
if [[ "${observatory_fqdn}" != "observatory.csm.agent-logic.ai" ]]; then
  echo "unexpected Observatory hostname" >&2
  exit 2
fi

# Fixed projections exclude ARNs, account IDs, policies, tags, signed URLs,
# credentials, and Terraform state from retained output.
aws cloudfront get-distribution --profile "${AWS_PROFILE}" --id "${distribution_id}" \
  --query 'Distribution.{status:Status,enabled:DistributionConfig.Enabled,domain_name:DomainName,logging:DistributionConfig.Logging}' --output json
aws s3api get-public-access-block --profile "${AWS_PROFILE}" --bucket "${site_bucket}" \
  --query 'PublicAccessBlockConfiguration' --output json
aws s3api get-bucket-versioning --profile "${AWS_PROFILE}" --bucket "${site_bucket}" \
  --query '{status:Status}' --output json
aws s3api list-object-versions --profile "${AWS_PROFILE}" --bucket "${site_bucket}" \
  --query 'Versions[].{key:Key,version_id:VersionId,is_latest:IsLatest,last_modified:LastModified}' --output json
aws s3api get-public-access-block --profile "${AWS_PROFILE}" --bucket "${log_bucket}" \
  --query 'PublicAccessBlockConfiguration' --output json
aws s3api get-bucket-lifecycle-configuration --profile "${AWS_PROFILE}" --bucket "${log_bucket}" \
  --query 'Rules[].{id:ID,status:Status,expiration_days:Expiration.Days}' --output json
aws acm list-certificates --profile "${AWS_PROFILE}" --region us-east-1 \
  --query "CertificateSummaryList[?DomainName=='${observatory_fqdn}'].{domain_name:DomainName,status:Status,type:Type,in_use:InUse}" --output json
aws route53 list-resource-record-sets --profile "${AWS_PROFILE}" --hosted-zone-id "${hosted_zone_id}" \
  --query "ResourceRecordSets[?Name=='${observatory_fqdn}.']|[?Type=='A' || Type=='AAAA'].{name:Name,type:Type,alias_dns_name:AliasTarget.DNSName}" --output json
