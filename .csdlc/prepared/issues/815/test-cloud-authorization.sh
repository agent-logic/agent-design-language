#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

python3 .csdlc/prepared/issues/815/test_cloud_authorization.py

if rg -q -- '--test-mode|--trusted-signers' \
  .csdlc/prepared/issues/727/validate-issue-727-authorization-envelope.sh \
  .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh \
  .csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh \
  .csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh; then
  echo "FAIL: a live mutation entrypoint exposes the test trust-anchor override" >&2
  exit 1
fi
if rg -q 'ADL_GCP_D1_NOW_EPOCH' .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh; then
  echo "FAIL: live GCP authorization expiry accepts a caller-controlled clock" >&2
  exit 1
fi
if rg -q 'ADL_GCP_D1_(NOW_EPOCH|REAPER_SLEEP_SECONDS)' \
  .csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh; then
  echo "FAIL: live GCP cleanup timing accepts a caller-controlled clock or reaper delay" >&2
  exit 1
fi
if rg -q 'jq[^\n]*[$]packet' \
  .csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh \
  .csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh; then
  echo "FAIL: GCP mutation wrapper rereads the mutable authorization packet" >&2
  exit 1
fi
if ! rg -q 'verified_plan_file' \
  .csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh; then
  echo "FAIL: GCP foundation apply does not consume the verified plan snapshot" >&2
  exit 1
fi
if rg -q 'Path[.]home' .csdlc/prepared/issues/815/verify-cloud-authorization.py; then
  echo "FAIL: production trust anchor must not follow caller-controlled HOME" >&2
  exit 1
fi

guard_dir="$repo_root/.csdlc/evidence/815/wrapper-preflight"
mkdir -p "$guard_dir"
trap 'rm -rf "$guard_dir"' EXIT
mock_gcloud="$guard_dir/gcloud"
mock_log="$guard_dir/provider-called.log"
cat > "$mock_gcloud" <<'MOCK'
#!/usr/bin/env bash
printf 'provider called\n' >> "${ADL_815_PROVIDER_LOG:?}"
exit 99
MOCK
chmod 700 "$mock_gcloud"
: > "$mock_log"
if ADL_GCP_D1_GCLOUD_BIN="$mock_gcloud" \
  ADL_GCP_D1_OUT_DIR="$guard_dir/workload" \
  ADL_815_PROVIDER_LOG="$mock_log" \
  bash .csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh >/dev/null 2>&1; then
  echo "FAIL: historical unsigned GCP packet reached mutation execution" >&2
  exit 1
fi
if test -s "$mock_log"; then
  echo "FAIL: GCP provider was called before authentic authorization" >&2
  exit 1
fi

if rg -q 'validate-gcp-d1-authorization-packet' .csdlc/prepared/issues/731/run-gcp-d1-readonly-preflight.sh; then
  echo "FAIL: GCP read-only preflight must not require mutation authorization" >&2
  exit 1
fi

bash docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh --lane=static >/dev/null

echo "PASS: read-only GCP and AWS proof remains available without mutation authority"
