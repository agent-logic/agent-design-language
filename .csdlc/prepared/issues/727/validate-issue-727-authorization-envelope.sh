#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-.}"
repo_root="$(cd "$repo_root" && git rev-parse --show-toplevel)"
cd "$repo_root"

authorization="${ADL_ISSUE_727_AUTHORIZATION_PATH:-.adl/requests/727/operator-authorization.json}"
if [[ ! -f "$authorization" ]]; then
  echo "operator authorization required before apply: $authorization" >&2
  exit 1
fi

if [[ "${AWS_PROFILE:-}" != "agent-logic-admin" ]]; then
  echo "AWS_PROFILE must be agent-logic-admin for issue #727 mutation authorization" >&2
  exit 1
fi

observed_account_id="$(aws --profile agent-logic-admin sts get-caller-identity --query Account --output text)"
if [[ ! "$observed_account_id" =~ ^[0-9]{12}$ ]]; then
  echo "authenticated AWS account identity is unavailable" >&2
  exit 1
fi

python3 .csdlc/prepared/issues/815/verify-cloud-authorization.py aws \
  --repo-root "$repo_root" \
  --packet "$authorization" \
  --observed-account-id "$observed_account_id"
