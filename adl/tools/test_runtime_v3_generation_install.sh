#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT_DIR/.csdlc/evidence/678/generation-test"
INSTALL_ROOT="$FIXTURE/runtime-v3"
rm -rf "$FIXTURE"
mkdir -p "$FIXTURE/sources" "$INSTALL_ROOT"
trap 'rm -rf "$FIXTURE"' EXIT

write_sources() {
  generation="$1"
  printf '#!/usr/bin/env bash\necho "%s:$*"\n' "$generation" >"$FIXTURE/sources/csm"
  chmod +x "$FIXTURE/sources/csm"
  for binary in adl-runtime-guardian adl-runtime-kernel; do
    printf '#!/usr/bin/env bash\nexit 0\n' >"$FIXTURE/sources/$binary"
    chmod +x "$FIXTURE/sources/$binary"
  done
}

assert_stable_csm_routes_to() {
  generation="$1"
  direct="$("$INSTALL_ROOT/current/bin/csm" status --json)"
  stable="$("$FIXTURE/bin/csm" status --json)"
  test "$direct" = "$generation:status --json"
  test "$stable" = "$direct"
}

write_stale_stable_csm() {
  mkdir -p "$FIXTURE/bin"
  printf '#!/usr/bin/env bash\necho "stale-stable-csm:$*"\n' >"$FIXTURE/bin/csm"
  chmod +x "$FIXTURE/bin/csm"
}

for binary in csm adl-runtime-guardian adl-runtime-kernel; do
  printf '#!/usr/bin/env bash\nexit 0\n' >"$FIXTURE/sources/$binary"
  chmod +x "$FIXTURE/sources/$binary"
done

install_generation() {
  write_sources "$1"
  "$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" install \
    --root "$INSTALL_ROOT" \
    --generation "$1" \
    --csm "$FIXTURE/sources/csm" \
    --guardian "$FIXTURE/sources/adl-runtime-guardian" \
    --kernel "$FIXTURE/sources/adl-runtime-kernel" \
    --source-revision "$2" \
    --build-profile release >/dev/null
}

install_generation generation-a revision-a
test "$(readlink "$INSTALL_ROOT/current")" = generations/generation-a
"$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" verify --root "$INSTALL_ROOT" >/dev/null
assert_stable_csm_routes_to generation-a

write_stale_stable_csm
test "$("$FIXTURE/bin/csm" status --json)" = "stale-stable-csm:status --json"
install_generation generation-b revision-b
test "$(readlink "$INSTALL_ROOT/current")" = generations/generation-b
test "$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["predecessor_generation"])' "$INSTALL_ROOT/generations/generation-b/receipt.json")" = generation-a
assert_stable_csm_routes_to generation-b

mv "$INSTALL_ROOT/generations/generation-a" "$FIXTURE/escaped-generation-a"
ln -s "$FIXTURE/escaped-generation-a" "$INSTALL_ROOT/generations/generation-a"
if "$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" rollback --root "$INSTALL_ROOT" >/dev/null 2>&1; then
  echo "rollback accepted a predecessor symlink outside generations" >&2
  exit 1
fi
test "$(readlink "$INSTALL_ROOT/current")" = generations/generation-b
unlink "$INSTALL_ROOT/generations/generation-a"
mv "$FIXTURE/escaped-generation-a" "$INSTALL_ROOT/generations/generation-a"

printf 'tampered\n' >>"$INSTALL_ROOT/generations/generation-b/bin/adl-runtime-kernel"
if "$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" verify --root "$INSTALL_ROOT" >/dev/null 2>&1; then
  echo "tampered mixed generation was accepted" >&2
  exit 1
fi
test "$(readlink "$INSTALL_ROOT/current")" = generations/generation-b

cp "$FIXTURE/sources/adl-runtime-kernel" "$INSTALL_ROOT/generations/generation-b/bin/adl-runtime-kernel"
"$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" rollback --root "$INSTALL_ROOT" >/dev/null
test "$(readlink "$INSTALL_ROOT/current")" = generations/generation-a
assert_stable_csm_routes_to generation-a

mv "$INSTALL_ROOT/current/bin/csm" "$INSTALL_ROOT/current/bin/csm.missing"
if "$FIXTURE/bin/csm" status --json >/dev/null 2>&1; then
  echo "stable csm route accepted a missing active generation CSM" >&2
  exit 1
fi
mv "$INSTALL_ROOT/current/bin/csm.missing" "$INSTALL_ROOT/current/bin/csm"
assert_stable_csm_routes_to generation-a

echo "runtime v3 generation installer: PASS"
