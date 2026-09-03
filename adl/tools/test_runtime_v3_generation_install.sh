#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT_DIR/.csdlc/evidence/656/generation-test"
rm -rf "$FIXTURE"
mkdir -p "$FIXTURE/sources" "$FIXTURE/install"
trap 'rm -rf "$FIXTURE"' EXIT

for binary in csm adl-runtime-guardian adl-runtime-kernel; do
  printf '#!/usr/bin/env bash\nexit 0\n' >"$FIXTURE/sources/$binary"
  chmod +x "$FIXTURE/sources/$binary"
done

install_generation() {
  "$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" install \
    --root "$FIXTURE/install" \
    --generation "$1" \
    --csm "$FIXTURE/sources/csm" \
    --guardian "$FIXTURE/sources/adl-runtime-guardian" \
    --kernel "$FIXTURE/sources/adl-runtime-kernel" \
    --source-revision "$2" \
    --build-profile release >/dev/null
}

install_generation generation-a revision-a
test "$(readlink "$FIXTURE/install/current")" = generations/generation-a
"$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" verify --root "$FIXTURE/install" >/dev/null

install_generation generation-b revision-b
test "$(readlink "$FIXTURE/install/current")" = generations/generation-b
test "$(readlink "$FIXTURE/install/previous")" = generations/generation-a

printf 'tampered\n' >>"$FIXTURE/install/generations/generation-b/bin/adl-runtime-kernel"
if "$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" verify --root "$FIXTURE/install" >/dev/null 2>&1; then
  echo "tampered mixed generation was accepted" >&2
  exit 1
fi
test "$(readlink "$FIXTURE/install/current")" = generations/generation-b

cp "$FIXTURE/sources/adl-runtime-kernel" "$FIXTURE/install/generations/generation-b/bin/adl-runtime-kernel"
"$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" rollback --root "$FIXTURE/install" >/dev/null
test "$(readlink "$FIXTURE/install/current")" = generations/generation-a
test "$(readlink "$FIXTURE/install/previous")" = generations/generation-b

echo "runtime v3 generation installer: PASS"
