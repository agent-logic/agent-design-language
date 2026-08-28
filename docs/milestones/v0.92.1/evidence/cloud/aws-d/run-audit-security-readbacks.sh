#!/usr/bin/env bash
set -euo pipefail

lane="${1:---lane=static}"
required_profile="agent-logic-admin"
profile="${AWS_PROFILE:-}"
environment="${AWS_D_ENVIRONMENT:-dev}"
name_prefix="${AWS_D_NAME_PREFIX:-agent-logic}"
resource_prefix="${name_prefix}-${environment}"
trail_name="${AWS_D_CLOUDTRAIL_NAME:-${resource_prefix}-account-activity}"
config_recorder_name="${AWS_D_CONFIG_RECORDER_NAME:-${resource_prefix}-config-recorder}"
config_channel_name="${AWS_D_CONFIG_CHANNEL_NAME:-${resource_prefix}-config-delivery}"
access_analyzer_name="${AWS_D_ACCESS_ANALYZER_NAME:-${resource_prefix}-access-analyzer}"
findings_topic_name="${AWS_D_FINDINGS_TOPIC_NAME:-${resource_prefix}-security-findings}"
event_rule_name="${AWS_D_EVENT_RULE_NAME:-${resource_prefix}-access-analyzer-findings}"
finding_owner="${AWS_D_FINDING_OWNER:-agent-logic-cloud-ops}"
finding_destination="${AWS_D_FINDING_DESTINATION:-security-ops-sns-topic}"
retention_days="${AWS_D_RETENTION_DAYS:-365}"

if [[ "$profile" != "$required_profile" ]]; then
  echo "AWS-D readback requires AWS_PROFILE=${required_profile}; got '${profile:-unset}'" >&2
  exit 1
fi

case "$lane" in
  --lane=static)
    echo "aws_d_readback_lane=static"
    echo "required_profile=${required_profile}"
    echo "cloud_calls=disabled"
    ;;
  --lane=aws-readonly)
    command -v aws >/dev/null 2>&1 || {
      echo "aws CLI is required for live readback" >&2
      exit 1
    }
    trail_bucket="$(aws cloudtrail describe-trails \
      --trail-name-list "$trail_name" \
      --query 'trailList[0].S3BucketName' \
      --output text)"
    if [[ -z "$trail_bucket" || "$trail_bucket" == "None" ]]; then
      echo "expected CloudTrail was not found" >&2
      exit 1
    fi

    trail_kms="$(aws cloudtrail describe-trails \
      --trail-name-list "$trail_name" \
      --query 'trailList[0].KmsKeyId' \
      --output text)"
    if [[ -z "$trail_kms" || "$trail_kms" == "None" ]]; then
      echo "expected CloudTrail KMS key was not found" >&2
      exit 1
    fi

    config_recorder_status="$(aws configservice describe-configuration-recorders \
      --configuration-recorder-names "$config_recorder_name" \
      --query 'ConfigurationRecorders[0].recordingGroup.allSupported' \
      --output text)"
    [[ "$config_recorder_status" == "True" ]] || {
      echo "expected AWS Config recorder is missing or not all-supported" >&2
      exit 1
    }

    config_channel_bucket="$(aws configservice describe-delivery-channels \
      --delivery-channel-names "$config_channel_name" \
      --query 'DeliveryChannels[0].s3BucketName' \
      --output text)"
    [[ "$config_channel_bucket" == "$trail_bucket" ]] || {
      echo "AWS Config delivery channel does not target the audit bucket" >&2
      exit 1
    }

    access_analyzer_count="$(aws accessanalyzer list-analyzers \
      --query "length(analyzers[?name=='${access_analyzer_name}' && status=='ACTIVE'])" \
      --output text)"
    [[ "$access_analyzer_count" == "1" ]] || {
      echo "expected active IAM Access Analyzer was not found" >&2
      exit 1
    }

    topic_arn="$(aws sns list-topics \
      --query "Topics[?ends_with(TopicArn, ':${findings_topic_name}')].TopicArn | [0]" \
      --output text)"
    if [[ -z "$topic_arn" || "$topic_arn" == "None" ]]; then
      echo "expected SNS findings topic was not found" >&2
      exit 1
    fi

    rule_state="$(aws events describe-rule \
      --name "$event_rule_name" \
      --query 'State' \
      --output text)"
    [[ "$rule_state" == "ENABLED" ]] || {
      echo "expected EventBridge findings rule is not enabled" >&2
      exit 1
    }

    target_count="$(aws events list-targets-by-rule \
      --rule "$event_rule_name" \
      --query "length(Targets[?Arn=='${topic_arn}'])" \
      --output text)"
    [[ "$target_count" == "1" ]] || {
      echo "EventBridge findings rule does not target the findings topic" >&2
      exit 1
    }

    encryption_ok="$(aws s3api get-bucket-encryption \
      --bucket "$trail_bucket" \
      --query "length(ServerSideEncryptionConfiguration.Rules[?ApplyServerSideEncryptionByDefault.SSEAlgorithm=='aws:kms'])" \
      --output text)"
    [[ "$encryption_ok" == "1" ]] || {
      echo "audit bucket is not KMS encrypted" >&2
      exit 1
    }

    lifecycle_ok="$(aws s3api get-bucket-lifecycle-configuration \
      --bucket "$trail_bucket" \
      --query "length(Rules[?Status=='Enabled' && Expiration.Days>=\`${retention_days}\`])" \
      --output text)"
    [[ "$lifecycle_ok" == "1" ]] || {
      echo "audit bucket retention lifecycle is missing or too short" >&2
      exit 1
    }

    tag_owner="$(aws sns list-tags-for-resource \
      --resource-arn "$topic_arn" \
      --query "Tags[?Key=='finding_owner'].Value | [0]" \
      --output text)"
    tag_destination="$(aws sns list-tags-for-resource \
      --resource-arn "$topic_arn" \
      --query "Tags[?Key=='finding_destination'].Value | [0]" \
      --output text)"
    [[ "$tag_owner" == "$finding_owner" && "$tag_destination" == "$finding_destination" ]] || {
      echo "findings topic owner/destination tags do not match expected contract" >&2
      exit 1
    }

    echo "aws_d_readback_lane=aws-readonly"
    echo "required_profile=${required_profile}"
    echo "cloudtrail_exact=present"
    echo "cloudtrail_kms=present"
    echo "config_recorder_exact=present"
    echo "config_delivery_bucket_matches=true"
    echo "access_analyzer_exact=active"
    echo "sns_findings_topic_exact=present"
    echo "eventbridge_findings_route=present"
    echo "audit_bucket_kms_encryption=present"
    echo "audit_bucket_retention_days_at_least=${retention_days}"
    echo "finding_owner_destination_tags=present"
    echo "redaction=names_and_arns_not_printed"
    ;;
  *)
    echo "unknown lane: ${lane}" >&2
    exit 2
    ;;
esac
