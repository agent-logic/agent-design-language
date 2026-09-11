#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

ruby -rjson -e '
  packet = JSON.parse(File.read(ARGV.fetch(0)))
  abort "wrong operation" unless packet["operation"] == "pull_request_ready"
  abort "missing two-attempt reproduction" unless packet.fetch("attempts").length == 2
  abort "fixture must retain draft readback" unless packet.dig("authenticated_readback", "isDraft") == true
  serialized = JSON.generate(packet).downcase
  abort "secret-shaped field retained" if serialized.match?(/(token|password|authorization)["_]*\s*:/)
' .csdlc/prepared/issues/824/814-pull-request-ready-reconciliation.json

if rg -n 'ready_for_review' csdlc-v3/src; then
  echo "deprecated REST ready route remains" >&2
  exit 1
fi

cargo test --manifest-path csdlc-v3/Cargo.toml -p csdlc-v3 \
  commands::remote::tests::pull_request_ready_
cargo test --manifest-path csdlc-v3/Cargo.toml -p csdlc-v3 \
  adapters::tests::github_operational_adapter_supports_only_the_typed_ready_graphql_mutation -- --exact
cargo test --manifest-path csdlc-v3/Cargo.toml -p csdlc-v3 --test operational_cli_commands \
  executable_github_pr_ready_uses_graphql_and_authenticated_readback -- --exact

echo "issue #824 ready reconciliation proof: PASS"
