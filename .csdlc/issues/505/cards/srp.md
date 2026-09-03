# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/505
csdlc-v3/src/commands/remote/mod.rs
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
- The visible v3 command surface still has five replacement gaps before cutover: shadow partial/local partial plus cutover, install, proof, and soak fail-closed/non-live routes.
- PR #591 must be republished at the reviewed head and must remain non-closing for #505 until the operator explicitly authorizes cutover.

## Review Result

Revision: Some("git-blake3:3774b1f23fee865e36db178c1ef4f7677cd62bf1:4b8d93e9d1d5fe6f4ad78f5b58e1f8ff359cee124cef8bd701d04a4b6631c74c")

Reviewer: Some("subagent:/root/review_505_remote_alias_delta")

Result: pass
