#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
git_common_dir="$(git rev-parse --git-common-dir)"
if [[ "${git_common_dir}" != /* ]]; then
  git_common_dir="${repo_root}/${git_common_dir}"
fi
cd "${repo_root}"

fail() {
  printf 'FAIL: %s\n' "$*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "${path}" ]] || fail "missing file: ${path}"
  [[ -s "${path}" ]] || fail "empty file: ${path}"
}

require_json() {
  local path="$1"
  require_file "${path}"
  jq -e . "${path}" >/dev/null || fail "invalid json: ${path}"
}

terminal_209="${git_common_dir}/csdlc-v2/derived-terminal/209.json"

require_json "${terminal_209}"
require_json ".csdlc/evidence/209/local-validation-manifest.json"
require_json ".csdlc/evidence/209/native-validation-manifest.json"
require_json ".csdlc/evidence/5832/acip-native-receipts.json"

jq -e '
  .schema == "csdlc.derived_terminal.v1"
  and .issue == 209
  and .repository == "agent-logic/agent-design-language"
  and .pull_request == 215
  and .disposition == "merged"
  and .head_sha == "c640066f284a915b638add377cc4b0a2e221e6f9"
  and .merge_sha == "a77519c3fca9f64752af41c9a2ebd396468891f7"
  and .issue_state == "closed_by_merged_pr"
  and (.digest | type == "string" and length == 64)
' "${terminal_209}" >/dev/null ||
  fail "derived terminal cache for #209 does not bind expected merged authority"

jq -e '
  .schema == "adl.wp14.production-acip.local-validation.v2"
  and .issue == 209
  and .status == "passed"
  and .source_revision == "aef6729640dc89918f34b4337a27167c6ed625fb"
  and (.proof | type == "array" and length >= 6)
  and (.proof[] | (.path | type == "string") and (.sha256 | type == "string" and length == 64))
' .csdlc/evidence/209/local-validation-manifest.json >/dev/null ||
  fail "#209 local validation manifest is not passed, exact-revision-bound, and artifact-bound"

jq -r '.proof[].path' .csdlc/evidence/209/local-validation-manifest.json |
  while IFS= read -r artifact; do
    require_file "${artifact}"
    actual="$(shasum -a 256 "${artifact}" | awk '{print $1}')"
    expected="$(jq -r --arg path "${artifact}" '.proof[] | select(.path == $path) | .sha256' .csdlc/evidence/209/local-validation-manifest.json)"
    [[ "${actual}" == "${expected}" ]] || fail "sha256 mismatch for ${artifact}: expected ${expected}, got ${actual}"
  done

jq -e '
  .schema == "adl.native_validation_manifest.v1"
  and .issue == 209
  and .repository == "agent-logic/agent-design-language"
  and .pull_request == 215
  and .validated_revision == "c640066f284a915b638add377cc4b0a2e221e6f9"
  and .workflow == ".github/workflows/wp14-production-acip-repair.yml"
  and .jobs.linux.status == "success"
  and .jobs.macos.status == "success"
  and .jobs.aggregate.status == "success"
  and .jobs.linux.tests_passed == 2
  and .jobs.macos.tests_passed == 2
  and .independent_validation.status == "passed"
  and (.artifact.archive_sha256 | type == "string" and length == 64)
' .csdlc/evidence/209/native-validation-manifest.json >/dev/null ||
  fail "#209 native validation manifest is not successful and exact-head-bound"

for artifact in \
  .csdlc/evidence/209/native-platform/linux.json \
  .csdlc/evidence/209/native-platform/macos.json \
  .csdlc/evidence/209/native-platform/linux-semantic.json \
  .csdlc/evidence/209/native-platform/macos-semantic.json \
  .csdlc/evidence/209/native-receipts-validation.log \
  .csdlc/evidence/5832/native/linux/receipt.json \
  .csdlc/evidence/5832/native/macos/receipt.json \
  .csdlc/evidence/5832/native/windows/receipt.json; do
  require_file "${artifact}"
done

printf 'PASS: ADR 0065 evidence inputs are present, non-empty, and classified for #283 reconciliation\n'
