# Structured Review Prompt

Template: 1.0.0

Issue: 47

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/tests/validation_selectors.rs
csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md

## Prompts

- Can any accepted named lane still fan out across unrelated Cargo test targets?
- Does exact schema proof select a nonzero intended unit-test set and exclude estimation_contracts?
- Do intentional broad commands remain supported without being misclassified?
- Are invalid-selector diagnostics specific enough to provide a corrected command?
- Do active skills/runbooks distinguish target boundaries from test-name filters?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted integration proof remains deferred to the exact published PR head.

## Review Result

Revision: Some("git-blake3:8f78d2d1ca4bf29051e1971ac82986b66230b2a1:0771d7304528a89df4d34165383958f3fdefb07b51d28398be1831cc84124702")

Reviewer: Some("codex:issue-47-exact-head-review")

Result: pass
