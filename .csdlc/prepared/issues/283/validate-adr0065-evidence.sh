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

require_sha256() {
  local path="$1"
  local expected="$2"
  require_file "${path}"
  [[ "${expected}" =~ ^[0-9a-f]{64}$ ]] || fail "invalid expected sha256 for ${path}: ${expected}"
  local actual
  actual="$(shasum -a 256 "${path}" | awk '{print $1}')"
  [[ "${actual}" == "${expected}" ]] || fail "sha256 mismatch for ${path}: expected ${expected}, got ${actual}"
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
    expected="$(jq -r --arg path "${artifact}" '.proof[] | select(.path == $path) | .sha256' .csdlc/evidence/209/local-validation-manifest.json)"
    require_sha256 "${artifact}" "${expected}"
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

require_sha256 \
  ".csdlc/evidence/209/native-platform/linux.json" \
  "$(jq -r '.jobs.linux.receipt_sha256' .csdlc/evidence/209/native-validation-manifest.json)"
require_sha256 \
  ".csdlc/evidence/209/native-platform/macos.json" \
  "$(jq -r '.jobs.macos.receipt_sha256' .csdlc/evidence/209/native-validation-manifest.json)"
require_sha256 \
  ".csdlc/evidence/209/native-platform/linux-nextest.log" \
  "$(jq -r '.jobs.linux.command_output_sha256' .csdlc/evidence/209/native-validation-manifest.json)"
require_sha256 \
  ".csdlc/evidence/209/native-platform/macos-nextest.log" \
  "$(jq -r '.jobs.macos.command_output_sha256' .csdlc/evidence/209/native-validation-manifest.json)"
require_sha256 \
  ".csdlc/evidence/209/native-platform/linux-semantic.json" \
  "$(jq -r '.jobs.linux.semantic_output_sha256' .csdlc/evidence/209/native-validation-manifest.json)"
require_sha256 \
  ".csdlc/evidence/209/native-platform/macos-semantic.json" \
  "$(jq -r '.jobs.macos.semantic_output_sha256' .csdlc/evidence/209/native-validation-manifest.json)"
require_sha256 \
  ".csdlc/evidence/209/native-platform/linux-source-manifest.json" \
  "$(jq -r '.source_manifest_sha256' .csdlc/evidence/209/native-validation-manifest.json)"
require_sha256 \
  ".csdlc/evidence/209/native-platform/macos-source-manifest.json" \
  "$(jq -r '.source_manifest_sha256' .csdlc/evidence/209/native-validation-manifest.json)"
require_sha256 \
  "$(jq -r '.independent_validation.output_ref' .csdlc/evidence/209/native-validation-manifest.json)" \
  "$(jq -r '.independent_validation.output_sha256' .csdlc/evidence/209/native-validation-manifest.json)"

jq -e '
  .schema == "adl.acip_native_receipts.v2"
  and .source_revision == "7c8569351ea4cbd1d9c9d94d7021a238c7c9599c"
  and (.receipts | type == "array" and length == 3)
  and all(.receipts[]; (.artifacts | type == "array" and length >= 1))
' .csdlc/evidence/5832/acip-native-receipts.json >/dev/null ||
  fail "#5832 native receipts manifest is not the expected historical/superseded native evidence shape"

jq -r '.receipts[].artifacts[] | [.path, .sha256] | @tsv' .csdlc/evidence/5832/acip-native-receipts.json |
  while IFS=$'\t' read -r artifact expected; do
    require_sha256 "${artifact}" "${expected}"
  done

printf 'PASS: ADR 0065 evidence inputs are present, non-empty, and classified for #283 reconciliation\n'
