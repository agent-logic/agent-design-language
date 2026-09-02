# Issue #51 design: podcast parent closeout readiness

Prepare the parent coordination closeout lane for `The Cognitive Stack` after the four podcast children.

The parent does not own provider submission, provider account mutation, mailbox-code handling, destination-link activation, public-launch announcement, or child implementation. It owns a reconciled launch-status view and the final parent decision about whether child truth permits closeout.

## Current routing

- #261, #262, and #263 are closed on GitHub and have retained child evidence in the repository.
- #264 is published as PR #649, green and mergeable, but not merged at this preparation point.
- #264 records a blocked external-action disposition: repository-side submission-gate materials are complete, while provider submissions and public-launch work remain blocked until future explicit operator authorization.
- #51 can close only if the operator explicitly accepts that #264 blocked disposition for parent routing after #649 merges.

## Prepared next action

After #649 merges, the #51 execution worker should:

1. Verify #261, #262, #263, and #264 live GitHub closure and merged PR truth.
2. Verify retained repository evidence agrees on show title `The Cognitive Stack`, feed URL, production audio/feed status, directory runbooks, and #264 no-submission ledger state.
3. Confirm whether the operator accepts #264's blocked external-action disposition for #51 parent routing.
4. If accepted, record parent reconciliation, exact-head review, publication, finish, and cleanup.
5. If not accepted, leave #51 open with the exact remaining external-action gate.

## Non-goals

- No directory submission.
- No provider account access.
- No public launch announcement.
- No destination-link activation.
- No parent closeout before #649 merge and explicit operator acceptance.
