#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

design=.csdlc/prepared/issues/592/design.md
doc=docs/runtime/VERTEX_AI_POLIS_CONFIGURATION.md
runtime_init=infra/runtime-v3/runtime-init.toml
agent=infra/runtime-v3/agents/ember.axioma.yaml
test -s "$design"
test -s "$doc"
test -s "$runtime_init"
test -s "$agent"
grep -q 'GCP project' "$design"
grep -q 'Vertex location' "$design"
grep -q 'secret JSON' "$design"
grep -q 'paid Vertex AI request' "$design"
grep -q 'project/location mismatch' "$design"
grep -q 'provider = "vertex_ai"' "$runtime_init"
grep -q 'gcp_project = "agent-logic-dev"' "$runtime_init"
grep -q 'vertex_location = "us-central1"' "$runtime_init"
grep -q 'kind = "application_default_credentials"' "$runtime_init"
grep -q 'kind: vertex_ai' "$agent"
grep -q 'gemini-2.5-flash' "$agent"
grep -q 'Secret JSON' "$doc"
grep -q 'Application Default Credentials' "$doc"
grep -q 'request remains deferred' "$doc"
grep -q 'project_location_mismatch' "$doc"

echo 'issue 592 Vertex configuration documentation contract: pass'
