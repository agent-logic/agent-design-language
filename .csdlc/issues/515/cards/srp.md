# Structured Review Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/provider/mod.rs
adl/tests/provider_shadow_isolation.rs
adl/tests/provider_shadow_comparison.rs
adl/tests/provider_shadow_fallback.rs
docs/milestones/v0.92.1/evidence/provider/prov-b/local-model-shadow-comparison.json
.csdlc/prepared/issues/515

## Prompts

- Can any shadow result mutate or replace the authoritative result?
- Are authority and shadow paths represented distinctly enough for reviewers and validators?
- Are comparison inputs and rules exact and deterministic?
- Do shadow failures preserve authoritative outputs and state?
- Does evidence redact credentials, private payloads, prompts, and host-local paths?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Read-only final review did not rerun Cargo validation; implementation session reran the focused local validation set and recorded it in SOR.

## Review Result

Revision: Some("git-blake3:03acd1e223009e0ebf8f012e3b478ee5e27ead1c:9a42de2ac8c2ed8fe289d74c88f051c12ec52a69971838c00a4e9293de44448e")

Reviewer: Some("fresh-session:9a0d288c-d5ab-4529-9b5c-9c7706136ada")

Result: pass
