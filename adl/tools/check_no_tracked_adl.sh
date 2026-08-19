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

legacy_policy_path=".adl/""worktree-policy.json"
legacy_policy_refs="$(git grep -n -F "$legacy_policy_path" -- ':!.csdlc/**' || true)"
if [[ -n "$legacy_policy_refs" ]]; then
  echo "active references to the legacy worktree-policy path are forbidden:" >&2
  echo "$legacy_policy_refs" >&2
  exit 1
fi

echo "PASS: .adl is untracked and worktree-policy authority is canonical"
