# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/soak.rs
csdlc-v2/tests/gate9.rs
csdlc-v3/src/commands/local/mod.rs
csdlc-v3/src/commands/proof.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/commands/remote/tests.rs
csdlc-v3/src/commands/terminal.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/local_commands.rs
csdlc-v3/tests/operational_cli_commands.rs
csdlc-v3/tests/proof_parity_install_commands.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs

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
- Hosted Gemini review artifacts are advisory and stale or truncated relative to the reviewed head; retained provider artifacts must not be treated as current typed review authority.
- C-SDLC v3 remains non-authoritative until explicit #505 operator approval, merge, typed finish, and cleanup reconciliation.
- The final hardening exposes local and remote operational CLI paths, plus cutover rollback/cleanup/install gates, but every mutating path remains fail-closed before canonical v3 selector and digest-bound #505 approval evidence.

## Review Result

Revision: Some("git-blake3:f58c19b6d4d7774b282c5ba92e258f52028f65a8:a4baf2b21564b8f454abd2ae738b332c2469d82d33692ed3f430ca9021a7f18c")

Reviewer: Some("local-independent:claude-style+gemini-style:no-provider-attestation")

Result: pass
