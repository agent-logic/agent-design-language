# Structured Intent Prompt

Template: 1.0.0

Issue: 5881

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete the deletion of claims as a C-SDLC v2 lifecycle concept while preserving one atomic issue-to-branch/worktree binding.

## Required Outcome

No active schema, command, gate, skill, or current operator document requires claim state; current claim-bearing legacy-format records normalize into canonical topology, historical evidence remains immutable, and the full lifecycle works from branch/worktree binding alone.

## Scope

- csdlc-v2/src/model.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin
- csdlc-v2/operator
- csdlc-v2/tests
- docs/tooling
- AGENTS.md

## Authority

- Issue authority remains danielbaustin/agent-design-language#5881
- Code PR publication targets agent-logic/agent-design-language
- PR body must use Closes danielbaustin/agent-design-language#5881
- This is split issue/code publication authority, not repository cutover or issue migration
- Historical evidence remains immutable
- Claim-specific decoding is temporary only for verified current-record normalization and must be deleted before completion

## Assumptions

- none

## Operator Constraints

- No AWS
- No wrappers, retries, or renamed claim abstraction
- No broad workspace suite
- Execute after #5895 and #5883 and rebase shared operator surfaces
- Prefer evidence over replacement code for already-satisfied acceptance
