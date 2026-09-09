# Structured Review Prompt

Template: 1.0.0

Issue: 771

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/771/ci-integration-after.log
.csdlc/evidence/771/ci-integration-before.log
.csdlc/evidence/771/ci-integration-defect.md
.csdlc/evidence/771/ci-integration-disk-full-shared-suite.json
.csdlc/evidence/771/ci-integration-disk-full-shared-suite.log
.csdlc/evidence/771/ci-integration-disk-full-suite.json
.csdlc/evidence/771/ci-integration-disk-full-suite.log
.csdlc/evidence/771/ci-integration-lock-suite.json
.csdlc/evidence/771/ci-integration-lock-suite.log
.csdlc/evidence/771/ci-integration-primary-proof-suite.json
.csdlc/evidence/771/ci-integration-primary-proof-suite.log
.csdlc/evidence/771/clippy.log
.csdlc/evidence/771/current-mapping.log
.csdlc/evidence/771/execution-status.md
.csdlc/evidence/771/generate-template-set.py
.csdlc/evidence/771/mapping-contract.json
.csdlc/evidence/771/mapping-contract.log
.csdlc/evidence/771/mapping-negative.log
.csdlc/evidence/771/mapping-validator-review.json
.csdlc/evidence/771/pagination-defect.log
.csdlc/evidence/771/pagination-defect.md
.csdlc/evidence/771/pagination-defect.rs
.csdlc/evidence/771/pagination-repair.md
.csdlc/evidence/771/suite.json
.csdlc/evidence/771/suite.log
.csdlc/evidence/771/team-validator-fixed-review.json
.csdlc/evidence/771/team-validator-initial-review.json
.csdlc/evidence/771/template-authority-defect.md
.csdlc/evidence/771/template-schemas.log
.csdlc/evidence/771/terminal-conflict-defect.md
.csdlc/evidence/771/terminal-conflict-repro.log
.csdlc/evidence/771/terminal-conflict-repro.rs
.csdlc/prepared/issues/771/design.md
.csdlc/prepared/issues/771/diagram.mmd
csdlc-v3/src/adapters/mod.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/commands/remote/tests.rs
csdlc-v3/src/commands/terminal.rs
csdlc-v3/tests/local_commands.rs
csdlc-v3/tests/operational_cli_commands.rs
csdlc-v3/tests/real_issue_canary.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/README.md
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7c5b787341/assignment.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7c5b787341/historical-fixture.diff
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7c5b787341/mapping.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7c5b787341/review.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7c5b787341/review.md
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7c5b787341/reviews/core.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7c5b787341/reviews/evidence.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7c5b787341/reviews/routes.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7c5b787341/suite.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7c5b787341/suite.log
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7fecd63398/assignment.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7fecd63398/mapping.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7fecd63398/review.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7fecd63398/review.md
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7fecd63398/reviews/core.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7fecd63398/reviews/routes.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7fecd63398/suite.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/7fecd63398/suite.log
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/8a597a5cb9/assignment.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/8a597a5cb9/mapping.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/8a597a5cb9/review.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/8a597a5cb9/review.md
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/8a597a5cb9/reviews/core.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/8a597a5cb9/reviews/routes.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/8a597a5cb9/suite.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/8a597a5cb9/suite.log
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/fba2bdf5c7/assignment.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/fba2bdf5c7/historical-fixture.diff
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/fba2bdf5c7/mapping.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/fba2bdf5c7/review.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/fba2bdf5c7/review.md
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/fba2bdf5c7/suite.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/archive/fba2bdf5c7/suite.log
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/assignment.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/historical-fixture.diff
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/mapping.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/review.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/review.md
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/reviews/core.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/reviews/routes.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/suite.json
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/suite.log
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/test_validate.py
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/validate.py
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py
docs/templates/prompts/1.0.5/pvf_lane_policy.json
docs/templates/prompts/1.0.5/schemas/sip.structure.json
docs/templates/prompts/1.0.5/schemas/sor.structure.json
docs/templates/prompts/1.0.5/schemas/spp.structure.json
docs/templates/prompts/1.0.5/schemas/srp.structure.json
docs/templates/prompts/1.0.5/schemas/stp.structure.json
docs/templates/prompts/1.0.5/schemas/vpp.structure.json
docs/templates/prompts/1.0.5/sip.md
docs/templates/prompts/1.0.5/sor.md
docs/templates/prompts/1.0.5/spp.md
docs/templates/prompts/1.0.5/srp.md
docs/templates/prompts/1.0.5/stp.md
docs/templates/prompts/1.0.5/vpp.md
docs/templates/prompts/current.json

## Prompts

- Independently review complete current V3-F coupled scope and exact-head receipt validator.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Full205-test locked suite is parent-run clean-detached evidence at220dbade0eb30e252f901b14c72a27f03bdcc715; all162 source blobs match finalHEAD. This reviewer verified receipts and logs, not an additional full-suite execution.
- PR801 still carried earlier remote head8599ff8 and197-test body when read; Closes #771 and main base verified. Typed republication must update head/body and hosted CI remains the integration gate.
- Synthetic transport and topology-specific real-record denial tests do not prove live GitHub mutation or unrestricted rollback. No merge, issue closure or release is authorized by this review.
- Retained diagnostics include historical temporary fixture paths and failed receipts whose original log field names suite.log; paired renamed logs were verified by digest. These are diagnosis evidence, not portable execution inputs or current accepted suite receipts.

## Review Result

Revision: Some("git-blake3:795244a96c53e604ea9f5785430038ab30f442e3:74aa3286406c2d6f6cdb17a1cbcd155f2bbae51f17ec313a0f4d4a6e650ec11b")

Reviewer: Some("codex:/root/review_771_routes")

Result: pass
