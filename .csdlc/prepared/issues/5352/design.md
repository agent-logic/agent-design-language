# Issue 5352 Design

## Metadata

- Issue: #5352
- Track: v0.91.8 WP-21 exact-revision v0.92 consumption handoff
- Preparation worktree: `/Volumes/FastWork/adl-wp-5352`
- Preparation branch: `codex/5352-v0918-preparation`
- Integrated source revision: `origin/main` at `51bc5ae51b57c19dbab693af1c5a45142995f4e5`
- Preparation boundary: no implementation, PR publication, merge, or closeout

## Scope

Prepare the WP-21 exact-revision v0.92 consumption handoff for later execution.
This packet does not write the final handoff ledger, implement v0.92 work,
publish a PR, claim deployment readiness, mutate GitHub, or close the issue.

## Source Revisions And Dependencies

Future execution must re-check live GitHub state and ancestry against the then
current `origin/main`. Preparation records the current dependency truth observed
after integrating `51bc5ae51b57c19dbab693af1c5a45142995f4e5`:

| Dependency | Current state | Reviewed head | Accepted merge | Current ancestry |
| --- | --- | --- | --- | --- |
| WP-14A integrated platform #5384 / PR #5726 | closed | `71e3b70b8f0d235d768ced0383074345547811d4` | `72fbf30c74a5193ea41f042c76c5986a48e59d6c` | ancestor of `51bc5ae51b57c19dbab693af1c5a45142995f4e5` |
| C-SDLC v2 acceptance #5358 / PR #5606 | closed | `e048230245b1ad101c8056678123a2747faa4b60` | `fc75f4fc697262f89f99461679a406be0b4b3775` | ancestor of `51bc5ae51b57c19dbab693af1c5a45142995f4e5` |
| Runtime v3 acceptance #5361 / PR #5650 | closed | `f7fc71421f4bcf70039b910c9b88b538bb111400` | `f7258b07e9da414bfee518f0c89a76071bc03ee8` | ancestor of `51bc5ae51b57c19dbab693af1c5a45142995f4e5` |
| ADL v2 soak and rollback #5344 / PR #5703 | closed via #5384 input | `141dfa20ccc3753060687259ad933397331df9c7` | `d4825d4be9ed14ed6060dd33cbdafe5eaa5efcd2` | consumed through #5384 |
| ADL v2 reversible default #5343 / PR #5704 | closed via #5384 input | `e4bbc988cad682cbb2ff8d24085e1a99bccec1ce` | `e1b6a34e4763a79d1c40c641e64c0c061a0aa96c` | consumed through #5384 |

Closeout receipts, stale projections, or claim reacquisition records are audit
evidence only. They cannot release #5352 execution without live issue closure
and merge ancestry at the future execution revision.

## Intended Issue-Local Paths

- Lifecycle cards: `.csdlc/issues/5352/cards/{sip,stp,spp,vpp,srp,sor}.md`
- Preparation design: `.csdlc/prepared/issues/5352/design.md`
- Preparation diagram: `.csdlc/prepared/issues/5352/diagram.mmd`
- Preparation review: `.csdlc/prepared/issues/5352/preparation-review.md`
- Preparation fixes: `.csdlc/prepared/issues/5352/preparation-review-fixes.md`
- Preparation validator: `.csdlc/prepared/issues/5352/validate_preparation.rb`
- Future ledger path: `docs/milestones/v0.91.8/handoff/issue-5352-v092-consumption-handoff.md`
- Evidence path: `.csdlc/evidence/5352/preparation/`

## COTS And Tool Boundary

- COTS/local tools: `git`, read-only `gh issue view`, `ruby`, Mermaid source,
  and typed C-SDLC v2 binaries when execution begins.
- No AWS, provider credentials, Unity, browser automation, remote model
  execution, external publication, or PR mutation belongs to preparation.
- The requested gpt-5.5 preparation-review lane is represented as a bounded
  review artifact; this tool context does not expose a separate callable
  gpt-5.5 endpoint, so no external model call is claimed.

## LoC And Time Budgets

- Preparation artifact budget: at most 350 nonblank lines across issue-local
  preparation markdown and Ruby validator changes.
- Future handoff ledger budget: target at most 300 nonblank lines, with tables
  for exact revisions, contracts, rollback boundaries, and residual risks.
- Future product-code budget: 0 LoC unless execution-time review explicitly
  discovers a docs-tooling bug in the handoff path and the operator widens scope.
- Preparation validation budget: 30 seconds local small lane.
- Future execution validation budget: 1,800 seconds and 12,000 tokens before PR
  review, with broader checks deferred only by explicit fail-closed rationale.

## PVF Lanes

- `preparation-contract`: deterministic small lane validating six cards, design,
  diagram, review/fix artifacts, no implementation phase, no active claim
  requirement, and no publication/merge/closeout state.
- `dependency-ancestry`: deterministic execution-time lane checking #5384,
  #5358, and #5361 are closed and their accepted merges are ancestral to current
  `origin/main`.
- `handoff-ledger-docs`: deterministic execution-time docs lane validating the
  future ledger path, links, YAML/doc cross-references, exact revisions, COTS,
  rollback, and residual-risk tables.
- `pre-pr-review`: bounded gpt-5.5/external-review lane before publication,
  with every actionable finding fixed or explicitly deferred fail-closed.

## Rollback And No-Deferral Criteria

- Rollback boundary must name the WP-14A rollback window ending
  `2026-08-12T09:04:24Z`, the ADL v2 selector rollback report, and the
  `deletion_authorized: false` truth from the WP-14A ledger.
- No deferral may hide missing exact revisions, missing stable binary/schema
  paths, absent merge ancestry, stale review truth, unsupported birthday claims,
  or Adaptive Learning implementation claims.
- If any required dependency is not closed and ancestral at execution time,
  #5352 remains blocked instead of using receipts, claims, or closeout records
  as substitutes.

## Future Implementation Plan

1. Re-check #5384, #5358, and #5361 live issue/PR state and ancestry against
   current `origin/main`.
2. Gather exact reviewed revisions, stable-install provenance, schema/binary
   contracts, rollback boundaries, residual risks, and child disposition truth
   from accepted ADL v2, Runtime v3, and C-SDLC v2 evidence.
3. Write the handoff ledger at
   `docs/milestones/v0.91.8/handoff/issue-5352-v092-consumption-handoff.md`.
4. Run focused docs/link/diff validation and one exact pre-PR review before any
   publication.
