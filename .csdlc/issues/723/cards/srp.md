# Structured Review Prompt

Template: 1.0.0

Issue: 723

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/proof.rs
csdlc-v3/tests/proof_parity_install_commands.rs
csdlc-v3/tests/real_issue_canary.rs

## Prompts

- Verify stderr fallback accepts only one typed document and preserves nonzero exit.
- Verify install authorization binds exact head and digests.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The broader all-target suite retains construction-era cutover fixtures outside issue #723's focused proof lane.

## Review Result

Revision: Some("git-blake3:c2d1770d81e5fc09ec090c5f577be885e9f19c12:80dee8d3e40d27e640884f878f4548445fb6bc587691d48708377fa94ebdc2c5")

Reviewer: Some("fresh-session:remediate-v3-725")

Result: pass
