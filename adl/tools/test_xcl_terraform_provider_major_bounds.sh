#!/usr/bin/env bash
set -euo pipefail

# PVF: lane=dependency-policy-static; issue=765; proof=reject open-ended XCL Terraform provider major floors.

usage() {
  cat >&2 <<'USAGE'
Usage:
  adl/tools/test_xcl_terraform_provider_major_bounds.sh [--self-test]

Validates that the XCL Terraform release roots pin provider eligibility to the
currently supported major selected by their lockfiles:
  - infra/aws/runtime/xcl-01: hashicorp/aws >= 6.0.0, < 7.0.0
  - infra/gcp/workloads/xcl-01: hashicorp/google >= 8.0.0, < 9.0.0

--self-test creates a repo-local ignored scratch fixture under .git/csdlc-v3
and proves the checker rejects an open-ended provider-major floor.
USAGE
}

repo_root="$(git rev-parse --show-toplevel)"
git_common_dir="$(git rev-parse --git-common-dir)"

check_root() {
  local provider="$1"
  local expected_constraint="$2"
  local expected_version_major="$3"
  local versions_file="$4"
  local lock_file="$5"

  local source_constraint
  source_constraint="$(awk -v provider="$provider" '
    $1 == provider && $2 == "=" { in_provider = 1 }
    in_provider && $1 == "version" && $2 == "=" {
      line = $0
      sub(/^[[:space:]]*version[[:space:]]*=[[:space:]]*"/, "", line)
      sub(/"[[:space:]]*$/, "", line)
      print line
      exit
    }
    in_provider && $1 == "}" { in_provider = 0 }
  ' "$versions_file")"

  if [[ "$source_constraint" != "$expected_constraint" ]]; then
    echo "FAIL: $versions_file uses $provider constraint '$source_constraint', expected '$expected_constraint'" >&2
    return 1
  fi

  if [[ "$source_constraint" =~ ^\>\=[[:space:]]*[0-9]+(\.[0-9]+)?(\.[0-9]+)?$ ]]; then
    echo "FAIL: $versions_file leaves $provider on an open-ended provider-major floor: $source_constraint" >&2
    return 1
  fi

  local lock_version
  lock_version="$(awk '
    $1 == "version" && $2 == "=" {
      gsub(/"/, "", $3)
      print $3
      exit
    }
  ' "$lock_file")"
  if [[ "$lock_version" != "$expected_version_major".* ]]; then
    echo "FAIL: $lock_file selected version '$lock_version' does not satisfy major $expected_version_major" >&2
    return 1
  fi

  local lock_constraint
  lock_constraint="$(awk '
    $1 == "constraints" && $2 == "=" {
      line = $0
      sub(/^[[:space:]]*constraints[[:space:]]*=[[:space:]]*"/, "", line)
      sub(/"[[:space:]]*$/, "", line)
      print line
      exit
    }
  ' "$lock_file")"
  if [[ "$lock_constraint" != "$expected_constraint" ]]; then
    echo "FAIL: $lock_file records constraint '$lock_constraint', expected '$expected_constraint'" >&2
    return 1
  fi
}

run_check() {
  local root="${1:-$repo_root}"
  local status=0
  check_root \
    "aws" \
    ">= 6.0.0, < 7.0.0" \
    "6" \
    "$root/infra/aws/runtime/xcl-01/versions.tf" \
    "$root/infra/aws/runtime/xcl-01/.terraform.lock.hcl" || status=1
  check_root \
    "google" \
    ">= 8.0.0, < 9.0.0" \
    "8" \
    "$root/infra/gcp/workloads/xcl-01/versions.tf" \
    "$root/infra/gcp/workloads/xcl-01/.terraform.lock.hcl" || status=1
  return "$status"
}

run_self_test() {
  local scratch="$git_common_dir/csdlc-v3/test-artifacts/765-provider-major-bounds/self-test-$$"
  mkdir -p "$scratch/infra/aws/runtime/xcl-01" "$scratch/infra/gcp/workloads/xcl-01"
  cp "$repo_root/infra/aws/runtime/xcl-01/versions.tf" "$scratch/infra/aws/runtime/xcl-01/versions.tf"
  cp "$repo_root/infra/aws/runtime/xcl-01/.terraform.lock.hcl" "$scratch/infra/aws/runtime/xcl-01/.terraform.lock.hcl"
  cp "$repo_root/infra/gcp/workloads/xcl-01/versions.tf" "$scratch/infra/gcp/workloads/xcl-01/versions.tf"
  cp "$repo_root/infra/gcp/workloads/xcl-01/.terraform.lock.hcl" "$scratch/infra/gcp/workloads/xcl-01/.terraform.lock.hcl"

  perl -0pi -e 's/>= 6\.0\.0, < 7\.0\.0/>= 6.0.0/' "$scratch/infra/aws/runtime/xcl-01/versions.tf"

  if run_check "$scratch" >/dev/null 2>&1; then
    echo "FAIL: self-test open-ended provider-major fixture unexpectedly passed" >&2
    return 1
  fi

  echo "self-test: open-ended provider-major fixture rejected"
}

case "${1:-}" in
  "")
    run_check "$repo_root"
    ;;
  "--self-test")
    run_self_test
    run_check "$repo_root"
    ;;
  "-h"|"--help")
    usage
    ;;
  *)
    usage
    exit 2
    ;;
esac
