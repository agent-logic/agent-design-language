# Structured Task Prompt

Template: 1.0.0

Issue: 640

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One provider-backed resident Shepherd configuration, preload, execution, health, recovery, API, and focused proof slice; no general dynamic-agent lifecycle redesign.

## Deliverables

- Declarative non-empty resident Shepherd set with provider/model/endpoint/preload configuration
- Provider-backed Shepherd executor through the governed operation boundary
- Startup preload and idempotent lifetime non-fatal recovery behavior
- One truthful readiness snapshot shared by /v1/ready, blocking_reasons, roster/detail, and Observatory
- Focused configuration, provider, preload, restart, isolation, consistency, and API tests
- Bounded Wuji restart and governed-inference acceptance evidence

## Acceptance

1. AC-1: Runtime configuration requires a non-empty resident Shepherd set, validates each canonical identity/provider/model/endpoint/preload policy, and rejects duplicate canonical identities
2. AC-2: Each Shepherd invokes its declared provider/model through the existing governed operation boundary
3. AC-3: A Shepherd remains model_loading until provider health and preload succeed, then reports ready
4. AC-4: Provider/model identity and admitted, model_loading, ready, or degraded health are observable without credentials; /v1/ready, blocking_reasons, roster/detail, and Observatory snapshot/feed agree
5. AC-5: Temporary provider or model failure visibly degrades and retries only the affected Shepherd without terminating the Runtime, globally blocking readiness, or disrupting unrelated agents
6. AC-6: Restart reconstructs one resident per unique configured Shepherd identity and repeats preload without manual provider commands or duplicate residents
7. AC-7: Existing checkpoint, continuity, canonical-name, and non-Shepherd lifecycle behavior remains valid
8. AC-8: Wuji acceptance with a local Ollama model proves restart, automatic preload, truthful consistent health, and one successful governed Shepherd inference

## Dependencies

- Issue #617 and PR #636 must be merged into the execution base

## Inputs

- agent-logic/agent-design-language#640
- agent-logic/agent-design-language#617
- agent-logic/agent-design-language#602
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/governed_operations.rs

## Non Goals

- Hard-coding one provider or model
- Requiring Ollama for cloud deployments
- Making temporary inference degradation terminate or globally block the Runtime
- General dynamic-agent lifecycle, migration, or checkpoint redesign
- Reimplementing #617 canonical-name work or stacking its unmerged PR into #640
- Broad workspace test execution when focused Runtime proof is sufficient
