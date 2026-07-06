#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

BIN="$TMP/bin"
mkdir -p "$BIN"

cat >"$BIN/aws" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
echo "aws $*" >>"${FAKE_AWS_LOG:?}"
case "$1 $2" in
  "sts get-caller-identity")
    printf '%s\n' "fixture-agent-logic-account"
    ;;
  *)
    exit 0
    ;;
esac
SH
chmod +x "$BIN/aws"

cat >"$BIN/csm" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
test "$1" = "storage"
test "$2" = "prove-s3"
OUT=""
BUCKET=""
PREFIX=""
PROFILE=""
REGION=""
EXPECTED=""
RUN_ID=""
AWS_BIN_ARG=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --out) OUT="${2:?--out requires a value}"; shift ;;
    --bucket) BUCKET="${2:?--bucket requires a value}"; shift ;;
    --prefix) PREFIX="${2:?--prefix requires a value}"; shift ;;
    --profile) PROFILE="${2:?--profile requires a value}"; shift ;;
    --region) REGION="${2:?--region requires a value}"; shift ;;
    --expected-account-sha256) EXPECTED="${2:?--expected-account-sha256 requires a value}"; shift ;;
    --run-id) RUN_ID="${2:?--run-id requires a value}"; shift ;;
    --aws-bin) AWS_BIN_ARG="${2:?--aws-bin requires a value}"; shift ;;
  esac
  shift
done
test "$PROFILE" = "agent-logic-admin"
test "$REGION" = "us-west-2"
test "$PREFIX" = "community-memory/"
test "$EXPECTED" = "${EXPECTED_FIXTURE_ACCOUNT_SHA256:?}"
test "$RUN_ID" = "fixture-run"
test -n "$AWS_BIN_ARG"
mkdir -p "$OUT/restore"
cat >"$OUT/polis_state_snapshot.json" <<JSON
{
  "schema": "adl.csm.polis_state_storage_payload.v1",
  "issue": 4913,
  "run_id": "$RUN_ID",
  "artifact_kind": "snapshot",
  "state": {"checkpoint_ref": "checkpoint-fixture"}
}
JSON
cp "$OUT/polis_state_snapshot.json" "$OUT/restore/polis_state_snapshot.restored.json"
printf '%s\ncorruption-for-negative-proof\n' "$(cat "$OUT/polis_state_snapshot.json")" >"$OUT/restore/polis_state_snapshot.corrupt.json"
PAYLOAD_SHA="$(shasum -a 256 "$OUT/polis_state_snapshot.json" | awk '{print $1}')"
PAYLOAD_BYTES="$(wc -c <"$OUT/polis_state_snapshot.json" | tr -d ' ')"
cat >"$OUT/artifact_durability_taxonomy.json" <<JSON
{
  "schema": "adl.csm.polis_artifact_durability_taxonomy.v1",
  "issue": 4913,
  "backend": {
    "kind": "aws_s3",
    "bucket_name": "$BUCKET",
    "prefix": "$PREFIX",
    "durability_posture": "vendor_11_nines_per_object_non_12_nines_claim",
    "controls": ["versioning", "object_lock_governance_retention", "sse_s3_encryption", "public_access_block", "lifecycle_transition_policy"]
  },
  "artifact_classes": [
    {"artifact_kind":"checkpoint"},
    {"artifact_kind":"event_log"},
    {"artifact_kind":"snapshot"},
    {"artifact_kind":"diff"},
    {"artifact_kind":"freeze_dry_bundle"},
    {"artifact_kind":"security_evidence"}
  ],
  "non_claims": ["single-region S3 Standard is not recorded as mathematical 12-nines durability"]
}
JSON
cat >"$OUT/polis_storage_proof_summary.json" <<JSON
{
  "schema": "adl.csm.polis_durable_storage_proof.v1",
  "issue": 4913,
  "status": "passed",
  "run_id": "$RUN_ID",
  "checked_at_utc": "2026-07-06T00:00:00Z",
  "aws_profile": "$PROFILE",
  "aws_region": "$REGION",
  "aws_account_hash": "2a33349e7e606a8a",
  "aws_account_matches_expected": true,
  "bucket_name": "$BUCKET",
  "bucket_name_hash": "abc123abc123abcd",
  "object": {
    "key": "community-memory/polis-state/fixture-run/snapshot.json",
    "version_id": "version-fixture",
    "payload_sha256": "$PAYLOAD_SHA",
    "payload_bytes": $PAYLOAD_BYTES,
    "metadata_sha256_matches": true,
    "server_side_encryption": "AES256",
    "object_lock_mode": "GOVERNANCE",
    "object_lock_retain_until_date": "2027-07-06T00:00:00Z"
  },
  "restored_artifact": {
    "restore_ref": "restore/polis_state_snapshot.restored.json",
    "restored_sha256": "$PAYLOAD_SHA",
    "checksum_matches": true
  },
  "negative_cases": {
    "missing_object": {"status": "passed", "expected_failure": "missing", "observed_failure_class": "s3_head_object_missing_or_not_found", "raw_error_retained": false},
    "corrupted_restore": {"status": "passed", "expected_failure": "corrupt", "observed_failure_class": "checksum_mismatch_detected", "raw_error_retained": false},
    "unsigned_access_denial": {"status": "passed", "expected_failure": "deny", "observed_failure_class": "unsigned_access_denied_or_forbidden", "raw_error_retained": false}
  },
  "durability_contract": {
    "target_class": "12-nines-class target; selected S3 backend is vendor 11-nines per-object durability and is therefore a non-12-nines mathematical claim",
    "backend": "AWS S3 Standard",
    "artifact_taxonomy_ref": "artifact_durability_taxonomy.json",
    "selected_backend_assumptions": ["vendor 11 nines", "object lock"],
    "local_proof_scope": ["write", "read", "restore", "missing", "corrupt", "deny"]
  },
  "retained_artifacts": ["polis_storage_proof_summary.json"],
  "non_claims": ["This proof does not claim mathematical 12-nines durability from a single-region S3 bucket."],
  "redaction": {
    "raw_account_id_retained": false,
    "full_account_digest_retained": false,
    "aws_credentials_retained": false,
    "raw_aws_errors_retained": false
  }
}
JSON
printf '{"status":"passed"}\n'
SH
chmod +x "$BIN/csm"

export PATH="$BIN:$PATH"
export FAKE_AWS_LOG="$TMP/aws.log"
export EXPECTED_FIXTURE_ACCOUNT_SHA256
EXPECTED_FIXTURE_ACCOUNT_SHA256="$(
  printf '%s' "fixture-agent-logic-account" | shasum -a 256 | awk '{print $1}'
)"

bash "$ROOT/adl/tools/run_wp08_polis_storage_live_proof.sh" \
  --out "$TMP/proof" \
  --profile agent-logic-admin \
  --region us-west-2 \
  --run-id fixture-run \
  --csm-bin "$BIN/csm" \
  --expected-account-sha256 "$EXPECTED_FIXTURE_ACCOUNT_SHA256" \
  >/tmp/wp08-polis-storage-test-output.json

python3 "$ROOT/adl/tools/validate_wp08_polis_storage_live_proof.py" \
  "$TMP/proof/polis_storage_proof_summary.json" >/dev/null

grep -F "sts get-caller-identity" "$FAKE_AWS_LOG" >/dev/null
grep -F "agent-logic-admin" "$FAKE_AWS_LOG" >/dev/null

if bash "$ROOT/adl/tools/run_wp08_polis_storage_live_proof.sh" \
  --out "$TMP/bad-proof" \
  --profile agent-logic-admin \
  --region us-west-2 \
  --run-id bad-fixture-run \
  --csm-bin "$BIN/csm" \
  --expected-account-sha256 ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff \
  >"$TMP/bad.out" 2>"$TMP/bad.err"; then
  echo "expected mismatched account hash to fail" >&2
  exit 1
fi
grep -F "AWS profile account hash does not match expected Agent Logic account hash" "$TMP/bad.err" >/dev/null

echo "PASS test_run_wp08_polis_storage_live_proof"
