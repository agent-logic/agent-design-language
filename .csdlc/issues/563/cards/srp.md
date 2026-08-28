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
.csdlc/issues/563
.csdlc/prepared/issues/563

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

Revision: Some("git-blake3:8248c0578cba18793d9a31ad02676485c24057e5:b895660cf27b8f75b090f6c13ffcc2edfa8d8050d6b8862d20e64e9bcedc805c")

Reviewer: Some("fresh-session:c82f0fa1-fc70-4ae2-9d08-2c0be680907b")

Result: pass
