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

- C-SDLC v3 remains non-authoritative; typed C-SDLC v2 remains the operational lifecycle authority.
- Issue #502 lifecycle kernel work has not been started and remains sequentially blocked on #501 publication/finish.

## Review Result

Revision: Some("git-blake3:7b7c8dd6f7c98ba2f1dcc9c00fab520074e4135a:9aed3fab194f6395b444be2827a6e62ed0c3405aa6da726b7f68b0147e1e3c83")

Reviewer: Some("issue_501_current_review")

Result: pass
