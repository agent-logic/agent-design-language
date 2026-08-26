# Implemented-phase card identity repair contract candidate

## Why this is needed for #112

#112 was decomposed after its authoritative bound branch already existed. The
live GitHub issue title/body are now canonical for the retained core:

`[v0.92][WP-18C.02a][112.a] Define shared Layer 8 signed authority core`

However the six local C-SDLC card value envelopes still carry the old
pre-decomposition identity:

- title: `[v0.92][WP-18C.02] Govern Layer 8 identity, authority, refusal, and audit`
- slug: `wp18c02-layer8-conversation-authority`

Because #119 review and publication require exact title/body/card/review
consistency, #112 cannot truthfully request a new fresh-session review or
publication while this mismatch is known.

## Existing route audit

No supported existing operation repairs this implemented-phase identity drift.

- `csdlc-edit schema` exposes `update_identity_version`, but no title/slug
  identity operation.
- `csdlc-v2/tests/card_identity.rs` covers version-only identity update and does
  not cover title/slug.
- `csdlc-v2/tests/gate2.rs` asserts normal edits preserve identity title and
  slug while only advancing generation.
- `csdlc-issue migrate-bound-topology` is not applicable. It migrates old
  bound-topology records and preserves cards; it is not a post-decomposition
  semantic identity repair route.
- #291 is initialized-phase only and explicitly says its proposed route should
  operate only on initialized records unless later extended.

## Minimal owner

Recommended owner: `csdlc-edit`.

The editor already owns:

- generation/digest CAS
- per-card value projections
- cross-card identity checks
- rendering and hydration
- audit events
- phase-aware semantic operation authorization

## Minimal operation shape

Add one semantic operation:

```json
{
  "operation": "correct_identity_title_slug_after_decomposition",
  "title": "[v0.92][WP-18C.02a][112.a] Define shared Layer 8 signed authority core",
  "slug": "wp18c02a-112a-shared-layer8-signed-authority-core"
}
```

The operation should be submitted through normal `csdlc-edit apply`:

```json
{
  "issue": 112,
  "card": "sip",
  "expected_generation": 77,
  "expected_digest": "713bfd99665a5e29b267391ebfa11d825d739ffe48d2a73c1ae8e4543ad96d0e",
  "actor": "codex-session-112",
  "reason": "repair implemented-phase card identity after approved #112 decomposition",
  "operation": {
    "operation": "correct_identity_title_slug_after_decomposition",
    "title": "[v0.92][WP-18C.02a][112.a] Define shared Layer 8 signed authority core",
    "slug": "wp18c02a-112a-shared-layer8-signed-authority-core"
  }
}
```

The `card` field should be ignored or constrained to `sip`; the operation must
update all six cards atomically so cross-card identity never becomes split.

## Required authorization predicates

The operation should be allowed only when all of the following are true:

1. The issue is in `implemented` phase.
2. `review_assignment`, `review`, `publication`, `readiness`, and `terminal`
   are all absent.
3. The latest review-related audit operation is `recover_review`.
4. The live issue title/body have been exact-read through typed
   `csdlc-github-issue issue_read`, or the request carries a retained evidence
   path and digest for that read.
5. The requested title equals the live canonical issue title from the evidence.
6. The requested title names the retained decomposed issue identity and does not
   claim #265, #270, #271, #114, #115, #116, #117, Runtime ingress, served
   acknowledgement/API protocol, Observatory UI, durable history, rooms, roster,
   or presence ownership.
7. The requested slug is nonempty, normalized, and does not collide with another
   issue record in the local store.
8. The operation changes only card identity `title`, `slug`, and normal
   generation/digest/audit projections.

## Audit record

The audit operation should retain old and new values:

```json
{
  "operation": "correct_identity_title_slug_after_decomposition",
  "previous_title": "[v0.92][WP-18C.02] Govern Layer 8 identity, authority, refusal, and audit",
  "new_title": "[v0.92][WP-18C.02a][112.a] Define shared Layer 8 signed authority core",
  "previous_slug": "wp18c02-layer8-conversation-authority",
  "new_slug": "wp18c02a-112a-shared-layer8-signed-authority-core",
  "live_issue_evidence": ".git/csdlc-v2/requests/112-title-gate-issue-read.json"
}
```

## Required tests

At minimum:

1. Implemented-phase card identity correction updates title/slug across all six
   cards and preserves card content.
2. Correction rejects stale generation/digest.
3. Correction rejects if any review assignment, review result, publication,
   readiness, or terminal truth is present.
4. Correction rejects outside implemented phase unless a separate phase-specific
   contract is deliberately added.
5. Correction rejects empty, malformed, or sibling-claiming title/slug.
6. Correction rejects if live issue evidence title does not match the requested
   title.
7. Correction preserves cross-card identity equality and advances card
   generation.
8. Correction leaves `csdlc-validate issue --issue 112` passing on the #112
   fixture.

## #112 use after tooling exists

After this operation exists and is routed to #112:

1. Apply the typed identity repair to #112.
2. Commit the resulting lifecycle-only repair.
3. Re-run `csdlc-validate issue --issue 112`.
4. Verify all six `.identity.title` and `.identity.slug` values are canonical.
5. Assign a new `fresh-session:<UUID>` reviewer after the repair commit and
   before any review activity.
6. Record PASS only if the fresh reviewer has no actionable findings.
7. Enforce publication title gate before any PR creation.
