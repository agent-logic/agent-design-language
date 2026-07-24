# Structured Review Prompt

Template: 1.0.0

Issue: 5655

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

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

- Review was read-only and did not rerun tests; validation evidence is the implementation owner run recorded in SOR after rebase.

## Review Result

Revision: Some("git-blake3:18798020d78d70b26937084f83edf89084c2567b:de3184f625db90eea82ebbe27516a5687b81433397629ce0f4c8418497d108c9")

Reviewer: Some("Boole")

Result: pass
