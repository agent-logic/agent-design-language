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
.csdlc/prepared/issues/505/recover-review-after-merge-main-conflict-fix.json
.csdlc/prepared/issues/505/review-assign-after-merge-main-conflict-fix.json
.csdlc/prepared/issues/505/review-record-after-merge-main-conflict-fix-pass.json

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

Revision: Some("git-blake3:6f668a26e08fd4180d2a8c2046e467690e8d1a94:1687e0eae3d906e63e805cb7dc2a4e577b2de1891ed1bbbc3faeb10167b14b3e")

Reviewer: Some("subagent:/root/review_591_head_6f668a26")

Result: pass
