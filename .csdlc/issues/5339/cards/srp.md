# Structured Review Prompt

Template: 1.0.0

Issue: 5339

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-v2/crates/adl-language
.csdlc/issues/5339
.csdlc/prepared/issues/5339
.csdlc/evidence/5339

## Prompts

- Does the model define exactly the six primitives without absorbing compiler, engine, runtime, adapter, CLI, or C-SDLC authority?
- Can YAML or JSON coercion, duplicate keys, unknown fields, aliases, ordering, or numeric representation bypass strict validation or canonical equality?
- Do generated schemas, Rust deserialization, semantic validation, and fixtures prove one aligned contract?
- Is every applicable #5337 characterization case mapped, and are intentional differences evidence-backed rather than normalized away?
- Are COTS choices and the provisional LoC/test/latency allocation minimal, measured, and subordinate to correctness?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- YAML alias and resource-exhaustion limits are delegated to the pinned parser and are not separately characterized in WP-04.
- Sequential forward saved-state references are modeled as dependency edges rather than rejected solely by declaration order; characterized ordering remains preserved.

## Review Result

Revision: Some("git-blake3:23c3bfa74484d0780dc802c4f4a2d49384acae74:80eb44dcbb1c085665705a6acd663bb337755a76fa35a52b5e0350cfe8a121c1")

Reviewer: Some("subagent:/root/review_5339_impl")

Result: pass
