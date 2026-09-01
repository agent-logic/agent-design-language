# Structured Review Prompt

Template: 1.0.0

Issue: 604

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/604/issue-604-implementation-validation.log
.csdlc/issues/604/index.json
.csdlc/issues/604/audit.jsonl
.csdlc/issues/604/cards/sip.md
.csdlc/issues/604/cards/sip.values.json
.csdlc/issues/604/cards/stp.md
.csdlc/issues/604/cards/stp.values.json
.csdlc/issues/604/cards/spp.md
.csdlc/issues/604/cards/spp.values.json
.csdlc/issues/604/cards/vpp.md
.csdlc/issues/604/cards/vpp.values.json
.csdlc/issues/604/cards/srp.md
.csdlc/issues/604/cards/srp.values.json
.csdlc/issues/604/cards/sor.md
.csdlc/issues/604/cards/sor.values.json
.csdlc/prepared/issues/604/design.md
.csdlc/prepared/issues/604/diagram.mmd
.csdlc/prepared/issues/604/finalize-request.json
.csdlc/prepared/issues/604/full-cycle-defects.md
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

Revision: Some("git-blake3:637860d61fc8c1461a8395ecf2baef88da3b4371:8345f4c4cc0d9f7dcc0b87626e9b735cb909213143cf529e146dd06ba1e50a6c")

Reviewer: Some("/root/review_604_pre_pr_repaired")

Result: pass
