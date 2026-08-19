#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

tracked_adl="$(git ls-files -- .adl)"
if [[ -n "$tracked_adl" ]]; then
  echo "tracked .adl paths are forbidden:" >&2
  echo "$tracked_adl" >&2
  exit 1
fi

canonical_policy="adl/config/worktree-policy.json"
if [[ ! -f "$canonical_policy" ]]; then
  echo "missing canonical worktree policy: $canonical_policy" >&2
  exit 1
fi

legacy_policy_refs=()
while IFS= read -r path; do
  case "$path" in
    adl/tools/check_no_tracked_adl.sh|adl/tools/test_check_no_tracked_adl.sh) continue ;;
  esac
  match="$(perl -0777 -ne \
    'print "$ARGV\n" if /\.adl.{0,160}worktree[-_ .]*policy/is || /worktree[-_ .]*policy.{0,160}\.adl/is' \
    "$path")"
  [[ -z "$match" ]] || legacy_policy_refs+=("$match")
done < <(git grep -l -F '.adl' -- AGENTS.md csdlc-v2 adl/src adl/tools .github 2>/dev/null || true)
if [[ "${#legacy_policy_refs[@]}" -ne 0 ]]; then
  echo "active reconstruction of legacy .adl worktree-policy authority is forbidden:" >&2
  printf '%s\n' "${legacy_policy_refs[@]}" >&2
  exit 1
fi

echo "PASS: .adl is untracked and worktree-policy authority is canonical"
