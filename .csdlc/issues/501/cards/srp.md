# Structured Review Prompt

Template: 1.0.0

Issue: 501

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/501/audit.jsonl
.csdlc/issues/501/cards/sip.md
.csdlc/issues/501/cards/sip.values.json
.csdlc/issues/501/cards/sor.md
.csdlc/issues/501/cards/sor.values.json
.csdlc/issues/501/cards/spp.md
.csdlc/issues/501/cards/spp.values.json
.csdlc/issues/501/cards/srp.md
.csdlc/issues/501/cards/srp.values.json
.csdlc/issues/501/cards/stp.md
.csdlc/issues/501/cards/stp.values.json
.csdlc/issues/501/cards/vpp.md
.csdlc/issues/501/cards/vpp.values.json
.csdlc/issues/501/index.json
.csdlc/prepared/issues/501/design.md
.csdlc/prepared/issues/501/diagram.mmd
csdlc-v3/Cargo.lock
csdlc-v3/Cargo.toml
csdlc-v3/src/application/mod.rs
csdlc-v3/src/bin/csdlc-v3-foundation.rs
csdlc-v3/src/lib.rs
csdlc-v3/src/repository/mod.rs
csdlc-v3/tests/foundation.rs

## Prompts

- Is every repository-context dependency explicit data rather than hidden process state?
- Does repeated projection replay produce byte-stable output?
- Do tests cover retained requirements #164 through #167 without fabricating lifecycle authority?
- Can any command or API surface be misread as C-SDLC v3 operational authority before cutover?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The v3 foundation importer remains read-only and non-authoritative; full v2 lifecycle enum/schema enforcement and authority cutover remain later v3 slices.

## Review Result

Revision: Some("git-blake3:c7439484b8b783fdc1d36e8480d8178e848dbdc8:49aaa4af3abb66686c9f5582e067415e387183509407077e948f96d88836cec3")

Reviewer: Some("issue_501_import_fix_review")

Result: pass
