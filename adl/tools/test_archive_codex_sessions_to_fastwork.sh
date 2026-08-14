#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
fixture="$(mktemp -d /Volumes/FastWork/adl-archive-test.XXXXXX)"
trap 'rm -rf -- "$fixture"' EXIT

mkdir -p "$fixture/source/sessions/2026/01/01" "$fixture/source/archived_sessions"
printf 'session-one\n' > "$fixture/source/sessions/2026/01/01/one.jsonl"
printf 'session-two\n' > "$fixture/source/archived_sessions/two.jsonl"
touch -t 202001010000 "$fixture/source/sessions/2026/01/01/one.jsonl"
touch -t 202001010000 "$fixture/source/archived_sessions/two.jsonl"

"$repo_root/adl/tools/archive_codex_sessions_to_fastwork.sh" \
  --source "$fixture/source" \
  --destination "$fixture/archive" \
  --min-age-days 0 >/dev/null

"$repo_root/adl/tools/archive_codex_sessions_to_fastwork.sh" \
  --verify-only "$fixture/archive/manifest.sha256"

test "$(jq -r '.files' "$fixture/archive/summary.json")" = "2"
test "$(jq -r '.source_deleted' "$fixture/archive/summary.json")" = "false"
cmp "$fixture/archive/source.sha256" "$fixture/archive/manifest.sha256"
test "$(stat -f '%Lp' "$fixture/archive")" = "700"
test "$(stat -f '%Lp' "$fixture/archive/data")" = "700"
test "$(stat -f '%Lp' "$fixture/archive/data/sessions/2026/01/01/one.jsonl")" = "600"
test "$(stat -f '%Lp' "$fixture/archive/manifest.sha256")" = "600"

ln -s /Users/daniel "$fixture/escaped-parent"
if "$repo_root/adl/tools/archive_codex_sessions_to_fastwork.sh" \
  --source "$fixture/source" \
  --destination "$fixture/escaped-parent/archive-must-not-exist" \
  --min-age-days 0 >/dev/null 2>&1; then
  echo "expected symlink-escaped destination refusal" >&2
  exit 1
fi
test ! -e /Users/daniel/archive-must-not-exist

mkdir "$fixture/preexisting"
ln -s /Users/daniel "$fixture/preexisting/data"
if "$repo_root/adl/tools/archive_codex_sessions_to_fastwork.sh" \
  --source "$fixture/source" \
  --destination "$fixture/preexisting" \
  --min-age-days 0 >/dev/null 2>&1; then
  echo "expected pre-existing destination refusal" >&2
  exit 1
fi

if "$repo_root/adl/tools/archive_codex_sessions_to_fastwork.sh" \
  --source "$fixture/source" \
  --destination "/Users/daniel/adl-archive-policy-must-refuse" \
  --min-age-days 0 >/dev/null 2>&1; then
  echo "expected non-FastWork destination refusal" >&2
  exit 1
fi

echo "PASS archive_codex_sessions_to_fastwork"
