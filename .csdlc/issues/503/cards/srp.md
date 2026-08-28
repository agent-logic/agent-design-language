# Structured Review Prompt

Template: 1.0.0

Issue: 503

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/Cargo.toml
csdlc-v3/Cargo.lock
csdlc-v3/src/lib.rs
csdlc-v3/src/bin/csdlc-v3-local.rs
csdlc-v3/src/commands/mod.rs
csdlc-v3/src/commands/local/mod.rs
csdlc-v3/tests/local_commands.rs
.csdlc/issues/503
.csdlc/evidence/503

## Prompts

- Do local preparation commands consume typed contracts and emit typed outputs without becoming live lifecycle authority?
- Does bind modeling require registered topology and reject branch-name-only authorization?
- Do cards render from the active prompt-template registry without hand edits?
- Does the doctor/PVF planning proof distinguish ready, blocked, failed, deferred, skipped, and passed outcomes?
- Does the issue-start simplification reduce ceremony while preserving typed v2 authority, validation, and review gates?
- Can any #503 surface be misread as C-SDLC v3 operational authority before V3-F?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final integration gate before merge.
- Issue #504 remote delivery, #570 docs/skills readiness, #571 V3-A corrective proof, and #505 authority cutover remain separate downstream gates.
- Publication must visibly include `Closes #503`; V3-D itself remains read-only and non-authoritative.

## Review Result

Revision: Some("git-blake3:a0df81ae7f6cb190ca27cb63d42c3a9e8dd60c8e:e6ce2f325d57dc2c0c52a8e7975cd5849a25093adb1a7e63c707463595ae3ad2")

Reviewer: Some("fresh-session:issue-503-evidence-review")

Result: pass
