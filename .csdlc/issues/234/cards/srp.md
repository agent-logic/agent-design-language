# Structured Review Prompt

Template: 1.0.0

Issue: 234

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.github/workflows/ci.yaml
.github/workflows/ci-out-of-band.yaml
adl/tools/validate_ci_workflow_policy.rb
adl/tools/test_validate_ci_workflow_policy.rb
adl/tools/test_ci_runtime_contracts.sh

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

Revision: Some("git-blake3:ab354119ca3b9e3e372379e8a90b65d57eb1240d:a4b139dfb7611a369fa308a04ba74773889a18b27f09ad87a21f7aaa109671cb")

Reviewer: Some("subagent:019ff210-ff6d-76e0-af5b-bd6bd6cb162c")

Result: pass
