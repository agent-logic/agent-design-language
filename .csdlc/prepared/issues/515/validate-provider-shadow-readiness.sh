#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$repo_root"

required=(
  ".csdlc/prepared/issues/515/design.md"
  ".csdlc/prepared/issues/515/diagram.mmd"
  ".csdlc/issues/515/cards/stp.md"
  ".csdlc/issues/515/cards/spp.md"
  ".csdlc/issues/515/cards/vpp.md"
)

for path in "${required[@]}"; do
  test -f "$path"
done

grep -F "Shadow output cannot acquire authority" .csdlc/issues/515/cards/spp.md >/dev/null
grep -F "Fail closed" .csdlc/issues/515/cards/vpp.md >/dev/null
grep -F "Do not write to /private/tmp" .csdlc/issues/515/cards/sip.md >/dev/null
