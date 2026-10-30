#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/rust_cache_env.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

fake_bin="$TMP/bin"
mkdir -p "$fake_bin"
cat >"$fake_bin/sccache" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  --version|--start-server|--zero-stats|--show-stats)
    exit 0
    ;;
  *)
    exit 0
    ;;
esac
EOF
cat >"$fake_bin/ld.lld" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
echo "LLD fixture"
EOF
chmod +x "$fake_bin/"*

shell_env="$TMP/cache-env.sh"
PATH="$fake_bin:$PATH" \
ADL_RUST_CACHE_TARGET_DIR="$TMP/target dir" \
ADL_RUST_CACHE_SCCACHE_DIR="$TMP/sccache dir" \
ADL_RUST_CACHE_SCCACHE_SIZE=9G \
ADL_RUST_CACHE_USE_LLD=1 \
RUSTFLAGS="-C debuginfo=1" \
  bash "$SCRIPT" write-shell-env "$shell_env"

grep -F "export RUSTC_WRAPPER=sccache" "$shell_env" >/dev/null
grep -F "export RUST_LINK_ACCEL=lld" "$shell_env" >/dev/null

# shellcheck disable=SC1090
. "$shell_env"
[[ "$CARGO_TARGET_DIR" == "$TMP/target dir" ]]
[[ "$SCCACHE_DIR" == "$TMP/sccache dir" ]]
[[ "$SCCACHE_CACHE_SIZE" == "9G" ]]
[[ "$RUSTC_WRAPPER" == "sccache" ]]
[[ "$RUST_LINK_ACCEL" == "lld" ]]
[[ "$RUSTFLAGS" == "-C debuginfo=1 -C link-arg=-fuse-ld=lld" ]]

github_env="$TMP/github-env"
PATH="$fake_bin:$PATH" \
ADL_RUST_CACHE_TARGET_DIR="$TMP/github-target" \
ADL_RUST_CACHE_SCCACHE_DIR="$TMP/github-sccache" \
ADL_RUST_CACHE_USE_LLD=auto \
  bash "$SCRIPT" write-github-env "$github_env"

grep -Fx "CARGO_TARGET_DIR=$TMP/github-target" "$github_env" >/dev/null
grep -Fx "SCCACHE_DIR=$TMP/github-sccache" "$github_env" >/dev/null
grep -Fx "RUSTC_WRAPPER=sccache" "$github_env" >/dev/null
grep -Fx "RUST_LINK_ACCEL=lld" "$github_env" >/dev/null

no_lld_bin="$TMP/no-lld-bin"
mkdir -p "$no_lld_bin"
cp "$fake_bin/sccache" "$no_lld_bin/sccache"
if PATH="$no_lld_bin:/bin:/usr/bin" ADL_RUST_CACHE_USE_LLD=definitely-not-valid bash "$SCRIPT" print-shell-env >/dev/null 2>"$TMP/no-lld.err"; then
  echo "expected required lld mode to fail when ld.lld is unavailable" >&2
  exit 1
fi
grep -F "unsupported ADL_RUST_CACHE_USE_LLD value" "$TMP/no-lld.err" >/dev/null

echo "PASS test_rust_cache_env"
