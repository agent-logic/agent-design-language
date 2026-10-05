#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
POLICY="$ROOT/adl/tools/ci_path_policy.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_line() {
  grep -Fqx -- "$2" <<<"$1" || {
    echo "missing routing result: $2" >&2
    echo "$1" >&2
    exit 1
  }
}

cd "$TMP"
git init -q
git config user.email public-adl-ci@example.invalid
git config user.name public-adl-ci
git commit -q --allow-empty -m baseline
base="$(git rev-parse HEAD)"

mkdir -p adl-schema/src adl-uts/src adl-v2/crates/adl-language/src \
  adl-v2/crates/adl-compiler/src adl-legacy-contracts/src tools/public_adl
for path in \
  adl-schema/src/lib.rs \
  adl-uts/src/lib.rs \
  adl-v2/crates/adl-language/src/lib.rs \
  adl-v2/crates/adl-compiler/src/lib.rs \
  adl-legacy-contracts/src/lib.rs \
  tools/public_adl/verify_install_validator.py
do
  printf 'fixture\n' >"$path"
done
git add .
git commit -q -m public-adl-only
public_head="$(git rev-parse HEAD)"
public_output="$($POLICY --event-name pull_request --base "$base" --head "$public_head" --ref refs/pull/1/merge)"
assert_line "$public_output" "public_adl_validation_required=true"
assert_line "$public_output" "rust_required=false"
assert_line "$public_output" "runtime_coverage_required=false"
assert_line "$public_output" "csdlc_v2_standalone_required=false"
assert_line "$public_output" "csdlc_v3_standalone_required=false"
assert_line "$public_output" "adl_v2_standalone_required=false"
assert_line "$public_output" "codefriend_ci_required=false"
assert_line "$public_output" "fail_closed=false"

git checkout -q -b mixed "$base"
mkdir -p tools/public_adl adl-runtime/src
printf 'fixture\n' >tools/public_adl/verify_install_validator.py
printf 'fixture\n' >adl-runtime/src/lib.rs
git add .
git commit -q -m public-adl-runtime-mixed
mixed_output="$($POLICY --event-name pull_request --base "$base" --head HEAD --ref refs/pull/2/merge)"
assert_line "$mixed_output" "public_adl_validation_required=true"
assert_line "$mixed_output" "rust_required=true"
assert_line "$mixed_output" "runtime_coverage_required=true"
assert_line "$mixed_output" "codefriend_ci_required=true"

git checkout -q -b unknown "$base"
mkdir -p unknown-surface
printf 'fixture\n' >unknown-surface/value.bin
git add .
git commit -q -m unknown
unknown_output="$($POLICY --event-name pull_request --base "$base" --head HEAD --ref refs/pull/3/merge)"
assert_line "$unknown_output" "fail_closed=true"
assert_line "$unknown_output" "full_coverage_required=true"

git checkout -q -b public-policy "$base"
mkdir -p tools/public_adl .github/workflows
printf 'fixture\n' >tools/public_adl/verify_install_validator.py
printf 'name: ci\n' >.github/workflows/ci.yaml
git add .
git commit -q -m public-adl-policy-change
policy_output="$($POLICY --event-name pull_request --base "$base" --head HEAD --ref refs/pull/4/merge)"
assert_line "$policy_output" "public_adl_validation_required=true"
assert_line "$policy_output" "validation_profile_run_lanes=ci_path_policy_contracts,public_adl_distribution"
assert_line "$policy_output" "ci_contracts_required=true"
assert_line "$policy_output" "rust_required=false"
assert_line "$policy_output" "coverage_required=false"
assert_line "$policy_output" "full_coverage_required=false"
assert_line "$policy_output" "reason=coverage_policy_surface_tooling_change_runs_contract_validation"
assert_line "$policy_output" "validation_profile_primary_reason=ci_policy_surface_requires_path_policy_contract_checks"
assert_line "$policy_output" "fail_closed=false"
