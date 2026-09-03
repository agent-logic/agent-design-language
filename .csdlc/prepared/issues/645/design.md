# Issue #645 design: stacked closing relation guard

## Problem

`csdlc-publish` can currently accept a pull request body that says `Closes #<issue>` as terminal closing publication evidence even when GitHub does not expose the issue through `closingIssuesReferences`. The observed reproducer is PR #644: its body starts `Closes #631`, but its base is a non-default stack branch and live GraphQL readback reports no closing issue references.

## Design

Treat live GitHub closing relation readback as mandatory authority for closing-mode publication. Body keyword validation remains necessary, but it is not sufficient. For a closing publication, the typed publish path must reconcile the PR state and require `linked_issue == issue` with `linkage_source == github_closing_issues_references`. If that relation is absent, fail closed with operator guidance to wait for the dependency branch merge, publish against the default branch, or choose an explicit non-closing checkpoint route.

Non-closing/checkpoint publication remains available only when the requested linkage mode is explicit and must not be recorded as terminal issue-closing authority.

## Validation

Add an offline regression test for the #631/#644 shape: body contains the closing keyword, base is non-default, and GitHub closing relation is absent. The test must prove closing-mode publication rejects that state. Run the focused C-SDLC v2 publication tests plus formatting/diff hygiene.
