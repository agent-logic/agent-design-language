# Structured Review Prompt

Template: 1.0.0

Issue: 604

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/604/issue-604-implementation-validation.log
.csdlc/prepared/issues/604/design.md
.csdlc/prepared/issues/604/diagram.mmd
.csdlc/prepared/issues/604/finalize-request.json
.csdlc/prepared/issues/604/full-cycle-defects.md
.csdlc/prepared/issues/604/publish-draft-request.json
.csdlc/prepared/issues/604/recover-stale-review-after-publication-prep.json
.csdlc/prepared/issues/604/review-assign-repaired.json
.csdlc/prepared/issues/604/review-record-repaired.json
.csdlc/prepared/issues/604/validate-implementation.sh
csdlc-v2/operator/skills.json
csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md
csdlc-v2/src/bin/csdlc-publish.rs
csdlc-v2/src/lib.rs
csdlc-v2/src/operator.rs
csdlc-v2/src/publication.rs
csdlc-v2/src/schema.rs
csdlc-v2/tests/gate10a.rs
csdlc-v2/tests/publication_ready.rs

## Prompts

- Does csdlc-publish ready verify exact live PR identity before and after mutation?
- Does reconcile-ready recover only from independently observed remote truth?
- Do stale generation/digest, wrong PR/head/repository, closed PR, and non-draft pre-state fail before lifecycle mutation?
- Are the publication skill and operator inventory aligned with the implemented command surface?
- Does the PR body include Closes #604 only after implementation, validation, and review truth are current?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live GitHub draft-to-ready mutation and remote readback remain to be exercised by the publication step for this canary issue.

## Review Result

Revision: Some("git-blake3:0b7dfd217754cf36f38eeee510c30649098765c6:d5a72e24983ebb99df24d311e8636d73c9467e315179708226210c24584319cd")

Reviewer: Some("/root/review_604_pre_pr_repaired")

Result: pass
