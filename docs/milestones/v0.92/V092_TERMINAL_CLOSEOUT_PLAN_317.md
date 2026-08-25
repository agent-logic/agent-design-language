# v0.92 Terminal Closeout Plan (#317 / WP-28A)

## Decision

The v0.92 release tail advances on independently reviewed, green merges whose
merge commits are ancestral to `main`. Typed finish, worktree cleanup, sprint
bookkeeping, and handoff reconciliation are asynchronous evidence maintenance;
they are never successor execution gates.

This plan is documentation-only. It does not merge, finish, clean, close, tag,
release, or activate another milestone.

## Canonical Identity

The old `danielbaustin/agent-design-language` issue numbers are immutable
provenance, not current routing authority.

| Work package | Legacy provenance | Canonical issue | Current role |
|---|---:|---:|---|
| WP-26 | #5847 | #314 | external review |
| WP-27 | #5848 | #315 | review remediation |
| WP-28 | #5849 | #316 | next-milestone planning |
| WP-28A | #5850 | #317 | terminal closeout plan |
| WP-29 | #5851 | #318 | independent next-milestone review |
| WP-30 | #5852 | #319 | release evidence and ceremony |

The machine-readable denominator is
`.csdlc/evidence/317/issue-universe.json`. Exactly one row must exist for each
mapping above. Missing, duplicate, ambiguous, extra, or self-declared rows fail
validation.

## Execution Gates

1. #316 / PR #472 was independently reviewed at exact head
   `8478f11e21a34530ba07bf64afc260e7a6eedd33`, passed its required routed
   checks, and merged as `5002b387b79f2d8dbf41a8c1a99e5a03bcb5c5d5`.
   That ancestral merge opens #317.
2. #317's independently reviewed green merge opens #318. Its typed finish and
   cleanup may occur later.
3. #319 requires both the independently reviewed green merge of #318 and the
   completed, independently reviewed remediation merge(s) owned by #315. It
   does not wait for typed finish or cleanup receipts.
4. #319 alone owns tag/release mutation and the release ceremony. #317 merely
   describes the sequence.

#314 and #315 are a review/remediation lane. #316 and #317 did not serialize
behind their bookkeeping. #315's substantive remediation must nevertheless be
landed before #319 can claim a reviewed release candidate.

## Evidence Model

`validate-closeout-plan.rb observe` is the only nondeterministic mode. It reads
GitHub issue and closing-PR state and retains:

- observation time and repository identity;
- canonical and legacy issue identity;
- issue state and source-response digest;
- closing PR identity, base, head, merge, checks, reviews, and response digest.

The `universe`, `dag`, `negative`, and `all` modes are deterministic over the
retained envelope and tracked repository state. The universe binds the exact
SHA-256 of that envelope, preventing a replacement self-attested snapshot from
silently satisfying the contract. Exact-head independent review remains bound
to tracked C-SDLC review evidence rather than inferred from an empty GitHub
review list.

## Current Disposition

| Issue | Classification | Owner | Next action |
|---:|---|---|---|
| #314 | review complete | WP-26 | retain review inputs; asynchronous reconciliation only |
| #315 | remediation in progress | WP-27 | land every actionable finding with focused proof and exact-head review |
| #316 | reviewed green merged | WP-28 | no execution gate remains; finish/cleanup asynchronously |
| #317 | active planning | WP-28A | validate and independently review this plan, then merge |
| #318 | queued | WP-29 | begin after #317's reviewed green ancestral merge |
| #319 | ceremony queued | WP-30 | begin after #318 and #315 reviewed green ancestral merges |

## Rollback And Stop Conditions

Discard and regenerate the candidate evidence if observation identity, mapping,
head, check, review, merge, or ancestry truth cannot be reproduced. Stop before
ceremony if the DAG contains a cycle, unknown or unowned node, partial release
identity, or any finish/cleanup/bookkeeping node used as a gate.

No remote mutation is required to roll back this plan.
