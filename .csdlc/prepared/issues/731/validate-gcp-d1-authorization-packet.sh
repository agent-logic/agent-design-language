#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

packet="${1:-.csdlc/evidence/731/mutation-authorization-request.json}"
now_epoch="${ADL_GCP_D1_NOW_EPOCH:-$(date -u +%s)}"

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
  (.impersonated_identity | test("^[A-Za-z0-9._-]+@[A-Za-z0-9._-]+[.]iam[.]gserviceaccount[.]com$")) and
  (.proof_run_spend_cap_usd == 1) and
  (.foundation_steady_state_30d_cap_usd == 5) and
  (.rollback_commands | type == "array" and length >= 2) and
  (.operator_authorization.status == "pending" or .operator_authorization.status == "approved")
' "$packet" >/dev/null || fail "authorization packet is incomplete or outside #731 bounds"

status="$(jq -r '.operator_authorization.status' "$packet")"
approved_at="$(jq -r '.operator_authorization.approved_at_utc // ""' "$packet")"
deadline_utc="$(jq -r '.cleanup_deadline_utc' "$packet")"

utc_epoch() {
  date -j -u -f '%Y-%m-%dT%H:%M:%SZ' "$1" +%s 2>/dev/null || return 1
}

deadline_epoch="$(utc_epoch "$deadline_utc")" || fail "cleanup deadline is not parseable UTC"
test "$deadline_epoch" -gt "$now_epoch" || fail "cleanup deadline is expired"

if test "$status" = "approved"; then
  test -n "$approved_at" || fail "approved authorization is missing approved_at_utc"
  approved_epoch="$(utc_epoch "$approved_at")" || fail "approved_at_utc is not parseable UTC"
  approval_age_seconds=$((now_epoch - approved_epoch))
  lifetime_seconds=$((deadline_epoch - approved_epoch))
  test "$approval_age_seconds" -ge 0 || fail "approved_at_utc is in the future"
  test "$approval_age_seconds" -le 7200 || fail "approved authorization is older than 120 minutes"
  test "$lifetime_seconds" -gt 0 || fail "cleanup deadline is before approval time"
  test "$lifetime_seconds" -le 1800 || fail "cleanup lifetime exceeds 30 minutes"
fi

printf 'PASS: #731 mutation authorization packet is complete and status=%s\n' "$status"
