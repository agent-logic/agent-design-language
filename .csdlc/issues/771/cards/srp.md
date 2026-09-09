# Structured Review Prompt

Template: 1.0.0

Issue: 771

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

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

- The 197-test locked suite is retained parent-run clean-detached evidence at 7fecd63398bb37530801dbf3375d47f73426d637; this review verified unchanged 162 source blobs and suite receipt/log consistency, and did not rerun the full Rust suite.
- Pagination regressions use synthetic transport; no live GitHub mutation, cloud, post-transfer rollback, cross-platform execution, merge, closure or release authority is established by this review.
- git diff --check reports only trailing blank lines in four retained raw suite logs; no product whitespace defect was found.

## Review Result

Revision: Some("git-blake3:ad65b8700dcd6086a0669804123b858c4d7dfb15:0b81ce2ffb92591bdfec9af6c5e4b68bd45a9bfbbddb590b2d7e7420b27f37dd")

Reviewer: Some("codex:/root/review_771_routes")

Result: pass
