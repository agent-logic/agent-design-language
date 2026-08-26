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

tmp_dir="$(git rev-parse --git-path csdlc-v2/tmp)"
mkdir -p "$tmp_dir"
tmp_refs="$(mktemp "$tmp_dir/dec01-runtime-refs.XXXXXX")"
bad_owner_manifest=""
duplicate_root_manifest=""
missing_root_manifest=""
authoritative_shared_surface_manifest=""
unknown_authority_field_manifest=""
runtime_v4_manifest=""
runtime_v4_key_manifest=""
deceptive_runtime_v4_manifest=""
conflicting_source_disposition_manifest=""
trap 'rm -f "$tmp_refs" "$bad_owner_manifest" "$duplicate_root_manifest" "$missing_root_manifest" "$authoritative_shared_surface_manifest" "$unknown_authority_field_manifest" "$runtime_v4_manifest" "$runtime_v4_key_manifest" "$deceptive_runtime_v4_manifest" "$conflicting_source_disposition_manifest"' EXIT

expected_source_roots_tsv=$'adl-runtime\truntime-v3-guardian\tauthoritative-runtime-v3-guardian-source\nadl-runtime-kernel\truntime-v3-kernel\tauthoritative-runtime-v3-kernel-source\nadl/src/runtime_v2\truntime-v2\tauthoritative-runtime-v2-source'
expected_shared_surfaces_tsv=$'docs/milestones/v0.92.1/evidence/runtime-decoupling\tdec-01\tevidence-only\ndocs/runtime\truntime-docs\tdocumentation-only'
expected_manifest_keys=$'compatibility_proofs\nissue\nmigration_dry_run\nplanned_id\nreverse_reference_dispositions\nreverse_reference_terms\nrollback_dry_run\nruntime_v4_excluded\nschema\nshared_surfaces\nsource_roots'

allowed_source_owners_regex='^(runtime-v2|runtime-v3-guardian|runtime-v3-kernel)$'
allowed_source_dispositions_regex='^(authoritative-runtime-v2-source|authoritative-runtime-v3-guardian-source|authoritative-runtime-v3-kernel-source)$'
allowed_reference_owners_regex='^(runtime-v2|runtime-v3-guardian|runtime-v3-kernel|runtime-docs|milestone-v0\.92\.1|dec-01)$'
allowed_reference_dispositions_regex='^(runtime-v2-to-v3-compatibility-bridge|runtime-v2-source|runtime-v3-source-or-compatibility-metadata|runtime-v3-source-or-release-gate-metadata|runtime-v3-proof|runtime-v3-support-surface|runtime-docs|runtime-planning-docs|dec-01-lifecycle-evidence|dec-01-lifecycle-state)$'
expected_reverse_reference_dispositions=$'dec-01-lifecycle-evidence\ndec-01-lifecycle-state\nruntime-docs\nruntime-planning-docs\nruntime-v2-source\nruntime-v2-to-v3-compatibility-bridge\nruntime-v3-proof\nruntime-v3-source-or-compatibility-metadata\nruntime-v3-source-or-release-gate-metadata\nruntime-v3-support-surface'

validate_manifest_shape() {
  local candidate="$1"
  local observed_keys
  observed_keys="$(jq -r 'keys[]' "$candidate")"
  if [[ "$observed_keys" != "$expected_manifest_keys" ]]; then
    echo "manifest top-level keys drifted" >&2
    return 1
  fi

  if jq -e '.source_roots[] | select((keys | sort) != ["disposition","owner","path"])' "$candidate" >/dev/null; then
    echo "source root row has unexpected keys" >&2
    return 1
  fi
  if jq -e '.shared_surfaces[] | select((keys | sort) != ["disposition","owner","path"])' "$candidate" >/dev/null; then
    echo "shared surface row has unexpected keys" >&2
    return 1
  fi
  if jq -e '.reverse_reference_dispositions[] | select((keys | sort) != ["authority_transfer","disposition","owner","path_prefix"])' "$candidate" >/dev/null; then
    echo "reverse-reference row has unexpected keys" >&2
    return 1
  fi
  if jq -e '.compatibility_proofs[] | select((keys | sort) != ["argv","id","purpose"])' "$candidate" >/dev/null; then
    echo "compatibility proof row has unexpected keys" >&2
    return 1
  fi
  if jq -e '.migration_dry_run | select((keys | sort) != ["argv","purpose"])' "$candidate" >/dev/null; then
    echo "migration dry-run row has unexpected keys" >&2
    return 1
  fi
  if jq -e '.rollback_dry_run | select((keys | sort) != ["argv","purpose"])' "$candidate" >/dev/null; then
    echo "rollback dry-run row has unexpected keys" >&2
    return 1
  fi
}

validate_static_manifest_contract() {
  local candidate="$1"
  if [[ ! -f "$candidate" ]]; then
    echo "missing manifest: $candidate" >&2
    return 1
  fi

  local schema
  schema="$(jq -r '.schema' "$candidate")"
  if [[ "$schema" != "adl.runtime_v2_v3_authority_topology.v1" ]]; then
    echo "unexpected manifest schema: $schema" >&2
    return 1
  fi

  validate_manifest_shape "$candidate" || return 1

  if [[ "$(jq -r '.runtime_v4_excluded' "$candidate")" != "true" ]]; then
    echo "Runtime v4 must remain excluded" >&2
    return 1
  fi

  local source_root_count unique_source_root_count
  source_root_count="$(jq '.source_roots | length' "$candidate")"
  unique_source_root_count="$(jq '[.source_roots[].path] | unique | length' "$candidate")"
  if [[ "$source_root_count" != "3" || "$unique_source_root_count" != "3" ]]; then
    echo "DEC-01 requires exactly three unique authoritative source roots" >&2
    return 1
  fi

  local observed_source_roots
  observed_source_roots="$(jq -r '.source_roots[] | [.path, .owner, .disposition] | @tsv' "$candidate" | sort)"
  if [[ "$observed_source_roots" != "$expected_source_roots_tsv" ]]; then
    echo "source-root owner/disposition contract drifted" >&2
    return 1
  fi

  local observed_shared_surfaces
  observed_shared_surfaces="$(jq -r '.shared_surfaces[] | [.path, .owner, .disposition] | @tsv' "$candidate" | sort)"
  if [[ "$observed_shared_surfaces" != "$expected_shared_surfaces_tsv" ]]; then
    echo "shared-surface owner/disposition contract drifted" >&2
    return 1
  fi

  if jq -e '
    paths(scalars) as $p
    | select($p[0] != "source_roots")
    | select(($p[-1] | tostring) == "disposition")
    | select(getpath($p) | tostring | test("^authoritative-"))
  ' "$candidate" >/dev/null; then
    echo "authoritative dispositions are allowed only in exact source_roots" >&2
    return 1
  fi

  while IFS=$'\t' read -r path owner disposition; do
    if [[ ! -e "$path" ]]; then
      echo "declared source root is missing: $path" >&2
      return 1
    fi
    if [[ ! "$owner" =~ $allowed_source_owners_regex ]]; then
      echo "source root has unexpected owner: $path -> $owner" >&2
      return 1
    fi
    if [[ ! "$disposition" =~ $allowed_source_dispositions_regex ]]; then
      echo "source root has unexpected disposition: $path -> $disposition" >&2
      return 1
    fi
  done < <(jq -r '.source_roots[] | [.path, .owner, .disposition] | @tsv' "$candidate")

  while IFS=$'\t' read -r prefix owner disposition transfer; do
    if [[ -z "$prefix" || ! "$owner" =~ $allowed_reference_owners_regex ]]; then
      echo "reverse-reference row has unexpected owner: $prefix -> $owner" >&2
      return 1
    fi
    if [[ ! "$disposition" =~ $allowed_reference_dispositions_regex ]]; then
      echo "reverse-reference row has unexpected disposition: $prefix -> $disposition" >&2
      return 1
    fi
    if [[ "$transfer" != "false" ]]; then
      echo "reverse-reference row transfers authority: $prefix" >&2
      return 1
    fi
    case "$prefix" in
      adl/src/runtime_v2/*)
        [[ "$owner" == "runtime-v2" ]] || {
          echo "Runtime v2 reverse-reference owner mismatch: $prefix -> $owner" >&2
          return 1
        }
        ;;
      adl-runtime/*)
        [[ "$owner" == "runtime-v3-guardian" ]] || {
          echo "Runtime v3 guardian reverse-reference owner mismatch: $prefix -> $owner" >&2
          return 1
        }
        ;;
      adl-runtime-kernel/*)
        [[ "$owner" == "runtime-v3-kernel" ]] || {
          echo "Runtime v3 kernel reverse-reference owner mismatch: $prefix -> $owner" >&2
          return 1
        }
        ;;
    esac
  done < <(jq -r '.reverse_reference_dispositions[] | [.path_prefix, .owner, .disposition, (.authority_transfer | tostring)] | @tsv' "$candidate")
}

validate_runtime_v4_manifest_contract() {
  local candidate="$1"
  if jq -e '
    paths as $p
    | ($p | map(tostring) | join(".")) as $joined
    | select($joined != "runtime_v4_excluded")
    | select(
        (any($p[]; tostring | test("runtime[-_ ]?v4|Runtime v4"; "i")))
        or ((getpath($p) | if type == "object" or type == "array" then "" else tostring end) | test("runtime[-_ ]?v4|Runtime v4"; "i"))
      )
  ' "$candidate" >/dev/null; then
    echo "Runtime v4 token appears in manifest authority-bearing data" >&2
    return 1
  fi
  if jq -e '.source_roots[] | select(.owner | test("runtime-v4"; "i"))' "$candidate" >/dev/null; then
    echo "Runtime v4 cannot own a DEC-01 source root" >&2
    return 1
  fi
}

validate_runtime_v4_markdown_contract() {
  local line
  while IFS= read -r line; do
    case "$line" in
      *":This document is the DEC-01 authority contract for v0.92.1. It separates Runtime v2 and Runtime v3 ownership without deleting Runtime v2, making Runtime v3 the default, or admitting Runtime v4; Runtime v4 is excluded."|\
      *":| \`docs/milestones/v0.92.1/evidence/runtime-decoupling/**\` | DEC-01 evidence | Machine-readable topology and executable validation; Runtime v4 authority is excluded. |"|\
      *":5. Stop for replanning if Runtime v4 is required because Runtime v4 is excluded."|\
      *":4. Runtime v4 remains excluded."|\
      *":- Runtime v4 becomes necessary despite the explicit Runtime v4 excluded boundary.")
        ;;
      *)
        echo "Runtime v4 appears outside the approved exclusion sentences: $line" >&2
        return 1
        ;;
    esac
  done < <(rg -n "runtime-v4|Runtime v4|runtime_v4" docs/runtime/runtime-v2-v3-authority-topology.md || true)
}

validate_documented_disposition_vocabulary() {
  local manifest_dispositions documented_dispositions
  manifest_dispositions="$(jq -r '.reverse_reference_dispositions[].disposition' "$manifest" | sort -u)"
  documented_dispositions="$(awk '
    /^## Reverse-Reference Dispositions/ { in_section=1; next }
    /^## Compatibility/ { in_section=0 }
    in_section && /^- `/ {
      line=$0
      sub(/^- `/, "", line)
      sub(/`.*/, "", line)
      print line
    }
  ' docs/runtime/runtime-v2-v3-authority-topology.md | sort -u)"

  if [[ "$manifest_dispositions" != "$expected_reverse_reference_dispositions" ]]; then
    echo "manifest reverse-reference disposition vocabulary drifted" >&2
    return 1
  fi
  if [[ "$documented_dispositions" != "$expected_reverse_reference_dispositions" ]]; then
    echo "documented reverse-reference disposition vocabulary drifted" >&2
    return 1
  fi
}

validate_documented_path_mapping() {
  local manifest_mapping documented_mapping
  manifest_mapping="$(jq -r '.reverse_reference_dispositions[] | [.path_prefix, .owner, .disposition] | @tsv' "$manifest" | sort)"
  documented_mapping="$(awk '
    function trim(s) {
      sub(/^[[:space:]]+/, "", s)
      sub(/[[:space:]]+$/, "", s)
      return s
    }
    /^## Reverse-Reference Path Mapping/ { in_section=1; next }
    /^## Compatibility/ { in_section=0 }
    in_section && /^\| `/ {
      line=$0
      gsub(/`/, "", line)
      split(line, cols, "|")
      print trim(cols[2]) "\t" trim(cols[3]) "\t" trim(cols[4])
    }
  ' docs/runtime/runtime-v2-v3-authority-topology.md | sort)"

  if [[ "$documented_mapping" != "$manifest_mapping" ]]; then
    echo "documented reverse-reference path mapping drifted from manifest" >&2
    return 1
  fi
}

classify_path() {
  local path="$1"
  jq -r --arg path "$path" '
    ([
      .reverse_reference_dispositions[]
      | . as $entry
      | select($path | startswith($entry.path_prefix))
      | {
          prefix_len: ($entry.path_prefix | length),
          row: ([$entry.owner, $entry.disposition, ($entry.authority_transfer | tostring)] | @tsv)
        }
    ]) as $matches
    | ($matches | map(.prefix_len) | max // 0) as $max_prefix
    | [$matches[] | select(.prefix_len == $max_prefix) | .row]
    | if length == 0 then ""
      elif length == 1 then .[0]
      else "AMBIGUOUS\t" + (length | tostring)
      end
  ' "$manifest" | head -n 1
}

validate_reverse_reference_census() {
  local reference_roots=(
    "adl/src/runtime_v2"
    "adl-runtime"
    "adl-runtime-kernel"
    "docs/runtime"
    "docs/milestones/v0.92.1"
    ".csdlc/prepared/issues/513"
    ".csdlc/evidence/513"
    ".csdlc/issues/513"
  )

  local terms=()
  while IFS= read -r term; do
    terms+=("$term")
  done < <(jq -r '.reverse_reference_terms[]' "$manifest")

  : >"$tmp_refs"
  for term in "${terms[@]}"; do
    rg -n --fixed-strings "$term" "${reference_roots[@]}" >>"$tmp_refs" || true
  done

  sort -u "$tmp_refs" -o "$tmp_refs"

  if [[ ! -s "$tmp_refs" ]]; then
    echo "no Runtime v2/v3 reverse references found; denominator is suspiciously empty" >&2
    return 1
  fi

  local unclassified=0
  local ambiguous=0
  local authority_transfers=0
  while IFS= read -r ref; do
    local path classification transfer
    path="${ref%%:*}"
    classification="$(classify_path "$path")"
    if [[ -z "$classification" ]]; then
      echo "unclassified reverse reference: $ref" >&2
      unclassified=$((unclassified + 1))
      continue
    fi
    if [[ "$classification" == AMBIGUOUS$'\t'* ]]; then
      echo "ambiguous reverse reference: $ref" >&2
      ambiguous=$((ambiguous + 1))
      continue
    fi
    transfer="$(printf '%s\n' "$classification" | cut -f3)"
    if [[ "$transfer" == "true" ]]; then
      echo "forbidden authority transfer reference: $ref" >&2
      authority_transfers=$((authority_transfers + 1))
    fi
  done < "$tmp_refs"

  if [[ "$unclassified" -ne 0 || "$ambiguous" -ne 0 || "$authority_transfers" -ne 0 ]]; then
    echo "reverse-reference census failed: unclassified=$unclassified ambiguous=$ambiguous authority_transfers=$authority_transfers" >&2
    return 1
  fi
}

run_negative_manifest_probes() {
  bad_owner_manifest="$(mktemp "$tmp_dir/dec01-bad-owner.XXXXXX")"
  duplicate_root_manifest="$(mktemp "$tmp_dir/dec01-duplicate-root.XXXXXX")"
  missing_root_manifest="$(mktemp "$tmp_dir/dec01-missing-root.XXXXXX")"
  authoritative_shared_surface_manifest="$(mktemp "$tmp_dir/dec01-authoritative-shared-surface.XXXXXX")"
  unknown_authority_field_manifest="$(mktemp "$tmp_dir/dec01-unknown-authority-field.XXXXXX")"
  runtime_v4_manifest="$(mktemp "$tmp_dir/dec01-runtime-v4.XXXXXX")"
  runtime_v4_key_manifest="$(mktemp "$tmp_dir/dec01-runtime-v4-key.XXXXXX")"
  deceptive_runtime_v4_manifest="$(mktemp "$tmp_dir/dec01-deceptive-runtime-v4.XXXXXX")"
  conflicting_source_disposition_manifest="$(mktemp "$tmp_dir/dec01-conflicting-source-disposition.XXXXXX")"

  jq '(.source_roots[] | select(.path == "adl/src/runtime_v2") | .owner) = "runtime-v3-kernel"' "$manifest" >"$bad_owner_manifest"
  jq '.source_roots += [.source_roots[0]]' "$manifest" >"$duplicate_root_manifest"
  jq '.source_roots = [.source_roots[] | select(.path != "adl-runtime")]' "$manifest" >"$missing_root_manifest"
  jq '.shared_surfaces += [{"path":"extra-runtime-source","owner":"runtime-v3-kernel","disposition":"authoritative-runtime-v3-kernel-source"}]' "$manifest" >"$authoritative_shared_surface_manifest"
  jq '.shared_surfaces[0].authority = "runtime-v3-kernel"' "$manifest" >"$unknown_authority_field_manifest"
  jq '.shared_surfaces += [{"path":"runtime-v4-authoritative-source","owner":"runtime-v4","disposition":"authoritative-runtime-v4-source"}]' "$manifest" >"$runtime_v4_manifest"
  jq '.runtime_v4_authority = true' "$manifest" >"$runtime_v4_key_manifest"
  jq '.shared_surfaces += [{"path":"future-excluded","owner":"runtime-docs","disposition":"authoritative-runtime-v4-excluded-source"}]' "$manifest" >"$deceptive_runtime_v4_manifest"
  jq '(.source_roots[] | select(.path == "adl/src/runtime_v2") | .disposition) = "runtime-v2-source"' "$manifest" >"$conflicting_source_disposition_manifest"

  if validate_static_manifest_contract "$bad_owner_manifest" >/dev/null 2>&1; then
    echo "negative probe failed: owner swap was accepted" >&2
    return 1
  fi
  if validate_static_manifest_contract "$duplicate_root_manifest" >/dev/null 2>&1; then
    echo "negative probe failed: duplicate root was accepted" >&2
    return 1
  fi
  if validate_static_manifest_contract "$missing_root_manifest" >/dev/null 2>&1; then
    echo "negative probe failed: missing root was accepted" >&2
    return 1
  fi
  if validate_static_manifest_contract "$authoritative_shared_surface_manifest" >/dev/null 2>&1; then
    echo "negative probe failed: authoritative shared surface was accepted" >&2
    return 1
  fi
  if validate_static_manifest_contract "$unknown_authority_field_manifest" >/dev/null 2>&1; then
    echo "negative probe failed: unknown authority field was accepted" >&2
    return 1
  fi
  if validate_runtime_v4_manifest_contract "$runtime_v4_manifest" >/dev/null 2>&1; then
    echo "negative probe failed: Runtime v4 authority data was accepted" >&2
    return 1
  fi
  if validate_runtime_v4_manifest_contract "$runtime_v4_key_manifest" >/dev/null 2>&1; then
    echo "negative probe failed: Runtime v4 key was accepted" >&2
    return 1
  fi
  if validate_runtime_v4_manifest_contract "$deceptive_runtime_v4_manifest" >/dev/null 2>&1; then
    echo "negative probe failed: deceptive Runtime v4 authority data was accepted" >&2
    return 1
  fi
  if validate_static_manifest_contract "$conflicting_source_disposition_manifest" >/dev/null 2>&1; then
    echo "negative probe failed: conflicting source disposition was accepted" >&2
    return 1
  fi
}

validate_all_static_contracts() {
  validate_static_manifest_contract "$manifest"
  validate_runtime_v4_manifest_contract "$manifest"
  validate_runtime_v4_markdown_contract
  validate_documented_disposition_vocabulary
  validate_documented_path_mapping
  validate_reverse_reference_census
  run_negative_manifest_probes
}

migration_dry_run() {
  validate_all_static_contracts
  jq -n \
    --arg schema "adl.runtime_v2_v3_migration_dry_run.v1" \
    --arg manifest "$manifest" \
    --arg issue "513" \
    '{schema: $schema, issue: ($issue | tonumber), manifest: $manifest, result: "passed", checked: ["source-root-exactness", "reverse-reference-classification", "runtime-v4-exclusion"]}'
}

rollback_dry_run() {
  validate_all_static_contracts
  jq -n \
    --arg schema "adl.runtime_v2_v3_rollback_dry_run.v1" \
    --arg manifest "$manifest" \
    --arg issue "513" \
    '{schema: $schema, issue: ($issue | tonumber), manifest: $manifest, result: "passed", runtime_v2_root: "adl/src/runtime_v2", runtime_v3_roots: ["adl-runtime", "adl-runtime-kernel"], checked: ["independent-source-owners", "authoritative-dispositions"]}'
}

case "${1:-}" in
  "")
    validate_all_static_contracts
    cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test contracts parity_baseline_manifest_is_a_captured_inventory_not_a_live_repo_dependency
    cargo test --manifest-path adl/Cargo.toml runtime_v2_reasoning_objects_execute_through_native_component_core
    migration_dry_run >/dev/null
    rollback_dry_run >/dev/null
    refs_count="$(wc -l < "$tmp_refs" | tr -d ' ')"
    jq -n \
      --arg schema "adl.runtime_v2_v3_authority_topology_validation.v1" \
      --arg manifest "$manifest" \
      --arg refs_count "$refs_count" \
      '{schema: $schema, issue: 513, manifest: $manifest, result: "passed", classified_reverse_references: ($refs_count | tonumber), compatibility_proofs: ["runtime-v3-captured-baseline", "runtime-v2-to-v3-reasoning-bridge"], negative_probes: ["owner-swap", "duplicate-root", "missing-root", "authoritative-shared-surface", "unknown-authority-field", "runtime-v4-authority-data", "runtime-v4-key", "deceptive-runtime-v4-authority-data", "conflicting-source-disposition"]}'
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
