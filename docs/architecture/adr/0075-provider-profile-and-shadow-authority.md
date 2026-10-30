# ADR 0075: Provider Profile and Shadow Authority

## Status

Proposed. Tracked by issue #745.

## Context

Tools need consistent provider/model identity and inference configuration. Local-model comparison is useful evidence, but a comparison result must not silently become an authoritative execution decision. Reloading configuration also needs explicit owner lifetime to avoid one workflow consuming another's profile state.

## Decision

Use the shared provider-profile mechanism to materialize explicit provider/model identity and validated bounded inference parameters before activation. Preserve last-known-good profile/materialization state when a candidate is rejected. Treat deterministic Ollama materialization as a reproducible configuration projection; it is not a guarantee of identical generated text or equivalent model capability.

Keep redacted evidence projections separate from raw execution configuration. Do not expose credentials, prompts, private payloads, or raw endpoint URLs through the declared proof projection. Provider-specific accepted bounds belong in the profile implementation and operator documentation rather than duplicated constants in this ADR.

Local-model shadow comparison remains non-authoritative. A shadow score, response, or local run cannot replace a governed decision, approve release, select a production provider, or authorize paid inference. MLX/Metal and OCI packaging remain deferred under the planned profile feature.

Provider reload consumes a provider-only sidecar and publishes immutable validated snapshots. Preserve explicit execution-owner scope and reject credential values or executable workflow/authority surfaces before activation; do not use a profile as a second workflow control plane.

## Consequences

Tools can share identity and validation semantics while retaining provider-specific constraints. Profile evidence proves materialization and validation, not live provider suitability. Shadow evidence must carry its comparison-only status through downstream reports.

## Alternatives Considered

Tool-specific profile semantics: rejected because equivalent inputs could activate different parameters. Promote shadow output automatically: rejected because model comparison grants no execution authority. Serialize raw provider documents as proof: rejected because public runtime configuration shape is not the redacted evidence boundary.

## Supersession Relationships

Refines [ADR 0004](../../adr/0004-provider-profiles.md) and [ADR 0041](../../adr/0041-provider-model-suitability-boundary-v2.md). Leaves [ADR 0071](0071-provider-neutral-multi-agent-proof-boundary.md) deferred: profile consistency is not multi-provider behavioral proof.

## Source Evidence

- [Provider feature](../../milestones/v0.92.1/features/PROVIDER_INFERENCE_PROFILES_v0.92.1.md)
- [Profile contract](../../provider/inference-profiles.md)
- [Provider reload implementation](../../../adl/src/provider/reload.rs)
- [Milestone decisions](../../milestones/v0.92.1/DECISIONS_v0.92.1.md)
- [Profile validation evidence](../../../.csdlc/evidence/514/profile-schema.log)
- [Materialization evidence](../../../.csdlc/evidence/514/ollama-materialization.log)
- [Redaction evidence](../../../.csdlc/evidence/514/redaction.log)

## Validation Notes

This packet inspected source contracts and retained evidence. It made no provider calls and does not claim provider neutrality, shadow completion, model suitability, or live rollout acceptance. Re-run focused materialization, redaction, last-known-good, and overlapping-owner tests when the implementation changes.

## Approval Boundary

Review may accept this bounded configuration/authority record. It cannot authorize a provider call or promote shadow results into governed execution.
