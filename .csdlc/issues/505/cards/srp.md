# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

exact PR #591 head 62e6a873a6e0934c590b120d5887d2da6cb1505c
csdlc-v3/tests/proof_parity_install_commands.rs bounded cold-build doctor fixture timeouts
fail-closed timeout and nonzero child-exit behavior
pre-existing timeout-expiry proof gap classification

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

- No dedicated test directly exercises timeout expiry, child termination, or rejection above the 300-second ceiling; this is a pre-existing non-blocking gap.

## Review Result

Revision: Some("git-blake3:62e6a873a6e0934c590b120d5887d2da6cb1505c:f45f49738600044db851016022c9bd4d6677600d8cf44a64c020e6de96f8d333")

Reviewer: Some("collab-agent:01a075bf-563e-70d3-b1d2-7b05b51f0c57")

Result: pass
