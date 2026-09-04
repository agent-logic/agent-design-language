# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/505
csdlc-v3/src/commands/replacement.rs
csdlc-v3/src/commands/mod.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/real_issue_canary.rs
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

- C-SDLC v3 remains construction and cutover-readiness evidence only until explicit V3-F/#505 operator approval and terminal reconciliation.
- The v3 replacement verifier routes are executable and non-mutating before cutover; they do not install, publish, finish, clean, mutate GitHub, or retire v2.
- PR #591 must be republished at this reviewed head and must remain non-closing for #505 until the operator explicitly authorizes cutover.

## Review Result

Revision: Some("git-blake3:cedd7034d9b44627d18166d37cb3fb0ff78e6710:491a02066e9bdb9d30daa8cef193e1d3cec3f8f9c10ee57c1b5e0605e8f4fa03")

Reviewer: Some("subagent:/root/review_505_replacement_verifiers")

Result: pass
