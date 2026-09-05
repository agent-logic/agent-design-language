# Structured Output Record

Template: 1.0.0

Issue: 689

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Replaced retired shell-controller guidance with a production operator runbook for the canonical Runtime v3 service command, made legacy Runtime shell verbs fail with repository-anchored replacement guidance, preserved Observatory-only commands, and added focused non-mutating route guards.

## Artifacts

- CSMctl
- adl/tools/test_csmctl_linux_backend.sh
- docs/tooling/START_CSM_RUNBOOK.md
- .csdlc/evidence/689/runtime-control-routing-docs.log
- .csdlc/evidence/689/canonical-runtime-ownership.log
- .csdlc/evidence/689/exact-range-diff.log

## Execution

- Documented the sole installed-generation Runtime control route, durable identity, preflight, state interpretation, convergent operations, transactional reload recovery, incident evidence, and post-operation checks.
- Rejected every retired CSMctl Runtime verb and empty invocation before configuration loading or external service commands while retaining the local Observatory interface.
- Anchored replacement guidance to the script repository root so invocation from another working directory remains correct.
- Added a focused guard for shell syntax, legacy refusal, alternate-working-directory routing, absence of service mutation, Observatory separation, and canonical runbook claims.
- Corrected the validation plan after proving that its original Rust selector ran zero tests.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_csmctl_linux_backend.sh"
    ],
    "purpose": "Prove shell syntax, legacy Runtime refusal before external mutation, repository-anchored replacement guidance, and Observatory separation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/689/runtime-control-routing-docs.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "csm_runtime_v3_cmd::tests::"
    ],
    "purpose": "Run the actual canonical Runtime service ownership, readiness, convergence, service-manager, and transactional reload tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/689/canonical-runtime-ownership.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Reject whitespace and conflict-marker defects across the exact issue range.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/689/exact-range-diff.log"
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
