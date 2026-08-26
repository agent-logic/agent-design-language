# Structured Review Prompt

Template: 1.0.0

Issue: 350

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

Exact HEAD 4a79237556462e38c6216b123071a8b9f8ed0749
adl-runtime/Cargo.toml
adl-runtime/src/distributed/authority_protocol.rs
adl-runtime/src/distributed/authority_protocol_contract_tests.rs
adl-runtime/src/distributed/serving_authority.rs
adl-runtime/tests/distributed_observatory_authority_projection.rs
.csdlc/prepared/issues/350/design.md
.csdlc/prepared/issues/350/diagram.mmd
.csdlc/issues/350
.csdlc/evidence/350
All R1/R2b finding, recovery, remediation, SOR mapping and interrupted no-result assignment audit history

## Prompts

- Can any caller, legacy-direct result, config value, or unrelated valid authority/cut pair construct, substitute, or combine sealed authority fields?
- Does replicated publication alone capture the complete authoritative old/joint voter eligibility basis and committed inclusive deadline, with restore revalidation and no synthetic legacy quorum truth?
- Does authority_protocol_contract_tests.rs remain a bounded compatibility-only change and does its exact 52-case denominator prove existing behavior on the replicated sealed path?
- Are canonical JCS bytes, every cut/operation/identity/quorum/lineage binding, deadline ordering, restore behavior, output, and errors exact and redacted?
- Do changed paths remain inside #350 while #274/#273/#203/#205/#275 stay excluded?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Inspection-only exact-head review; hosted CI and terminal merge authority remain pending.

## Review Result

Revision: Some("git-blake3:4a79237556462e38c6216b123071a8b9f8ed0749:84b06f5351d2fb4e6eba6e8ba94ddc7bac9b074111fb29b3153e8a03e80ad9b3")

Reviewer: Some("fresh-session:4f98d5e6-7a3c-4b21-9d60-2c8a1f73b945")

Result: pass
