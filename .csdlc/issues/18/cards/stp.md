# Structured Task Prompt

Template: 1.0.0

Issue: 18

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove clean EPIPE handling for the shared output path used by the split GitHub issue and PR binaries.

## Deliverables

- Shared JSON stdout writer with explicit BrokenPipe handling
- Split issue and PR binary adoption
- Process-level early-reader-close regression tests
- Updated stdout/stderr contract documentation

## Acceptance

1. Schema output piped to an early-closing reader exits without a panic
2. No backtrace hint, broken-pipe panic, or human panic text contaminates machine-readable output
3. Focused regression coverage exercises the shared output path used by both split C-SDLC GitHub binaries

## Dependencies

- Current canonical Agent Logic main
- C-SDLC v2 split GitHub binaries

## Inputs

- AGENTS.md
- csdlc-v2/src/bin/csdlc-github-issue.rs
- csdlc-v2/src/bin/csdlc-github-pr.rs
- csdlc-v2/tests/gate_github_actions.rs
- docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md

## Non Goals

- Changing GitHub API behavior or schemas
- Changing stderr observability policy
- Refactoring unrelated binaries
- AWS, hosted validation, or CI policy changes
