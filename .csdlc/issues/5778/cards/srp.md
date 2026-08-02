# Structured Review Prompt

Template: 1.0.0

Issue: 5778

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

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

- Independent review did not repeat a live merge, interruption, or reopen exercise; exact-head focused tests and the implementation session's 168-test strict-clippy proof passed.

## Review Result

Revision: Some("git-blake3:b408fcf242d16d0337f7e53953188ab456e367c0:fc7571b043c39893d4f8fd21140c46063dc7bd31da41b9f5d7dee345df209888")

Reviewer: Some("codex-subagent:review_5778_exact_head")

Result: pass
