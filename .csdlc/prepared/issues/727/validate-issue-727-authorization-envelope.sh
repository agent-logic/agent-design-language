#!/usr/bin/env bash
set -euo pipefail

root="${1:-.}"
cd "$root"

authorization=".adl/requests/727/operator-authorization.json"
if [[ ! -f "$authorization" ]]; then
  echo "operator authorization required before apply: $authorization" >&2
  exit 1
fi

python3 - "$authorization" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as handle:
    data = json.load(handle)

required = [
    "account_identity_verified",
    "region",
    "terraform_root",
    "terraform_workspace",
    "state_key",
    "saved_plan_digest",
    "permitted_resources",
    "mutation_deadline",
    "cost_ceiling",
    "rollback_destroy_disposition",
]
missing = [key for key in required if key not in data]
if missing:
    raise SystemExit(f"authorization missing required fields: {', '.join(missing)}")

if data["region"] != "us-west-2":
    raise SystemExit("authorization region must be us-west-2")
if data["terraform_root"] != "infra/aws/account-foundation":
    raise SystemExit("authorization terraform_root must be infra/aws/account-foundation")
if data["terraform_workspace"] != "aws-d-account-foundation-live":
    raise SystemExit("authorization terraform_workspace must be aws-d-account-foundation-live")
if not isinstance(data["permitted_resources"], list) or not data["permitted_resources"]:
    raise SystemExit("authorization permitted_resources must be a non-empty list")
if not data["account_identity_verified"]:
    raise SystemExit("authorization must affirm account_identity_verified")

print("aws-d issue 727 authorization envelope validation passed")
PY
