# Structured Review Prompt

Template: 1.0.0

Issue: 234

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.github/workflows
adl/tools/ci_path_policy.sh
adl/tools/test_ci_path_policy.sh
adl/tools/test_ci_runtime_contracts.sh
adl/tools/validate_ci_workflow_policy.rb
csdlc-v2/src/github.rs
csdlc-v2/src/finish.rs
docs/tooling/CI_REQUIRED_AND_OPTIONAL_LANES.md
.csdlc/evidence/234
.csdlc/issues/234
.csdlc/prepared/issues/234

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

Revision: None

Reviewer: None

Result: pre_review
