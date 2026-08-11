# Structured Review Prompt

Template: 1.0.0

Issue: 234

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.github/workflows/ci.yaml
adl/tools/test_ci_runtime_contracts.sh
.csdlc/issues/234

## Prompts

- Does any optional, unrelated, retained-proof, soak, demo, provider, nightly, or release workflow still acquire a runner automatically for an ordinary PR?
- Do all required heavy lanes remain path-policy gated and routed to the configured 16-core runner?
- Can two PR objects for one branch and head SHA execute duplicate required fleets?
- Can an unknown or focused shared-path change fan out to optional workflows or full coverage?
- Are long soaks explicitly isolated from normal tests and PR coverage?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:b63314b2cd02a668f5ab07c427aa922ca9516c34:47528bcd3c8657705c5d7644b0ce0c9b1d0764e7ed302ec40e165f67e078a363")

Reviewer: Some("provider:gemini-3.1-pro-preview")

Result: pass
