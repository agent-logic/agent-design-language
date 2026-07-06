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
    printf '%s\n' "123456789012"
    ;;
  "cloudfront list-distributions")
    cat <<'JSON'
{"DistributionList":{"Quantity":1,"Items":[{"Id":"E123ABC456DEF","DomainName":"fixture.cloudfront.net","Status":"Deployed","Enabled":true,"LastModifiedTime":"2026-07-06T00:00:00Z","Aliases":{"Quantity":1,"Items":["polis.example.test"]}}]}}
JSON
    ;;
  "cloudfront get-distribution")
    id=""
    while [ "$#" -gt 0 ]; do
      if [ "$1" = "--id" ]; then
        id="${2:-}"
        break
      fi
      shift
    done
    if [ "$id" = "E-NOTFOUND" ]; then
      echo "An error occurred (NoSuchDistribution) when calling the GetDistribution operation" >&2
      exit 255
    fi
    cat <<'JSON'
{"ETag":"ETAG-FIXTURE","Distribution":{"Id":"E123ABC456DEF"}}
JSON
    ;;
  *)
    echo "unexpected aws args: $*" >&2
    exit 2
    ;;
esac
SH
chmod +x "$BIN/aws"

cat >"$BIN/csm" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
exec "${REAL_CSM:?}" "$@" --aws-bin "${AWS_BIN:?}"
SH
chmod +x "$BIN/csm"

export FAKE_AWS_LOG="$TMP/aws.log"
export AWS_BIN="$BIN/aws"
export REAL_CSM="$ROOT/adl/target/debug/csm"

cargo build --manifest-path "$ROOT/adl/Cargo.toml" --bin csm >/dev/null

bash "$ROOT/adl/tools/run_wp08_cloudfront_control_proof.sh" \
  --out "$TMP/proof" \
  --profile agent-logic-admin \
  --region us-west-2 \
  --run-id fixture-run \
  --csm-bin "$BIN/csm" \
  --expected-account-sha256 2a33349e7e606a8ad2e30e3c84521f9377450cf09083e162e0a9b1480ce0f972 \
  --negative-distribution-id E-NOTFOUND >/dev/null

python3 "$ROOT/adl/tools/validate_wp08_cloudfront_control_proof.py" \
  "$TMP/proof/cloudfront_status_summary.json" >/dev/null

python3 - "$TMP/proof/cloudfront_status_summary.json" "$FAKE_AWS_LOG" <<'PY'
import json
import sys
from pathlib import Path

summary = json.loads(Path(sys.argv[1]).read_text())
aws_log = Path(sys.argv[2]).read_text()
assert summary["schema"] == "adl.wp08.cloud_control_cloudfront.v1"
assert summary["status"] == "passed"
assert summary["cloudfront"]["distribution_count"] == 1
assert summary["cloudfront"]["selected_status"] == "Deployed"
assert summary["live_negative_cases"]["nonexistent_distribution"] == "cloudfront_distribution_not_found"
assert "123456789012" not in Path(sys.argv[1]).read_text()
assert "E123ABC456DEF" not in Path(sys.argv[1]).read_text()
assert "fixture.cloudfront.net" not in Path(sys.argv[1]).read_text()
for required in [
    "sts get-caller-identity",
    "cloudfront list-distributions",
    "cloudfront get-distribution",
]:
    assert required in aws_log, required
PY

echo "PASS test_run_wp08_cloudfront_control_proof"
