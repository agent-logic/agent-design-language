#!/usr/bin/env bash
# PVF: deterministic isolated tooling contract; no native admission or man rendering.
# The fake executable/manifest below prove lookup behavior only, never activation.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
mkdir -p "$ROOT/target"
FIXTURE="$(mktemp -d "$ROOT/target/operator-man-lookup.XXXXXX")"
trap 'rm -rf "$FIXTURE"' EXIT
REPO="$FIXTURE/repo"
PACKAGE="$FIXTURE/package with spaces/installed"
PREFIX="$FIXTURE/selected bin"
MAN_ROOT="$PACKAGE/resources/docs/csdlc-v3/man"
mkdir -p "$REPO/adl/tools" "$REPO/docs/csdlc-v3/man/man1" "$PACKAGE/bin" "$MAN_ROOT/man1" "$PREFIX"
cp "$ROOT/adl/tools/install_owner_binaries.sh" "$ROOT/adl/tools/install_csdlc_man_pages.sh" "$REPO/adl/tools/"
printf '#!/bin/sh\nprintf "lookup fixture only; no admission\\n"\n' > "$PACKAGE/bin/csdlc"
chmod 0755 "$PACKAGE/bin/csdlc"
printf '{"fixture_only":true,"grants_authority":false}\n' > "$PACKAGE/manifest.json"
for name in csdlc csdlc-workflow csdlc-requests; do
  printf '.TH %s 1\n.SH NAME\nowner package lookup fixture\n' "$name" > "$MAN_ROOT/man1/$name.1"
  printf '.TH %s 1\n.SH NAME\nunrelated ADL source fixture\n' "$name" > "$REPO/docs/csdlc-v3/man/man1/$name.1"
done
ln -s "$PACKAGE/bin/csdlc" "$PREFIX/csdlc"
cp "$PACKAGE/bin/csdlc" "$FIXTURE/original-binary"
cp "$MAN_ROOT/man1/csdlc.1" "$FIXTURE/original-page"
BEFORE_INODE="$(ls -i "$PACKAGE/bin/csdlc")"
HELPER="$REPO/adl/tools/install_csdlc_man_pages.sh"
INSTALLER="$REPO/adl/tools/install_owner_binaries.sh"
refuses() {
  if "$@" > "$FIXTURE/refused.log" 2>&1; then
    echo 'lookup unexpectedly accepted invalid input' >&2
    exit 1
  fi
}
# Selected executable aliases are allowed; resource aliases are not.
bash "$HELPER" --stable-bin-dir "$PREFIX" > "$FIXTURE/lookup.txt"
grep -Fq "csdlc package manual directory: $MAN_ROOT" "$FIXTURE/lookup.txt"
grep -Fq 'neither authenticates nor activates' "$FIXTURE/lookup.txt"
bash "$HELPER" --stable-bin-dir "$PACKAGE/bin" > "$FIXTURE/direct.txt"
cmp "$FIXTURE/lookup.txt" "$FIXTURE/direct.txt"
# No private source, Cargo manifest, or source hash is needed on the no-op path.
bash "$INSTALLER" --bin csdlc --no-build --stable-bin-dir "$PREFIX" > "$FIXTURE/noop.txt"
cmp "$FIXTURE/lookup.txt" "$FIXTURE/noop.txt"
ADL_OWNER_BIN_DIR="$PREFIX" bash "$INSTALLER" --bin csdlc > "$FIXTURE/env.txt"
cmp "$FIXTURE/lookup.txt" "$FIXTURE/env.txt"
printf '\nsource-only change must never update package pages\n' >> "$REPO/docs/csdlc-v3/man/man1/csdlc.1"
bash "$INSTALLER" --bin csdlc --stable-bin-dir "$PREFIX" >/dev/null
cmp "$FIXTURE/original-page" "$MAN_ROOT/man1/csdlc.1"
cmp "$FIXTURE/original-binary" "$PACKAGE/bin/csdlc"
[[ "$(ls -i "$PACKAGE/bin/csdlc")" == "$BEFORE_INODE" ]]
[[ ! -e "$PREFIX/share" && ! -e "$PREFIX/.provenance" && ! -e "$REPO/.adl" ]]
refuses bash "$INSTALLER" --bin csdlc --source-bin-dir "$PACKAGE/bin" --stable-bin-dir "$PREFIX"
refuses bash "$INSTALLER" --bin csdlc --bin adl --stable-bin-dir "$PREFIX"
refuses bash "$HELPER"
refuses bash "$HELPER" --stable-bin-dir "$FIXTURE/missing"
# Missing and aliased resources fail closed, without changing the fixture package.
for relative in manifest.json resources/docs/csdlc-v3/man/man1/csdlc.1 resources/docs/csdlc-v3/man/man1/csdlc-workflow.1 resources/docs/csdlc-v3/man/man1/csdlc-requests.1; do
  mv "$PACKAGE/$relative" "$FIXTURE/saved-resource"
  refuses bash "$HELPER" --stable-bin-dir "$PREFIX"
  ln -s "$FIXTURE/saved-resource" "$PACKAGE/$relative"
  refuses bash "$HELPER" --stable-bin-dir "$PREFIX"
  rm "$PACKAGE/$relative"
  mv "$FIXTURE/saved-resource" "$PACKAGE/$relative"
done
mv "$MAN_ROOT/man1" "$FIXTURE/saved-man1"
ln -s "$FIXTURE/saved-man1" "$MAN_ROOT/man1"
refuses bash "$HELPER" --stable-bin-dir "$PREFIX"
rm "$MAN_ROOT/man1"
mv "$FIXTURE/saved-man1" "$MAN_ROOT/man1"
chmod 0644 "$PACKAGE/bin/csdlc"
refuses bash "$HELPER" --stable-bin-dir "$PREFIX"
chmod 0755 "$PACKAGE/bin/csdlc"
mkdir "$FIXTURE/source-bin"
cp "$PACKAGE/bin/csdlc" "$FIXTURE/source-bin/csdlc"
refuses bash "$HELPER" --stable-bin-dir "$FIXTURE/source-bin"
cmp "$FIXTURE/original-page" "$MAN_ROOT/man1/csdlc.1"
printf 'read-only package manual lookup, no-op preservation and refusal fixtures passed; no native admission or real-man proof\n'
