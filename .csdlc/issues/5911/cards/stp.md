# Structured Task Prompt

Template: 1.0.0

Issue: 5911

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement issue 5911 only; archive and verify but do not delete source transcripts or existing worktrees.

## Deliverables

- typed bind enforcement for the canonical FastWork worktree parent
- focused positive and negative tests
- aligned operator policy documentation
- FastWork transcript archive manifest and verification result
- exact deletion proposal requiring separate approval

## Acceptance

1. material local transcript consumers are identified with byte counts
2. FastWork archive is complete with a machine-readable checksum manifest
3. source and archive checksums verify before any deletion proposal
4. every newly bound ADL issue worktree resolves beneath /Volumes/FastWork/adl-worktrees
5. typed binding refuses a worktree outside the mandatory FastWork parent
6. repo policy and binding guidance name the same canonical parent
7. no transcript or existing worktree is deleted

## Dependencies

- none

## Inputs

- .csdlc/prepared/issues/5911/design.md
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate2.rs
- AGENTS.md

## Non Goals

- product-code changes unrelated to binding
- automatic relocation or deletion of existing worktrees
- transcript deletion
- use or inspection of /private/tmp
