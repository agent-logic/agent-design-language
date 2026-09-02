# Structured Output Record

Template: 1.0.0

Issue: 630

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented non-authoritative C-SDLC v3 finish, clean, and cutover command routes under the one csdlc binary while preserving v2 as live authority before #505.

## Artifacts

- csdlc-v3/src/commands/mod.rs
- csdlc-v3/src/commands/terminal.rs
- csdlc-v3/src/main.rs
- csdlc-v3/tests/command_manifest.rs
- csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
- docs/csdlc-v3/v3-command-manifest.json
- .csdlc/prepared/issues/630/validate-v3-h4-terminal-clean-cutover.sh
- .csdlc/evidence/630/630-terminal-clean-cutover-tests.log
- .csdlc/evidence/630/630-full-v3-regression.log
- .csdlc/evidence/630/630-issue-validator.log
- .csdlc/evidence/630/630-rustfmt.log
- .csdlc/evidence/630/630-diff-hygiene.log
- .csdlc/evidence/630/630-typed-issue-validation.log

## Execution

- Added a terminal command module for finish, clean, and cutover route planning.
- Wired csdlc finish, csdlc clean, and csdlc cutover to typed JSON request handling and non-authoritative machine-readable reports.
- Made public finish fail closed without authenticated typed adapter receipt and proved positive terminal closeout only through a sealed typed readback constructor.
- Made cleanup derive target authority from actual Git worktree registration and preserve absent, unregistered, dirty, live, already removed, removable, and removed outcomes.
- Made cutover produce a non-executing decision packet that requires #505 approval, binary provenance, rollback evidence, and fail-closed undo boundaries.
- Updated the v3 command manifest and command-manifest tests for the implemented non-authoritative terminal routes.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/630/validate-v3-h4-terminal-clean-cutover.sh"
    ],
    "purpose": "Prove #630 v3 finish, clean, and cutover route behavior, denial paths, command manifest truth, and no v2 source changes using the issue-owned validator.",
    "outcome": "passed",
    "evidence_ref": "issue-630-terminal-clean-cutover-validation.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
