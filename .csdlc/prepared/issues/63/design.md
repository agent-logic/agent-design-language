# Issue 63 design: implemented-phase SIP scope correction

## Decision

Add one explicit semantic editor operation,
`correct_declared_scope_before_publication`, owned by SIP and authorized only in
the `implemented` phase when review assignment, recorded review, publication,
and readiness truth are all absent. The operation replaces `declared_scope`
through the existing typed card mutation, render, cross-card validation,
transactional commit, generation, digest, and audit pipeline.

The request continues to use `EditRequest.reason`; this route additionally
rejects an empty actor or reason. Its audit operation records the complete
previous and replacement scope arrays, while the audit event records the actor
and human reason. This makes any widening explicit and reviewable without
trying to infer whether an individual path is semantically broader.

## Exact implementation seam

1. Extend `SemanticOperation` in `csdlc-v2/src/cards.rs` with
   `CorrectDeclaredScopeBeforePublication { values: Vec<String> }`.
2. In `cards::apply`, validate the replacement collection and mutate only
   `SipValues.declared_scope`; every other card fails with `field_ownership`.
3. In `store::authorize_card_operation`, authorize that exact operation only
   for `(implemented, sip)`. Initialized, ready, bound, reviewed, published,
   merge-ready, and closed-out phases remain rejected for this operation.
   Normal pre-implementation planning continues to use
   `replace_planning_collection`.
4. Extend `csdlc-review recover` to accept `implemented` only when a review
   assignment or recorded review is present. Reuse its existing atomic cleanup
   of SRP, SOR, assignment, review, publication, readiness, and terminal truth;
   remain in `implemented` rather than adding a new lifecycle phase transition.
   An ordinary clean implemented record still rejects recovery.
5. In `store::edit_issue`, before mutation, require a non-empty actor and
   reason and require `review_assignment`, `review`, `publication`, and
   `readiness` to be absent. A typed review/publication recovery may return the
   record to `implemented` and clear those fields; only then may the correction
   proceed.
6. Build the audit operation from the verified current SIP card, recording
   `previous_values` and `new_values`. Then use the existing `apply`,
   `validate_cross_card`, renderer, generation/digest, atomic store commit, and
   audit event logic unchanged.

## State and trust boundary

- The typed JSON request is the only mutation input.
- The canonical SIP values AST remains machine truth; Markdown is a rendered
  projection and is never edited directly.
- Expected generation and digest remain mandatory optimistic-concurrency
  guards before authorization or mutation.
- The existing store verifies current JSON/Markdown projections before the
  correction and revalidates/rerenders all cards before atomic commit.
- Review/publication recovery remains owned by `csdlc-review`; this issue
  extends that existing typed operation for the otherwise-dead-end
  implemented-with-review-truth case, without adding a lifecycle phase.

## Proof design

A focused real-CLI regression creates a repository-local issue fixture through
typed bootstrap, advances it through bind/finalize to `implemented`, then runs
`csdlc-edit apply` with the new operation. It proves:

- corrected SIP values and rendered Markdown contain the replacement scope;
- the old and new arrays, actor, and reason are retained in audit truth;
- generation advances once, digest changes, and `csdlc-validate` passes;
- stale generation and digest fail before mutation;
- empty actor/reason, empty replacement, wrong card, and ordinary bound-phase
  use of the repair operation fail closed;
- reviewed/published state rejects the operation;
- assigned or changes-required implemented review truth rejects the editor
  until typed `csdlc-review recover` atomically clears it; clean implemented
  then permits correction;
- an ordinary clean implemented record cannot invoke review recovery, and any
  retained review/publication/readiness truth still rejects the editor;
- direct Markdown drift remains detected by the validator.

Keep the proof in the existing focused editor/lifecycle regression surface. No
workspace-wide test suite, network call, Python helper, shell lifecycle wrapper,
or broad compatibility rewrite is required.

## Non-goals

- Inferring whether a path is a semantic widening or narrowing.
- Permitting arbitrary implemented-phase SIP planning edits.
- Changing direct Markdown ownership, prompt templates, or lifecycle recovery.
- Adding a new lifecycle phase or letting the editor clear review truth.
- Mutating issue #53 or any historical lifecycle evidence.

## Risks and mitigations

- **Silent scope widening:** require the dedicated operation, actor/reason, and
  full old/new audit values for exact-head review.
- **Post-publication rewrite:** phase and cleared-truth guards fail closed; the
  editor cannot perform recovery itself.
- **Projection drift:** reuse existing verified-card, AST render,
  cross-card-validation, and atomic commit path.
- **Overbroad recovery:** accept implemented recovery only when assignment or
  review evidence exists, and retain rejection for an ordinary clean record.
- **Overbroad authorization:** match the exact operation/card/phase tuple and
  retain negative tests for adjacent phases and cards.
