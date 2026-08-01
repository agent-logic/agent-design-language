#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
common_dir="$(git rev-parse --git-common-dir)"
if [[ "$common_dir" != /* ]]; then
  common_dir="$repo_root/$common_dir"
fi

doctor="$repo_root/.adl/bin/csdlc-v2/csdlc-doctor"
register="$repo_root/.csdlc/prepared/issues/5748/fail-closed-exceptions.md"

terminal_issues=(
  4739 4741 4758 4759 4760 4761 4762 4763 5107 5332 5336 5337 5338
  5339 5340 5341 5342 5343 5344 5345 5349 5350 5352 5354 5358 5361
  5384 5438 5470 5497 5498 5499 5500 5501 5502 5526 5527 5540 5541
  5548 5563 5566 5569 5572 5587 5589 5590 5591 5592 5594 5597 5600
  5602 5605 5610 5613 5615 5624 5627 5632 5645 5648 5653 5658 5662
  5665 5666 5670 5671 5679 5683 5686 5687 5691 5695 5697 5698 5702
  5708 5710 5711 5715 5717 5718 5719 5727 5728 5735 5737 5746
)
exception_issues=(5007 5558 5657 5663 5664 5675 5678 5701 5722 5733)

[[ ${#terminal_issues[@]} -eq 90 ]]
[[ ${#exception_issues[@]} -eq 10 ]]
[[ -f "$register" ]]

for issue in "${terminal_issues[@]}"; do
  index="$repo_root/.csdlc/issues/$issue/index.json"
  receipt="$common_dir/csdlc-v2/closeout/$issue.json"
  [[ -f "$index" ]]
  [[ -f "$receipt" ]]
  [[ "$(jq -r '.phase' "$index")" == "closed_out" ]]
  [[ "$(jq -r '.claim == null' "$index")" == "true" ]]
  "$doctor" --repo "$repo_root" --issue "$issue" >/dev/null
  jq -e --slurpfile receipt "$receipt" '. == $receipt[0].record' "$index" >/dev/null
  for card in sip stp spp vpp srp sor; do
    jq -e --arg card "$card" --slurpfile receipt "$receipt" \
      '. == $receipt[0].cards[$card]' \
      "$repo_root/.csdlc/issues/$issue/cards/$card.values.json" >/dev/null
  done
done

for issue in "${exception_issues[@]}"; do
  [[ ! -f "$common_dir/csdlc-v2/closeout/$issue.json" ]]
  rg -q "^## #$issue —" "$register"
done

[[ ! -f "$common_dir/csdlc-v2/closeout/5335.json" ]]
rg -q '^## #5335 — outside the merged-PR eligibility boundary$' "$register"

printf 'v0.91.8 terminal inventory PASS: 90 terminal, 10 fail-closed exceptions, 1 noneligible exclusion\n'
