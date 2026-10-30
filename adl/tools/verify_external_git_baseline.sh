#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  printf 'usage: %s <external-source-baseline.json>\n' "$0" >&2
  exit 64
fi

manifest=$1
for command in git jq mktemp sort diff; do
  command -v "$command" >/dev/null 2>&1 || {
    printf 'missing required command: %s\n' "$command" >&2
    exit 69
  }
done

jq -e '
  .schema == "adl.external_source_baseline.v1" and
  (.repository | type == "string" and length > 0) and
  (.default_branch | type == "string" and length > 0) and
  (.revision | test("^[0-9a-f]{40}$")) and
  (.objects | length > 0) and
  all(.objects[];
    .kind == "blob" and
    (.path | type == "string" and length > 0) and
    (.oid | test("^[0-9a-f]{40}$")))
' "$manifest" >/dev/null

repository=$(jq -r '.repository' "$manifest")
default_branch=$(jq -r '.default_branch' "$manifest")
revision=$(jq -r '.revision' "$manifest")

remote_head=$(git ls-remote --symref "$repository" HEAD | awk '$1 == "ref:" {sub("refs/heads/", "", $2); print $2; exit}')
[[ "$remote_head" == "$default_branch" ]] || {
  printf 'default branch mismatch: expected %s, observed %s\n' "$default_branch" "$remote_head" >&2
  exit 1
}

cache_root=${XDG_CACHE_HOME:-"$HOME/.cache"}/adl-external-source-baseline
mkdir -p "$cache_root"
scratch=$(mktemp -d "$cache_root/verify.XXXXXX")
trap 'rm -rf "$scratch"' EXIT

git -C "$scratch" init --bare --quiet
git -C "$scratch" remote add origin "$repository"
git -C "$scratch" config remote.origin.promisor true
git -C "$scratch" config remote.origin.partialclonefilter blob:none
git -C "$scratch" fetch --quiet --depth=1 --filter=blob:none origin "$revision"

observed_revision=$(git -C "$scratch" rev-parse 'FETCH_HEAD^{commit}')
[[ "$observed_revision" == "$revision" ]] || {
  printf 'revision mismatch: expected %s, observed %s\n' "$revision" "$observed_revision" >&2
  exit 1
}

expected="$scratch/expected.tsv"
observed="$scratch/observed.tsv"
jq -r '.objects[] | [.kind, .oid, .path] | @tsv' "$manifest" | sort >"$expected"

: >"$observed"
while IFS= read -r path; do
  git -C "$scratch" ls-tree FETCH_HEAD -- "$path" |
    awk '{kind=$2; oid=$3; sub(/^[^\t]*\t/, ""); print kind "\t" oid "\t" $0}' >>"$observed"
done < <(jq -r '.objects[].path' "$manifest")
sort -o "$observed" "$observed"

diff -u "$expected" "$observed"
while IFS= read -r oid; do
  git -C "$scratch" cat-file -e "${oid}^{blob}"
done < <(jq -r '.objects[].oid' "$manifest")

printf 'verified %s objects at %s (%s)\n' "$(jq '.objects | length' "$manifest")" "$revision" "$default_branch"
