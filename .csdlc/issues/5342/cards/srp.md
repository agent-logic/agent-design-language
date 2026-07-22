# Structured Review Prompt

Template: 1.0.0

Issue: 5342

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/5342
.csdlc/locks/5342.lock
.csdlc/prepared/issues/5342
.csdlc/evidence/5342
adl-v2/crates/adl-records

## Prompts

- Can two semantically different records produce the same canonical bytes, digest, or signed preimage?
- Can an envelope select or modify the trust policy, key permissions, profile, kind, validity, or revocation decision that authorizes it?
- Does every malformed, tampered, oversized, replayed, unknown-field, duplicate-key, and wrong-key/profile/kind case fail closed?
- Are channel and fresh-process proofs genuinely independent of in-process object identity and implicit host state?
- Are all cryptographic operations delegated to reviewed COTS and all product inputs explicitly bounded?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
