# Structured Review Prompt

Template: 1.0.0

Issue: 5828

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5828
.csdlc/prepared/issues/5828/design.md
.csdlc/prepared/issues/5828/produce-native-receipt.rb
.csdlc/prepared/issues/5828/validate-native-receipts.rb
.csdlc/prepared/issues/5828/validate-obsmem-trace-integration.rb
.csdlc/evidence/5828
.github/workflows/wp11-native-memory-palace.yml
adl-runtime-kernel/src/memory_palace.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/memory_palace.rs
adl-runtime-kernel/tests/fixtures/memory_palace
docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md

## Prompts

- Does every loaded item bind valid identity, continuity, provenance, temporal anchor, hash, and redaction evidence?
- Are selection and overflow bounded and deterministic for fixed inputs and observation time?
- Do stale/hash/continuity mismatch, unauthorized state, private paths, and host paths fail closed?
- Are #5826, #5827, ObsMem/trace proof, schema compatibility, and every acceptance claim current at exact HEAD?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Exact-head native macOS and Linux proof remains mandatory after publication before merge.

## Review Result

Revision: Some("git-blake3:d00fc6605f97a89059236585d90ed7a19909a0c1:824ed21ca4009ed165f39901a212232e7d9890c54ad0e894eb33bf0e65b86687")

Reviewer: Some("/root/sprint4_5857/review_5828_exact_head")

Result: pass
