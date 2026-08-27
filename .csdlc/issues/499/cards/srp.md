# Structured Review Prompt

Template: 1.0.0

Issue: 499

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/resilience.rs
adl/src/resilience/*.rs
.csdlc/prepared/issues/499/validate-*.rb
.csdlc/issues/499 lifecycle metadata
publication safety at exact integrated head

## Prompts

- Does the implementation stay inside the declared unit boundary?
- Does every acceptance criterion have proving evidence?
- Are operator-only actions and private material kept outside Git?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the fail-closed integration proof after publication.
- The independent API reviewer noted that the API-parity validator is lexical rather than a full semantic compatibility checker; focused compile, behavior, fmt, and clippy proof passed, and hosted CI remains required.
- The supported public resilience facade paths remain re-exported and source-compatible; internal Rust definition paths may differ after the module split.

## Review Result

Revision: Some("git-blake3:2474118a22f95c45e0ca44730df8e492e5dcc0c7:c4c0712ab562022ba614c82bdd853a9afedc20a96eb4034d65245e4a65f52f7a")

Reviewer: Some("openai-responses-api:resp_099758cbc0d0767c006a90883ba19487d0b37ddbecef1b3a4d")

Result: pass
