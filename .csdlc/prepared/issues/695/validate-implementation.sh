#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"
tracked_before="$(git status --short --untracked-files=no)"

cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --all -- --check
cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_partial_checkpoint --lib
cargo test --manifest-path adl-runtime-kernel/Cargo.toml \
  agent_partial_checkpoint_coordinator_tracks_roster_cycles_and_restart_restore --lib
cargo test --manifest-path adl-runtime-kernel/Cargo.toml \
  agent_lifecycle::archived_restore_monotonically_merges_newer_completed_turns --lib
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test agent_roster
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract
node demos/html-observatory/tests/agent_continuity.test.mjs
infra/aws/runtime/agent-checkpoint-archive/validate.sh
git diff --check
bash .csdlc/prepared/issues/695/validate-acceptance.sh \
  .csdlc/prepared/issues/695/acceptance-manifest.json \
  .csdlc/evidence/695/acceptance-results.json
git diff --check
tracked_after="$(git status --short --untracked-files=no)"
test "$tracked_after" = "$tracked_before"
