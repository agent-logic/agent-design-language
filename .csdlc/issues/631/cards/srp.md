# Structured Review Prompt

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/cli/csm_runtime_v3_cmd.rs
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
.csdlc/prepared/issues/631/validate-v3-h5-proof-parity-install.sh
.csdlc/evidence/631

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

- C-SDLC v3 proof, shadow, soak, and install routes remain construction evidence only until explicit #505 cutover approval and terminal reconciliation.
- The v3 command manifest still records the local command as partial and non-authoritative, so full v2 replacement remains unproven until the remaining cutover gates complete.

## Review Result

Revision: Some("git-blake3:ce33d377149e30e1acf802d2391d95ad78c0f722:d2bb1e5b2ab908f4f62129d5b13dc3208795c8f0cda0ec32565df5c2cde0c9eb")

Reviewer: Some("codex-reviewer:review_631_coverage_gate_fix")

Result: pass
