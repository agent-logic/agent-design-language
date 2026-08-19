# V3-06 Design

Issue: #167

## Objective

Define the versioned v3 aggregate and deterministically render all six lifecycle cards and declared evidence projections.

## Scope

`state.json`, embedded typed audit events and state-size guard, schema evolution, closed enums, canonical serialization, card AST values, SIP-STP-SPP-VPP-SRP-SOR rendering, per-phase field optionality and placeholders, digest rules, projection manifests, and drift detection.

## Dependencies

- V3-04: issue #165
- V3-05: issue #166

## Architecture Decisions

- `V3-D05`

## Deliverables

- State/schema module, embedded audit-event model and no-pruning initial policy, projection engine, card templates or AST builders, per-card/per-phase optionality table, digest profile, fixture corpus, and state/card compatibility report.

## Owned Paths

- `csdlc-v3/src/state/**`
- `csdlc-v3/src/cards/**`
- `csdlc-v3/schemas/state/**`
- `csdlc-v3/tests/state/**`
- `.csdlc/issues/167/**`
- `.csdlc/prepared/issues/167/**`
- `.csdlc/prepared/issues/167/validate-outcome.rb`
- `.csdlc/evidence/167/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. `state.json` is the sole machine authority and every projection is reproducible from it plus declared immutable inputs.
2. Unknown schema versions and enum values fail explicitly.
3. All six cards preserve their distinct lifecycle semantics.
4. Missing required fields fail with a typed error; optional unset fields render only the declared placeholder at each lifecycle phase.
5. `audit.jsonl` is reproducible from embedded state events and has no separate mutation or integrity authority.
6. Projection drift is diagnosable and repair never treats Markdown as authority.

## PVF Lanes

- `v3-06-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/167/validate-outcome.rb`.
- `v3-06-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-06-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Schema round trips, canonical-byte golden tests, all-card structure/schema validation, randomized closed-enum tests, drift and repair fixtures, and v2 normalized parity comparisons.

## Authority Boundary

- Issue V3-06 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Lifecycle transition authorization, transaction recovery, GitHub observation, direct Markdown authority, or compatibility dual writes.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A card requires undeclared authority, rendering is nondeterministic, or state evolution can silently discard unknown fields.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-06`
