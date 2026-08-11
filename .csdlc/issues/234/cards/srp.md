# Structured Review Prompt

Template: 1.0.0

Issue: 234

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.github/workflows/ci.yaml
adl/tools/test_ci_runtime_contracts.sh
adl/tools/validate_ci_workflow_policy.rb
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

- Manual workflow_dispatch remains operator-authorized and may intentionally select non-PR proof routes; ordinary pull requests cannot select Spot/AWS.

## Review Result

Revision: Some("git-blake3:2aecc985c6d67c3d6f6d57a816181e91644e7ca0:c9568578e22e9fcfd620d7a0c827f171b52a2758bb44bce498ef66c4b6fce77d")

Reviewer: Some("subagent:019ff295-bb90-7dd2-b121-266e74fd384a")

Result: pass
