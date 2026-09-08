# Structured Task Prompt

Template: 1.0.0

Issue: 721

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement the #721 v3 parity and reporting defects only; record adjacent v3 defects as backlog issues.

## Deliverables

- v3 IssueCreate mutation support
- authenticated issue-create reconciliation and receipt of the assigned issue number
- truthful terminal authority reporting tests and implementation
- operator documentation showing the real v3 issue-create path
- backlog issue records for adjacent defects

## Acceptance

1. AC-1: csdlc-v3 finish --observe-github reports operational authority truthfully after cutover when it persists terminal state.
2. AC-2: Pre-cutover denial remains fail-closed and non-mutating.
3. AC-3: v3 GitHub issue-create supports title, body, labels, assignees, and milestone with structured argv, credential-child injection, operation marker, reconciliation, and idempotent replay.
4. AC-4: The issue-create receipt records GitHub's assigned issue number.
5. AC-5: Documentation distinguishes local issue-state preparation from real GitHub issue creation and shows a non-fake operational command.
6. AC-6: Adjacent v3 defects discovered during execution are recorded as backlog issues.

## Dependencies

- #505 v3 cutover authority state on main
- #723 proof/shadow/install test defect
- #724 simple gh-style issue-create UX follow-up
- #725 native v3 authority after v2 removal follow-up

## Inputs

- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/commands/terminal.rs
- csdlc-v3/src/adapters/mod.rs
- csdlc-v3/src/main.rs
- csdlc-v3/README.md
- csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
- csdlc-v3/src/commands/remote/tests.rs

## Non Goals

- Do not remove v2 code.
- Do not implement the simplified gh-style front-end command; that is #724.
- Do not fix proof/shadow/install validation failures; that is #723.
- Do not replace v2 authority dependencies after cutover; that is #725.
