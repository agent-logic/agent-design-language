# #292 design brief: implemented card identity repair

Implement a bounded `csdlc-edit` semantic operation named
`correct_identity_title_slug_after_decomposition`.

The operation exists to repair implemented-phase card identity drift after an
issue has been decomposed and the live GitHub issue title is the canonical
retained parent/child title. It must update title and slug atomically across all
six card value envelopes while preserving card content, generation/digest CAS,
rendering, and audit truth.

The first fixture is #112, but this tooling issue must not mutate the #112
worktree. #112 is read-only evidence for fixture construction only.

Required predicates:

- issue phase is `implemented`;
- no review assignment, review result, publication, readiness, or terminal truth
  exists;
- latest review-related audit state is compatible with review recovery;
- typed live issue evidence binds the requested title to the current GitHub issue
  title;
- requested title is the decomposed retained issue identity and rejects sibling
  scope claims;
- requested slug is normalized, nonempty, and does not collide locally;
- only identity title, identity slug, generation, digest, rendering, and audit
  projections change.

Review-audit compatibility is intentionally narrow. The operation must identify
the latest audit event whose structured operation is review-related:
`approve_design`, `assign_review`, `record_review`, `recover_review`,
`publish`, `record_readiness`, or terminal/finish materialization. Compatible
states are either no such event, or a latest `recover_review` event that
explicitly clears downstream review/publication/readiness authority while
leaving the record in implemented phase. Incompatible states include any latest
review assignment, review result, publication, readiness, terminal/finish event,
manual or unknown review-like audit mutation, or recovery event that does not
clear the downstream authority fields required above. Tests must include one
positive no-review-history case, one positive current `recover_review` case, and
negative cases for stale assignment/review/publication/readiness/terminal and
unknown review-like audit operations.

Non-goals:

- no #112 lifecycle mutation;
- no #291 initialized-phase recovery expansion;
- no direct Markdown/state editing;
- no product behavior changes.
