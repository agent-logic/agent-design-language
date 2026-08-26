# Structured Task Prompt

Template: 1.0.0

Issue: 331

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One lifecycle-tool defect: allow initialized/unbound records with preserved legacy issue authority to declare canonical code repository through typed audited recovery.

## Deliverables

- Typed initialized code-repository migration/recovery support
- Regression covering #5837/#5838 style initialized legacy issue authority
- Doctor/readiness proof that repository_identity_drift clears after typed operation
- Backwards-compatible bound/implemented/reviewed migration behavior
- Fresh exact-head review and ready PR
- Terminal finish if hosted gates pass

## Acceptance

1. AC-1: A typed request can set code_repository = agent-logic/agent-design-language for an initialized issue whose issue repository remains danielbaustin/agent-design-language.
2. AC-2: The route requires exact issue, expected generation, expected digest, actor, reason, source issue repository, target code repository, canonical issue collision evidence ref, and collision evidence digest, and fails closed unless source issue repository exactly equals the existing record repository.
3. AC-3: The route fails closed on stale generation/digest, malformed repository identity, existing code_repository, source-repository mismatch, missing or digest-mismatched collision evidence, unsupported same-number canonical collision disposition, non-null branch/worktree, publication/merge/closeout mutation, or missing reason.
4. AC-4: The route appends auditable initialized/unbound lifecycle evidence using csdlc.initialized_code_repository_migration_report.v1 / csdlc.initialized_code_repository_migration_evidence.v1, increments generation/digest, permits explicit null branch/worktree only under topology_state initialized_unbound, and does not rewrite prior audit entries, design/diagram bytes, unrelated cards, readiness, review, terminal, or publication state.
5. AC-5: After the route plus valid design review, csdlc-doctor and csdlc-validate issue can pass for initialized/unbound preparation records that are otherwise ready, proven by a specifically named nonzero regression.
6. AC-6: Bound/implemented/reviewed migration behavior and the existing csdlc.code_repository_migration_report.v1 / evidence.v1 shapes remain backward compatible.
7. AC-7: Regression fixtures cover #5837/#5838 style initialized legacy issue authority with canonical code repository and no bind.

## Dependencies

- #330 root projection must clear before #331 bootstrap
- #331 live issue created by typed operation v092-create-initialized-code-repository-recovery-defect-20260813
- #5837 gen42 and #5838 gen35 are reproduction consumers only

## Inputs

- agent-logic/agent-design-language#331
- .git/csdlc-v2/requests/5837-migrate-code-repository-current-20260813.error.json
- .git/csdlc-v2/requests/5838-migrate-code-repository-current-20260813.error.json
- csdlc-v2/src/migration.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/doctor.rs
- csdlc-v2/tests/code_repository_migration.rs

## Non Goals

- GitHub issue transfer or bulk repository migration
- Mutating #5837/#5838 product or design semantics
- Binding, publishing, merging, or finishing #5837/#5838
- Changing cross-repository publication/finish semantics
- Credential, provider, Runtime, or Unity live proof
