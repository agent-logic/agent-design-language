# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/adapters/mod.rs
csdlc-v3/src/commands/local/mod.rs
csdlc-v3/src/commands/proof.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/commands/terminal.rs
csdlc-v3/tests/local_commands.rs
csdlc-v3/tests/proof_parity_install_commands.rs
csdlc-v3/tests/remote_publication_commands.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
csdlc-v3/tests/real_issue_canary.rs
docs/csdlc-v3/CUTOVER_READINESS_NOTICE.md
docs/csdlc-v3/authority-transition-disposition.json
.csdlc/prepared/issues/505/validate-authority-transition-prep.rb

## Prompts

- Verify #505 remains pre-bind preparation only until #504 is terminal, reconciled, and ancestral.
- Verify the packet preserves C-SDLC v2 live authority and rejects silent v2 retirement before explicit operator approval.
- Verify requirements #179 and #180 are named in the acceptance denominator and future proof plan.
- Verify the future PR body requirement visibly uses `Closes #505`.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted Claude review attempts did not produce usable current provider review authority; retained provider artifacts must not be treated as a Claude pass.
- Hosted Gemini review artifacts are advisory and stale or truncated relative to the reviewed 9cc8ddae head; retained provider artifacts must not be treated as current typed review authority.
- C-SDLC v3 remains non-authoritative until explicit #505 operator approval, merge, typed finish, and cleanup reconciliation.
- Post-cutover v2 retirement remains out of scope until the rollback window and retirement evidence are separately satisfied.

## Review Result

Revision: Some("git-blake3:9cc8ddaeac71d7816ff65b3d50ece288fa91fc25:1aa3c787603bb3993b233abf94c96e8f5041491ade431bd724bdc4e831b247c3")

Reviewer: Some("local-independent:claude-style+gemini-style:no-provider-attestation")

Result: pass
