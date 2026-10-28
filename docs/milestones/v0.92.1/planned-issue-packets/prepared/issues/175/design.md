# V3-12 Design

Issue: #175

## Objective

Implement independent exact-revision review assignment, result recording, staleness, finding disposition, and publication authorization.

## Scope

`review assign/record/recover/status`, structurally bound reviewer principals, independence enforcement and policy-only limitation handling, exact scope/revision identity, findings and dispositions, non-substantive change proof, typed recovery provenance and invalidation, mode-bound publication intent, and fail-closed review guard.

## Dependencies

- V3-04: issue #165
- V3-08: issue #169
- V3-10A: issue #171
- V3-10B: issue #172

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Review schemas, authenticated/provider-evidence reviewer principal model, independence predicate and typed override boundary, staleness classifier, finding model, `review recover` transition and command, publication guard, typed intents, mode-bound publication authorization evidence, and review fixture corpus.

## Owned Paths

- `csdlc-v3/src/commands/review/**`
- `csdlc-v3/src/commands/publish/**`
- `csdlc-v3/tests/review/**`
- `.csdlc/issues/175/**`
- `.csdlc/prepared/issues/175/**`
- `.csdlc/prepared/issues/175/validate-outcome.rb`
- `.csdlc/evidence/175/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

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

## PVF Lanes

- `v3-12-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/175/validate-outcome.rb`.
- `v3-12-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-12-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Exact-head/staleness matrix, independence-policy tests, finding lifecycle tests, recover/correct/re-review positive journeys, wrong phase/provenance/invalidation negatives, non-substantive proof negatives, same/split-repository positive and negative linkage matrices, publication guard tests, and tampered-review fixtures.

## Authority Boundary

- Issue V3-12 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Hosting model providers, merging PRs, watching checks, terminal finish, cleanup, or treating review prose as state authority.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Review can approve an unknown revision, actionable findings can be hidden, recovery can strand a record or leave dependent truth current, publication can bypass review, linkage mode is implicit or ambiguous, or provider identity is overstated.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-12`
