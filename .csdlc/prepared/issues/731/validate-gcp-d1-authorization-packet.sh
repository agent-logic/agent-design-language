#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

packet="${1:-.csdlc/evidence/731/mutation-authorization-request.json}"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

test -f "$packet" || fail "missing authorization packet: $packet"

jq -e '
  .schema == "adl.gcp_d1.mutation_authorization_request.v1" and
  .issue == 731 and
  .project == "cs-host-377d41e71a824f92802120" and
  .region == "us-west2" and
  .zone == "us-west2-a" and
  .network == "axioma-dev-csm-private" and
  .subnet == "axioma-dev-csm-private-us-west2" and
  (.plan_digest | type == "string" and length >= 64) and
  (.run_id | test("^gcp-d1-[0-9]{8}t[0-9]{6}z-[a-z0-9]{6,}$")) and
  (.instance_name | test("^adl-gcp-d1-[a-z0-9-]+$")) and
  (.cleanup_deadline_utc | test("Z$")) and
  (.impersonated_identity | type == "string" and length > 5) and
  (.proof_run_spend_cap_usd == 1) and
  (.foundation_steady_state_30d_cap_usd == 5) and
  (.rollback_commands | type == "array" and length >= 2) and
  (.operator_authorization.status == "pending")
' "$packet" >/dev/null || fail "authorization packet is incomplete or outside #731 bounds"

printf 'PASS: #731 mutation authorization packet is complete and pending operator approval\n'
