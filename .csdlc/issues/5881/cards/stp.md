# Structured Task Prompt

Template: 1.0.0

Issue: 5881

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Remove residual active claim lifecycle authority and prove canonical branch/worktree ownership end to end.

## Deliverables

- Active-versus-legacy occurrence classification
- Claim-free canonical schemas and commands
- Verified one-time current-record normalization with claim-specific production logic deleted afterward
- Atomic, concurrent, and interrupted-bind recovery proof
- Claim-free review through cleanup proof
- Corrected current operator guidance

## Acceptance

1. AC-1: No active request or canonical schema contains claim, claim_id, protected_paths, lease, or heartbeat fields
2. AC-2: No operator command requires claim acquisition, refresh, recovery, release, revoke, rehome, or amend
3. AC-3: Existing current issue records normalize without manual claim repair while preserving branch/worktree topology and audit truth, and no claim-specific production decoder remains afterward
4. AC-4: Concurrent binds for one issue cannot produce two authoritative worktrees
5. AC-5: Different issues can bind and execute concurrently without path reservations
6. AC-6: Review, publication, finish, and cleanup pass from bound topology alone
7. AC-7: Focused Rust tests and owner validation pass
8. AC-8: Claim-specific active code, schemas, fixtures, docs, and binaries are deleted rather than wrapped
9. AC-9: An interrupted bind transaction recovers to one valid authoritative topology without claim state or partial records

## Dependencies

- #5861 established claim-free creation and binding
- #5895 settles retired installer authority first
- #5883 settles duplicate creation entrypoint second

## Inputs

- danielbaustin/agent-design-language#5881
- csdlc-v2/src/model.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/operator/skills
- docs/tooling/C_SDLC_V2_ISSUE_CREATION_AND_BINDING_RUNBOOK.md
- docs/tooling/adl_pr_cycle_skill.md

## Non Goals

- Remove issue-bound branches or worktrees
- Weaken review, validation, publication, finish, or cleanup
- Rewrite historical evidence
- Replace claims with another reservation ledger
- Broad C-SDLC redesign
