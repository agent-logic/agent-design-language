# Structured Intent Prompt

Template: 1.0.0

Issue: 297

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide typed, evidence-preserving classification, recovery, archive, and exact cleanup for rollback-preserved issue projections.

## Required Outcome

Under the issue lock, typed operations can classify interrupted canonical/backup/preserved namespaces, restore one verified prior projection while preserving rejected evidence, resume idempotently after every rename/fsync boundary, and separately clean only an exact operation-owned archived inode without weakening ordinary commit or lifecycle authority.

## Scope

- typed classify, recover, and cleanup request/result contracts
- issue-store preserved projection namespace and immutable recovery ledger
- candidate identity, manifest, topology, CAS, collision, and symlink validation
- restart/idempotency/failpoint and ordinary-commit integration proof

## Authority

- Only typed owner operations under the existing issue lock may classify or mutate recovery namespaces
- Recovery never rewrites lifecycle phase/history and never deletes rejected evidence
- Cleanup is separate and requires exact completed-recovery, inode, manifest, CAS, and topology authority
- Issue #296 is frozen until #297 is terminal and ancestral
- Issues #291 and #294 are not mutated or absorbed

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 binaries only
- Bind under /Volumes/FastWork/adl-worktrees
- Keep main free of tracked/staged writes and quarantine root bootstrap staging after bind
- Serialize exclusive csdlc-v2/src/store.rs ownership while #296 is frozen
- Use canonical fresh-session UUID design and exact-head review evidence
- Publish ready with Closes #297 and stop before merge
