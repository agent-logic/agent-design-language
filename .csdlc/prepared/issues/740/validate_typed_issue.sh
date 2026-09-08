#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
owner_validate="/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate"

if [[ ! -x "$owner_validate" ]]; then
  echo "missing stable owner validator: $owner_validate" >&2
  exit 65
fi

"$owner_validate" --root "$repo_root" issue --issue 740
