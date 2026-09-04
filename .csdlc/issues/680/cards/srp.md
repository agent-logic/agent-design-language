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

- No live Moonshot API call was performed; #680 proof uses deterministic loopback provider tests plus setup/profile/runtime-adapter validation, with live credentials remaining operator-controlled.

## Review Result

Revision: Some("git-blake3:589c88d578ebf9d5598a5ed81e8ca3a448bb3829:70034edda94c1af786e3a4c5334e04bfa82ba95d04190414970a76147766da1a")

Reviewer: Some("fresh-session:5b00b20d-9764-4bd5-8986-936c89a77d1e")

Result: pass
