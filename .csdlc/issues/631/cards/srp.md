# Structured Review Prompt

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/proof.rs
csdlc-v3/src/commands/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/proof_parity_install_commands.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/631/validate-v3-h5-proof-parity-install.sh
.csdlc/prepared/issues/631/finalize-implementation.json
.csdlc/evidence/631/v3-h5-issue-validator.log
.csdlc/evidence/631/v3-h5-full-v3-regression.log
.csdlc/evidence/631/v3-h5-rustfmt.log
.csdlc/evidence/631/v3-h5-diff-hygiene.log

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

- Review was limited to P1/P2 pre-publication risks for the #631 proof, shadow, soak, and install construction slice; #505 cutover and live operational authority remain out of scope.

## Review Result

Revision: Some("git-blake3:a257ab2da611803775850d73a0e893236e4b91af:59fc2d56201d649192e78727f397ccdd0e82d4364fe63924d85a068241818b47")

Reviewer: Some("codex:exec-review-01a0632a")

Result: pass
