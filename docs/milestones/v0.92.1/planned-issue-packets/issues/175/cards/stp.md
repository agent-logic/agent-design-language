# Structured Task Prompt

Template: 1.0.0

Issue: 175

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-12 within its exact owned paths and authority boundary.

## Deliverables

- Review schemas, authenticated/provider-evidence reviewer principal model, independence predicate and typed override boundary, staleness classifier, finding model, `review recover` transition and command, publication guard, typed intents, mode-bound publication authorization evidence, and review fixture corpus.

## Acceptance

1. Review names exact revision, scope, reviewer, findings, and dispositions.
2. Substantive head changes stale review; non-substantive exceptions require deterministic proof.
3. `review recover` is accepted only from `reviewed`, `published`, or `merge_ready`; it is rejected from `merged` and `closed_out`. It requires actor/reason and stale-truth provenance, returns to `implemented`, and atomically clears every dependent review, publication, readiness, and terminal field declared by the capability row before a card correction can proceed.
4. Recovery followed by a semantic card correction and fresh review is a complete executable path; direct state/card edits and abstract operator dispositions cannot satisfy it.
5. Both linkage modes prove the full review journey: review, publish, recover, semantic correction, re-review, and republish preserve the exact normalized target and invalidate the superseded mode-bound authorization.
6. Publication fails closed on missing, stale, blocked, or actionable review.
7. Model/provider output is evidence input, never direct lifecycle authority.
8. Same-principal implementation/review/publication is rejected; policy-only identity cannot pass the publication gate without a named typed override.
9. Human-review publication remains fail-closed until a concrete authenticated principal observer implements the V3-04 interface; V3-12 proves this with a fake and does not depend on the V3-13 GitHub implementation.
10. Authorization consumes the V3-01 `PublicationLinkage` value, binds it to the exact reviewed revision and target issue, and rejects absent, mixed, ambiguous, or wrong-repository linkage.
11. `PartOf` rejects a closing keyword for its target and `Closing` rejects a non-closing-only relation.

## Dependencies

- V3-04: issue #165
- V3-08: issue #169
- V3-10A: issue #171
- V3-10B: issue #172

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-12
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Hosting model providers, merging PRs, watching checks, terminal finish, cleanup, or treating review prose as state authority.
