#!/usr/bin/env bash
set -euo pipefail

manifest="docs/milestones/v0.92.1/evidence/runtime-decoupling/runtime-authority-topology.json"

usage() {
  echo "usage: $0 [--migration-dry-run|--rollback-dry-run]" >&2
}

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

require_tool() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required tool: $1" >&2
    exit 1
  fi
}

require_tool jq
require_tool rg

if [[ ! -f "$manifest" ]]; then
  echo "missing manifest: $manifest" >&2
  exit 1
fi

schema="$(jq -r '.schema' "$manifest")"
if [[ "$schema" != "adl.runtime_v2_v3_authority_topology.v1" ]]; then
  echo "unexpected manifest schema: $schema" >&2
  exit 1
fi

if [[ "$(jq -r '.runtime_v4_excluded' "$manifest")" != "true" ]]; then
  echo "Runtime v4 must remain excluded" >&2
  exit 1
fi

while IFS=$'\t' read -r path owner disposition; do
  if [[ ! -e "$path" ]]; then
    echo "declared source root is missing: $path" >&2
    exit 1
  fi
  if [[ -z "$owner" || -z "$disposition" ]]; then
    echo "source root lacks owner/disposition: $path" >&2
    exit 1
  fi
done < <(jq -r '.source_roots[] | [.path, .owner, .disposition] | @tsv' "$manifest")

if jq -e '.source_roots[] | select(.owner | test("runtime-v4"; "i"))' "$manifest" >/dev/null; then
  echo "Runtime v4 cannot own a DEC-01 source root" >&2
  exit 1
fi

if rg -n "runtime-v4|Runtime v4|runtime_v4" \
  docs/runtime/runtime-v2-v3-authority-topology.md \
  docs/milestones/v0.92.1/evidence/runtime-decoupling/runtime-authority-topology.json \
  | rg -v "excluded|excludes|out of scope|replanning|Stop Conditions|cannot own|does not appear|non_goals|Non-Goals|runtime_v4_excluded|runtime-v4\"; \"i\"|Runtime v4 cannot own|exclusion language" >/dev/null; then
  echo "Runtime v4 appears outside exclusion language" >&2
  exit 1
fi

classify_path() {
  local path="$1"
  jq -r --arg path "$path" '
    .reverse_reference_dispositions[]
    | . as $entry
    | select($path | startswith($entry.path_prefix))
    | [$entry.owner, $entry.disposition, ($entry.authority_transfer | tostring)]
    | @tsv
  ' "$manifest" | head -n 1
}

reference_roots=(
  "adl/src/runtime_v2"
  "adl-runtime"
  "adl-runtime-kernel"
  "docs/runtime"
  "docs/milestones/v0.92.1"
  ".csdlc/prepared/issues/513"
  ".csdlc/issues/513"
)

terms=()
while IFS= read -r term; do
  terms+=("$term")
done < <(jq -r '.reverse_reference_terms[]' "$manifest")

tmp_dir="$(git rev-parse --git-path csdlc-v2/tmp)"
mkdir -p "$tmp_dir"
tmp_refs="$(mktemp "$tmp_dir/dec01-runtime-refs.XXXXXX")"
trap 'rm -f "$tmp_refs"' EXIT

for term in "${terms[@]}"; do
  rg -n --fixed-strings "$term" "${reference_roots[@]}" >>"$tmp_refs" || true
done

sort -u "$tmp_refs" -o "$tmp_refs"

if [[ ! -s "$tmp_refs" ]]; then
  echo "no Runtime v2/v3 reverse references found; denominator is suspiciously empty" >&2
  exit 1
fi

unclassified=0
authority_transfers=0
while IFS= read -r ref; do
  path="${ref%%:*}"
  classification="$(classify_path "$path")"
  if [[ -z "$classification" ]]; then
    echo "unclassified reverse reference: $ref" >&2
    unclassified=$((unclassified + 1))
    continue
  fi
  transfer="$(printf '%s\n' "$classification" | cut -f3)"
  if [[ "$transfer" == "true" ]]; then
    echo "forbidden authority transfer reference: $ref" >&2
    authority_transfers=$((authority_transfers + 1))
  fi
done < "$tmp_refs"

if [[ "$unclassified" -ne 0 || "$authority_transfers" -ne 0 ]]; then
  echo "reverse-reference census failed: unclassified=$unclassified authority_transfers=$authority_transfers" >&2
  exit 1
fi

migration_dry_run() {
  jq -n \
    --arg schema "adl.runtime_v2_v3_migration_dry_run.v1" \
    --arg manifest "$manifest" \
    --arg issue "513" \
    '{schema: $schema, issue: ($issue | tonumber), manifest: $manifest, result: "passed", runtime_v4_excluded: true}'
}

rollback_dry_run() {
  jq -n \
    --arg schema "adl.runtime_v2_v3_rollback_dry_run.v1" \
    --arg manifest "$manifest" \
    --arg issue "513" \
    '{schema: $schema, issue: ($issue | tonumber), manifest: $manifest, result: "passed", runtime_v2_root: "adl/src/runtime_v2", runtime_v3_roots: ["adl-runtime", "adl-runtime-kernel"]}'
}

case "${1:-}" in
  "")
    cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test contracts parity_baseline_manifest_is_a_captured_inventory_not_a_live_repo_dependency
    migration_dry_run >/dev/null
    rollback_dry_run >/dev/null
    refs_count="$(wc -l < "$tmp_refs" | tr -d ' ')"
    jq -n \
      --arg schema "adl.runtime_v2_v3_authority_topology_validation.v1" \
      --arg manifest "$manifest" \
      --arg refs_count "$refs_count" \
      '{schema: $schema, issue: 513, manifest: $manifest, result: "passed", classified_reverse_references: ($refs_count | tonumber)}'
    ;;
  "--migration-dry-run")
    migration_dry_run
    ;;
  "--rollback-dry-run")
    rollback_dry_run
    ;;
  *)
    usage
    exit 2
    ;;
esac
