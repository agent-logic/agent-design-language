#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
BIN="$TMP/bin"; mkdir -p "$BIN"
cat >"$BIN/aws" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
echo "aws $*" >>"${FAKE_AWS_LOG:?}"
case "$1 $2" in
  "sts get-caller-identity") printf '123456789012\n' ;;
  "s3api head-bucket") exit 1 ;;
  "s3api get-public-access-block") printf '{"PublicAccessBlockConfiguration":{"BlockPublicAcls":true,"IgnorePublicAcls":true,"BlockPublicPolicy":true,"RestrictPublicBuckets":true}}\n' ;;
  "s3api get-bucket-encryption") printf '{"ServerSideEncryptionConfiguration":{"Rules":[{"ApplyServerSideEncryptionByDefault":{"SSEAlgorithm":"AES256"},"BucketKeyEnabled":true}]}}\n' ;;
  "s3api get-bucket-versioning") printf '{"Status":"Enabled"}\n' ;;
  "s3api get-object-lock-configuration") printf '{"ObjectLockConfiguration":{"ObjectLockEnabled":"Enabled","Rule":{"DefaultRetention":{"Mode":"GOVERNANCE","Days":365}}}}\n' ;;
  "s3api get-bucket-lifecycle-configuration") printf '{"Rules":[{"ID":"adl-wp08-community-memory-archive-tiering","Status":"Enabled","Filter":{"Prefix":"community-memory/"},"Transitions":[{"Days":90,"StorageClass":"GLACIER_IR"},{"Days":365,"StorageClass":"DEEP_ARCHIVE"}],"NoncurrentVersionTransitions":[{"NoncurrentDays":30,"StorageClass":"GLACIER_IR"},{"NoncurrentDays":180,"StorageClass":"DEEP_ARCHIVE"}],"AbortIncompleteMultipartUpload":{"DaysAfterInitiation":7}}]}\n' ;;
  "s3api create-bucket"|"s3api put-public-access-block"|"s3api put-bucket-encryption"|"s3api put-bucket-versioning"|"s3api put-object-lock-configuration"|"s3api put-bucket-lifecycle-configuration") exit 0 ;;
  *) echo "unexpected aws call $*" >&2; exit 1 ;;
esac
SH
chmod +x "$BIN/aws"
export AWS_BIN="$BIN/aws" FAKE_AWS_LOG="$TMP/aws.log"
EXPECTED="$(printf '123456789012' | shasum -a 256 | awk '{print $1}')"
bash "$ROOT/adl/tools/setup_wp08_s3_obsmem_archive_policy.sh" --out "$TMP/proof" --expected-account-sha256 "$EXPECTED"
python3 "$ROOT/adl/tools/validate_wp08_s3_obsmem_archive_policy.py" "$TMP/proof/archive_policy_summary.json" >/dev/null
for call in "create-bucket" "put-public-access-block" "put-bucket-encryption" "put-bucket-versioning" "put-object-lock-configuration" "put-bucket-lifecycle-configuration"; do
  grep -F "$call" "$FAKE_AWS_LOG" >/dev/null
done
: >"$FAKE_AWS_LOG"
if bash "$ROOT/adl/tools/setup_wp08_s3_obsmem_archive_policy.sh" --out "$TMP/bad" --expected-account-sha256 0000000000000000000000000000000000000000000000000000000000000000 2>/tmp/wp08-s3-bad.err; then
  echo "expected mismatch failure" >&2; exit 1
fi
if grep -F "s3api" "$FAKE_AWS_LOG" >/dev/null; then
  echo "mismatch reached s3api" >&2; cat "$FAKE_AWS_LOG" >&2; exit 1
fi
echo "PASS test_setup_wp08_s3_obsmem_archive_policy"
