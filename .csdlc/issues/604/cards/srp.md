# Structured Review Prompt

Template: 1.0.0

Issue: 604

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/604/full-cycle-defects-tail.md
.csdlc/evidence/604/issue-604-implementation-validation.log
.csdlc/evidence/604/pr610-state-request.json
.csdlc/evidence/604/publish-draft-request.json
.csdlc/evidence/604/publish-final-ready-request.json
.csdlc/evidence/604/publish-ready-update-request.json
.csdlc/evidence/604/ready-request.json
.csdlc/evidence/604/recover-after-ready-tail-gap.json
.csdlc/evidence/604/recover-after-resume-push-gap.json
.csdlc/evidence/604/review-assign-after-ready-tail-fix.json
.csdlc/evidence/604/review-assign-publication-valid.json
.csdlc/evidence/604/review-record-after-ready-tail-fix.json
.csdlc/evidence/604/review-record-publication-valid.json
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

- PR #610 had already been marked ready by the initial live canary run before the ready metadata-tail fix; the repaired ready/reconcile-ready path is now covered by focused tests and the existing PR publication path is reconciled through typed publish.
- The final GitHub CI result is external and may still be pending after the typed publication update.

## Review Result

Revision: Some("git-blake3:6c0d71d2fb74d851b23eb116b1cb8fce9e4ca303:952ad0da0207337e34c2ca750a27de6f6644f4be1e7b88b5d374f011660b487c")

Reviewer: Some("/root/review_604_resume_tail")

Result: pass
