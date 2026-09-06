# Structured Output Record

Template: 1.0.0

Issue: 708

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the Runtime agent orientation resource path: the Axioma Polis welcome package is loaded as a typed versioned resource, stamped per admitted agent with blake3 delivery provenance, injected ahead of model-facing task content, retained per existing agent across orientation reloads, exposed through the Runtime roster/read model, and rendered by the Observatory as non-authoritative orientation metadata.

## Artifacts

- adl-runtime-kernel/src/agent_orientation.rs
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/resident_shepherd.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/control.rs
- demos/html-observatory/app.js
- demos/html-observatory/tests/agent_orientation.test.mjs

## Execution

- Added a first-class Runtime agent-orientation resource with schema, version, source path, deterministic full projection, blake3 digest over exact injected bytes, and validation that rejects disabled, malformed, unreadable, or non-welcome-package content.
- Made the default canonical welcome-package path load the bundled trusted package unconditionally, so process cwd cannot shadow the default orientation source; explicit custom source paths continue to load and validate from disk.
- Routed resident Shepherd preload through the same per-agent orientation injection path so the READY preload prompt cannot be the first model-facing call without the welcome package.
- Stamped resident and dynamically admitted agents with per-agent orientation delivery provenance and retained exact delivered resources so existing agents keep their original package while new admissions receive the current valid package.
- Initialized startup residents from the configured Runtime orientation resource before dynamic admissions load, and updated config reload so valid reloads change only the active future-admission package while invalid reloads fail closed.
- Prepended the retained per-agent welcome package before model-facing shepherd and runtime-agent conversation task content without treating the package as authority or capability.
- Routed the governed resident Shepherd recovery/readiness probe through the same retained per-agent welcome package before the READY instruction, closing the remaining production path that could reach a model without orientation.
- Raised the resident Shepherd oriented request decode limit so Runtime-generated welcome-package prompts are accepted without weakening the original user-message bound.
- Moved provider-backed agent_runtime orientation delivery into a first-class internal task envelope so the original 4 KiB user-message bound remains intact while the model-facing prompt still receives the welcome package before task content.
- Raised bounded Runtime agent-conversation single-message and per-part payloads from 4 KiB to 32 KiB, while retaining a finite 64-part and 256 KiB aggregate cap.
- Added first-class multipart `input_parts` and `message_parts` handling across Runtime conversation tasks, provider-selected A2A tool actions, and public output projection so larger governed resident handoffs do not flatten into an undersized single string.
- Tightened multipart validation so an optional scalar `input` or `message` counts as one logical part against the shared 64-part cap, preventing scalar-plus-64 requests from slipping past the declared bound.
- Kept provider-selected A2A `message` as the summary/single-message field and `message_parts` as the separate multipart chunks so runtime-derived dispatch does not duplicate multipart task content when forwarding to the recipient.
- Updated integration-test fixtures to carry explicit no-orientation state for pre-existing samples so the public roster/feed structures compile while production admissions continue to stamp real per-agent orientation delivery.
- Exposed orientation delivery metadata through Runtime roster evidence/read-model entries and rendered it in the Observatory selected-agent details as non-authoritative provenance.

## Validation

[
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--bin",
      "adl-runtime-kernel",
      "resident_shepherd_governed_probe_prompt_includes_orientation_before_ready",
      "--",
      "--nocapture"
    ],
    "purpose": "Regression for the production resident Shepherd governed READY probe so each model-facing resident agent request receives the welcome package before the readiness task.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/708/runtime-orientation-shepherd-governed-probe.log"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "shepherd",
      "resident_shepherd_model_health_gates_inference_and_recovers",
      "--",
      "--nocapture"
    ],
    "purpose": "Regression for the resident Shepherd orientation-aware request decode limit and health-gated recovery path.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/708/runtime-orientation-shepherd-governed-probe.log"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "control::layer8_conversation_ingress_tests::agent_to_agent_model_action_from_conversation_delivers_peer_response",
      "--",
      "--nocapture"
    ],
    "purpose": "Regression for provider-backed A2A delivery after moving the orientation package out of the 4 KiB user-input field and into a separate internal task envelope.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/708/runtime-orientation-a2a-envelope.log"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "agent_to_agent_model_action_from_conversation_delivers_peer_response"
    ],
    "purpose": "Regression for runtime-derived provider A2A multipart forwarding: the recipient prompt receives orientation before task content, and each multipart task chunk appears exactly once after provider tool-call normalization.",
    "outcome": "passed",
    "evidence_ref": "terminal:running 1 test; 1 passed"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "provider_conversation_action_tests",
      "--",
      "--nocapture"
    ],
    "purpose": "Regression for 32 KiB single agent conversation inputs, multipart Runtime input assembly, and provider-projected multipart governed A2A envelopes.",
    "outcome": "passed",
    "evidence_ref": "terminal:running 7 tests; 7 passed"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "logical_message_parts",
      "--",
      "--nocapture"
    ],
    "purpose": "Regression for the current-head review finding: scalar message plus 64 multipart entries must fail because the scalar counts as one logical part.",
    "outcome": "passed",
    "evidence_ref": "terminal:running 2 tests; 2 passed"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "conversation_task_input_accepts_32k_single_part_and_multipart_input",
      "--",
      "--nocapture"
    ],
    "purpose": "Regression for Runtime task input assembly: scalar input and multipart input_parts share the same 64 logical-part cap while 32 KiB accepted parts remain valid.",
    "outcome": "passed",
    "evidence_ref": "terminal:running 1 test; 1 passed"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "provider_conversation_tool_tests",
      "--",
      "--nocapture"
    ],
    "purpose": "Regression for native provider A2A tool-call normalization accepting 32 KiB multipart message parts and rejecting over-limit parts.",
    "outcome": "passed",
    "evidence_ref": "terminal:running 7 tests; 7 passed"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "agent_initiation",
      "--",
      "--nocapture"
    ],
    "purpose": "Regression for public output projection preserving multipart A2A initiation actions and rejecting over-limit multipart chunks while existing governed initiation behavior still passes.",
    "outcome": "passed",
    "evidence_ref": "terminal:running 8 tests; 8 passed"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "orientation_tests",
      "--",
      "--nocapture"
    ],
    "purpose": "Regression for integration-test compilation after adding required orientation fields to public roster/feed structs.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/708/runtime-orientation-integration-compile.log"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "orientation"
    ],
    "purpose": "Issue 708 Runtime orientation contract validation: delivered package ordering, per-agent provenance, startup initialization, reload retention, future-admission update, fail-closed invalid reload, explicit custom package loading, cwd-shadow rejection for the default bundled package, and resident Shepherd preload orientation before READY probe.",
    "outcome": "passed",
    "evidence_ref": "terminal:running 7 tests; 7 passed"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--bins"
    ],
    "purpose": "Issue 708 Runtime binary compile validation after startup orientation wiring.",
    "outcome": "passed",
    "evidence_ref": "terminal:cargo check --bins finished"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Full adl-runtime-kernel regression validation after orienting provider-backed A2A envelopes and the governed resident Shepherd READY probe.",
    "outcome": "passed",
    "evidence_ref": "terminal:all adl-runtime-kernel unit, integration, and doc tests passed"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--bins",
      "--tests",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict lint validation for the touched Runtime library, binary, and test surfaces.",
    "outcome": "passed",
    "evidence_ref": "terminal:cargo clippy --lib --bins --tests -D warnings finished"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "node",
      "demos/html-observatory/tests/agent_orientation.test.mjs"
    ],
    "purpose": "Issue 708 Observatory orientation display and normalization validation.",
    "outcome": "passed",
    "evidence_ref": "terminal:2 tests passed"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "bash",
      ".csdlc/prepared/issues/708/validate-orientation-plan.sh"
    ],
    "purpose": "Issue 708 prepared planning contract validation.",
    "outcome": "passed",
    "evidence_ref": "terminal:validate-orientation-plan.sh exited 0"
  },
  {
    "command": [
      "git",
      "diff",
      "--exit-code",
      "origin/main...HEAD",
      "--",
      "docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md"
    ],
    "purpose": "Issue 708 welcome-package source immutability validation.",
    "outcome": "passed",
    "evidence_ref": "terminal:no diff for source welcome package"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Issue 708 diff hygiene validation.",
    "outcome": "passed",
    "evidence_ref": "terminal:git diff --check exited 0"
  },
  {
    "command": [
      "env",
      "TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-708-runtime-agent-orientation-resource/.tmp",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib"
    ],
    "purpose": "CI-matching Runtime v3 library validation after replacing the sleep-timed duplicate conversation in-flight assertion with the existing deterministic test barrier.",
    "outcome": "passed",
    "evidence_ref": "terminal:183 tests passed; formerly failing shepherd_conversation_invokes_configured_provider_and_preserves_canonical_wss_ingress passed"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
