# #112 title identity gate audit

## Current gate

#112 publication and fresh-session review are held until exact title/body/card/review
consistency is true, or until the canonical C-SDLC v2 standard explicitly
permits a retained historical card identity label after decomposition.

Canonical live issue title, exact-read through `csdlc-github-issue`:

`[v0.92][WP-18C.02a][112.a] Define shared Layer 8 signed authority core`

Typed read request:

`.git/csdlc-v2/requests/112-title-gate-issue-read.json`

The live issue body is core-only and names:

- `112.a / #112`: CORE shared Layer 8 signed authority domain and primitives
- `112.b / #265`: Runtime kernel conversation ingress enforcement
- `112.c / #270`: trusted recipient-acknowledgement Runtime API protocol
- `112.d / #271`: Observatory UI/tooling integration

## Local implemented record

Worktree:

`/Volumes/FastWork/adl-worktrees/adl-issue-112-layer8-authority-preparation-v2`

Current committed checkpoint:

`e7aa9eadf51d2899205ba8742b52bd0d93de3924`

C-SDLC issue state:

- phase: `implemented`
- generation: `77`
- digest: `713bfd99665a5e29b267391ebfa11d825d739ffe48d2a73c1ae8e4543ad96d0e`
- review assignment: `null`
- validation: `csdlc-validate issue --issue 112` reports `status: pass`, `findings: []`

## Repaired through supported typed post-recovery routes

The following semantic repairs were made through `csdlc-edit apply` after
typed review recovery:

- SIP declared scope now names canonical issue identity/title.
- STP deliverables now name canonical issue identity/title.
- SPP invariants now name canonical issue identity/title and #265/#270/#271/#114
  non-ownership.
- SRP review prompts now require checking canonical issue/title and no
  #265/#270/#271/#114 claims.
- SOR follow-ups record the remaining title/card-identity limitation and hold
  publication/fresh-session review.

## Remaining inconsistency

All six rendered card value envelopes still retain historical identity title and
slug metadata:

- title: `[v0.92][WP-18C.02] Govern Layer 8 identity, authority, refusal, and audit`
- slug: `wp18c02-layer8-conversation-authority`

Observed in:

- `.csdlc/issues/112/cards/sip.values.json`
- `.csdlc/issues/112/cards/stp.values.json`
- `.csdlc/issues/112/cards/spp.values.json`
- `.csdlc/issues/112/cards/vpp.values.json`
- `.csdlc/issues/112/cards/srp.values.json`
- `.csdlc/issues/112/cards/sor.values.json`

This is not a VPP-only issue; VPP is the clearest visible example because its
content did not otherwise need semantic repair, but the hidden identity metadata
is stale across all six card value files.

## Supported-route audit

`csdlc-edit schema` exposes `update_identity_version` only. It does not expose a
typed operation to update card identity title or slug.

Implemented-phase `csdlc-edit apply` supports the semantic repairs listed above,
but not arbitrary SIP/STP/SPP/VPP/SRP/SOR field surgery and not card identity
title/slug replacement.

An attempted direct SIP `set_field required_outcome` during implemented phase
failed closed with:

`csdlc-edit: sip mutation is not allowed during implemented`

That failed request did not advance generation/digest.

## #291 non-coverage

Typed read request:

`.git/csdlc-v2/requests/291-scope-read-for-112-title-gate.json`

Live #291 title:

`[v0.92][C-SDLC][defect] Recover initialized decomposed issue cards without rewriting history`

#291 explicitly addresses initialized-phase recovery. Its proposed contract says
the new route should operate only on `initialized` issue records unless
explicitly extended later. Therefore #291 does not currently authorize
implemented-phase #112 card identity title/slug repair.

## Publication and review disposition

Do not assign a new #119 fresh-session reviewer or publish #112 while this
known cross-card identity inconsistency remains unresolved or undispositioned.

Smallest supported next route:

1. Extend typed C-SDLC v2 recovery tooling, probably in `csdlc-edit`, to support
   implemented-phase post-decomposition card identity title/slug repair under
   generation/digest CAS and cleared review/publication truth.
2. Or record an explicit canonical standard decision that retained historical
   card identity title/slug metadata is permissible after live issue
   decomposition when semantic card truth, live GitHub issue title/body, PR
   title, and review assignment all use the canonical split identity.
3. Then rerun #112 title/card consistency checks, assign a new canonical
   `fresh-session:<UUID>` reviewer with no inherited implementation context,
   and only publish after that exact-head review passes.
