# Design: Closed-Issue Terminal Reconciliation

## Context

Current `csdlc-finish` correctly requires retained review and publication truth for routine work. Several already-closed v0.92 issues predate that retained record shape or crossed the repository migration boundary, even though their terminal GitHub outcomes are exact and observable.

## Decision

Add one compatibility operation to `csdlc-finish`, keeping it the sole terminal authority. The operation accepts an explicit historical-reconciliation request and emits the existing minimal `csdlc.derived_terminal.v1` envelope only after exact live observation.

Every request supplies the canonical issue number, expected generation and digest, issue repository, disposition, actor, and token source. A `merged` request additionally requires PR repository and number, expected PR head SHA, and expected merge SHA, and forbids an approved non-merge reason. A `closed_unmerged` request requires PR repository and number, expected PR head SHA, and an approved reason, and forbids a merge SHA. A `closed_no_pr` request requires an approved reason and forbids every PR identity field. Inapplicable, missing, or contradictory fields fail validation. The result records `source: live_github_historical_reconciliation`; it does not populate or imply review or publication fields.

Allowed outcomes are:

- `merged`: the issue is closed, the exact named PR is merged, its head and merge SHA match, and GitHub reports the exact cross-repository closing reference;
- `closed_unmerged`: the issue and exact named PR are closed, the PR is not merged, and an explicit operator-approved reason is supplied;
- `closed_no_pr`: the issue is closed, carries the canonical no-PR approval label, no PR is supplied, and an explicit operator-approved reason is supplied.

Terminal precedence is deterministic. Exactly one merged closing PR wins over abandoned closed-unmerged attempts; more than one merged candidate fails closed. A closed-unmerged disposition is admissible only when no merged candidate exists and exactly one closed-unmerged candidate matches. Missing linkage, pagination beyond the bounded inventory, repository migration ambiguity, or any mismatch in issue, PR, head, merge, or disposition fails closed.

Routine finish remains unchanged. Historical reconciliation never claims that missing pre-v2 review or publication evidence existed; its distinct provenance records only exact terminal observation under the compatibility operation.

The implementation PR contains only the binary contract and focused tests. After that exact head is reviewed, published, green, and merged, the installed binary reconciles #5800 first. Its retained envelope must pass cached-terminal validation before any other issue is processed. Derived terminal envelopes remain Git-common cache authority and do not modify tracked cards or `main`; `csdlc-clean compatibility-index` materializes read-only compatibility projections from those envelopes when needed. The remainder is a frozen post-merge operational inventory; each issue is independently reconciled and validated without mutating the reviewed implementation head or opening closeout-only PRs.

## Invariants

- `csdlc-finish` remains the only terminal writer.
- Open or ambiguous remote state fails closed.
- Merged outcomes require exact PR, head SHA, merge SHA, repository, and issue-closing linkage.
- No canonical card or terminal envelope is hand-edited.
- The compatibility path cannot weaken normal review-before-publication.
- #5800 failure is a hard stop before the remaining inventory.

## Validation

Focused `gate_finish` tests prove success, idempotency, all identity and disposition mismatches, ambiguous linkage rejection, distinct historical provenance, and unchanged routine finish behavior. After merge, #5800 is reconciled from an exact request and `csdlc-finish --validate-cached-issue 5800` must pass before the frozen remainder begins.
