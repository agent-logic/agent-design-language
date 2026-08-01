#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
common_dir="$(git rev-parse --git-common-dir)"
if [[ "$common_dir" != /* ]]; then
  common_dir="$repo_root/$common_dir"
fi

doctor="$repo_root/.adl/bin/csdlc-v2/csdlc-doctor"
installer="$repo_root/.adl/bin/csdlc-v2/csdlc-install"
inventory="$repo_root/csdlc-v2/operator/coexistence.json"
register="$repo_root/.csdlc/prepared/issues/5748/fail-closed-exceptions.md"
universe="$repo_root/.csdlc/evidence/5748/v0918-closed-issue-universe.json"

fail() {
  printf 'v0.91.8 terminal inventory FAIL: %s\n' "$1" >&2
  exit 1
}

require_no_symlink_components() {
  local root="$1"
  local path="$2"
  local current="$path"
  case "$path" in
    "$root"|"$root"/*) ;;
    *) fail "governed path escapes its declared root: $path" ;;
  esac
  while [[ "$current" != "$root" ]]; do
    [[ ! -L "$current" ]] || fail "governed path contains a symlink: $current"
    current="${current%/*}"
  done
  [[ ! -L "$root" ]] || fail "governed root is a symlink: $root"
}

require_file() {
  require_no_symlink_components "$1" "$2"
  [[ -f "$2" && ! -L "$2" ]] || fail "missing canonical regular file: $2"
}

require_absent() {
  require_no_symlink_components "$1" "$2"
  [[ ! -e "$2" && ! -L "$2" ]] || fail "unexpected path exists: $2"
}

require_eq() {
  [[ "$1" == "$2" ]] || fail "$3 (expected $2, observed $1)"
}

path_guard_self_test() {
  local scratch="$repo_root/.csdlc/evidence/5748/.validator-path-guard-self-test"
  local target="$scratch/real/target"
  local file_link="$scratch/file-link"
  local dir_link="$scratch/dir-link"
  local dangling="$scratch/dangling"
  path_guard_cleanup() {
    unlink "$file_link" 2>/dev/null || true
    unlink "$dir_link" 2>/dev/null || true
    unlink "$dangling" 2>/dev/null || true
    unlink "$target" 2>/dev/null || true
    rmdir "$scratch/real" 2>/dev/null || true
    rmdir "$scratch" 2>/dev/null || true
  }
  trap path_guard_cleanup EXIT
  require_absent "$repo_root" "$scratch"
  mkdir -p "$scratch/real"
  printf 'canonical\n' >"$target"
  ln -s "$target" "$file_link"
  ln -s "$scratch/real" "$dir_link"
  ln -s "$scratch/missing" "$dangling"
  require_file "$repo_root" "$target"
  if (require_file "$repo_root" "$file_link") 2>/dev/null; then
    fail "path guard accepted a final file symlink"
  fi
  if (require_file "$repo_root" "$dir_link/target") 2>/dev/null; then
    fail "path guard accepted a symlinked parent component"
  fi
  if (require_absent "$repo_root" "$dangling") 2>/dev/null; then
    fail "path guard treated a dangling symlink as absent"
  fi
  path_guard_cleanup
  trap - EXIT
  printf 'v0.91.8 inventory path-guard self-test PASS\n'
}

if [[ "${1:-}" == "--self-test-path-guards" ]]; then
  path_guard_self_test
  exit 0
fi

terminal_issues=(
  4739 4741 4758 4759 4760 4761 4762 4763 5107 5332 5336 5337 5338
  5339 5340 5341 5342 5343 5344 5345 5346 5347 5349 5350 5352 5354 5358 5361
  5384 5438 5470 5497 5498 5499 5500 5501 5502 5526 5527 5540 5541
  5548 5563 5566 5569 5572 5587 5589 5590 5591 5592 5594 5597 5600
  5602 5605 5610 5613 5615 5624 5627 5632 5645 5648 5653 5657 5658 5662
  5663 5665 5666 5670 5671 5678 5679 5683 5686 5687 5691 5695 5697
  5698 5701 5702 5708 5710 5711 5713 5715 5717 5718 5719 5722 5727 5728
  5733 5735 5737 5746
)
exception_issues=(5007 5558 5664 5675)
noneligible_issues=(5335)
claim_free_exception_issues=(5664 5675)
dormant_exception_issues=(5664 5675)

array_contains() {
  local target="$1"
  shift
  local candidate
  for candidate in "$@"; do
    [[ "$candidate" == "$target" ]] && return 0
  done
  return 1
}

exception_projection() {
  case "$1" in
    5664) printf '%s\t%s\t%s\n' published 5 8c254685618757825b8b738c551e5a54b41894f896f0ddb24214e9f935a537f8 ;;
    5675) printf '%s\t%s\t%s\n' reviewed 13 cc151e358e674d07613646d4fc1f6ed71a3613a2f145b9065a73bc0103770818 ;;
    *) return 1 ;;
  esac
}

terminal_count="${#terminal_issues[@]}"
exception_count="${#exception_issues[@]}"
noneligible_count="${#noneligible_issues[@]}"
completed_count="$((terminal_count + exception_count))"
closed_count="$((completed_count + noneligible_count))"
for issue in "${claim_free_exception_issues[@]}" "${dormant_exception_issues[@]}"; do
  array_contains "$issue" "${exception_issues[@]}" ||
    fail "exception-only issue #$issue is absent from the exception partition"
done
require_file "$repo_root" "$register"
require_file "$repo_root" "$universe"
require_file "$repo_root" "$installer"
require_file "$repo_root" "$doctor"
require_file "$repo_root" "$inventory"
"$installer" verify --repo "$repo_root" --bin-dir .adl/bin/csdlc-v2 \
  --inventory "$inventory" >/dev/null || fail "owner-binary provenance is stale"

declared_completed="$(
  printf '%s\n' "${terminal_issues[@]}" "${exception_issues[@]}" | sort -n | tr '\n' ' '
)"
observed_completed="$(
  jq -r '.issues[] | select(.state == "CLOSED" and .state_reason == "COMPLETED") |
    .number' "$universe" | sort -n | tr '\n' ' '
)"
require_eq "$observed_completed" "$declared_completed" \
  "retained live completed-issue universe differs from the declared partition"
require_eq "$(printf '%s\n' "${terminal_issues[@]}" "${exception_issues[@]}" | sort -nu | wc -l | tr -d ' ')" \
  "$completed_count" "declared completed-issue partition contains duplicates"
require_eq "$(jq -r '[.issues[] | select(.state == "CLOSED" and .state_reason == "NOT_PLANNED") | .number] | sort | @csv' "$universe")" \
  "$(IFS=,; printf '%s' "${noneligible_issues[*]}")" "retained noneligible issue universe mismatch"
jq -e --argjson closed_count "$closed_count" '.schema == "adl.v0918.closed_issue_universe.v1" and
  .repository == "danielbaustin/agent-design-language" and
  .label == "version:v0.91.8" and .state == "closed" and
  (.issues | length) == $closed_count and
  ([.issues[].number] | length) == ([.issues[].number] | unique | length)' \
  "$universe" >/dev/null || fail "retained closed-issue universe metadata is invalid"
declared_register_issues="$(
  printf '%s\n' "${exception_issues[@]}" "${noneligible_issues[@]}" | sort -n | tr '\n' ' '
)"
observed_register_issues="$(
  sed -n 's/^## #\([0-9][0-9]*\) —.*/\1/p' "$register" | sort -n | tr '\n' ' '
)"
require_eq "$observed_register_issues" "$declared_register_issues" \
  "exception register headings differ from the declared exception and noneligible partition"

for issue in "${terminal_issues[@]}"; do
  index="$repo_root/.csdlc/issues/$issue/index.json"
  receipt="$common_dir/csdlc-v2/closeout/$issue.json"
  require_file "$repo_root" "$index"
  require_file "$common_dir" "$receipt"
  require_eq "$(jq -r '.phase' "$index")" closed_out \
    "terminal issue #$issue phase mismatch"
  require_eq "$(jq -r '.issue' "$index")" "$issue" \
    "terminal issue #$issue projection namespace mismatch"
  jq -e --argjson issue "$issue" \
    '.issue == $issue and .record.issue == $issue and
     .receipt_ref == ("csdlc-v2/closeout/" + ($issue | tostring) + ".json")' \
    "$receipt" >/dev/null || fail "terminal issue #$issue receipt namespace mismatch"
  require_eq "$(jq -r '.claim == null' "$index")" true \
    "terminal issue #$issue retained an active claim"
  "$doctor" --repo "$repo_root" --issue "$issue" >/dev/null || \
    fail "terminal issue #$issue failed doctor"
  jq -e --slurpfile receipt "$receipt" '. == $receipt[0].record' "$index" \
    >/dev/null || fail "terminal issue #$issue index differs from receipt"
  for card in sip stp spp vpp srp sor; do
    require_file "$repo_root" \
      "$repo_root/.csdlc/issues/$issue/cards/$card.values.json"
    jq -e --arg card "$card" --slurpfile receipt "$receipt" \
      '. == $receipt[0].cards[$card]' \
      "$repo_root/.csdlc/issues/$issue/cards/$card.values.json" >/dev/null || \
      fail "terminal issue #$issue $card values differ from receipt"
    require_eq "$(jq -r '.identity.issue' \
      "$repo_root/.csdlc/issues/$issue/cards/$card.values.json")" "$issue" \
      "terminal issue #$issue $card namespace mismatch"
  done
done

for issue in "${exception_issues[@]}"; do
  if [[ "$issue" == 5007 ]]; then
    require_file "$common_dir" "$common_dir/csdlc-v2/closeout/5007.json"
    jq -e '.issue == 5007 and .record.issue == 5007 and
      .record.phase == "closed_out" and .record.claim == null and
      .record.generation == 6 and
      .record.digest == "fd4abaf3f7b219c72c685e4968928a01364bd749a4580ba39930ccf4a1a5a98a" and
      .digest == "83a9539680ed730193da404790e6b42ff7b0ee3dc1865c3e831094973d5d380b"' \
      "$common_dir/csdlc-v2/closeout/5007.json" >/dev/null || \
      fail "exception #5007 terminal receipt mismatch"
  else
    require_absent "$common_dir" "$common_dir/csdlc-v2/closeout/$issue.json"
  fi
  rg -q "^## #$issue —" "$register" || \
    fail "exception #$issue is missing from the register"
done

for issue in "${claim_free_exception_issues[@]}"; do
  index="$repo_root/.csdlc/issues/$issue/index.json"
  IFS=$'\t' read -r expected_phase expected_generation expected_digest \
    <<<"$(exception_projection "$issue")"
  require_file "$repo_root" "$index"
  require_eq "$(jq -r '.claim == null' "$index")" true \
    "exception #$issue retained an active claim"
  require_eq "$(jq -r '.digest' "$index")" "$expected_digest" \
    "exception #$issue digest mismatch"
  require_eq "$(jq -r '.phase' "$index")" "$expected_phase" \
    "exception #$issue phase mismatch"
  require_eq "$(jq -r '.generation' "$index")" "$expected_generation" \
    "exception #$issue generation mismatch"
  rg -q "\`$expected_digest\`" "$register" || \
    fail "exception #$issue digest is missing from the register"
done

for issue in "${dormant_exception_issues[@]}"; do
  tail -n 1 "$repo_root/.csdlc/issues/$issue/audit.jsonl" | jq -e \
    '(.operation | if type == "string" then fromjson else . end |
      .operation) == "revoke_active_claim"' >/dev/null || \
    fail "exception #$issue audit does not end in typed claim revocation"
  doctor_report=""
  if doctor_report="$("$doctor" --repo "$repo_root" --issue "$issue" 2>&1)"; then
    printf 'exception #%s unexpectedly passed doctor\n' "$issue" >&2
    exit 1
  fi
  printf '%s\n' "$doctor_report" | jq -e \
    '.status == "block" and .ready == false and
     (.findings | length) == 1 and .findings[0].code == "claim_dormant"' \
    >/dev/null || fail "exception #$issue doctor state mismatch"
done

if array_contains 5007 "${exception_issues[@]}"; then
  # #5007 is intentionally preserved as the exact corrupt projection while it
  # remains an exception. Moving it into terminal_issues switches authority to
  # the generic receipt-backed terminal checks above.
  corrupt_index="$repo_root/.csdlc/issues/5007/index.json"
  require_file "$repo_root" "$corrupt_index"
  require_eq "$(git rev-parse HEAD:.csdlc/issues/5007)" \
    773eb443b05aac396c0d17705374edd4f754cfdf \
    "exception #5007 committed projection tree mismatch"
  git diff --quiet HEAD -- .csdlc/issues/5007 || \
    fail "exception #5007 working projection differs from its pinned commit"
  require_eq "$(git ls-files --others --exclude-standard -- .csdlc/issues/5007)" "" \
    "exception #5007 contains untracked projection files"
  while IFS= read -r path; do
    require_file "$repo_root" "$repo_root/$path"
  done < <(git ls-files .csdlc/issues/5007)
  require_eq "$(jq -r '.phase' "$corrupt_index")" published \
    "exception #5007 phase mismatch"
  require_eq "$(jq -r '.generation' "$corrupt_index")" 5 \
    "exception #5007 generation mismatch"
  require_eq "$(jq -r '.digest' "$corrupt_index")" \
    12194eb860c30b87b2e8929d2fe0726fbe7006d0c901454b581ee82fa693f6ed \
    "exception #5007 claimed digest mismatch"
  require_eq "$(jq -r '.claim.id' "$corrupt_index")" \
    exec-5007-memory-palace-adr-20260731 "exception #5007 claim id mismatch"
  require_eq "$(jq -r '.claim.owner' "$corrupt_index")" \
    codex:5007-execution-2026-07-31 "exception #5007 claim owner mismatch"
  require_eq "$(jq -r '.claim.branch' "$corrupt_index")" \
    codex/5007-v0918-wp14-preparation "exception #5007 claim branch mismatch"
  require_eq "$(jq -r '.claim.worktree' "$corrupt_index")" . \
    "exception #5007 claim worktree mismatch"
  require_eq "$(jq -r '.claim.expires_unix_seconds' "$corrupt_index")" 1786138590 \
    "exception #5007 claim expiry mismatch"
  if corrupt_report="$("$doctor" --repo "$repo_root" --issue 5007 2>&1)"; then
    printf 'exception #5007 unexpectedly passed doctor\n' >&2
    exit 1
  fi
  printf '%s\n' "$corrupt_report" | jq -e \
    '.status == "corrupt" and .ready == false and
     (.findings | length) == 1 and
     .findings[0].code == "corrupt_record" and
     .findings[0].message == "index digest mismatch"' >/dev/null || \
    fail "exception #5007 doctor state mismatch"
fi

# These two issues have no local lifecycle projection. Their absence is part
# of the fail-closed evidence and must remain explicit.
for issue in 5558 5722; do
  if array_contains "$issue" "${exception_issues[@]}"; then
    require_absent "$repo_root" "$repo_root/.csdlc/issues/$issue"
  fi
done

require_absent "$common_dir" "$common_dir/csdlc-v2/closeout/5335.json"
rg -q '^## #5335 — outside the merged-PR eligibility boundary$' "$register" || \
  fail "noneligible exclusion #5335 is missing from the register"

require_eq "$(git status --porcelain -- .csdlc/locks .csdlc/requests)" "" \
  "generated lock or request state dirties the publication worktree"

printf 'v0.91.8 terminal inventory PASS: %s terminal, %s fail-closed exceptions, %s noneligible exclusion\n' \
  "$terminal_count" "$exception_count" "$noneligible_count"
