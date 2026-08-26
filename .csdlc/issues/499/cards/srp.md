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

## Review Result

Revision: Some("git-blake3:578d26d87e7a88a18b037b3ef2cc6f753638c466:9c0ad4ba3305583e6981aa87bffb0f26d50bbce47de476ff41459477e2365686")

Reviewer: Some("openai-responses-api:resp_01aaf45742faa6f8006a8f52ce1cec87d0abb39c5870e7db56")

Result: pass
