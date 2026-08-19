# Structured Intent Prompt

Template: 1.0.0

Issue: 414

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Bridge multiple distinct CPU-local resident Shepherd agents into existing Runtime v2 continuity and prove useful work, complete dehydration, exact rehydration, and deterministic resume on r7i.2xlarge capacity.

## Required Outcome

Existing Runtime v2 lifecycle/snapshot/rehydration authority preserves every admitted resident's identity, state, sequence, predecessor, model and configuration digests across confirmed Spot interruption; replacement restores the exact complete population before admission and every resident performs useful bounded work before and after recovery within 8 vCPU/64 GiB.

## Scope

- Resident Shepherd population subrecord inside the existing signed live_kernel participant blob
- Confirmed IMDS interruption to admission-close and existing snapshot/dehydration control
- Dedicated retained Runtime volume distinct from build cache and restore-before-admission
- Multiple pinned CPU-local resident Shepherd useful-work and resource proof
- Issue-local lifecycle, preparation, evidence, and review

## Authority

- Runtime v2 lifecycle states, snapshot schema, lineage, invariants, duplicate denial, and wake authority are consumed read-only
- No new lifecycle, snapshot, lineage, or recovery system
- No paid #268 launch and no #269 mutation
- Optional external model use is non-authoritative and never required

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 and a bound FastWork worktree
- One active Ollama worker at a time and no concurrent compilation during resource proof
- Use pinned llama3.1:8b Q4 baseline, qwen3:8b structured-agent comparison, phi4-mini utility worker, and optional non-authoritative gpt-oss:20b escalation
- Start Ollama at parallel=2, max-loaded-models=2, context=8192; measure and fail closed on RAM, swap, or latency
- No AWS mutation in #414 local implementation and no paid #268 launch
