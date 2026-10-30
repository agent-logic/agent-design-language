#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat >&2 <<'USAGE'
usage: rust_cache_env.sh write-shell-env <path>|write-github-env <path>|print-shell-env

Environment:
  ADL_RUST_CACHE_TARGET_DIR       Cargo target directory to export.
  ADL_RUST_CACHE_SCCACHE_DIR      sccache directory. Defaults to $SCCACHE_DIR or $HOME/.cache/sccache.
  ADL_RUST_CACHE_SCCACHE_SIZE     sccache cache size. Defaults to $SCCACHE_CACHE_SIZE or 2G.
  ADL_RUST_CACHE_REQUIRE_SCCACHE  Require sccache to be installed. Defaults to 1.
  ADL_RUST_CACHE_REQUIRE_LLD      Require ld.lld when lld is enabled. Defaults to 0.
  ADL_RUST_CACHE_USE_LLD          1, 0, or auto. Defaults to auto.
USAGE
}

shell_quote() {
  printf '%q' "$1"
}

require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "rust_cache_env: required command is unavailable: $cmd" >&2
    exit 1
  fi
}

target_dir="${ADL_RUST_CACHE_TARGET_DIR:-${CARGO_TARGET_DIR:-}}"
sccache_dir="${ADL_RUST_CACHE_SCCACHE_DIR:-${SCCACHE_DIR:-$HOME/.cache/sccache}}"
sccache_size="${ADL_RUST_CACHE_SCCACHE_SIZE:-${SCCACHE_CACHE_SIZE:-2G}}"
require_sccache="${ADL_RUST_CACHE_REQUIRE_SCCACHE:-1}"
require_lld="${ADL_RUST_CACHE_REQUIRE_LLD:-0}"
use_lld="${ADL_RUST_CACHE_USE_LLD:-auto}"

if [[ "$require_sccache" == "1" ]]; then
  require_cmd sccache
fi

lld_enabled=false
case "$use_lld" in
  1|true|yes)
    require_cmd ld.lld
    lld_enabled=true
    ;;
  0|false|no)
    lld_enabled=false
    ;;
  auto)
    if command -v ld.lld >/dev/null 2>&1; then
      lld_enabled=true
    elif [[ "$require_lld" == "1" ]]; then
      require_cmd ld.lld
    fi
    ;;
  *)
    echo "rust_cache_env: unsupported ADL_RUST_CACHE_USE_LLD value: $use_lld" >&2
    exit 2
    ;;
esac

mkdir -p "$sccache_dir"
if [[ -n "$target_dir" ]]; then
  mkdir -p "$target_dir"
fi

rustflags="${RUSTFLAGS:-}"
if [[ "$lld_enabled" == true ]]; then
  case " $rustflags " in
    *" -C link-arg=-fuse-ld=lld "*) ;;
    *) rustflags="${rustflags:+$rustflags }-C link-arg=-fuse-ld=lld" ;;
  esac
fi

write_shell_env() {
  local destination="$1"
  {
    if [[ -n "$target_dir" ]]; then
      printf 'export CARGO_TARGET_DIR=%s\n' "$(shell_quote "$target_dir")"
    fi
    printf 'export SCCACHE_DIR=%s\n' "$(shell_quote "$sccache_dir")"
    printf 'export SCCACHE_CACHE_SIZE=%s\n' "$(shell_quote "$sccache_size")"
    printf 'export RUSTC_WRAPPER=sccache\n'
    if [[ "$lld_enabled" == true ]]; then
      printf 'export RUSTFLAGS=%s\n' "$(shell_quote "$rustflags")"
      printf 'export RUST_LINK_ACCEL=lld\n'
    else
      printf 'export RUST_LINK_ACCEL=default\n'
    fi
  } >"$destination"
}

write_github_env() {
  local destination="$1"
  {
    if [[ -n "$target_dir" ]]; then
      printf 'CARGO_TARGET_DIR=%s\n' "$target_dir"
    fi
    printf 'SCCACHE_DIR=%s\n' "$sccache_dir"
    printf 'SCCACHE_CACHE_SIZE=%s\n' "$sccache_size"
    printf 'RUSTC_WRAPPER=sccache\n'
    if [[ "$lld_enabled" == true ]]; then
      printf 'RUSTFLAGS=%s\n' "$rustflags"
      printf 'RUST_LINK_ACCEL=lld\n'
    else
      printf 'RUST_LINK_ACCEL=default\n'
    fi
  } >>"$destination"
}

case "${1:-}" in
  write-shell-env)
    [[ -n "${2:-}" ]] || { usage; exit 2; }
    write_shell_env "$2"
    ;;
  write-github-env)
    [[ -n "${2:-}" ]] || { usage; exit 2; }
    write_github_env "$2"
    ;;
  print-shell-env)
    tmp_file="$(mktemp "${TMPDIR:-/tmp}/adl-rust-cache-env.XXXXXX")"
    trap 'rm -f "$tmp_file"' EXIT
    write_shell_env "$tmp_file"
    cat "$tmp_file"
    ;;
  *)
    usage
    exit 2
    ;;
esac
