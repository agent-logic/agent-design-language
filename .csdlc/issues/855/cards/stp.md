---
issue_card_schema: adl.issue.v1
wp: "[v0.92.2][RT-PROVIDER] Provider-neutral dynamic agent lifecycle"
slug: "855-provider-neutral-lifecycle"
title: "[v0.92.2][RT-PROVIDER] Provider-neutral dynamic agent lifecycle"
labels:
  - "track:roadmap"
issue_number: 855
generated_at: "2026-09-11T23:55:22.557345+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "production-behavior"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/855"
canonical_files: []
demo_required: yes
demo_names: []
issue_graph_notes:
  - "Use only numeric prerequisites from source issue; no sprint-wide or closeout dependency."
pr_start:
  enabled: true
  slug: "855-provider-neutral-lifecycle"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T23:55:22.557345+00:00

# Structured Task Prompt

## Summary

[v0.92.2][RT-PROVIDER] Provider-neutral dynamic agent lifecycle

## Goal

Allow an operator to add, inspect, communicate with, checkpoint, migrate, rehydrate, and remove an agent backed by any provider adapter registered with the Runtime, without restarting the Runtime or editing its canonical initialization file. Dynamic agent lifecycle operations dispatch through the Runtime's provider registry and capability contract rather than provider-name match statements. Provider-specific authentication, endpoint validation, request execution, tool capability, streaming, accounting, and failure classification remain owned by the selected provider adapter. - Hard-coding only the four providers used by the initial acceptance demonstration. - Embedding provider secrets in agent configuration. - Requiring all providers to support identical optional capabilities. - Restarting Runtime merely to add, replace, or remove an agent. - Changing subscription or cloud billing configuration.

## Required Outcome

Allow an operator to add, inspect, communicate with, checkpoint, migrate, rehydrate, and remove an agent backed by any provider adapter registered with the Runtime, without restarting the Runtime or editing its canonical initialization file. Dynamic agent lifecycle operations dispatch through the Runtime's provider registry and capability contract rather than provider-name match statements. Provider-specific authentication, endpoint validation, request execution, tool capability, streaming, accounting, and failure classification remain owned by the selected provider adapter. - Hard-coding only the four providers used by the initial acceptance demonstration. - Embedding provider secrets in agent configuration. - Requiring all providers to support identical optional capabilities. - Restarting Runtime merely to add, replace, or remove an agent. - Changing subscription or cloud billing configuration.

## Deliverables

Allow an operator to add, inspect, communicate with, checkpoint, migrate, rehydrate, and remove an agent backed by any provider adapter registered with the Runtime, without restarting the Runtime or editing its canonical initialization file. Dynamic agent lifecycle operations dispatch through the Runtime's provider registry and capability contract rather than provider-name match statements. Provider-specific authentication, endpoint validation, request execution, tool capability, streaming, accounting, and failure classification remain owned by the selected provider adapter. - Hard-coding only the four providers used by the initial acceptance demonstration. - Embedding provider secrets in agent configuration. - Requiring all providers to support identical optional capabilities. - Restarting Runtime merely to add, replace, or remove an agent. - Changing subscription or cloud billing configuration. Include production proof, failure handling and operator documentation required by the source issue.

## Acceptance Criteria

- `csmctl agent add --config <agent.yaml>` accepts every provider registered with the running Runtime and rejects only an unknown provider or a provider whose declared capability requirements are unsatisfied. - Adding or replacing a dynamic agent does not require a Runtime or Guardian restart and does not require editing the Runtime initialization file. - OpenAI/ChatGPT, Anthropic/Claude, Gemini through the appropriate Google provider route, Ollama local, and an OpenAI-compatible local endpoint each pass add, generated conversation, roster projection, checkpoint, removal, and rehydration tests. - Agent-to-agent communication uses the same provider-neutral execution path and canonical agent names for every admitted provider. - Provider adapters declare whether tools, streaming, model discovery, token accounting, and health checks are supported; the Runtime does not infer these capabilities from provider names. - Hosted providers require HTTPS and approved credential references. Local plaintext endpoints remain limited to loopback, private, or explicitly trusted local-network bindings. - Admission performs no recurring paid inference. Health checks and model validation follow provider capabilities and the metered-call safeguards tracked by #854. - Provider failures retain actionable classifications such as credentials, quota, unsupported capability, model unavailable, transport, timeout, and invalid response. - The API and Observatory report the effective provider/model and current capability/readiness state without exposing credentials. - Deterministic tests prove that a newly registered fixture provider works without modifying Runtime admission or conversation match statements.

## Repo Inputs

Full canonical issue https://github.com/agent-logic/agent-design-language/issues/855; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json.

## Dependencies

Accepted merged output required from #876. No sprint-wide barrier or asynchronous closeout dependency.

## Target Files / Surfaces

- `adl-runtime-kernel/src/control.rs` - `adl-runtime-kernel/src/config.rs` - `adl/src/cli/csmctl_cmd.rs` - Existing provider adapter and capability architecture - `docs/architecture/PROVIDER_CAPABILITY_AND_TRANSPORT_ARCHITECTURE.md` - Issue #602 dynamic agent lifecycle behavior - Issue #854 metered cloud inference safeguards

## Validation Plan

Execute every proving case in the source issue; nonzero production scenarios, focused negative tests, independent exact-head review and required CI.

## Demo Expectations

Execute and retain the complete acceptance/proving cases in the source issue; no schema-only or fixture-only substitution.

## Non-goals

- Hard-coding only the four providers used by the initial acceptance demonstration. - Embedding provider secrets in agent configuration. - Requiring all providers to support identical optional capabilities. - Restarting Runtime merely to add, replace, or remove an agent. - Changing subscription or cloud billing configuration.

## Issue-Graph Notes

Use only numeric prerequisites from source issue; no sprint-wide or closeout dependency.

## Notes

Prepared only. Child implementation and its review have not run.

## Tooling Notes

Native v3 bind/edit/validate/review/publish; stable installed owners; primary main inspection-only.
