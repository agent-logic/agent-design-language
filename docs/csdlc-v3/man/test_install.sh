#!/usr/bin/env bash
# PVF: isolated tooling proof, real host man lookup; fixture binary, no activation.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
mkdir -p "$ROOT/target"
FIXTURE="$(mktemp -d "$ROOT/target/operator-man-install.XXXXXX")"
trap 'rm -rf "$FIXTURE"' EXIT
REPO="$FIXTURE/repo"
mkdir -p "$REPO/adl/tools" "$REPO/csdlc-v3" "$REPO/docs/csdlc-v3" "$FIXTURE/source"
cp "$ROOT/adl/tools/install_owner_binaries.sh" "$ROOT/adl/tools/install_csdlc_man_pages.sh" "$REPO/adl/tools/"
cp "$ROOT/csdlc-v3/Cargo.toml" "$ROOT/csdlc-v3/Cargo.lock" "$REPO/csdlc-v3/"
cp -R "$ROOT/csdlc-v3/src" "$REPO/csdlc-v3/"
cp -R "$ROOT/docs/csdlc-v3/man" "$REPO/docs/csdlc-v3/"
git -C "$REPO" init -q
printf '#!/bin/sh\nprintf "fixture owner only\\n"\n' > "$FIXTURE/source/csdlc"
chmod 0755 "$FIXTURE/source/csdlc"
PREFIX="$FIXTURE/prefix with spaces"
bash "$REPO/adl/tools/install_owner_binaries.sh" --bin csdlc --no-build --source-bin-dir "$FIXTURE/source" --stable-bin-dir "$PREFIX"
cmp "$FIXTURE/source/csdlc" "$PREFIX/csdlc"
BEFORE="$(ls -i "$PREFIX/csdlc")"
for page in "$REPO/docs/csdlc-v3/man/man1"/*.1; do
  name="$(basename "$page" .1)"
  found="$(MANPATH="$PREFIX/share/man:" man -w "$name")"
  [[ "$found" == *"$PREFIX/share/man/man1/$name.1"* ]] || { echo "lookup failed: $name" >&2; exit 1; }
  MANPATH="$PREFIX/share/man:" MANPAGER=cat PAGER=cat man "$name" > "$FIXTURE/rendered.txt"
  [[ -s "$FIXTURE/rendered.txt" ]] || { echo "render failed: $name" >&2; exit 1; }
done
# Docs refresh must run through the owner no-op path without replacing its binary.
printf '\n.\\" documentation refresh fixture\n' >> "$REPO/docs/csdlc-v3/man/man1/csdlc.1"
bash "$REPO/adl/tools/install_owner_binaries.sh" --bin csdlc --no-build --source-bin-dir "$FIXTURE/source" --stable-bin-dir "$PREFIX"
[[ "$(ls -i "$PREFIX/csdlc")" == "$BEFORE" ]]
cmp "$REPO/docs/csdlc-v3/man/man1/csdlc.1" "$PREFIX/share/man/man1/csdlc.1"
# Missing pages do not silently report a successful manual installation.
mv "$REPO/docs/csdlc-v3/man/man1/csdlc.1" "$FIXTURE/saved.1"
if bash "$REPO/adl/tools/install_owner_binaries.sh" --bin csdlc --no-build --stable-bin-dir "$PREFIX" > "$FIXTURE/missing.log" 2>&1; then
  echo 'missing overview unexpectedly accepted' >&2
  exit 1
fi
printf 'operator manual clean install, all-page lookup/render, docs refresh and missing-source denial passed on %s\n' "$(uname -s)"
