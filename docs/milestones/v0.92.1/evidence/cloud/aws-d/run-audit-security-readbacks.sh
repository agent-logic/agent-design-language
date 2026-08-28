#!/usr/bin/env bash
set -euo pipefail

lane="${1:---lane=static}"
required_profile="agent-logic-admin"
profile="${AWS_PROFILE:-}"

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
    echo "aws_d_readback_lane=aws-readonly"
    echo "required_profile=${required_profile}"
    echo "cloudtrail_count=$(aws cloudtrail describe-trails --query 'length(trailList)' --output text)"
    echo "config_recorders=$(aws configservice describe-configuration-recorders --query 'length(ConfigurationRecorders)' --output text)"
    echo "access_analyzers=$(aws accessanalyzer list-analyzers --query 'length(analyzers)' --output text)"
    echo "sns_topics_checked=redacted"
    ;;
  *)
    echo "unknown lane: ${lane}" >&2
    exit 2
    ;;
esac

