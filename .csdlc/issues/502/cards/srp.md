# Structured Review Prompt

Template: 1.0.0

Issue: 502

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/502/audit.jsonl
.csdlc/issues/502/cards/sip.md
.csdlc/issues/502/cards/sip.values.json
.csdlc/issues/502/cards/sor.md
.csdlc/issues/502/cards/sor.values.json
.csdlc/issues/502/cards/spp.md
.csdlc/issues/502/cards/spp.values.json
.csdlc/issues/502/cards/srp.md
.csdlc/issues/502/cards/srp.values.json
.csdlc/issues/502/cards/stp.md
.csdlc/issues/502/cards/stp.values.json
.csdlc/issues/502/cards/vpp.md
.csdlc/issues/502/cards/vpp.values.json
.csdlc/issues/502/index.json
.csdlc/prepared/issues/502/design.md
.csdlc/prepared/issues/502/diagram.mmd
csdlc-v3/AGENTS.md
csdlc-v3/src/adapters/mod.rs
csdlc-v3/src/lib.rs
csdlc-v3/src/lifecycle/mod.rs
csdlc-v3/src/storage/mod.rs
csdlc-v3/tests/transactions.rs

## Prompts

- Does every lifecycle command/state pair have an explicit capability-checked allowed or rejected outcome?
- Can any partial or interrupted write acquire authority?
- Does recovery replay preserve audit provenance and converge deterministically?
- Do typed adapters preserve argv/status/stdout/stderr/timeout/cancellation and credential-scope boundaries?
- Does csdlc-v3/AGENTS.md preserve the v2 authority boundary while making future v3 issue starts faster and simpler?
- Can any command or API surface be misread as C-SDLC v3 operational authority before cutover?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 lifecycle-kernel code remains construction-only; typed C-SDLC v2 remains the live lifecycle and GitHub authority until explicit V3-F cutover.
- Issue #502 is stacked on #501 and must not be merged before #501 is accepted.

## Review Result

Revision: Some("git-blake3:cde476b136e746172b7ac8a8c6cbdc8237e64f2d:01c2d20cabd1dbff7a028c1cff7e6b148fd49d41bfabf312010dc0df99b54c02")

Reviewer: Some("issue_502_restacked_review")

Result: pass
