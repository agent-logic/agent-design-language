#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

usage() {
  cat <<'EOF'
Usage:
  bash adl/tools/demo_v0912_workflow_guardrails_showcase.sh --request <path> --registry <path> --registrations <path> [--root <repo-root>] [--csdlc <installed-path>]

Runs one bounded operator-facing showcase pass over the WP-16 workflow
guardrails and prints the pass/block outcomes for each guardrail family.
Card-drift requires real caller-provided native doctor inputs; none are fabricated.
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

init_repo() {
  local repo="$1"
  git -C "$repo" init -q
  git -C "$repo" config user.name "Demo User"
  git -C "$repo" config user.email "demo@example.com"
  printf '.adl/\n' >"$repo/.gitignore"
  printf 'seed\n' >"$repo/README.md"
  git -C "$repo" add .gitignore README.md
  git -C "$repo" commit -q -m "init"
  git -C "$repo" branch -M main
}

echo "WP-16 workflow guardrails showcase"

repo="$TMP/main-write-repo"
mkdir -p "$repo"
init_repo "$repo"

clean_main="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" main-write --repo "$repo")"
printf 'tracked drift\n' >"$repo/README.md"
dirty_main="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" main-write --repo "$repo" 2>&1 || true)"

unsafe_cmd="$TMP/unsafe-report-command.txt"
cat >"$unsafe_cmd" <<'EOF'
cat <<EOF2 > report.md
$(pwd)
EOF2
EOF
unsafe_report="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" safe-report-command --file "$unsafe_cmd" 2>&1 || true)"

safe_cmd="$TMP/safe-report-command.txt"
cat >"$safe_cmd" <<'EOF'
cat > report.md <<'MD'
# Report

```bash
echo safe
```
MD
EOF
safe_report="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" safe-report-command --file "$safe_cmd")"

card_drift_out="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" card-drift "$@")"

printf '%s\n' "main_write_clean: $clean_main"
printf '%s\n' "main_write_dirty: $dirty_main"
printf '%s\n' "safe_report_command_blocked: $unsafe_report"
printf '%s\n' "safe_report_command_pass: $safe_report"
printf '%s\n' "card_drift_output: $card_drift_out"
