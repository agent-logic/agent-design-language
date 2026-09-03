# Structured Review Prompt

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/proof.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/proof_parity_install_commands.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/631/design.md
.csdlc/prepared/issues/631/diagram.mmd
.csdlc/prepared/issues/631/validate-v3-h5-proof-parity-install.sh

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

- C-SDLC v3 proof, shadow, soak, and install routes remain construction-only and non-authoritative until explicit #505 cutover.
- The existing PR #644 was opened against a stacked base and cannot prove GitHub closing linkage until republished or retargeted through a typed route.

## Review Result

Revision: Some("git-blake3:6475bc82bf78f40e19c766afeab602a498d0d726:6777227f5e869fd94a424498e7aa9cd9c1edbf927132d1d647f8c99fbdfa8e14")

Reviewer: Some("codex-reviewer:review_631_head_6475")

Result: pass
