# Issue 571 design

## Boundary

#571 is a corrective V3-A follow-up for the merged #500/#565 artifacts. It repairs only the C-SDLC v3 predecessor-proof and lifecycle-gate contract surfaces that were found insufficient after merge.

The issue does not rewrite #500/#565 history as passing, does not implement later v3 slices, and does not move lifecycle authority from C-SDLC v2 to v3. V3-F/#505 remains blocked until this issue is merged or explicitly dispositioned by the operator.

## Inputs

- `docs/csdlc-v3/predecessor-coverage.json`
- `docs/csdlc-v3/CONTRACT.md`
- `docs/csdlc-v3/proportional-lifecycle.json`
- `.csdlc/prepared/issues/500/validate-implementation.rb`
- The recorded #500/#565 review findings that require a bounded corrective follow-up.

## Required corrections

1. Every retained #161-#163 predecessor requirement must identify one concrete owner issue and one proof lane.
2. The v3 contract must record the measured #162 construction-slice disposition, the decision criteria or thresholds used, and the #163 / Decision 11 approval binding.
3. The proportional lifecycle default must not permit consumers to skip retained bind, publication, finish, or cleanup gates.
4. Diff hygiene validation must run against an explicit base/head range, not only a clean working tree.

## Validation design

The issue-owned validator `.csdlc/prepared/issues/571/validate-v3a-followup.rb` is the focused proof lane. It parses the affected JSON and Markdown contract surfaces and fails closed for:

- retained predecessor rows missing `owner_issue` or `proof_lane`;
- duplicate predecessor identifiers or duplicated issue/acceptance rows;
- lifecycle defaults that omit retained bind, publication, finish, or cleanup gates;
- construction decisions that omit #162 evidence, #163 / Decision 11 binding, thresholds, or promoted/discarded disposition;
- V3-A diff hygiene that still checks only uncommitted working-tree changes.

The validator intentionally fails against the current stale baseline before implementation, proving that #571 is real corrective work rather than a vacuous readiness patch.

## Review focus

Review must verify that the repaired artifacts are machine-readable enough to guide later v3 implementation, while staying within #571’s narrow corrective scope. In particular, review should reject broad document-section mappings, prose-only construction decisions, lifecycle defaults that silently omit retained gates, and any claim that v3 is already live authority.
