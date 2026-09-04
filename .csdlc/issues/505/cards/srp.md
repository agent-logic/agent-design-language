# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/mod.rs
csdlc-v3/src/commands/remote/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/local_commands.rs
docs/csdlc-v3/full-replacement-denominator.json
docs/csdlc-v3/v3-command-manifest.json

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

- C-SDLC v3 remains non-authoritative construction and cutover-readiness evidence only until explicit #505 operator approval, merge, finish, and cleanup reconciliation.
- PR #591 must remain non-closing until the explicit cutover approval path is exercised; current publication must use part_of linkage only.

## Review Result

Revision: Some("git-blake3:38411a248d287950c795d6daa3d34625d2a94131:9e0fc7191770af2958e58864ddf90a48ed8850ba4cf38cdab740cff7a0db2381")

Reviewer: Some("subagent:/root/review_591_head_38411a24")

Result: pass
