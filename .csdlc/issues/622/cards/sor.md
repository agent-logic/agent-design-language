# Structured Output Record

Template: 1.0.0

Issue: 622

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Addressed PR #646 exact-head review blockers by making provider/profile reload state run-scoped for execution and adding overlap/shutdown-order regression coverage.

## Artifacts

- adl/src/provider/reload.rs
- adl/src/provider/mod.rs
- adl/src/provider/local.rs
- adl/src/execute/runner.rs
- adl/src/execute/tests.rs
- adl-runtime-kernel/src/config_reload.rs
- .csdlc/prepared/issues/622/validate-provider-profile-hotload.sh
- docs/providers/provider-profile-hot-loading.md
- adl/src/long_lived_agent.rs
- adl/src/long_lived_agent/tests.rs
- adl/src/provider/reload.rs
- adl/src/execute/tests.rs
- .csdlc/prepared/issues/622/validate-provider-profile-hotload.sh
- docs/providers/provider-profile-hot-loading.md
- adl-runtime-kernel/src/control.rs
- adl/src/cli/csmctl_cmd.rs
- adl/src/execute/mod.rs
- adl/src/execute/runner.rs
- adl/src/execute/tests.rs
- adl/src/long_lived_agent.rs
- adl/src/provider/reload.rs

## Execution

- Added a provider reload owner and global handle in adl/src/provider/reload.rs for provider-only sidecar loading, validation, diagnostics, stable provider digests, shutdown, and last-known-good preservation.
- Wired the production execution runner to resolve provider specs from the current reload snapshot for each step while preserving the in-flight step's start snapshot.
- Added deterministic mock-provider configuration hooks for fixed output and bounded sleep to prove reload behavior without external provider calls.
- Added focused provider reload, production runner, and runtime-kernel watcher tests covering valid reloads, invalid reload retention, credential-value rejection, duplicate debounce, shutdown, and in-flight snapshot isolation.
- Updated the #622 validation script to run focused production and safety lanes that avoid unrelated binary compile surfaces.
- Documented provider profile hot-loading behavior, accepted sidecar shape, credential boundary, snapshot semantics, validation evidence, and non-goals.
- Added workflow.run_args.provider_reload_sidecar_path support to the CSM adl_workflow production cycle so it starts ProviderReloadOwner, installs the global handle for the execution lifetime, records reload status, and shuts the owner down after execution.
- Extended provider sidecar credential rejection to cover password, client_secret, private_key, access/refresh tokens, and credential-shaped raw scalar values under neutral containers such as auth.value while retaining env-reference fields.
- Tracked provider-level reload generations in ProviderReloadSnapshot and diagnostics instead of hard-coding zero.
- Added a long-lived-agent production tick test proving the real CSM ADL workflow entrypoint consumes sidecar provider output through the reload owner.
- Updated #622 validation lanes and documentation to name the production config knob and stronger credential boundary.
- Ran rustfmt across the ADL workspace and accepted the single formatting change in adl-runtime-kernel/src/control.rs required by the hosted fmt check.
- Fixed the csmctl command import to use the public adl_runtime_kernel::agent_roster::is_canonical_agent_name path instead of the private control re-export surface.
- Added execute_sequential_with_provider_reload_handle so production execution can consume an explicit run-scoped ProviderReloadHandle instead of process-global reload state.
- Threaded the run-scoped reload handle through sequential execution, concurrent execution, step retry execution, and called-workflow recursion while preserving the existing global fallback for compatibility callers.
- Changed CSM adl_workflow production wiring to store the ProviderReloadOwner handle on ActiveProviderReload and pass it directly into execution, removing CSM reliance on global provider reload registration.
- Made the compatibility global provider reload guard identity-aware so dropping an older guard cannot clear a newer registration.
- Added a two-workflow overlap and shutdown-order regression test proving same-provider-id workflows keep separate reload snapshots and shutting down one workflow does not clear the other's handle.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/622/validate-provider-profile-hotload.sh",
      "production"
    ],
    "purpose": "Prove the production provider/profile hot-loading lane, including existing provider profile invariants, accepted reloads without restart, invalid reload last-known-good retention, credential-value rejection, and in-flight step snapshot isolation.",
    "outcome": "passed",
    "evidence_ref": "terminal:issue-622-production-lane:provider_mod_profile 14 passed; provider_reload 4 passed; in-flight snapshot 1 passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/622/validate-provider-profile-hotload.sh",
      "safety"
    ],
    "purpose": "Prove the safety lane for reload debounce, shutdown, invalid-update last-known-good retention, credential boundary, focused production invariants, and whitespace/diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "terminal:issue-622-safety-lane:config_reload 2 passed; provider_mod_profile 14 passed; provider_reload 4 passed; in-flight snapshot 1 passed; git diff --check passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/622/validate-provider-profile-hotload.sh",
      "production"
    ],
    "purpose": "Prove the refreshed production lane after exact-head review fixes, including real CSM adl_workflow reload startup through workflow.run_args.provider_reload_sidecar_path.",
    "outcome": "passed",
    "evidence_ref": "terminal:issue-622-post-review-production-lane:provider_mod_profile 14 passed; provider_reload 5 passed; in-flight snapshot 1 passed; tick_adl_workflow_starts_hotload_owner_from_run_args 1 passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/622/validate-provider-profile-hotload.sh",
      "safety"
    ],
    "purpose": "Prove the refreshed safety lane after exact-head review fixes, including runtime-kernel debounce/shutdown, stronger credential rejection, generation truth, production tick proof, and diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "terminal:issue-622-post-review-safety-lane:config_reload 2 passed; provider_mod_profile 14 passed; provider_reload 5 passed; in-flight snapshot 1 passed; tick_adl_workflow_starts_hotload_owner_from_run_args 1 passed; git diff --check passed"
  },
  {
    "command": [
      "cargo",
      "fmt --all -- --check && cargo clippy --all-targets -- -D warnings"
    ],
    "purpose": "Prove the local equivalent of the failed hosted adl-rust-fmt-clippy lane after bounded CI janitor fixes.",
    "outcome": "passed",
    "evidence_ref": "terminal:issue-622-ci-janitor-local-fmt-clippy: cargo fmt --all -- --check passed; cargo clippy --all-targets -- -D warnings finished successfully in adl/"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/622/validate-provider-profile-hotload.sh",
      "production"
    ],
    "purpose": "Prove the #622 production provider/profile hot-loading lane after replacing process-global production wiring with a run-scoped reload handle.",
    "outcome": "passed",
    "evidence_ref": "terminal:issue-622-process-global-repair-production-lane:provider_mod_profile 14 passed; provider_reload 6 passed including overlap/shutdown regression; in-flight snapshot 1 passed; tick_adl_workflow_starts_hotload_owner_from_run_args 1 passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/622/validate-provider-profile-hotload.sh",
      "safety"
    ],
    "purpose": "Prove the #622 safety lane after run-scoped reload repair, including runtime-kernel reload safety, credential boundary, production tick proof, and whitespace/diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "terminal:issue-622-process-global-repair-safety-lane:config_reload 2 passed; provider_mod_profile 14 passed; provider_reload 6 passed including overlap/shutdown regression; in-flight snapshot 1 passed; tick_adl_workflow_starts_hotload_owner_from_run_args 1 passed"
  },
  {
    "command": [
      "cargo",
      "fmt --manifest-path adl/Cargo.toml --all -- --check && cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings"
    ],
    "purpose": "Prove formatting and clippy cleanliness for the #622 process-global reload repair before fresh exact-head review.",
    "outcome": "passed",
    "evidence_ref": "terminal:issue-622-process-global-repair-local-fmt-clippy: cargo fmt --manifest-path adl/Cargo.toml --all -- --check passed; cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings finished successfully"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
