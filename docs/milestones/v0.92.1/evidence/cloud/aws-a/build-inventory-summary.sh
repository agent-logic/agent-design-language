#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"
READBACKS="${ROOT}/docs/milestones/v0.92.1/evidence/cloud/aws-a/readbacks"
OUT="${ROOT}/docs/operations/cloud/aws/inventory/AWS_RESOURCE_OWNERSHIP_INVENTORY.md"

account_id="$(jq -r '.Account // "unknown"' "${READBACKS}/account-identity.json")"
principal_arn="$(jq -r '.Arn // "unknown"' "${READBACKS}/account-identity.json")"
region_count="$(jq -r '[.Regions[] | select(.OptInStatus == null or .OptInStatus == "opt-in-not-required" or .OptInStatus == "opted-in")] | length' "${READBACKS}/regions.json")"

{
  echo "# AWS Resource Ownership Inventory"
  echo
  echo "Issue: #484 / AWS-A"
  echo
  echo "Scope: Agent Logic business AWS account, read-only inventory only"
  echo
  echo "Profile: \`agent-logic-admin\`"
  echo
  echo "Account: \`${account_id}\`"
  echo
  echo "Caller principal: \`${principal_arn}\`"
  echo
  echo "Enabled/available region denominator: ${region_count}"
  echo
  echo "## Disposition vocabulary"
  echo
  echo "- \`owned\` — Agent Logic/ADL-owned resource or resource family."
  echo "- \`externally-owned\` — known non-ADL resource that must not be changed by ADL."
  echo "- \`frozen-unknown\` — discovered but not yet safely attributable; preserve until classified."
  echo "- \`not-observed\` — read-only census found no resource on that inspected surface."
  echo "- \`read-failed\` — read-only inventory for this surface failed and must be retried before mutation or cleanup."
  echo
  echo "## Evidence packet"
  echo
  echo "- Readbacks: \`docs/milestones/v0.92.1/evidence/cloud/aws-a/readbacks/\`"
  echo "- Command manifest: \`docs/milestones/v0.92.1/evidence/cloud/aws-a/readbacks/command-manifest.md\`"
  echo
  echo "## Resource inventory"
  echo
  echo "| Surface | Region | Resource | Disposition | Evidence |"
  echo "| --- | --- | --- | --- | --- |"
  echo "| account | global | \`${account_id}\` | owned | \`readbacks/account-identity.json\` |"
  echo "| regions | all | \`${region_count} enabled/available regions\` | owned | \`readbacks/regions.json\` |"

  jq -r '
    (.Buckets // [])[] |
    "| s3-bucket | global | `" + .Name + "` | frozen-unknown | `readbacks/s3-buckets.json` |"
  ' "${READBACKS}/s3-buckets.json"

  jq -r '
    (.HostedZones // [])[] |
    "| route53-zone | global | `" + (.Name // .Id) + "` | frozen-unknown | `readbacks/route53-hosted-zones.json` |"
  ' "${READBACKS}/route53-hosted-zones.json"

  jq -r '
    (.DistributionList.Items // [])[] |
    "| cloudfront-distribution | global | `" + (.Id // .ARN // .DomainName) + "` | frozen-unknown | `readbacks/cloudfront-distributions.json` |"
  ' "${READBACKS}/cloudfront-distributions.json"

  for file in "${READBACKS}"/regions/*.json; do
    base="$(basename "${file}" .json)"
    region="${base%%-*}"
    rest="${base#${region}-}"
    case "${rest}" in
      ec2-instances)
        jq -r --arg region "${region}" --arg evidence "readbacks/regions/${base}.json" '
          [(.Reservations // [])[].Instances[]?] as $items |
          if ($items | length) == 0 then
            "| ec2-instance | " + $region + " | `none observed` | not-observed | `" + $evidence + "` |"
          else
            $items[] | "| ec2-instance | " + $region + " | `" + .InstanceId + "` | frozen-unknown | `" + $evidence + "` |"
          end
        ' "${file}"
        ;;
      ec2-volumes)
        jq -r --arg region "${region}" --arg evidence "readbacks/regions/${base}.json" '
          (.Volumes // []) as $items |
          if ($items | length) == 0 then
            "| ebs-volume | " + $region + " | `none observed` | not-observed | `" + $evidence + "` |"
          else
            $items[] | "| ebs-volume | " + $region + " | `" + .VolumeId + "` | frozen-unknown | `" + $evidence + "` |"
          end
        ' "${file}"
        ;;
      vpcs)
        jq -r --arg region "${region}" --arg evidence "readbacks/regions/${base}.json" '
          (.Vpcs // []) as $items |
          if ($items | length) == 0 then
            "| vpc | " + $region + " | `none observed` | not-observed | `" + $evidence + "` |"
          else
            $items[] | "| vpc | " + $region + " | `" + .VpcId + "` | frozen-unknown | `" + $evidence + "` |"
          end
        ' "${file}"
        ;;
      subnets)
        jq -r --arg region "${region}" --arg evidence "readbacks/regions/${base}.json" '
          (.Subnets // []) as $items |
          if ($items | length) == 0 then
            "| subnet | " + $region + " | `none observed` | not-observed | `" + $evidence + "` |"
          else
            $items[] | "| subnet | " + $region + " | `" + .SubnetId + "` | frozen-unknown | `" + $evidence + "` |"
          end
        ' "${file}"
        ;;
      security-groups)
        jq -r --arg region "${region}" --arg evidence "readbacks/regions/${base}.json" '
          (.SecurityGroups // []) as $items |
          if ($items | length) == 0 then
            "| security-group | " + $region + " | `none observed` | not-observed | `" + $evidence + "` |"
          else
            $items[] | "| security-group | " + $region + " | `" + .GroupId + "` | frozen-unknown | `" + $evidence + "` |"
          end
        ' "${file}"
        ;;
      load-balancers)
        jq -r --arg region "${region}" --arg evidence "readbacks/regions/${base}.json" '
          (.LoadBalancers // []) as $items |
          if ($items | length) == 0 then
            "| load-balancer | " + $region + " | `none observed` | not-observed | `" + $evidence + "` |"
          else
            $items[] | "| load-balancer | " + $region + " | `" + (.LoadBalancerName // .LoadBalancerArn) + "` | frozen-unknown | `" + $evidence + "` |"
          end
        ' "${file}"
        ;;
      acm-certificates)
        jq -r --arg region "${region}" --arg evidence "readbacks/regions/${base}.json" '
          (.CertificateSummaryList // []) as $items |
          if ($items | length) == 0 then
            "| acm-certificate | " + $region + " | `none observed` | not-observed | `" + $evidence + "` |"
          else
            $items[] | "| acm-certificate | " + $region + " | `" + (.DomainName // .CertificateArn) + "` | frozen-unknown | `" + $evidence + "` |"
          end
        ' "${file}"
        ;;
      cloudformation-stacks)
        jq -r --arg region "${region}" --arg evidence "readbacks/regions/${base}.json" '
          (.StackSummaries // []) as $items |
          if ($items | length) == 0 then
            "| cloudformation-stack | " + $region + " | `none observed` | not-observed | `" + $evidence + "` |"
          else
            $items[] | "| cloudformation-stack | " + $region + " | `" + (.StackName // .StackId) + "` | frozen-unknown | `" + $evidence + "` |"
          end
        ' "${file}"
        ;;
    esac
  done
} > "${OUT}"
