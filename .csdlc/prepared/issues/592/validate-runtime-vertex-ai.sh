#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

design=.csdlc/prepared/issues/592/design.md
stp=.csdlc/issues/592/cards/stp.values.json
config=adl-runtime-kernel/src/config.rs
test_file=adl-runtime-kernel/tests/configuration.rs
test -s "$design"
test -s "$stp"
test -s "$config"
test -s "$test_file"
grep -q 'must not silently fall back' "$design"
grep -q 'production provider-selection path' "$design"
grep -q 'Mock provider acceptance evidence' "$stp"
grep -q 'PolisVertexAiInitConfig' "$config"
grep -q 'classify_vertex_ai_provider_failure' "$config"
grep -q 'polis_vertex_ai_configuration_is_explicit_and_redacted' "$test_file"
grep -q 'vertex_ai_failure_classification_distinguishes_operator_actions' "$test_file"

if rg -n --hidden '(private_key|access_token)[[:space:]]*[:=][[:space:]]*[^$<{[:space:]]+' \
  .csdlc/prepared/issues/592 .csdlc/issues/592 docs/runtime/VERTEX_AI_POLIS_CONFIGURATION.md infra/runtime-v3/runtime-init.toml; then
  echo 'possible credential material found in issue 592 records' >&2
  exit 1
fi

echo 'issue 592 Runtime Vertex AI execution contract: pass'
