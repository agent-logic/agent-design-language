# Structured Review Prompt

Template: 1.0.0

Issue: 5778

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.

## Prompts

- Does finish preserve exact reviewed-head, required-check, publication identity, and expected-SHA merge guarantees?
- Can any interruption or concurrent call create conflicting terminal results or require a second PR?
- Does terminal claim release avoid weakening active nonterminal collision safety?
- Are legacy records readable without becoming competing current authority?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The repaired exact head still requires GitHub Actions integration proof before merge.

## Review Result

Revision: Some("git-blake3:a27886d23681b61261c4e7abb08ddb57be52760c:060fd05f4bc410a42c20ddc8d78b67339cd301a7ea991e59bac363bf64503a9e")

Reviewer: Some("codex-subagent:review_5778_exact_head")

Result: pass
