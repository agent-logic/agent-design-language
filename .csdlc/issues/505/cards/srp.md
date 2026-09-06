# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

exact PR #591 head ce92cdfbc3897cb6ad049275a5888fad999ac745
csdlc-v2/src/soak.rs canonical v3 authority contract
csdlc-v2/src/operator.rs origin/main activation boundary
csdlc-v2/tests/gate10a.rs pre-merge v2 resolution
csdlc-v3/src/authority.rs canonical v3 authority contract
csdlc-v3/tests/local_commands.rs process-isolated authority fixtures
live PR #591 body closing linkage and merge-as-cutover wording

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

- none

## Review Result

Revision: Some("git-blake3:ce92cdfbc3897cb6ad049275a5888fad999ac745:f85012e9b002d5b8ea6182d95c2c1c0316ad9b9f4c9b952525e846a337f1cbed")

Reviewer: Some("collab-agent:01a07599-3e3b-7a31-b19a-8e7359c31d6e")

Result: pass
