#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
checker="$repo_root/adl/tools/check_no_tracked_adl.sh"

bash "$checker"

fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

git -C "$fixture_root" init -q
git -C "$fixture_root" config user.email "issue-432@example.invalid"
git -C "$fixture_root" config user.name "Issue 432 Validator"
mkdir -p "$fixture_root/.adl" "$fixture_root/adl/config" "$fixture_root/adl/tools"
cp "$checker" "$fixture_root/adl/tools/check_no_tracked_adl.sh"
cp "$repo_root/adl/config/worktree-policy.json" "$fixture_root/adl/config/worktree-policy.json"
printf 'tracked\n' > "$fixture_root/.adl/forbidden.txt"
git -C "$fixture_root" add -f .adl/forbidden.txt adl/config/worktree-policy.json adl/tools/check_no_tracked_adl.sh
if bash -c "cd '$fixture_root' && bash adl/tools/check_no_tracked_adl.sh" >/dev/null 2>&1; then
  echo "checker accepted a tracked .adl path" >&2
  exit 1
fi

git -C "$fixture_root" rm --cached -q .adl/forbidden.txt
mkdir -p "$fixture_root/csdlc-v2/src"
printf '%s\n' 'let path = Path::new(".adl").join("worktree-policy.json");' > "$fixture_root/csdlc-v2/src/legacy.rs"
git -C "$fixture_root" add csdlc-v2/src/legacy.rs
if bash -c "cd '$fixture_root' && bash adl/tools/check_no_tracked_adl.sh" >/dev/null 2>&1; then
  echo "checker accepted reconstructed legacy policy authority" >&2
  exit 1
fi

echo "PASS: repository-boundary guard positive and negative cases"
