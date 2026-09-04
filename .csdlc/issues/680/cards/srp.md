# Structured Review Prompt

Template: 1.0.0

Issue: 680

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/provider/http_family.rs
adl/src/provider/mod.rs
adl/src/provider/profiles.rs
adl/src/provider_adapter.rs
adl/src/provider_communication.rs
adl/src/provider_substrate.rs
adl/src/cli/provider_cmd.rs
adl/src/cli/usage.rs
docs/tooling/PROVIDER_SETUP.md
adl/tests/provider_moonshot_kimi_k3.rs
.csdlc/evidence/680

## Prompts

- Does the change make Moonshot/Kimi K3 first-class without breaking existing kimi:k2.5 or OpenRouter Kimi behavior?
- Do tests prove setup/profile/provider selection and auth behavior without leaking credentials or claiming live provider proof?
- Is the external model-id truth recorded accurately enough for future catalog drift?

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
