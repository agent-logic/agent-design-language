#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

design=.csdlc/prepared/issues/592/design.md
stp=.csdlc/issues/592/cards/stp.values.json
test -s "$design"
test -s "$stp"
grep -q 'must not silently fall back' "$design"
grep -q 'production provider-selection path' "$design"
grep -q 'Mock provider acceptance evidence' "$stp"

if rg -n --hidden '(private_key|access_token)[[:space:]]*[:=][[:space:]]*[^$<{[:space:]]+' \
  .csdlc/prepared/issues/592 .csdlc/issues/592; then
  echo 'possible credential material found in issue 592 records' >&2
  exit 1
fi

echo 'issue 592 Runtime Vertex AI execution contract: pass'
