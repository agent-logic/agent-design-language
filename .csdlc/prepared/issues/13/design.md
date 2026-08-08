# Issue #13 design: explicit optional coverage producer routing

## Problem

The canonical path-policy job exposes only the broad `coverage_required` and
`full_coverage_required` booleans. Runtime-local validation profiles therefore
cannot express “run Runtime coverage, skip both workspace producers.” In run
`31143017130`, the new `validate_v092_browser_trusted_observatory.mjs` surface
was unmapped, so path policy truthfully emitted both booleans as `true` under
its fail-closed rule. The existing workspace job-level guard honored that
value and launched both shards. The defect is the overly broad classification
and missing Runtime-only producer selector, not a false job guard.

The required aggregate check also infers expected producer results from the two
broad booleans. That makes an intentionally skipped producer harder to
distinguish from a routing defect.

## Design

Keep the existing validation-manager and path-policy authority, but make its
producer selection explicit:

- `runtime_coverage_required`
- `workspace_fast_coverage_required`
- `workspace_full_coverage_required`

The existing `coverage_required` remains the aggregate “some coverage work is
required” signal, and `full_coverage_required` remains the compatibility signal
for authoritative full coverage. The only valid selector states are:

| Route | Runtime | Workspace fast | Workspace full |
|---|---:|---:|---:|
| No coverage | false | false | false |
| Runtime only | true | false | false |
| Focused workspace | false | true | false |
| Authoritative full | true | false | true |

Every other combination is invalid. The `adl_path_policy` job validates the
truth table and fails before producer jobs become eligible; the aggregator
revalidates it as defense in depth.

Add a bounded Runtime-owner route for a ready validation-manager profile whose
only selected lanes are Runtime owner proof, focused Rust proof, docs hygiene,
and the HTML Observatory contract. That route selects Runtime coverage without
selecting either workspace producer. Map the v0.92 HTML Observatory validator
to its existing Observatory contract lane so it no longer creates an unmapped
surface escalation.

Every producer job consumes its validated selector in its job-level `if` expression,
combined with the existing hosted-versus-Spot decision. No optional hosted
producer performs checkout, toolchain installation, cache setup, or coverage
unless its selector is true **and** the hosted route is active. The hosted
aggregator computes expected execution as `producer selected AND hosted route
active`; it treats GitHub `skipped` as the only valid result otherwise. The
same-repository `ci:spot` opt-in behavior is preserved. Runtime-only provenance
is checked without installing workspace aggregation tools.

## Invariants

1. Full coverage still runs Runtime coverage plus both workspace shards.
2. Runtime-local coverage runs only the Runtime producer.
3. Focused workspace coverage runs only the fast workspace producer.
4. No-coverage changes select no producer and the required aggregate succeeds.
5. A selected producer that is skipped, or an unselected producer that runs,
   fails the aggregate contract when the hosted route is active; a selected
   producer is expected to be skipped when the approved Spot route is active.
6. Coverage thresholds and Runtime TLS behavior are unchanged.
7. Invalid selector combinations fail in `adl_path_policy` before artifact
   work, and are rejected again by the aggregator.

## Validation

- Extend path-policy fixtures with the exact Runtime + HTML Observatory +
  lifecycle-record surface from PR #9 and assert Runtime-only producer output.
- Extend workflow contract tests to assert all three job-level guards and the
  selected/unselected aggregator matrix.
- Run the focused path-policy and CI runtime contract suites plus diff hygiene.
- Retain one live Actions canary showing that the Runtime-local route leaves
  both workspace matrix jobs in GitHub `skipped` state with no runner start,
  while Runtime coverage, `adl-coverage-hosted`, and `adl-coverage` succeed.

## Rollback

Revert the producer-selector outputs, job guards, Runtime-owner focused route,
and associated tests together. The prior broad booleans remain intact during
the change, so rollback does not require data migration.
