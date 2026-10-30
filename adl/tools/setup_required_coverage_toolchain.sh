#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

usage() {
  cat >&2 <<'USAGE'
usage: setup_required_coverage_toolchain.sh install-lld|configure <github-env>|verify|stats
USAGE
}

require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "::error::required command is unavailable: $cmd" >&2
    exit 1
  fi
}

install_lld() {
  if command -v ld.lld >/dev/null 2>&1; then
    ld.lld --version
  else
    echo "ld.lld unavailable; continuing with the default system linker" >&2
  fi
}

configure() {
  local github_env="${1:-}"
  if [ -z "$github_env" ]; then
    echo "::error::configure requires a GITHUB_ENV path" >&2
    exit 1
  fi
  require_cmd sccache
  ADL_RUST_CACHE_SCCACHE_DIR="${ADL_RUST_CACHE_SCCACHE_DIR:-$HOME/.cache/sccache}" \
  ADL_RUST_CACHE_SCCACHE_SIZE="${ADL_RUST_CACHE_SCCACHE_SIZE:-2G}" \
  ADL_RUST_CACHE_REQUIRE_SCCACHE=1 \
  ADL_RUST_CACHE_REQUIRE_LLD=0 \
  ADL_RUST_CACHE_USE_LLD=auto \
    bash "$ROOT_DIR/adl/tools/rust_cache_env.sh" write-github-env "$github_env"
  if ! sccache --start-server 2>/tmp/adl-sccache-start.err; then
    if ! sccache --show-stats >/dev/null 2>&1; then
      cat /tmp/adl-sccache-start.err >&2
      exit 1
    fi
  fi
  rm -f /tmp/adl-sccache-start.err
  sccache --zero-stats
}

verify() {
  require_cmd rustc
  require_cmd cargo
  require_cmd sccache
  rustc -vV
  cargo --version
  cargo llvm-cov --version
  cargo nextest --version
  sccache --version
  if command -v ld.lld >/dev/null 2>&1; then
    ld.lld --version
  else
    echo "ld.lld unavailable; using default system linker"
  fi
}

stats() {
  require_cmd sccache
  echo "Linker mode: ${RUST_LINK_ACCEL:-unknown}"
  sccache --show-stats
}

case "${1:-}" in
  install-lld)
    install_lld
    ;;
  configure)
    shift
    configure "${1:-}"
    ;;
  verify)
    verify
    ;;
  stats)
    stats
    ;;
  *)
    usage
    exit 2
    ;;
esac
