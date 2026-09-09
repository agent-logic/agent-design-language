#!/usr/bin/env bash
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel)
MANIFEST="$ROOT/.csdlc/prepared/issues/816/obs-b-publication-paths.txt"
CLEAN_FIXTURE="$ROOT/.csdlc/prepared/issues/816/fixtures/obs-b-publication-clean.json"

fail() {
  printf 'OBS-B redaction validation failed: %s\n' "$1" >&2
  exit 1
}

detect_category() {
  local input=$1 field
  if grep -Eq '/Users/|/Volumes/|/private/tmp/' <<<"$input"; then
    printf '%s\n' machine_local_path
  elif grep -Eq -- '-----BEGIN [A-Z0-9 ]*PRIVATE KEY-----' <<<"$input"; then
    printf '%s\n' private_key_block
  elif grep -Eq 'gh[pousr]_[A-Za-z0-9]{20,}' <<<"$input"; then
    printf '%s\n' github_token
  elif grep -Eq 'AKIA[A-Z0-9]{16}' <<<"$input"; then
    printf '%s\n' aws_access_key
  elif grep -Eq 'Bearer[[:space:]]+[A-Za-z0-9._~+/=-]{16,}' <<<"$input"; then
    printf '%s\n' bearer_token
  elif grep -Eiq '(OPENAI|ANTHROPIC|DEEPSEEK|GEMINI|GOOGLE|AWS|GITHUB)_[A-Z0-9_]*(API_)?(KEY|TOKEN)[[:space:]]*=[[:space:]]*[^[:space:]]{8,}' <<<"$input"; then
    printf '%s\n' provider_credential
  elif grep -Eq '"(provider_payload|prompt|output|tool_arguments)"[[:space:]]*:' <<<"$input"; then
    for field in provider_payload prompt output tool_arguments; do
      if grep -Eq "\"$field\"[[:space:]]*:" <<<"$input" \
        && ! grep -Eq "\"$field\"[[:space:]]*:[[:space:]]*\"\\[REDACTED\\]\"" <<<"$input"; then
        printf '%s\n' unredacted_provider_payload
        return
      fi
    done
  fi
}

scan_file() {
  local path=$1 source="$ROOT/$1" field matches
  ! grep -Eq '/Users/|/Volumes/|/private/tmp/' "$source" || fail "$path:machine_local_path"
  ! grep -Eq -- '-----BEGIN [A-Z0-9 ]*PRIVATE KEY-----' "$source" || fail "$path:private_key_block"
  ! grep -Eq 'gh[pousr]_[A-Za-z0-9]{20,}' "$source" || fail "$path:github_token"
  ! grep -Eq 'AKIA[A-Z0-9]{16}' "$source" || fail "$path:aws_access_key"
  ! grep -Eq 'Bearer[[:space:]]+[A-Za-z0-9._~+/=-]{16,}' "$source" || fail "$path:bearer_token"
  ! grep -Eiq '(OPENAI|ANTHROPIC|DEEPSEEK|GEMINI|GOOGLE|AWS|GITHUB)_[A-Z0-9_]*(API_)?(KEY|TOKEN)[[:space:]]*=[[:space:]]*[^[:space:]]{8,}' "$source" || fail "$path:provider_credential"
  for field in provider_payload prompt output tool_arguments; do
    matches=$(grep -E "\"$field\"[[:space:]]*:" "$source" || true)
    [[ -z "$matches" ]] && continue
    ! grep -Evq "\"$field\"[[:space:]]*:[[:space:]]*\"\\[REDACTED\\]\"" <<<"$matches" \
      || fail "$path:unredacted_provider_payload:$field"
  done
}

[[ -f "$MANIFEST" ]] || fail 'publication manifest missing'
[[ -f "$CLEAN_FIXTURE" ]] || fail 'clean publication fixture missing'

runtime_count=0
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
    runtime) runtime_count=$((runtime_count + 1)) ;;
    ui) ui_count=$((ui_count + 1)) ;;
    evidence) evidence_count=$((evidence_count + 1)) ;;
    *) fail "unknown publication role: $role" ;;
  esac
  scan_file "$path"
  path_count=$((path_count + 1))
done < "$MANIFEST"

(( runtime_count >= 3 )) || fail 'runtime projection denominator is too small'
(( ui_count >= 4 )) || fail 'UI denominator is too small'
(( evidence_count >= 9 )) || fail 'evidence denominator is too small'

scan_file '.csdlc/prepared/issues/816/fixtures/obs-b-publication-clean.json'
for field in provider_payload prompt output tool_arguments; do
  grep -Fq "\"$field\": \"[REDACTED]\"" "$CLEAN_FIXTURE" || fail "clean fixture lacks redacted $field"
done

negative_count=0
while IFS='|' read -r expected fragment; do
  category=$(detect_category "$fragment")
  [[ "$category" == "$expected" ]] || fail "negative fixture accepted or misclassified: $expected"
  negative_count=$((negative_count + 1))
done <<EOF
machine_local_path|/Users/example/runtime/config.toml
private_key_block|-----BEGIN PRIVATE KEY-----
github_token|ghp_$(printf 'x%.0s' {1..24})
aws_access_key|AKIA$(printf 'A%.0s' {1..16})
bearer_token|Bearer $(printf 'x%.0s' {1..24})
provider_credential|OPENAI_API_KEY=$(printf 'x%.0s' {1..24})
unredacted_provider_payload|{"provider_payload":"raw provider response"}
unredacted_provider_payload|{"provider_payload":"raw provider response","prompt":"[REDACTED]"}
EOF

printf '{"status":"pass","publication_paths":%d,"runtime_paths":%d,"ui_paths":%d,"evidence_paths":%d,"clean_fixtures":1,"negative_fixtures":%d,"redaction_findings":0}\n' \
  "$path_count" "$runtime_count" "$ui_count" "$evidence_count" "$negative_count"
