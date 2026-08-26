# Design: #331 initialized code-repository declaration for legacy issue authority

## Problem

Initialized legacy-authority C-SDLC records can be otherwise preparation-ready but still fail doctor because the issue repository remains `danielbaustin/agent-design-language` while the active code origin is `agent-logic/agent-design-language` and no explicit `code_repository` is recorded.

The existing `csdlc-issue migrate-code-repository` route is the correct owner family, but today it rejects initialized records with `invalid_transition`, forcing #5837/#5838 to remain blocked or tempting hand edits.

## Approach

Extend the typed code-repository migration authority to support initialized, unbound, nonpublished records when the operation is strictly repository-identity declaration:

- retain issue repository identity;
- set `record.code_repository` to the canonical code repository;
- require exact generation/digest CAS, actor, reason, issue, source issue repository, and target code repository;
- authorize the request by comparing `source_repository` exactly against `record.repository`;
- reject if a supplied source repository differs from the record, if the target code repository is malformed, if the target repository is not the effective GitHub origin, or if same-number canonical issue ambiguity is not explicitly represented in typed evidence;
- keep branch/worktree null;
- preserve audit history, cards, design/diagram bytes, publication, review, terminal, and readiness truth except for normal generation/digest/audit advancement;
- reuse existing origin/repository authorization checks where they are meaningful for the current checkout.

## Typed contract details

The repair must make the initialized path explicit rather than treating it as a hidden variation of the bound path.

Request contract:

- `schema`
- `issue`
- `expected_generation`
- `expected_digest`
- `actor`
- `reason`
- `source_repository`
- `code_repository`
- `canonical_issue_collision_evidence_ref`
- `canonical_issue_collision_evidence_digest`

Initialized authorization:

- load the record by issue number and fail closed unless phase is `initialized`;
- require `branch == null` and `worktree == null`;
- require `publication`, `terminal`, and active implementation/review/publication truth to remain absent;
- require `source_repository == record.repository`;
- require the requested `code_repository` to match the effective GitHub origin repository;
- reject when `record.code_repository` is already present;
- require `canonical_issue_collision_evidence_ref` to point to a typed JSON evidence file whose SHA-256 matches `canonical_issue_collision_evidence_digest`;
- reject same-number canonical ambiguity unless the collision evidence declares one of the allowed dispositions:
  - `same_number_absent`: typed live read proves the canonical repository has no issue with the same number;
  - `same_number_non_authoritative`: typed live read proves the canonical same-number issue exists but is unrelated/non-authoritative for this legacy issue, and legacy issue authority remains with `source_repository`;
  - `same_number_successor`: typed live read proves the canonical same-number issue is the reviewed successor, and the request is explicitly not attempting issue migration;
- reject if the collision evidence issue number, source repository, code repository, observed URL, digest, or disposition does not match the request and record.

Result/evidence contract:

- preserve the existing bound/implemented/reviewed report shape for existing callers;
- introduce `csdlc.initialized_code_repository_migration_report.v1` with an evidence payload `csdlc.initialized_code_repository_migration_evidence.v1`;
- keep the existing `csdlc.code_repository_migration_report.v1` and `csdlc.code_repository_migration_evidence.v1` unchanged for bound/implemented/reviewed callers;
- for the initialized report/evidence, make `branch` and `worktree` explicit null fields with a discriminator `topology_state: "initialized_unbound"`;
- include previous and resulting generation/digest, issue number, source issue repository, target code repository, previous code repository, resulting phase, branch/worktree nullness, collision evidence ref/digest/disposition, and the explicit cross-repository authority disposition;
- require decoders/consumers to branch on the report schema; they must not parse initialized null topology through the legacy non-null report shape;
- append audit evidence without rewriting cards, design/diagram files, readiness, publication, review, or terminal state.

## Owned paths

- `csdlc-v2/src/migration.rs`
- `csdlc-v2/src/store.rs`
- `csdlc-v2/src/doctor.rs` only if needed for diagnosis/readiness compatibility
- `csdlc-v2/src/bin/csdlc-issue.rs` only if CLI/schema routing needs an explicit initialized mode
- `csdlc-v2/tests/code_repository_migration.rs`
- focused doctor/gate2 regressions if needed
- `.csdlc/evidence/331`

## Non-goals

- GitHub issue transfer or bulk repository migration
- binding, implementation, publication, merge, finish, or cleanup for #5837/#5838
- hand-editing lifecycle state
- changing publication/finish cross-repository semantics
- credential or provider/runtime proof

## Validation plan

Run focused code-repository migration tests, relevant doctor/gate2 repository-identity tests, strict C-SDLC v2 Clippy/fmt, then exact-head review before publication.

The doctor/readiness proof must not use a Cargo filter that can pass with zero tests. Add explicit named regressions in `csdlc-v2/tests/code_repository_migration.rs`, including:

- `initialized_code_repository_migration_requires_digest_bound_collision_evidence`
- `initialized_code_repository_migration_emits_v1_initialized_unbound_evidence`
- `initialized_code_repository_migration_clears_doctor_and_validate_issue`
- `bound_code_repository_migration_report_schema_remains_unchanged`
