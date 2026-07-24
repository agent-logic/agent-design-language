# Structured Review Prompt

Template: 1.0.0

Issue: 5655

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/github.rs
csdlc-v2/tests/gate_github_actions.rs
.csdlc/issues/5655/cards/sor.md
.csdlc/issues/5655/index.json

## Prompts

- Does one Rust command surface cover every declared issue mutation without connector or wrapper fallback?
- Are ambiguous remote outcomes reconciled before retry or local state mutation?
- Are repository, issue, operation key, labels, assignees, comments, and close identity checked exactly?
- Are tokens bounded and never emitted?
- Do tests prove failure behavior rather than only happy paths?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was read-only and did not rerun tests; validation evidence is the implementation owner run recorded in SOR.

## Review Result

Revision: Some("git-blake3:7ea92d637fca550a7ef834ac89a96dba987579b3:b08e0d46befe0d91c8bd82164299cb72335ca2d55e3f4ce34274f5e55fc5354d")

Reviewer: Some("Boole")

Result: pass
