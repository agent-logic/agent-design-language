#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
fresh="$root/.adl/tmp/5613-fresh-worktree"
doctor="${ADL_CSDLC_DOCTOR:-}"
if [[ -z "$doctor" ]]; then
  primary="$(git -C "$root" worktree list --porcelain | awk '/^worktree / {print substr($0, 10); exit}')"
  doctor="$primary/.adl/bin/csdlc-v2/csdlc-doctor"
fi
test -x "$doctor"

cleanup() {
  git -C "$root" worktree remove --force "$fresh" >/dev/null 2>&1 || true
}
trap cleanup EXIT
cleanup
mkdir -p "$(dirname "$fresh")"
git -C "$root" worktree add --detach "$fresh" HEAD >/dev/null

for issue in 5337 5339 5591; do
  jq -e '.phase == "closed_out" and .claim == null' \
    "$fresh/.csdlc/issues/$issue/index.json" >/dev/null
  "$doctor" --root "$fresh" --issue "$issue" \
    | jq -e '.status == "pass" and .phase == "closed_out" and (.findings | length == 0)' >/dev/null
done

"$doctor" --root "$fresh" --issue 5613 \
  | jq -e '.status == "pass"' >/dev/null

printf 'issue 5613 fresh-worktree terminal proof: pass\n'
