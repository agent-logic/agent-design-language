#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
ROOT_DIR="$(cd "$ROOT_DIR/.." && pwd)"

cd "$ROOT_DIR"

fail() {
  echo "validate-v3-h4-terminal-clean-cutover: $*" >&2
  exit 1
}

[[ -f csdlc-v3/src/commands/terminal.rs ]] || fail "missing terminal route module"
[[ -f csdlc-v3/tests/terminal_cleanup_cutover_commands.rs ]] || fail "missing focused test target"

if git diff --name-only origin/main...HEAD -- csdlc-v2 | grep -q .; then
  fail "csdlc-v2 source changed in #630 scope"
fi

grep -Fq 'authenticated_adapter_required' csdlc-v3/src/commands/terminal.rs || fail "finish must fail closed without authenticated adapter"
grep -Fq 'VerifiedTerminalReadback' csdlc-v3/src/commands/terminal.rs || fail "missing sealed terminal readback type"
grep -Fq 'pub(crate) fn from_typed_adapter_receipt' csdlc-v3/src/commands/terminal.rs || fail "verified terminal readback constructor must not be public"
grep -Fq '.arg("worktree")' csdlc-v3/src/commands/terminal.rs || fail "cleanup must derive registration from git worktree list"
grep -Fq 'AlreadyRemoved' csdlc-v3/src/commands/terminal.rs || fail "cleanup already-removed state missing"
grep -Fq 'Unregistered' csdlc-v3/src/commands/terminal.rs || fail "cleanup unregistered state missing"
grep -Fq 'Dirty' csdlc-v3/src/commands/terminal.rs || fail "cleanup dirty state missing"
grep -Fq 'Live' csdlc-v3/src/commands/terminal.rs || fail "cleanup live state missing"
grep -Fq 'Absent' csdlc-v3/src/commands/terminal.rs || fail "cleanup absent state missing"
grep -Fq 'RemovalDeniedPreCutover' csdlc-v3/src/commands/terminal.rs || fail "cleanup removal must be denied before cutover"
if grep -Fq 'remove_dir_all(&candidate)' csdlc-v3/src/commands/terminal.rs; then
  fail "v3 clean must not remove registered worktrees before #505 cutover"
fi
grep -Fq 'executes_cutover: false' csdlc-v3/src/commands/terminal.rs || fail "cutover route must not execute cutover"
grep -Fq '#505' csdlc-v3/src/commands/terminal.rs || fail "cutover approval must cite #505"
grep -Fq 'terminal_verified_readback_denies_stale_nonmerged_and_open_issue' csdlc-v3/src/commands/terminal.rs || fail "missing stale/nonmerged/open issue finish denial tests"
grep -Fq 'cleanup_denies_symlink_escape_from_approved_parent' csdlc-v3/tests/terminal_cleanup_cutover_commands.rs || fail "missing symlink escape cleanup denial test"
grep -Fq 'cleanup_denies_nonexistent_parent_traversal_escape' csdlc-v3/tests/terminal_cleanup_cutover_commands.rs || fail "missing nonexistent traversal cleanup denial test"
grep -Fq 'cleanup_denies_nonexistent_child_under_symlink_escape' csdlc-v3/tests/terminal_cleanup_cutover_commands.rs || fail "missing nonexistent child under symlink cleanup denial test"

cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands
cargo test --manifest-path csdlc-v3/Cargo.toml --lib terminal_verified_readback

echo "validate-v3-h4-terminal-clean-cutover: passed"
