# Structured Intent Prompt

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement the v3 GitHub, PR-state, review, and publication command routes under the single csdlc binary.

## Required Outcome

The v3 one-binary GitHub/publication routes expose typed, authenticated, redacted, non-authoritative behavior and prove closing-linkage readback without v2 operational fallback.

## Scope

- csdlc-v3/src/main.rs
- csdlc-v3/src/commands/remote
- csdlc-v3/src/publication
- csdlc-v3/src/review
- csdlc-v3/tests/command_manifest.rs
- csdlc-v3/tests/real_issue_canary.rs
- docs/csdlc-v3/v3-command-manifest.json
- .csdlc/prepared/issues/629/**
- .csdlc/issues/629/**

## Authority

- C-SDLC v2 remains live operational authority until explicit #505 cutover.
- Issue #629 may implement v3 GitHub/publication route behavior only.
- Issue #629 must not merge, finish, clean, install, retire v2, or close #505.
- No raw gh lifecycle writes and no hidden v2 operational fallback are allowed.

## Assumptions

- none

## Operator Constraints

- Do not use raw gh.
- Do not use /private/tmp; keep temporary and durable artifacts inside the repo or FastWork worktrees.
- Keep #505 open.
- Capture every defect for #632/cutover readiness.
