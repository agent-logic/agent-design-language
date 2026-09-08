#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-.}"
cd "$repo_root"

test -f csdlc-v3/src/main.rs
test -f csdlc-v3/src/commands/remote/mod.rs
test -f csdlc-v3/tests/remote_publication_commands.rs
test -f csdlc-v3/tests/operational_cli_commands.rs

cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands
cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands
cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest

if rg -n 'Command::new\("gh"\)|raw gh|gh issue create' csdlc-v3/src; then
  echo "forbidden raw-gh issue creation route detected" >&2
  exit 1
fi

git diff --check
echo "issue 724 focused validation passed"
