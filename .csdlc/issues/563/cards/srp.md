# Structured Review Prompt

Template: 1.0.0

Issue: 563

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/operator/owner-source-set.json
csdlc-v2/operator/skills.json
csdlc-v2/src/operator.rs
csdlc-v2/src/store.rs
csdlc-v2/src/lifecycle.rs
csdlc-v2/src/bin
csdlc-v2/tests/gate10a.rs
csdlc-v2/tests/primary_checkout_bootstrap_guard.rs

## Prompts

- Does every repository-mutating installed owner reach one shared read-only gate before locks or filesystem writes?
- Does owner-source freshness avoid whole-repository HEAD false positives while detecting actual C-SDLC drift?
- Can any partial or mixed installation become selected?
- Are primary, linked, isolated, and pre-existing-residue cases proven with exact before/after state?
- Are diagnostics portable, actionable, and free of credential or host-path disclosure?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final pre-merge integration gate.
- The exhaustive preservation proof runs only in a tiny synthetic fixture; production freshness uses two bounded Git queries and installed executable digests.

## Review Result

Revision: Some("git-blake3:a7e667e0b28526ea324f31438b13e5624e39b269:e6805f6e0bc2a128728eb84d5787d4fcc59879980b2beaf10a90e8d0109db450")

Reviewer: Some("fresh-session:d831dd0b-3c3b-4726-b58e-f5242b1364f6")

Result: pass
