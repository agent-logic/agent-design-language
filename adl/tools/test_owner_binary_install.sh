#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INSTALL_SRC="$ROOT_DIR/adl/tools/install_owner_binaries.sh"
RESOLUTION_SRC="$ROOT_DIR/adl/tools/owner_binary_resolution.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

mtime_seconds() {
  if stat -f %m "$1" >/dev/null 2>&1; then
    stat -f %m "$1"
    return 0
  fi
  stat -c %Y "$1"
}

repo="$tmpdir/repo"
source_bin_dir="$tmpdir/source-bins"
mkdir -p "$repo/adl/tools" "$repo/adl/src" "$source_bin_dir"
cp "$INSTALL_SRC" "$repo/adl/tools/install_owner_binaries.sh"
cp "$RESOLUTION_SRC" "$repo/adl/tools/owner_binary_resolution.sh"
chmod +x "$repo/adl/tools/install_owner_binaries.sh"
cat >"$repo/adl/Cargo.toml" <<'EOF_CARGO'
[package]
name = "adl"
version = "0.0.0"
edition = "2021"
EOF_CARGO
printf 'pub fn seed() {}\n' >"$repo/adl/src/lib.rs"
cat >"$source_bin_dir/adl-pr-closeout" <<'EOF_BIN'
#!/usr/bin/env bash
printf 'closeout-v1:%s\n' "$*"
EOF_BIN
chmod +x "$source_bin_dir/adl-pr-closeout"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add adl/Cargo.toml adl/src/lib.rs adl/tools/install_owner_binaries.sh adl/tools/owner_binary_resolution.sh
  git commit -q -m "init"
)

stable_bin="$repo/.adl/bin/adl-pr-closeout"
provenance="$repo/.adl/bin/.provenance/adl-pr-closeout.sha256"

(
  cd "$repo"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --bin adl-pr-closeout \
    --source-bin-dir "$source_bin_dir" \
    --no-build >/dev/null
)
[[ -x "$stable_bin" ]] || {
  echo "assertion failed: stable owner binary was not installed outside target" >&2
  exit 1
}
[[ -f "$provenance" ]] || {
  echo "assertion failed: stable owner binary provenance was not recorded" >&2
  exit 1
}
[[ "$stable_bin" != *"/target/"* ]] || {
  echo "assertion failed: stable owner binary must not live under target" >&2
  exit 1
}

mtime_before="$(mtime_seconds "$stable_bin")"
sleep 1
(
  cd "$repo"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --bin adl-pr-closeout \
    --source-bin-dir "$source_bin_dir" \
    --no-build >/dev/null
)
mtime_after_noop="$(mtime_seconds "$stable_bin")"
[[ "$mtime_before" == "$mtime_after_noop" ]] || {
  echo "assertion failed: no-op reinstall replaced an unchanged stable binary" >&2
  exit 1
}

resolved="$(
  cd "$repo"
  # shellcheck source=/dev/null
  source adl/tools/owner_binary_resolution.sh
  root="$(adl_owner_manifest_root)"
  primary="$(adl_owner_primary_root "$root")"
  adl_owner_stable_binary_if_fresh adl-pr-closeout "$root" "$primary"
)"
[[ "$resolved" == "$stable_bin" ]] || {
  echo "assertion failed: resolver did not select fresh stable owner binary" >&2
  echo "resolved=$resolved" >&2
  exit 1
}

printf 'pub fn seed() { let _ = 1; }\n' >"$repo/adl/src/lib.rs"
set +e
stale_resolved="$(
  cd "$repo"
  # shellcheck source=/dev/null
  source adl/tools/owner_binary_resolution.sh
  root="$(adl_owner_manifest_root)"
  primary="$(adl_owner_primary_root "$root")"
  adl_owner_stable_binary_if_fresh adl-pr-closeout "$root" "$primary"
)"
stale_status=$?
set -e
[[ "$stale_status" -ne 0 && -z "$stale_resolved" ]] || {
  echo "assertion failed: resolver accepted stale stable owner binary after source changed" >&2
  exit 1
}

cat >"$source_bin_dir/adl-pr-closeout" <<'EOF_BIN'
#!/usr/bin/env bash
printf 'closeout-v2:%s\n' "$*"
EOF_BIN
chmod +x "$source_bin_dir/adl-pr-closeout"
sleep 1
(
  cd "$repo"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --bin adl-pr-closeout \
    --source-bin-dir "$source_bin_dir" \
    --no-build >/dev/null
)
mtime_after_update="$(mtime_seconds "$stable_bin")"
[[ "$mtime_after_update" -gt "$mtime_after_noop" ]] || {
  echo "assertion failed: changed source did not intentionally replace stable binary" >&2
  exit 1
}
grep -Fq 'closeout-v2' "$stable_bin" || {
  echo "assertion failed: stable binary content was not updated after source change" >&2
  exit 1
}

printf 'pub fn untracked_owner_input() {}\n' >"$repo/adl/src/untracked_owner_input.rs"
set +e
untracked_resolved="$(
  cd "$repo"
  # shellcheck source=/dev/null
  source adl/tools/owner_binary_resolution.sh
  root="$(adl_owner_manifest_root)"
  primary="$(adl_owner_primary_root "$root")"
  adl_owner_stable_binary_if_fresh adl-pr-closeout "$root" "$primary"
)"
untracked_status=$?
set -e
[[ "$untracked_status" -ne 0 && -z "$untracked_resolved" ]] || {
  echo "assertion failed: resolver accepted stale stable owner binary after untracked source was added" >&2
  exit 1
}
rm -f "$repo/adl/src/untracked_owner_input.rs"

nogit="$tmpdir/nogit"
mkdir -p "$nogit/adl/tools" "$nogit/adl/src" "$tmpdir/nogit-source-bins"
cp "$INSTALL_SRC" "$nogit/adl/tools/install_owner_binaries.sh"
cp "$RESOLUTION_SRC" "$nogit/adl/tools/owner_binary_resolution.sh"
chmod +x "$nogit/adl/tools/install_owner_binaries.sh"
cp "$repo/adl/Cargo.toml" "$nogit/adl/Cargo.toml"
printf 'pub fn nongit_seed() {}\n' >"$nogit/adl/src/lib.rs"
cat >"$tmpdir/nogit-source-bins/adl-pr-closeout" <<'EOF_BIN'
#!/usr/bin/env bash
printf 'closeout-nongit:%s\n' "$*"
EOF_BIN
chmod +x "$tmpdir/nogit-source-bins/adl-pr-closeout"
(
  cd "$nogit"
  "$BASH_BIN" adl/tools/install_owner_binaries.sh \
    --bin adl-pr-closeout \
    --source-bin-dir "$tmpdir/nogit-source-bins" \
    --no-build >/dev/null
)
"$nogit/.adl/bin/adl-pr-closeout" | grep -Fq 'closeout-nongit:' || {
  echo "assertion failed: non-git stable owner binary install did not produce runnable binary" >&2
  exit 1
}

echo "owner binary stable install: ok"
