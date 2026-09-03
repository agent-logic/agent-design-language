# Structured Intent Prompt

Template: 1.0.0

Issue: 640

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Give every Polis at least one resident Shepherd whose reasoning is backed by a declaratively configured provider and model without making transient inference failure fatal to the Runtime.

## Required Outcome

Runtime configuration selects a non-empty set of uniquely named resident Shepherds and each provider profile, model, endpoint reference, and preload policy; startup preloads models before Shepherd readiness; Shepherd reasoning uses the governed provider path; one health snapshot keeps Runtime readiness and Observatory projections consistent; lifetime recovery with bounded probes isolates temporary model failures from the rest of the Runtime.

## Scope

- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/governed_operations.rs
- adl-runtime-kernel/tests/assembly.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/governed_operations.rs
- adl-runtime-kernel/tests/shepherd.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- docs/api/runtime-v3/v1/observatory.openapi.json
- .csdlc/prepared/issues/640
- .csdlc/evidence/640
- .csdlc/issues/640

## Authority

- Issue authority is agent-logic/agent-design-language#640
- The execution base must contain merged #617 canonical-name projection
- Runtime configuration is authoritative for each resident Shepherd provider, model, endpoint reference, preload policy, and canonical identity
- Provider credentials remain external and must never appear in API, logs, or evidence
- Existing governed-operation and agent-continuity boundaries remain authoritative

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Do not hard-code Ollama or qwen3:8b
- Do not use short fixed launch or inference deadlines
- Temporary provider failure must not terminate the Runtime or unrelated agents
- Require at least one resident Shepherd and reject duplicate configured canonical identities
- Use typed C-SDLC v2 authority
