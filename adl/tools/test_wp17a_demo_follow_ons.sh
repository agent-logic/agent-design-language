#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_contains() {
  local needle="$1"
  local haystack="$2"
  local label="$3"
  if ! grep -Fq "$needle" <<<"$haystack"; then
    echo "assertion failed: expected '$needle' in $label" >&2
    echo "$haystack" >&2
    exit 1
  fi
}

test_workflow_guardrails_showcase() {
  local out
  local inputs="$TMP/native inputs"
  mkdir -p "$inputs"
  printf '{}\n' >"$inputs/request.json"
  printf '{}\n' >"$inputs/registry.json"
  printf '{}\n' >"$inputs/registrations.json"
  # Routing double only: these fixtures establish no native authority.
  cat >"$inputs/csdlc" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$@" >"${CSDLC_TEST_ARGV:?}"
printf 'fixture doctor output\n'
exit "${CSDLC_TEST_EXIT:-0}"
EOF
  chmod +x "$inputs/csdlc"
  export CSDLC_TEST_ARGV="$inputs/observed-argv"
  local args=(--request "$inputs/request.json" --registry "$inputs/registry.json"
    --registrations "$inputs/registrations.json" --root "$inputs" --csdlc "$inputs/csdlc")
  out="$(bash "$ROOT/adl/tools/demo_v0912_workflow_guardrails_showcase.sh" "${args[@]}")"
  assert_contains "WP-16 workflow guardrails showcase" "$out" "workflow showcase header"
  assert_contains "main_write_clean: PASS main-write branch=main clean=true" "$out" "workflow clean main"
  assert_contains "main_write_dirty: BLOCKED main-write branch=main clean=false" "$out" "workflow dirty main"
  assert_contains "safe_report_command_blocked: BLOCKED safe-report-command" "$out" "workflow unsafe report"
  assert_contains "safe_report_command_pass: PASS safe-report-command" "$out" "workflow safe report"
  assert_contains "card_drift_output: fixture doctor output" "$out" "workflow card drift"
  printf '%s\n' doctor --request "$inputs/request.json" --registry "$inputs/registry.json" \
    --registrations "$inputs/registrations.json" --repo-root "$inputs" >"$inputs/expected-argv"
  cmp "$inputs/expected-argv" "$CSDLC_TEST_ARGV"
  local status=0
  CSDLC_TEST_EXIT=37 bash "$ROOT/adl/tools/demo_v0912_workflow_guardrails_showcase.sh" \
    "${args[@]}" >"$TMP/refused.out" 2>"$TMP/refused.err" || status=$?
  [[ "$status" -eq 37 ]] || { echo "expected native doctor refusal status37, got $status" >&2; return 1; }
}

test_workflow_guardrails_showcase

echo "PASS test_wp17a_demo_follow_ons"
