#!/usr/bin/env bash
# Shared owner-binary resolution helpers for ADL shell wrappers. Source this file.

adl_owner_manifest_root() {
  if [[ -n "${ADL_TOOLING_MANIFEST_ROOT:-}" ]]; then
    if [[ -f "$ADL_TOOLING_MANIFEST_ROOT/adl/Cargo.toml" ]]; then
      printf '%s\n' "$ADL_TOOLING_MANIFEST_ROOT"
      return 0
    fi
    echo "ERROR: ADL_TOOLING_MANIFEST_ROOT does not contain adl/Cargo.toml: $ADL_TOOLING_MANIFEST_ROOT" >&2
    return 1
  fi

  local script_dir root
  script_dir="$(cd "$(dirname "${BASH_SOURCE[1]}")" && pwd)"
  root="$(cd "$script_dir/../.." && pwd)"
  if [[ -f "$root/adl/Cargo.toml" ]]; then
    printf '%s\n' "$root"
    return 0
  fi

  echo "ERROR: unable to locate ADL tooling manifest root; set ADL_TOOLING_MANIFEST_ROOT to the primary checkout root" >&2
  return 1
}

adl_owner_primary_root() {
  local root="$1"
  if [[ -n "${ADL_PRIMARY_CHECKOUT_ROOT:-}" ]]; then
    if [[ -f "$ADL_PRIMARY_CHECKOUT_ROOT/adl/Cargo.toml" ]]; then
      printf '%s\n' "$ADL_PRIMARY_CHECKOUT_ROOT"
      return 0
    fi
    echo "ERROR: ADL_PRIMARY_CHECKOUT_ROOT does not contain adl/Cargo.toml: $ADL_PRIMARY_CHECKOUT_ROOT" >&2
    return 1
  fi

  case "$root" in
    */.worktrees/*)
      printf '%s\n' "${root%%/.worktrees/*}"
      ;;
    *)
      printf '%s\n' "$root"
      ;;
  esac
}

adl_owner_run_if_executable() {
  local candidate="$1"
  shift
  if [[ -n "$candidate" && -x "$candidate" ]]; then
    exec "$candidate" "$@"
  fi
}

adl_owner_source_hash() {
  local root="$1"
  if git -C "$root" rev-parse --show-toplevel >/dev/null 2>&1; then
    (
      cd "$root"
      git ls-files --cached --others --exclude-standard -- adl/Cargo.toml adl/Cargo.lock adl/build.rs adl/src adl/tools/adl_provider_adapter.rs |
        grep -Ev '(^adl/src/cli/tests/|/tests\.rs$|/tests/)' |
        LC_ALL=C sort |
        while IFS= read -r path; do
          [[ -f "$path" ]] || continue
          shasum -a 256 "$path"
        done |
        shasum -a 256 |
        awk '{print $1}'
    )
    return 0
  fi
  (
    cd "$root"
    find adl -type f \( -path 'adl/src/*' -o -path 'adl/tools/adl_provider_adapter.rs' -o -name Cargo.toml -o -name Cargo.lock -o -name build.rs \) -print 2>/dev/null |
      grep -Ev '(^adl/src/cli/tests/|/tests\.rs$|/tests/)' |
      LC_ALL=C sort |
      while IFS= read -r path; do
        [[ -f "$path" ]] || continue
        shasum -a 256 "$path"
      done |
      shasum -a 256 |
      awk '{print $1}'
  )
}

adl_owner_stable_bin_dirs() {
  local root_dir="$1" primary_root="$2"
  if [[ -n "${ADL_OWNER_BIN_DIR:-}" ]]; then
    printf '%s\n' "$ADL_OWNER_BIN_DIR"
  fi
  printf '%s\n' "$root_dir/.adl/bin"
  if [[ "$primary_root" != "$root_dir" ]]; then
    printf '%s\n' "$primary_root/.adl/bin"
  fi
}

adl_owner_stable_binary_if_fresh() {
  local binary_name="$1" root_dir="$2" primary_root="$3"
  local bin_dir candidate provenance expected_hash actual_hash source_root
  while IFS= read -r bin_dir; do
    [[ -n "$bin_dir" ]] || continue
    candidate="$bin_dir/$binary_name"
    provenance="$bin_dir/.provenance/$binary_name.sha256"
    [[ -x "$candidate" && -f "$provenance" ]] || continue
    source_root="$root_dir"
    case "$bin_dir" in
      "$primary_root"/*) source_root="$primary_root" ;;
    esac
    expected_hash="$(cat "$provenance" 2>/dev/null || true)"
    actual_hash="$(adl_owner_source_hash "$source_root" 2>/dev/null || true)"
    [[ -n "$expected_hash" && -n "$actual_hash" && "$expected_hash" == "$actual_hash" ]] || continue
    printf '%s\n' "$candidate"
    return 0
  done < <(adl_owner_stable_bin_dirs "$root_dir" "$primary_root")
  return 1
}

adl_owner_run_target_dir_binary_if_present() {
  local binary_name="$1" target_dir="$2" root_dir="$3" primary_root="$4"
  shift 4
  [[ -n "$target_dir" ]] || return 0
  case "$target_dir" in
    /*)
      adl_owner_run_if_executable "$target_dir/debug/$binary_name" "$@"
      ;;
    *)
      adl_owner_run_if_executable "$PWD/$target_dir/debug/$binary_name" "$@"
      adl_owner_run_if_executable "$root_dir/adl/$target_dir/debug/$binary_name" "$@"
      adl_owner_run_if_executable "$primary_root/adl/$target_dir/debug/$binary_name" "$@"
      ;;
  esac
}

adl_owner_run_binary_resolution() {
  local binary_name="$1" explicit_bin="$2" disable_path_lookup="$3" root_dir="$4" primary_root="$5"
  local stable_bin
  shift 5
  adl_owner_run_if_executable "$explicit_bin" "$@"
  stable_bin="$(adl_owner_stable_binary_if_fresh "$binary_name" "$root_dir" "$primary_root" || true)"
  adl_owner_run_if_executable "$stable_bin" "$@"
  adl_owner_run_target_dir_binary_if_present "$binary_name" "${CARGO_TARGET_DIR:-}" "$root_dir" "$primary_root" "$@"
  adl_owner_run_target_dir_binary_if_present "$binary_name" "${CARGO_LLVM_COV_TARGET_DIR:-}" "$root_dir" "$primary_root" "$@"
  adl_owner_run_if_executable "$root_dir/adl/target/debug/$binary_name" "$@"
  adl_owner_run_if_executable "$primary_root/adl/target/debug/$binary_name" "$@"
  adl_owner_run_if_executable "$root_dir/adl/target/llvm-cov-target/debug/$binary_name" "$@"
  adl_owner_run_if_executable "$primary_root/adl/target/llvm-cov-target/debug/$binary_name" "$@"
  if [[ "$disable_path_lookup" != "1" ]] && command -v "$binary_name" >/dev/null 2>&1; then
    exec "$binary_name" "$@"
  fi
}
