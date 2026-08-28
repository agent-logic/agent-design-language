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
csdlc-v2/tests/gate2.rs
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
- The full local suite reached an unrelated shared-git derived-terminal cache mismatch for issue 298; a clean hosted checkout does not inherit that mutable cache.

## Review Result

Revision: Some("git-blake3:09fd782985f4e5483f96ee8015ec54e59454332d:65f0c1290a292b2ef58b7d481d4fa28e8593b2be77e6780fd39492831a641b5a")

Reviewer: Some("vertex:gemini-3.1-pro-preview")

Result: pass
