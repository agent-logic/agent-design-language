#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-.}"
agents="${repo_root}/AGENTS.md"
coordination="${repo_root}/docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

require_literal() {
  local file="$1"
  local literal="$2"
  grep -Fq -- "$literal" "$file" || fail "missing required policy text in ${file#${repo_root}/}: ${literal}"
}

is_uint() {
  [[ "$1" =~ ^[1-9][0-9]*$ ]]
}

is_repo() {
  local owner
  local name
  local extra
  IFS=/ read -r owner name extra <<<"$1"
  [[ -z "$extra" && "$owner" =~ ^[A-Za-z0-9][A-Za-z0-9_.-]*$ \
    && "$name" =~ ^[A-Za-z0-9][A-Za-z0-9_.-]*$ \
    && "$owner" != '.' && "$owner" != '..' \
    && "$name" != '.' && "$name" != '..' ]]
}

is_branch() {
  [[ "$1" =~ ^[A-Za-z0-9][A-Za-z0-9._/-]*$ \
    && "$1" != *'..'* && "$1" != *'//'* ]] \
    && git check-ref-format --branch "$1" >/dev/null 2>&1
}

is_body_path() {
  local path="$1"
  local invocation
  local mode
  local owner
  [[ "$path" =~ ^\.git/csdlc-v2/break-glass/([A-Za-z0-9][A-Za-z0-9_.-]*)/body\.md$ ]] || return 1
  invocation="${BASH_REMATCH[1]}"
  [[ "$invocation" != '.' && "$invocation" != '..' ]] || return 1
  [[ -f "$path" && ! -L "$path" ]] || return 1
  mode="$(stat -f '%Lp' "$path" 2>/dev/null || true)"
  if [[ ! "$mode" =~ ^[0-7]+$ ]]; then
    mode="$(stat -c '%a' "$path" 2>/dev/null || true)"
  fi
  owner="$(stat -f '%u' "$path" 2>/dev/null || true)"
  if [[ ! "$owner" =~ ^[0-9]+$ ]]; then
    owner="$(stat -c '%u' "$path" 2>/dev/null || true)"
  fi
  [[ "$mode" == 600 && "$owner" == "$(id -u)" ]]
}

allowed_argv() {
  (($# >= 2)) || return 1
  [[ "$1" == gh ]] || return 1
  case "$2" in
    issue)
      case "${3:-}" in
        comment|edit)
          (($# == 8)) || return 1
          is_uint "$4" && [[ "$5" == --repo ]] && is_repo "$6" \
            && [[ "$7" == --body-file ]] && is_body_path "$8"
          ;;
        *) return 1 ;;
      esac
      ;;
    pr)
      case "${3:-}" in
        create)
          (($# == 13 || $# == 14)) || return 1
          [[ "$4" == --repo ]] && is_repo "$5" \
            && [[ "$6" == --base ]] && is_branch "$7" \
            && [[ "$8" == --head ]] && is_branch "$9" \
            && [[ "${10}" == --title ]] && [[ -n "${11}" && "${11}" != -* ]] \
            && [[ "${12}" == --body-file ]] && is_body_path "${13}" \
            && { (($# == 13)) || [[ "${14}" == --draft ]]; }
          ;;
        edit|comment)
          (($# == 8)) || return 1
          is_uint "$4" && [[ "$5" == --repo ]] && is_repo "$6" \
            && [[ "$7" == --body-file ]] && is_body_path "$8"
          ;;
        ready)
          (($# == 6)) || return 1
          is_uint "$4" && [[ "$5" == --repo ]] && is_repo "$6"
          ;;
        *) return 1 ;;
      esac
      ;;
    *) return 1 ;;
  esac
}

expect_allowed() {
  allowed_argv "$@" || fail "expected allowed argv rejected: $*"
}

expect_denied() {
  if allowed_argv "$@"; then
    fail "expected denied argv accepted: $*"
  fi
}

for file in "$agents" "$coordination"; do
  [[ -f "$file" ]] || fail "missing policy target: ${file#${repo_root}/}"
done

require_literal "$agents" 'Typed C-SDLC v2 remains the default'
require_literal "$agents" 'confirmed, reproducible tooling regression'
require_literal "$agents" 'durable tooling-regression issue'
require_literal "$agents" 'explicit operator authorization'
require_literal "$agents" 'exact repository, issue or pull request, and operation'
require_literal "$agents" 'canonical argv shapes'
require_literal "$agents" 'merge, issue close, finish, cleanup, deletion, release, administration, secret or variable mutation, workflow mutation or dispatch, force operation, and bulk mutation'
require_literal "$agents" 'append-only local break-glass receipt'
require_literal "$agents" '.git/csdlc-v2/break-glass/'
require_literal "$agents" 'typed reconciliation'
require_literal "$agents" 'must not proceed'

require_literal "$coordination" 'Break-glass receipt protocol'
require_literal "$coordination" 'intent.json'
require_literal "$coordination" 'result.json'
require_literal "$coordination" 'reconciliation.json'
require_literal "$coordination" 'must never be overwritten'
require_literal "$coordination" 'typed_generation'
require_literal "$coordination" 'typed_digest'
require_literal "$coordination" 'remote_pre_state'
require_literal "$coordination" 'authorization_reference'
require_literal "$coordination" 'redacted_argv'
require_literal "$coordination" 'no credentials, token values, token-file contents, environment dumps, sensitive request bodies, or raw response bodies'
require_literal "$coordination" 'reconciliation_status'
require_literal "$coordination" 'freeze'

if grep -Eiq 'raw `?gh`? (is|remains) (a )?(general|routine|default) fallback' "$agents" "$coordination"; then
  fail 'policy text appears to authorize a general raw-gh fallback'
fi

fixture_parent="${repo_root}/.csdlc/evidence/418"
mkdir -p "$fixture_parent"
fixture_root="$(mktemp -d "${fixture_parent}/policy-fixtures.XXXXXX")"
trap 'rm -rf "$fixture_root"' EXIT
mkdir -p "$fixture_root/.git/csdlc-v2/break-glass"
for invocation in invoke-1 invoke-2 invoke-3 invoke-4 invoke-5 invoke-6 unsafe-mode symlink-target; do
  mkdir -p "$fixture_root/.git/csdlc-v2/break-glass/$invocation"
  install -m 600 /dev/null "$fixture_root/.git/csdlc-v2/break-glass/$invocation/body.md"
done
chmod 0644 "$fixture_root/.git/csdlc-v2/break-glass/unsafe-mode/body.md"
mkdir -p "$fixture_root/.git/csdlc-v2/break-glass/unsafe-link"
ln -s ../symlink-target/body.md "$fixture_root/.git/csdlc-v2/break-glass/unsafe-link/body.md"
cd "$fixture_root"

# Positive fixtures: every exact allowed canonical shape with owned mode-0600 files.
expect_allowed gh issue comment 418 --repo agent-logic/agent-design-language --body-file .git/csdlc-v2/break-glass/invoke-1/body.md
expect_allowed gh issue edit 418 --repo agent-logic/agent-design-language --body-file .git/csdlc-v2/break-glass/invoke-2/body.md
expect_allowed gh pr create --repo agent-logic/agent-design-language --base main --head codex/418-policy --title policy --body-file .git/csdlc-v2/break-glass/invoke-3/body.md
expect_allowed gh pr create --repo agent-logic/agent-design-language --base main --head codex/418-policy --title policy --body-file .git/csdlc-v2/break-glass/invoke-4/body.md --draft
expect_allowed gh pr edit 419 --repo agent-logic/agent-design-language --body-file .git/csdlc-v2/break-glass/invoke-5/body.md
expect_allowed gh pr ready 419 --repo agent-logic/agent-design-language
expect_allowed gh pr comment 419 --repo agent-logic/agent-design-language --body-file .git/csdlc-v2/break-glass/invoke-6/body.md

# Negative fixtures: missing identity/authorization surfaces and every denied family.
expect_denied gh issue create --repo agent-logic/agent-design-language --title defect --body-file .git/csdlc-v2/break-glass/x/body.md
expect_denied gh issue edit 418 --repo agent-logic/agent-design-language --state closed
expect_denied gh issue edit 418 --repo agent-logic/agent-design-language --add-label bug
expect_denied gh issue comment 418 --body-file .git/csdlc-v2/break-glass/x/body.md
expect_denied gh issue comment 418 --repo ../agent-design-language --body-file .git/csdlc-v2/break-glass/invoke-1/body.md
expect_denied gh issue comment 418 --repo agent-logic/.. --body-file .git/csdlc-v2/break-glass/invoke-1/body.md
expect_denied gh issue comment all --repo agent-logic/agent-design-language --body-file .git/csdlc-v2/break-glass/x/body.md
expect_denied gh issue comment 418 --repo agent-logic/agent-design-language --body secret
expect_denied gh issue comment 418 --repo agent-logic/agent-design-language --body-file /tmp/body.md
expect_denied gh issue comment 418 --repo agent-logic/agent-design-language --body-file ../body.md
expect_denied gh issue comment 418 --repo agent-logic/agent-design-language --body-file .git/csdlc-v2/break-glass/../body.md
expect_denied gh issue comment 418 --repo agent-logic/agent-design-language --body-file .git/csdlc-v2/break-glass/./body.md
expect_denied gh issue comment 418 --repo agent-logic/agent-design-language --body-file .git/csdlc-v2/break-glass/unsafe-mode/body.md
expect_denied gh issue comment 418 --repo agent-logic/agent-design-language --body-file .git/csdlc-v2/break-glass/unsafe-link/body.md
expect_denied gh pr edit 419 --repo agent-logic/agent-design-language --base other
expect_denied gh pr edit 419 --repo agent-logic/agent-design-language --add-reviewer user
expect_denied gh pr create --repo agent-logic/agent-design-language --base main --head '$(unsafe)' --title policy --body-file .git/csdlc-v2/break-glass/invoke-3/body.md
expect_denied gh pr merge 419 --repo agent-logic/agent-design-language
expect_denied gh pr close 419 --repo agent-logic/agent-design-language
expect_denied gh release create v0.92
expect_denied gh workflow run ci.yml
expect_denied gh secret set TOKEN
expect_denied gh api repos/agent-logic/agent-design-language/issues/418
expect_denied gh alias set bypass 'pr merge'
expect_denied gh extension exec anything
expect_denied gh pr ready 419
expect_denied gh pr ready branch-name --repo agent-logic/agent-design-language
expect_denied gh pr create --repo agent-logic/agent-design-language --base main --head codex/418-policy --title policy --body-file .git/csdlc-v2/break-glass/x/body.md --draft --reviewer user

printf 'PASS: typed gh break-glass policy contract (text_guards=27 argv_positive=7 argv_negative=27)\n'
