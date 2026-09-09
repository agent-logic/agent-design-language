#!/usr/bin/env bash
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel)
MANIFEST="$ROOT/.csdlc/prepared/issues/816/obs-b-publication-paths.txt"
CLEAN_FIXTURE="$ROOT/.csdlc/prepared/issues/816/fixtures/obs-b-publication-clean.json"
JSON_VALIDATOR="$ROOT/.csdlc/prepared/issues/816/validate-publication-json.py"

fail() {
  printf 'OBS-B redaction validation failed: %s\n' "$1" >&2
  exit 1
}

scan_file() {
  local path=$1 source="$ROOT/$1"
  ! grep -Eq '/Users/|/Volumes/|/private/tmp/' "$source" || fail "$path:machine_local_path"
  ! grep -Eq -- '-----BEGIN [A-Z0-9 ]*PRIVATE KEY-----' "$source" || fail "$path:private_key_block"
  ! grep -Eq 'gh[pousr]_[A-Za-z0-9]{20,}' "$source" || fail "$path:github_token"
  ! grep -Eq 'AKIA[A-Z0-9]{16}' "$source" || fail "$path:aws_access_key"
  ! grep -Eq 'Bearer[[:space:]]+[A-Za-z0-9._~+/=-]{16,}' "$source" || fail "$path:bearer_token"
  ! grep -Eiq '(OPENAI|ANTHROPIC|DEEPSEEK|GEMINI|GOOGLE|AWS|GITHUB)_[A-Z0-9_]*(API_)?(KEY|TOKEN)[[:space:]]*=[[:space:]]*[^[:space:]]{8,}' "$source" || fail "$path:provider_credential"
}

scan_publication_path() {
  local path=$1
  scan_file "$path"
  case "$path" in
    *.json) python3 "$JSON_VALIDATOR" "$ROOT/$path" ;;
    *) python3 "$JSON_VALIDATOR" --text "$ROOT/$path" ;;
  esac
}

[[ -f "$MANIFEST" ]] || fail 'publication manifest missing'
[[ -f "$CLEAN_FIXTURE" ]] || fail 'clean publication fixture missing'
[[ -x "$JSON_VALIDATOR" ]] || fail 'structural publication JSON validator missing'

evidence_root="$ROOT/.csdlc/evidence/816"
mkdir -p "$evidence_root"
scratch=$(mktemp -d "$evidence_root/obs-b-redaction.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
runtime_projection="$scratch/runtime-projection.json"

if ! ADL_OBS_B_PROJECTION_OUTPUT="$runtime_projection" cargo test --quiet --locked \
  --manifest-path "$ROOT/adl-runtime/Cargo.toml" \
  --test distributed_projection coherent_projection_is_deterministic_redacted_and_openapi_aligned \
  -- --exact > "$scratch/runtime-projection-test.log" 2>&1; then
  sed 's/^/OBS-B projection proof: /' "$scratch/runtime-projection-test.log" >&2
  fail 'production projection test failed'
fi
[[ -s "$runtime_projection" ]] || fail 'production projection test emitted no Runtime JSON'
python3 "$JSON_VALIDATOR" --expected-schema adl.distributed.projection.v1 "$runtime_projection"
scan_file "${runtime_projection#"$ROOT/"}"

runtime_count=1
ui_count=0
evidence_count=0
path_count=0
seen_paths=''
while IFS='|' read -r role path; do
  [[ -z "$role" || "$role" == \#* ]] && continue
  [[ "$path" != /* && "$path" != *'..'* ]] || fail "unsafe manifest path: $path"
  [[ -f "$ROOT/$path" ]] || fail "missing publication path: $path"
  ! grep -Fqx "$path" <<<"$seen_paths" || fail "duplicate publication path: $path"
  seen_paths="${seen_paths}${path}"$'\n'
  case "$role" in
    ui) ui_count=$((ui_count + 1)) ;;
    evidence) evidence_count=$((evidence_count + 1)) ;;
    *) fail "unknown publication role: $role" ;;
  esac
  scan_publication_path "$path"
  path_count=$((path_count + 1))
done < "$MANIFEST"

(( runtime_count >= 1 )) || fail 'runtime projection denominator is too small'
(( ui_count >= 4 )) || fail 'UI denominator is too small'
(( evidence_count >= 9 )) || fail 'evidence denominator is too small'

python3 "$JSON_VALIDATOR" --require-sensitive-fields "$CLEAN_FIXTURE"
scan_file '.csdlc/prepared/issues/816/fixtures/obs-b-publication-clean.json'

negative_count=0
while IFS='|' read -r expected document; do
  negative="$scratch/negative-$negative_count.json"
  printf '%s\n' "$document" > "$negative"
  if python3 "$JSON_VALIDATOR" "$negative" 2> "$scratch/negative-$negative_count.err"; then
    fail "negative fixture accepted: $expected"
  fi
  grep -Fq "$expected" "$scratch/negative-$negative_count.err" \
    || fail "negative fixture misclassified: $expected"
  negative_count=$((negative_count + 1))
done <<EOF
machine_local_path|{"safe":"/Users/example/runtime/config.toml"}
private_key_block|{"safe":"-----BEGIN PRIVATE KEY-----"}
github_token|{"safe":"ghp_$(printf 'x%.0s' {1..24})"}
aws_access_key|{"safe":"AKIA$(printf 'A%.0s' {1..16})"}
bearer_token|{"safe":"Bearer $(printf 'x%.0s' {1..24})"}
provider_credential|{"safe":"OPENAI_API_KEY=$(printf 'x%.0s' {1..24})"}
unredacted_provider_payload|{"provider_payload":"raw provider response"}
unredacted_provider_payload|{"provider_payload":"raw provider response","prompt":"[REDACTED]"}
duplicate_key|{"prompt":"[REDACTED]","prompt":"raw prompt"}
EOF

manifest_leak="${scratch#"$ROOT/"}/manifest-leak.json"
printf '{"provider_payload":"raw provider response"}\n' > "$ROOT/$manifest_leak"
negative_manifest="$scratch/negative-manifest.txt"
printf '# role|repository-relative published path\nevidence|%s\n' "$manifest_leak" > "$negative_manifest"
if (
  while IFS='|' read -r role path; do
    [[ -z "$role" || "$role" == \#* ]] && continue
    scan_publication_path "$path"
  done < "$negative_manifest"
) 2> "$scratch/negative-manifest.err"; then
  fail 'manifest-path payload leak was accepted'
fi
grep -Fq 'unredacted_provider_payload' "$scratch/negative-manifest.err" \
  || fail 'manifest-path payload leak was misclassified'
negative_count=$((negative_count + 1))

printf '{"status":"pass","publication_paths":%d,"runtime_paths":%d,"ui_paths":%d,"evidence_paths":%d,"clean_fixtures":1,"negative_fixtures":%d,"redaction_findings":0}\n' \
  "$path_count" "$runtime_count" "$ui_count" "$evidence_count" "$negative_count"
