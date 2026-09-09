# Structured Review Prompt

Template: 1.0.0

Issue: 518

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/518/audit-cargo-manifests.py
.csdlc/prepared/issues/518/refresh-handoff-content.py
.csdlc/prepared/issues/518/validate-documentation-handoff.rb
AGENTS.md
CONTRIBUTING.md
README.md
REVIEW.md
adl/CONTRIBUTING.md
adl/README.md
adl/tools/README.md
csdlc-v2/AGENTS.md
csdlc-v2/README.md
csdlc-v3/AGENTS.md
csdlc-v3/README.md
docs/README.md
docs/architecture/ADL_ARCHITECTURE.md
docs/codex_playbook.md
docs/default_workflow.md
docs/milestones/v0.92.1/CANONICAL_DOC_INVENTORY_v0.92.1.md
docs/milestones/v0.92.1/DECISIONS_v0.92.1.md
docs/milestones/v0.92.1/DEMO_MATRIX_v0.92.1.md
docs/milestones/v0.92.1/DESIGN_v0.92.1.md
docs/milestones/v0.92.1/DISTRIBUTED_TEST_PLAN_CONSULTATION.md
docs/milestones/v0.92.1/FEATURE_PROOF_COVERAGE_v0.92.1.md
docs/milestones/v0.92.1/MILESTONE_CHECKLIST_v0.92.1.md
docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md
docs/milestones/v0.92.1/QUALITY_GATE_v0.92.1.md
docs/milestones/v0.92.1/README.md
docs/milestones/v0.92.1/RELEASE_NOTES_v0.92.1.md
docs/milestones/v0.92.1/SPRINT_v0.92.1.md
docs/milestones/v0.92.1/WBS_v0.92.1.md
docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/accounting-closeout.md
docs/milestones/v0.92.1/evidence/release/tail-02/CARGO_MANIFEST_REVIEW.md
docs/milestones/v0.92.1/evidence/release/tail-02/README.md
docs/milestones/v0.92.1/evidence/release/tail-02/cargo-independent-review.json
docs/milestones/v0.92.1/evidence/release/tail-02/cargo-manifest-audit.json
docs/milestones/v0.92.1/evidence/release/tail-02/creation-map.json
docs/milestones/v0.92.1/evidence/release/tail-02/dependency-observation.json
docs/milestones/v0.92.1/evidence/release/tail-02/document-inventory.json
docs/milestones/v0.92.1/evidence/release/tail-02/final-validation.json
docs/milestones/v0.92.1/evidence/release/tail-02/finding-dispositions.json
docs/milestones/v0.92.1/evidence/release/tail-02/handoff-content.json
docs/milestones/v0.92.1/evidence/release/tail-02/independent-review.json
docs/milestones/v0.92.1/evidence/release/tail-02/local-validation.json
docs/milestones/v0.92.1/evidence/release/tail-02/review-addendum.json
docs/milestones/v0.92.1/evidence/release/tail-02/source-proof-snapshot.json
docs/milestones/v0.92.1/features/OBSERVATORY_REDESIGN_v0.92.1.md
docs/onboarding.md
docs/planning/ADL_FEATURE_LIST.md
docs/planning/codefriend/README.md
docs/tooling/README.md
docs/tooling/editor/README.md
csdlc-v2/src/registry.rs
csdlc-v2/tests/gate9.rs
csdlc-v2/tests/gate10a.rs

## Prompts

- Is the canonical document denominator complete?
- Can every path, link, issue reference, and claim be resolved context-free?
- Are residual risks and non-claims explicit?
- Is the packet bound to one exact candidate revision?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Independent review approves #754 integration; hosted CI pending on new published head.
- Accounting completion does not grant release approval; operator merge hold remains.

## Review Result

Revision: Some("git-blake3:e3b10595fbe720afef70f4506965c922514a662f:2caf33498b6e24c33f287ff2e6ba06a16a5ec1319933cde02dfb415b15e1f958")

Reviewer: Some("codex:root/review_518_docs")

Result: pass
