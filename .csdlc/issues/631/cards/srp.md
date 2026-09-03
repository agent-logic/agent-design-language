# Structured Review Prompt

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/proof.rs
csdlc-v3/tests/proof_parity_install_commands.rs
csdlc-v3/tests/real_issue_canary.rs
docs/csdlc-v3/full-replacement-denominator.json

## Prompts

- Can any route claim proof, parity, soak, or install success without durable bounded evidence?
- Does shadow refuse broad equivalence and report exact mismatches?
- Does install plan a stable one-binary artifact without selector mutation before #505?
- Are all tests and validators exercising behavior instead of strings?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 proof, shadow, soak, and install routes remain non-authoritative until explicit V3-F/#505 cutover.
- The provenance ref is repo-contained and readable, but the #631 schema does not yet define a separate source provenance digest field.
- Remaining v3 production-readiness gaps in the GitHub publication, terminal cleanup, and cutover groups continue to block #505.

## Review Result

Revision: Some("git-blake3:64f143bac44c0649e21dce0c10a9a38d8ecae7a4:b3a1c92e7658c5607d3e25ba3effd45f9911195195ed8a3d76a55fa3a3d426e6")

Reviewer: Some("codex-reviewer:review_631_head_64f143")

Result: pass
