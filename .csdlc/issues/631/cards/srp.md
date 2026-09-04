# Structured Review Prompt

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/main.rs
csdlc-v3/src/commands/mod.rs
csdlc-v3/src/commands/proof.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/commands/terminal.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/proof_parity_install_commands.rs
csdlc-v3/tests/remote_publication_commands.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
csdlc-v3/tests/real_issue_canary.rs
docs/csdlc-v3/v3-command-manifest.json
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

- Hosted CI remains the final integration gate before merge.
- C-SDLC v3 proof, shadow, soak, and install routes remain construction evidence only until explicit #505 cutover approval and terminal reconciliation.

## Review Result

Revision: Some("git-blake3:76ce02d58f8cccb9122d775bcf4862598a844dd0:8f58d44f941a6873c750c727d3c1cedc65a54a794e29a338620edd78f54f9f2e")

Reviewer: Some("codex-reviewer:review_631_ci_proof_root_fix")

Result: pass
