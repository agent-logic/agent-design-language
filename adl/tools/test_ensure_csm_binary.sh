#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

fixture="$TMP/repo"
mkdir -p "$fixture/adl/tools" "$fixture/adl/src/bin" "$TMP/bin"
export CARGO_TARGET_DIR="$fixture/adl/target"
cp "$ROOT_DIR/adl/tools/ensure_csm_binary.sh" "$fixture/adl/tools/ensure_csm_binary.sh"
cp "$ROOT_DIR/adl/tools/owner_binary_resolution.sh" "$fixture/adl/tools/owner_binary_resolution.sh"
cp "$ROOT_DIR/adl/tools/rust_validation_warm_cache.sh" "$fixture/adl/tools/rust_validation_warm_cache.sh"
chmod +x "$fixture/adl/tools/ensure_csm_binary.sh"
cat >"$fixture/adl/Cargo.toml" <<'EOF'
[package]
name = "adl"
version = "0.0.0"
edition = "2021"
EOF
touch "$fixture/adl/Cargo.lock"
cat >"$fixture/adl/src/bin/csm.rs" <<'EOF'
fn main() {}
EOF
cat >"$TMP/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
target_dir="${CARGO_TARGET_DIR:-}"
manifest=""
profile="debug"
while [ "$#" -gt 0 ]; do
  case "$1" in
    --manifest-path) manifest="$2"; shift ;;
    --release) profile="release" ;;
    --profile) profile="$2"; shift ;;
  esac
  shift || true
done
if [ -z "$manifest" ]; then
  echo "missing manifest" >&2
  exit 2
fi
root="$(cd "$(dirname "$manifest")/.." && pwd)"
if [ -z "$target_dir" ]; then
  target_dir="$root/adl/target"
fi
mkdir -p "$target_dir/$profile"
cat >"$target_dir/$profile/csm" <<'CSM'
#!/usr/bin/env bash
echo csm-fixture
CSM
chmod +x "$target_dir/$profile/csm"
EOF
chmod +x "$TMP/bin/cargo"

missing_json="$TMP/missing.json"
if ADL_CSM_SKIP_WARM_CACHE=1 PATH="$TMP/bin:$PATH" \
    bash "$fixture/adl/tools/ensure_csm_binary.sh" --json --check-only >"$missing_json"; then
  echo "expected check-only missing CSM binary to fail" >&2
  exit 1
fi
python3 - "$missing_json" <<'PY'
import json, sys
payload = json.loads(open(sys.argv[1]).read())
assert payload["schema"] == "adl.csm.binary_availability.v1"
assert payload["status"] == "missing"
assert payload["runtime_owner"] == "csm"
assert payload["action"] == "check_only"
assert payload["source_presence"] is True
PY

restore_json="$TMP/restore.json"
ADL_CSM_SKIP_WARM_CACHE=1 PATH="$TMP/bin:$PATH" \
  bash "$fixture/adl/tools/ensure_csm_binary.sh" --json >"$restore_json"
python3 - "$restore_json" "$fixture/adl/target/debug/csm" <<'PY'
import json, pathlib, sys
payload = json.loads(open(sys.argv[1]).read())
expected = pathlib.Path(sys.argv[2]).resolve()
assert payload["status"] == "restored"
assert payload["action"] == "rebuilt"
assert payload["provenance"] == "cargo_build"
assert pathlib.Path(payload["binary"]).resolve() == expected
assert expected.exists()
PY

out_stdout_json="$TMP/out-stdout.json"
out_evidence_json="$TMP/out-evidence.json"
rm -f "$fixture/adl/target/debug/csm"
ADL_CSM_SKIP_WARM_CACHE=1 PATH="$TMP/bin:$PATH" \
  bash "$fixture/adl/tools/ensure_csm_binary.sh" --json --out "$out_evidence_json" >"$out_stdout_json"
python3 - "$out_stdout_json" "$out_evidence_json" "$fixture" <<'PY'
import json, pathlib, sys
stdout_payload = json.loads(open(sys.argv[1]).read())
evidence_payload = json.loads(open(sys.argv[2]).read())
fixture = pathlib.Path(sys.argv[3]).resolve()
assert pathlib.Path(stdout_payload["binary"]).is_absolute()


def walk(value):
    if isinstance(value, dict):
        for item in value.values():
            yield from walk(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk(item)
    else:
        yield value


assert evidence_payload["binary"].startswith(("<repo>/", "<primary-repo>/"))
for value in walk(evidence_payload):
    if isinstance(value, str):
        assert not value.startswith(str(fixture)), value
PY

available_json="$TMP/available.json"
ADL_CSM_SKIP_WARM_CACHE=1 PATH="$TMP/bin:$PATH" \
  bash "$fixture/adl/tools/ensure_csm_binary.sh" --json --check-only >"$available_json"
python3 - "$available_json" <<'PY'
import json, sys
payload = json.loads(open(sys.argv[1]).read())
assert payload["status"] == "available"
assert payload["action"] == "reused"
assert payload["provenance"] == "cargo_target_dir"
PY

stable_json="$TMP/stable.json"
rm -rf "$fixture/adl/target"
mkdir -p "$fixture/.adl/bin/.provenance"
cat >"$fixture/.adl/bin/csm" <<'EOF'
#!/usr/bin/env bash
echo csm-stable-fixture
EOF
chmod +x "$fixture/.adl/bin/csm"
# shellcheck source=adl/tools/owner_binary_resolution.sh
source "$fixture/adl/tools/owner_binary_resolution.sh"
adl_owner_source_hash "$fixture" >"$fixture/.adl/bin/.provenance/csm.sha256"
ADL_CSM_SKIP_WARM_CACHE=1 PATH="$TMP/bin:$PATH" \
  bash "$fixture/adl/tools/ensure_csm_binary.sh" --json --check-only >"$stable_json"
python3 - "$stable_json" "$fixture/.adl/bin/csm" <<'PY'
import json, pathlib, sys
payload = json.loads(open(sys.argv[1]).read())
expected = pathlib.Path(sys.argv[2]).resolve()
assert payload["status"] == "available"
assert payload["action"] == "reused"
assert payload["provenance"] == "stable_owner_binary"
assert pathlib.Path(payload["binary"]).resolve() == expected
PY

nested_primary="$TMP/primary"
nested_worktree="$nested_primary/.worktrees/issue-4977"
mkdir -p "$nested_primary/adl/src/bin" "$nested_worktree/adl/tools" "$nested_worktree/adl/src/bin" "$nested_worktree/.adl/bin/.provenance"
cp "$ROOT_DIR/adl/tools/ensure_csm_binary.sh" "$nested_worktree/adl/tools/ensure_csm_binary.sh"
cp "$ROOT_DIR/adl/tools/owner_binary_resolution.sh" "$nested_worktree/adl/tools/owner_binary_resolution.sh"
cp "$ROOT_DIR/adl/tools/rust_validation_warm_cache.sh" "$nested_worktree/adl/tools/rust_validation_warm_cache.sh"
chmod +x "$nested_worktree/adl/tools/ensure_csm_binary.sh"
cat >"$nested_primary/adl/Cargo.toml" <<'EOF'
[package]
name = "adl"
version = "0.0.0"
edition = "2021"
EOF
touch "$nested_primary/adl/Cargo.lock"
cat >"$nested_primary/adl/src/bin/csm.rs" <<'EOF'
fn main() { println!("primary"); }
EOF
cp "$nested_primary/adl/Cargo.toml" "$nested_worktree/adl/Cargo.toml"
cp "$nested_primary/adl/Cargo.lock" "$nested_worktree/adl/Cargo.lock"
cat >"$nested_worktree/adl/src/bin/csm.rs" <<'EOF'
fn main() { println!("worktree"); }
EOF
cat >"$nested_worktree/.adl/bin/csm" <<'EOF'
#!/usr/bin/env bash
echo csm-worktree-stable-fixture
EOF
chmod +x "$nested_worktree/.adl/bin/csm"
adl_owner_source_hash "$nested_worktree" >"$nested_worktree/.adl/bin/.provenance/csm.sha256"
nested_stable_json="$TMP/nested-stable.json"
ADL_CSM_SKIP_WARM_CACHE=1 PATH="$TMP/bin:$PATH" \
  bash "$nested_worktree/adl/tools/ensure_csm_binary.sh" --json --check-only >"$nested_stable_json"
python3 - "$nested_stable_json" "$nested_worktree/.adl/bin/csm" <<'PY'
import json, pathlib, sys
payload = json.loads(open(sys.argv[1]).read())
expected = pathlib.Path(sys.argv[2]).resolve()
assert payload["status"] == "available"
assert payload["action"] == "reused"
assert payload["provenance"] == "stable_owner_binary"
assert pathlib.Path(payload["binary"]).resolve() == expected
PY

release_json="$TMP/release.json"
rm -rf "$fixture/adl/target" "$fixture/.adl/bin"
ADL_CSM_PROFILE=release ADL_CSM_SKIP_WARM_CACHE=1 PATH="$TMP/bin:$PATH" \
  bash "$fixture/adl/tools/ensure_csm_binary.sh" --json >"$release_json"
python3 - "$release_json" "$fixture/adl/target/release/csm" <<'PY'
import json, pathlib, sys
payload = json.loads(open(sys.argv[1]).read())
expected = pathlib.Path(sys.argv[2]).resolve()
assert payload["status"] == "restored"
assert payload["profile"] == "release"
assert pathlib.Path(payload["binary"]).resolve() == expected
assert expected.exists()
PY

echo "PASS test_ensure_csm_binary"
