# Structured Review Prompt

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/Cargo.toml
adl-runtime/src/distributed/certificates.rs
adl-runtime/src/distributed/fencing.rs
adl-runtime/src/distributed/lease.rs
adl-runtime/tests/distributed_identity_lease_authority.rs
adl-runtime/tests/distributed_runtime_transport.rs
.csdlc/issues/258
.csdlc/prepared/issues/258
.csdlc/evidence/258/postpub-stale-helper-repair-r4

## Prompts

- Review whether raw store access is sealed and whether published receipt view is sufficient for the authority-serving boundary.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Inspection-only review relied on retained proving logs and immutable source inspection; hosted CI remains required after republication.
- The access boundary rejects defined copied-magic and representation attacks but does not claim safety against arbitrary undefined behavior.

## Review Result

Revision: Some("git-blake3:707fdca541faa65da7fa4340ad3803f83cfca9a3:348499fe649b7cd19a0ecb2684f1e5d87a95ce4e996ef1cf6a6cab2147bb06ed")

Reviewer: Some("fresh-session:9beade3c-58ad-451d-b6b6-e1883766f61c")

Result: pass
