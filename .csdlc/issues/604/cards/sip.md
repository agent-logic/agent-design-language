# Structured Intent Prompt

Template: 1.0.0

Issue: 604

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Restore typed C-SDLC v2 publication commands that can safely mark governed draft PRs ready and reconcile uncertain ready mutations.

## Required Outcome

The authoritative csdlc-publish owner supports exact-identity ready and reconcile-ready operations with durable lifecycle truth and focused tests.

## Scope

- csdlc-v2/src/publication.rs
- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/github.rs
- csdlc-v2/tests/**
- csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md
- csdlc-v2/operator/skills.json
- .csdlc/prepared/issues/604/**
- .csdlc/issues/604/**

## Authority

- C-SDLC v2 remains the sole live lifecycle authority until explicit #505 cutover.
- csdlc-publish owns publication and publication-ready reconciliation for this issue.
- The implementation must not use raw gh or ChatGPT GitHub connector writes.
- Ready publication truth is recorded only after exact live PR readback proves the expected remote state.
- This issue does not merge, finish, clean, or close any PR.

## Assumptions

- none

## Operator Constraints

- Use the canonical FastWork issue worktree.
- Keep the primary checkout clean.
- Do not use raw gh.
- Publish with Closes #604 after implementation, validation, and review pass.
- Capture any full-cycle tooling defects for later resolution.
