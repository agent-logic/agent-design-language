# v0.91.8 Feature Preservation Crosswalk

Source: `docs/planning/ADL_FEATURE_LIST.md`  
Pinned feature rows: 123  
Normalized row digest: `5ecc0649f731c7b6afc71e33441924266df540a0997e2aa7b7f889db0005df65`

This crosswalk prevents a canonical feature from disappearing during ADL v2
or Runtime v3 cutover. It is a planning disposition, not implementation proof.
Every source row is classified deterministically by
`.csdlc/prepared/issues/5594/validate_feature_crosswalk.rb`.

## Classification Order

| Class | Owner | Rule | Required terminal disposition |
| --- | --- | --- | --- |
| `secure_access_observatory` | #5590 | Access, remote, communications, A2A/ACIP, transport, Observatory, telemetry, or guardian terms | Runtime v3 implementation or explicit accepted non-runtime/defer disposition |
| `reasoning_adaptive_cognition` | #5592 | Reasoning, loop, adaptive/learning, affect, cognitive, curiosity, Constructability, Godel, Theory of Mind, skill, guild, or economic terms | Runtime v3 implementation or explicit accepted non-runtime/defer disposition |
| `governed_operations` | #5589 | Governance/Freedom Gate, delegation, agent lifecycle, provider, scheduler, tool, identity, memory, Chronosense, checkpoint, lifelog, resilience, Shepherd, or private-state terms | Runtime v3 implementation or explicit accepted non-runtime/defer disposition |
| `kernel_continuity_ingress` | #5591 | Runtime, execution, replay, continuity, backpressure, lifecycle, or bounded-concurrency terms not already classified above | Runtime v3 implementation or explicit accepted non-runtime/defer disposition |
| `csdlc_external_owner` | #5358 | C-SDLC, review, issue/PR lifecycle, prompt-card, or workflow-control terms not owned by Runtime | Exact C-SDLC v2 acceptance or explicit residual blocker |
| `retained_or_external` | #5336 and #5347 | Every remaining row | Named retained/external/future owner; no deletion until the reviewed ownership and deletion manifests agree |

The rules intentionally over-include ambiguous rows. A false-positive Runtime
candidate must receive an explicit non-runtime disposition; it may not be
silently dropped. Rules are evaluated in table order, so every row has exactly
one planning owner.

## Gate

The validator fails if:

- the source row count or digest changes;
- a row has empty feature, status, evidence, or next-target fields;
- feature names are duplicated;
- any row has zero or multiple classifications;
- any class lacks a named issue owner.

WP-02 `#5336` may deliberately revise the pinned baseline and classification
rules, but that change requires review. Runtime v2 deletion remains forbidden
until #5591/#5592/#5589/#5590 and #5347 consume the resulting per-row
dispositions at exact revisions.
