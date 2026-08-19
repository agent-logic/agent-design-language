# Structured Output Record

Template: 1.0.0

Issue: 32

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Added a typed csdlc-github runner-preflight operation that reads GitHub runner, runner-group, repository, workflow-restriction, selected-workflow, and optional canary-job state without mutation. It reports capacity, policy eligibility, stale workflow references, and live dispatchability separately so a Ready runner is never treated as dispatch proof.

## Artifacts

- .csdlc/prepared/issues/32/design.md
- .csdlc/prepared/issues/32/diagram.mmd
- .csdlc/prepared/issues/32/live-preflight.json

## Execution

- csdlc-v2/src/runner_preflight.rs
- csdlc-v2/src/bin/csdlc-github.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/schema.rs
- docs/tooling/GITHUB_LARGER_RUNNER_PREFLIGHT.md
- docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "runner_preflight"
    ],
    "purpose": "Prove capacity, policy, dispatchability, queue-timeout, stale-reference, and branch-independent runner-group classifications.",
    "outcome": "passed",
    "evidence_ref": "Seven matching focused tests passed, including the runner-preflight schema discoverability test."
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "schema::tests"
    ],
    "purpose": "Prove the runner-preflight request and diagnostic packet remain discoverable in the public C-SDLC schema bundle.",
    "outcome": "passed",
    "evidence_ref": "All three focused schema-bundle tests passed."
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Prove the new operation preserves the existing C-SDLC GitHub client boundary and source guardrails.",
    "outcome": "passed",
    "evidence_ref": "All three focused GitHub Actions gate tests passed."
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "--bin",
      "csdlc-github",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings in the library and owning GitHub binary.",
    "outcome": "passed",
    "evidence_ref": "Strict Clippy completed successfully for csdlc-v2 and csdlc-github."
  },
  {
    "command": [
      "cargo",
      "run",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bin",
      "csdlc-github",
      "--",
      "runner-preflight",
      "--request",
      ".csdlc/prepared/issues/32/live-preflight.json"
    ],
    "purpose": "Read live organization runner and group configuration, then require an assigned expected-label canary before declaring dispatch eligible.",
    "outcome": "deferred",
    "evidence_ref": "Live read-only preflight observed runner adl-ubuntu-24.04-16core Ready with max_count 10; group 3 selected to agent-logic/agent-design-language with workflow restriction disabled and no selected workflow refs. It correctly returned configuration_eligible_dispatch_unproven (exit 2). Terminal dispatch proof requires the published issue #32 PR canary job id."
  },
  {
    "command": [
      "cargo",
      "run",
      "--quiet",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bin",
      "csdlc-github",
      "--",
      "runner-preflight",
      "--request",
      ".csdlc/prepared/issues/32/live-canary-preflight.json"
    ],
    "purpose": "Prove branch-independent repository policy and terminal dispatch on the exact expected run, workflow, head, label, and runner group.",
    "outcome": "passed",
    "evidence_ref": "Run 31236518300 attempt 5 job 93054382571 completed success at 2026-08-08T04:00:53Z on head 3558f41b2395e9cb80f2804ba09f68914e9690ec; csdlc-github reported capacity=ready, policy=eligible, dispatchability=proven, label adl-ubuntu-24.04-16core, group adl-build-experiment, runner adl-ubuntu-24.04-16core-1000001117."
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "runner_preflight"
    ],
    "purpose": "Prove exact run/workflow/head/group binding, terminal-unassigned classification, pagination helpers, capacity classification, and stale-ref authorization uncertainty.",
    "outcome": "passed",
    "evidence_ref": "Eleven matching focused tests passed after review remediation."
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_runner_preflight"
    ],
    "purpose": "Exercise the real csdlc-github binary against a loopback GitHub API with paginated runner and repository results, secret-bearing request input, JSON stdout, and non-eligible exit semantics.",
    "outcome": "passed",
    "evidence_ref": "The loopback integration test passed; it required page 2 for both lists, found the target runner/repository there, returned diagnostic exit 2, emitted valid JSON, and did not expose the token or token path."
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "runner_preflight"
    ],
    "purpose": "Prove that a mismatched canary cannot pass and its diagnostic names every enforced identity dimension.",
    "outcome": "passed",
    "evidence_ref": "Eleven focused tests passed; stale_job_context_cannot_prove_dispatch asserts the diagnostic covers run, workflow, head, PR, label, and runner group."
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
