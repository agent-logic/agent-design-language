# Issue 291 design

## Purpose

Implement a typed `csdlc-edit` recovery route for initialized C-SDLC v2 records
whose card semantics became stale after issue-graph decomposition, without
rewriting historical design evidence or mutating the decomposed product issues.

The concrete golden fixture is current #114 generation 35 in:

- `/Volumes/FastWork/adl-worktrees/adl-issue-114-durable-history-preparation`

That fixture must be read-only. The recovery route must prove that #114 can be
transformed from stale implementation-parent wording to coordination-only parent
truth while preserving generation/digest CAS, append-only audit history,
unbound branch/worktree state, and byte-identical historical design and diagram
evidence.

## Required contract

Add a typed initialized-phase post-decomposition recovery operation owned by
`csdlc-edit`. The route must:

- operate only on initialized issue records;
- require exact issue, expected generation, and expected digest;
- require actor, reason, and explicit recovery scope;
- require preserved design and diagram byte and authored digests;
- preserve existing design and diagram files byte-for-byte;
- distinguish historical/reference design evidence from current implementation
  authority;
- atomically replace selected semantic fields needed for design/card review:
  identity title/slug, STP acceptance criteria, SPP plan steps and selected
  planning fields, SRP review scope/prompts/result, and SOR preparation truth;
- keep branch/worktree null and avoid bind, publication, merge, closeout,
  GitHub writes, #112 worktree mutation, or product implementation;
- fail closed with typed JSON diagnostics for stale CAS, unsupported phase,
  preserved-evidence drift, unsafe scope, and invalid child graph.

The request/result contract must also record any design-review recovery as exact
old/new authority truth. The #291 bootstrap defect falsely claimed reviewer
`operator:planning-1-assignment`; current #291 state is intentionally pending
and approval cannot be inferred from Planning assignment. Current typed v2 has
no standalone initialized design-approval reversal audit event, so R2 review
finding `R2-P1-1` is retained in `.csdlc/evidence/291/review-r2-findings.json`
and the new route must make exact reversal audit an acceptance requirement
rather than hand-editing existing audit.

## Transaction semantics

Recovery must be a single CAS transaction implemented with a concrete
write-ahead journal, not with an asserted multi-file atomic rename.

The required primitive is:

1. Resolve one canonical request root and load the issue record, cards, audit
   projection, design file, and diagram file through that root.
2. Validate every requested replacement, graph input, preserved evidence digest,
   scope guard, identity update, and non-goal guard before writing any target
   path.
3. Render and hydrate every would-be output in memory, including changed card
   values, rendered Markdown, index projection, audit JSONL append text, and
   any generated manifest metadata.
4. Write every post-state byte to content-addressed staged blobs under an
   issue-local journal directory such as
   `.csdlc/issues/<issue>/.recovery-journal/<txid>/blobs/`. The blob manifest
   records target relative path, preimage hash, postimage hash, byte length,
   executable mode if relevant, and whether the target is append-only audit or
   replace-in-place.
5. Fsync each staged blob, the blob directory, the manifest file, and the
   journal transaction directory. If a platform cannot fsync a directory, the
   implementation must either use the nearest supported durable-directory
   primitive and record a typed degraded-durability result, or fail closed for
   publication-grade recovery.
6. Write and fsync `manifest.prepared.json` only after all staged bytes and
   preimage checks are complete. The prepared manifest is the durable point of
   no return: readers and recovery code must ignore transactions without a
   prepared manifest, but any transaction with a prepared manifest must be
   completed by roll-forward to the exact recorded post-state.
7. Apply the transaction target-by-target only after rechecking all preimage
   hashes and the expected generation/digest CAS. For each target, write to a
   same-directory temporary file, fsync the file, atomically rename it over the
   target, then fsync the target parent directory. Append-only audit is written
   by constructing the full post-state audit file and replacing it with the same
   temp-file/rename/parent-fsync sequence so readers never observe a torn line.
8. Write and fsync `commit.marker` only after every target path has the recorded
   postimage hash on disk and the post-state index digest matches the manifest.
9. Recovery startup scans issue-local journal directories:
   - no prepared manifest: remove the transaction as abandoned;
   - prepared manifest without commit marker: re-verify every target hash is
     either the manifest preimage or postimage, then complete every target to
     the recorded postimage using staged blobs; the externally visible result
     must become the exact post-state and new generation/digest;
   - commit marker present: verify every target equals the manifest postimage;
     complete any missing postimage write using staged blobs, then report the
     exact post-state and new generation/digest;
   - any target hash outside the manifest's preimage/postimage set fails closed
     as manual intervention required and must not be overwritten by recovery.
10. Repeating a successful request with the old generation/digest must fail
    stale without mutation. Repeating with the current digest and byte-identical
    requested truth may return a typed no-op/idempotent result only when no new
    journal, generation, or recovery audit event is added.

Crash tests must inject failures before prepared-manifest fsync, after
prepared-manifest fsync, after individual target replacement, before
parent-directory fsync, and after commit-marker fsync. Each injection must prove
deterministic reader and recovery behavior: before a durable prepared manifest
the transaction is abandoned and the exact pre-state remains valid; after a
durable prepared manifest the transaction is completed to the exact post-state.
No mixed values/rendered/index/audit state is accepted as valid, and unexpected
hashes fail closed without repair.

## Request root, path, and fixture safety

The route must require a canonical request root and command context:

- the request root and command cwd must resolve to the same repository identity
  or the request must fail closed with typed JSON;
- all issue, card, design, diagram, evidence, and fixture paths must be
  repository-contained after canonicalization;
- symlinks, `..` escapes, absolute target rewrites, mismatched issue numbers,
  mismatched repositories, and path identity drift must be rejected before
  mutation;
- the #114 golden fixture must be copied to an isolated issue-owned golden root
  before any mutation tests run; tests must prove the live #114 worktree and
  root `.csdlc/locks/114.lock` are unchanged;
- request output must name the canonical root, cwd, issue, expected generation,
  expected digest, design/diagram byte digests, authored digests, and issue path
  identity that were actually verified.

## Generic graph input

The graph validator must be generic. The request must provide typed nodes and
directed edges:

- node id, issue id, role, repository, and in-scope status;
- directed dependency edges with explicit orientation;
- one parent integration owner node;
- acyclic ordering from child decomposition through parent integration;
- explicit rejection of missing nodes, inverted edges, duplicate roles,
  out-of-scope nodes, and cross-child trust redefinition such as #270-style trust
  semantics being moved into a sibling without declared authority.

The concrete `#276 -> #277 -> #278 -> parent` chain belongs only in the #114
golden fixture and must not be hard-coded into the production route.

## Semantic replacement surface

The allowed field set must be explicit and closed:

- SIP title/slug/version identity, required outcome, declared scope, authority
  boundary, assumptions, and operator constraints when requested;
- STP task boundary, deliverables, acceptance criteria, repo inputs,
  dependencies, and non-goals when requested;
- SPP affected areas, invariants, risks, replan triggers, stop conditions,
  summary, and plan steps when requested;
- VPP validation lanes, failure policy, and design/diagram bindings when
  requested;
- SRP review scope, prompts, result reset to pre-review, findings clearing only
  as part of explicit recovery truth, and residual risk when requested;
- SOR preparation-only summary, artifacts, validation/follow-up placeholders,
  and nonterminal integration/publication/merge/closeout states when requested.

Shared identity fields must be updated consistently across every card projection
or the request must fail before writing.

## Golden fixture

The #114 fixture at generation 35 is the proving input:

- generation: 35
- digest: `3ceb6fa642b537692a097960e4c216f354de777ece387cc82c3b8022e27b2e51`
- phase: initialized
- branch/worktree: null / null
- design byte SHA-256:
  `1017526669c138e76ed815304afaddd316665149ce9739b2b143f6827936a2c8`
- design authored digest:
  `b70cf7e77e06cad287166597bf0b70bfcd43392f6452325c905dcec6fab65c08`
- diagram byte SHA-256:
  `06818f0c057e01a83e9c54c7f7a7812b20565ce9a371883b17bf46d48d69760f`
- diagram authored digest:
  `b8e5984d673cff4cb398de9deeb653bfb7dda81c388243371d91bd7562bebf42`

The fixture's child graph is #276 -> #277 -> #278 -> parent integration. That
chain is a golden input for tests only; production graph validation must use the
generic typed graph input above and fail closed if the fixture is missing,
inverted, out of scope, or attempts cross-child trust redefinition.

## Non-goals

This issue does not implement #114, #276, #277, or #278 product behavior; does
not bind #114; does not mutate #114, #112, or root lock state; does not create,
update, or close other GitHub issues; and does not replace typed lifecycle/card
mutation with hand-edited rendered Markdown.

## Review requirement

Before publication, commit the exact substantive head, assign a #119-compliant
fresh review session before review activity, record a canonical fresh-session
UUID/no-inheritance proof where supported, resolve findings in the
implementation session, and repeat with a new fresh session after any
substantive fix.
