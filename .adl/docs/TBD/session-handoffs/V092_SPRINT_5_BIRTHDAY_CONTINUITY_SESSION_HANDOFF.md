# v0.92 Sprint 5 Birthday Continuity Session Handoff

## Documentation Scope

- Sprint: v0.92 Sprint 5, **Demonstration, Handoff, and Publication**.
- Umbrella issue: `#5854`.
- Current blocking lane: `#5836` / WP-18, blocked by corrective issue `agent-logic/agent-design-language#237`.
- Audience: a brand-new Codex task with no conversation history.
- Mode: session-recovery handoff only.
- Publication attempted: false.
- Release approval claimed: false.
- Review approval claimed: false, except where an exact retained review is identified below.
- Broad rewrite performed: false.

The operator's stand-by remains in force except for preparing this file. Do not
resume implementation, testing, CI, publication, cleanup, lifecycle mutation,
or delegation until the operator explicitly says to resume.

## Canonical Sprint Boundary And Order

Current `origin/main` records the canonical Sprint 5 packet at
`.csdlc/prepared/issues/5854/sprint-execution-packet.md`:

1. `#5835` / WP-17.
2. `#5836` / WP-18.
3. `#5838` / WP-18B only after `#5836` is terminal.
4. `#5839` / WP-19 only after `#5835` is terminal and a v0.93 allocation and accepting owner are explicit.
5. Integrated Sprint 5 review and umbrella `#5854` closeout only after all four operative children are terminal.

WP-20 `#5840` belongs to release-tail Sprint `#5856`, not Sprint 5. WP-24
`#5844` is a completed product stream, not an execution child. WP-24A `#5845`
is out of band and cannot gate Sprint 5.

## Current Repository And Worktree State

- Current bound worktree for the blocking corrective issue:
  `/Volumes/FastWork/adl-worktrees/adl-issue-237-continuity-binding-reconciliation`
- Current branch: `codex/237-continuity-binding-reconciliation`
- Current committed HEAD: `d5344a88fda5ec8178cd3a7dbeb689fd918ba221`
- Current typed phase: `implemented`, generation `20`, digest
  `bc0f0cfb56a5644dec890e6b1ed2780dd6b4397af96b8ad5cef642e0a60290f2`.
- Current `origin/main`: `e172257b50ec9d6e07bbb0ab62a69a001ad1774f`.
- The primary checkout was on `main` at
  `1567469e395f9a6ea6c2e736366a8008f5ee1e06`, behind `origin/main`; do not
  write there or use it as current execution truth.

Parent WP-18 worktree:

- Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-5836-wp18-first-birthday-demo`
- Branch: `codex/5836-wp18-first-birthday-demo`
- Committed HEAD: `1567469e395f9a6ea6c2e736366a8008f5ee1e06`
- Typed phase: `bound`, generation `50`, digest
  `d1477766bcc491d43566a4971a744126ee34f77538d6652182f9d06bd7a6093a`.

Umbrella coordination worktree:

- Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-5854-sprint5-readiness`
- Branch: `codex/5854-sprint5-readiness-current`
- Local HEAD: `18532536a3746ed63f2d5b5c11ec607d28d82e3d`
- Remote branch head: `6223ba4c42cef563c77311c9bc95d74cd44b8d7a`
- Readiness PR `#224` merged as
  `83d8f41bd55f068d754b9b62d6bc5b5fdceb970a`; this was readiness only, not Sprint 5 closeout.

## Exact Issue And PR State

| Issue | State | Exact PR/head/merge truth |
|---|---|---|
| `#5835` WP-17 | Product delivered and dependency-satisfying | PR `#238`, exact PR head `0a607266287458e34e41c7f600b571dc3a23ed03`, squash merge `a4c14b4ae51ec5fbc3c3b585b217958972a3246c`. Exact review approved substantive head `1d765ecfd61fcd7bccd50b37d0dc660a8ccf9f43`. |
| `#239` terminal-envelope follow-up | Terminal and merged | PR `#240`, exact PR head `a7b672d768dfe5e1aec1fac3036744f6aeb7307f`, squash merge/current `origin/main` `e172257b50ec9d6e07bbb0ab62a69a001ad1774f`. Exact review approved implementation `a89ff0333f4a21844dd0e3504d1bf338908038ac`. Cached `#5835` and `#239` validation passed after merge. |
| `#237` continuity authority correction | Active, no PR, not published, not merged | Last committed metadata HEAD `d5344a88f...`; last assigned substantive revision `4ae0125e46bc3b36162662d412713881a5e33bf9` received **CHANGES_REQUESTED**. Review assignment was recovered; current `review_assignment`, `review`, `publication`, and `terminal` are null. |
| `#5836` WP-18 | Active bound WIP, no PR | No implementation review, publication, or merge. Its integrated positive is blocked on `#237`. |
| `#5838` WP-18B | Not bound or started | No branch/worktree/PR. Gate: terminal reviewed `#5836` plus its other declared dependencies. |
| `#5839` WP-19 | Not bound or started | No branch/worktree/PR. `#5835` is terminal, but the repository still describes the v0.93 WBS as candidate allocation; an accepted allocation and accepting owner have not been verified for execution credit. |
| `#5854` umbrella | Active, not closed | Do not perform integrated review or closeout until `#5836`, `#5838`, and `#5839` are terminal and child truth is reconciled. |

Retained issue indexes for squash-merged `#5835` and `#239` remain
pre-merge `published/open` records in the merge trees. Do not mistake that
retained projection for live GitHub state. The repository-grounded #239 fix
makes derived cached terminal validation authoritative across the metadata-only
publication head.

## Completed Work

### WP-17 and terminal-envelope reconciliation

- `#5835` produced the reviewed cross-polis continuity/migration plan and merged through PR `#238`.
- The squash merge exposed a C-SDLC v2 terminal-envelope bug when a clean metadata-only publication head followed the recorded publication revision.
- `#239` repaired the root-aware matcher across finish, cached validation, and cleanup consumers by reusing `git::metadata_only_changed_paths` and retaining fail-closed ancestry/scope checks.
- PR `#240` merged with one large runner maximum and optional jobs skipped.

### WP-18 and continuity-authority correction

- `#5836` has a production-oriented birthday-demo module, binary, focused test,
  fixture, and typed replan materialized but uncommitted.
- Its real composition exposed an incompatibility between capability and
  cognitive continuity bindings. This was routed to dedicated issue `#237`
  rather than bypassed in the demo.
- `#237` made raw capability/governed-cognition authority APIs crate-private,
  added opaque `VerifiedBirthdayContinuity`, removed the permissive either/or
  predicate, added real `LiveContinuity` composition, and added public-boundary
  compile-fail proof.
- The latest exact review then proved that caller-recomputable envelope fields
  still permit a valid token-B rewrite. A new design now requires a
  crate-private Runtime-established opaque `CapabilityAuthorityPolicy` binding
  the canonical provisioned policy digest and exact continuity head/record
  digest. The current design SHA-256 is
  `2c3e0b8b8ea07c7916dc8e4f41584524cec5d91ebd12c6afc582440550798da3`;
  diagram SHA-256 is
  `e502f00461f7f395a85947bb739b871797ff3383263abbc367d83a450fbac865`.
- That opaque-authority design is independently approved, but its source
  implementation and proof have **not** been performed.

## Validation Actually Run

For committed `#237` revision `4ae0125e4` before the latest P1 review:

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --locked --test capability_envelope`: `1/1` passed.
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --locked --lib`: `79/79` passed.
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --locked --doc`: `8/8` passed.
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --locked --lib -- -D warnings`: passed.
- Retained logs exist under `.csdlc/evidence/237/`.

These checks do **not** prove the new opaque `CapabilityAuthorityPolicy` design
and did not catch the self-consistent token-B capability rewrite. They must not
be reused as completion evidence after the next source change.

For `#5836` before opening `#237`:

- Focused target compiled.
- Three of four focused tests passed.
- The real positive failed with typed `[ContinuityMismatch, PrivacyViolation]`;
  the structural continuity mismatch is the reason `#237` exists. Do not claim
  the WP-18 positive is green.

For `#239`:

- Focused PR-238 regression: `1/1` passed.
- Full `gate_finish`: `18/18` passed.
- Cached `#5835` validation passed with `canonical_match=true` before and after merge.
- Required CI run `31544962208` passed; one large `csdlc-v2-standalone` runner was used, standard routing/tooling passed, and optional lanes were skipped.
- One unrelated `gate_cleanup` historical census test failed identically on the untouched base and was not caused by `#239`.

For this documentation-only recovery:

- Read-only branch/worktree/index/diff inspection was performed.
- Current `#237` design and diagram SHA-256 values were recomputed.
- `git diff --check` passed before adding this handoff.
- No implementation test, CI job, lifecycle command, publication, cleanup, or broad validation was run.

## Unresolved Findings And Blockers

1. **P1 — #237 opaque capability authority is not implemented.** The reviewed
   `4ae0125e4` implementation accepts a self-consistent token-B substitution
   when the caller rewrites and re-hashes the capability and rebuilds/re-signs
   cognition. The exact reviewer required a non-caller-recomputable authority
   anchor. The approved replacement design exists only in dirty design/card
   state.
2. **#237 execution truth is stale.** `sor.values.json` still describes the
   rejected exact-token implementation as green. It must be replaced after the
   opaque-authority implementation and fresh proof; do not hand-edit cards.
3. **#5836 is blocked on #237.** Its WIP must remain preserved and must not be
   rebased, finalized, or published until `#237` merges and is ancestral.
4. **#5838 is serially blocked on terminal #5836.**
5. **#5839 lacks verified execution authority.** The repository contains a
   v0.93 planning allocation, but `docs/milestones/v0.93/WBS_v0.93.md` labels it
   candidate allocation and the required accepting owner has not been verified.
6. **Sprint closeout is blocked.** `#5836`, `#5838`, and `#5839` are nonterminal.

## Dirty And Uncommitted Paths

Preserve all of these. Do not reset, clean, or overwrite them.

Current `#237` worktree, before this handoff was added:

```text
.csdlc/issues/237/audit.jsonl
.csdlc/issues/237/cards/sip.values.json
.csdlc/issues/237/cards/sor.values.json
.csdlc/issues/237/cards/spp.md
.csdlc/issues/237/cards/spp.values.json
.csdlc/issues/237/cards/srp.md
.csdlc/issues/237/cards/srp.values.json
.csdlc/issues/237/cards/stp.values.json
.csdlc/issues/237/cards/vpp.values.json
.csdlc/issues/237/index.json
.csdlc/prepared/issues/237/design.md
.csdlc/locks/237.lock (untracked)
```

This handoff adds the following untracked documentation file:

```text
.adl/docs/TBD/V092_SPRINT_5_BIRTHDAY_CONTINUITY_SESSION_HANDOFF.md
```

Parent `#5836` worktree:

```text
.csdlc/issues/5836/audit.jsonl
.csdlc/issues/5836/cards/sip.md
.csdlc/issues/5836/cards/sip.values.json
.csdlc/issues/5836/cards/sor.values.json
.csdlc/issues/5836/cards/spp.md
.csdlc/issues/5836/cards/spp.values.json
.csdlc/issues/5836/cards/srp.values.json
.csdlc/issues/5836/cards/stp.md
.csdlc/issues/5836/cards/stp.values.json
.csdlc/issues/5836/cards/vpp.md
.csdlc/issues/5836/cards/vpp.values.json
.csdlc/issues/5836/index.json
.csdlc/prepared/issues/5836/design.md
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/src/bin/adl-runtime-birthday-demo.rs (untracked)
adl-runtime-kernel/src/birthday_demo.rs (untracked)
adl-runtime-kernel/tests/birthday_demo.rs (untracked)
adl-runtime-kernel/tests/fixtures/birthday_demo/ (untracked)
```

## Exact Next Action

Only after the operator explicitly resumes Sprint 5:

1. Enter the bound `#237` worktree and verify `git status` exactly preserves the
   paths above; do not start from main and do not recreate the issue.
2. Read root `AGENTS.md`, `.csdlc/prepared/issues/237/design.md`, the typed
   `#237` cards/index, and the last exact review finding.
3. Implement the approved crate-private Runtime-established opaque
   `CapabilityAuthorityPolicy`. Public capability build/validate and governed
   cognition must require it; token B is valid only after explicit Runtime
   reauthorization. Under retained authority A, rewrite/re-hash capability to B
   and rebuild/re-sign cognition; the attack must reject.
4. Use typed C-SDLC v2 editors for card/SOR changes. Rerun the one required
   serial proof job: focused public target, full library tests, doctests, and
   strict library Clippy. No optional jobs and at most one large runner.
5. Commit exact source/evidence, assign a fresh independent exact-head review,
   fix every actionable finding, then publish and merge `#237` only when the
   exact reviewed PR head is required-green/clean.
6. Rebase or otherwise reconcile `#5836` onto the reviewed ancestral `#237`
   merge without losing its dirty WIP; rerun the real birthday demo proof and
   continue its own fresh review/publication/finish lifecycle.
7. Start `#5838` only after `#5836` is terminal. Start `#5839` only after a real
   v0.93 allocation and accepting owner are explicitly verified. Close `#5854`
   only after all four operative children and the integrated sprint review are
   truthful and terminal.

## Non-Goals And Ownership Boundaries

- Do not write on `main` or use the stale primary checkout as an implementation base.
- Do not resume work while the operator's stand-by remains active.
- Do not create duplicate issues, branches, worktrees, or remediation tracks for `#237`, `#5836`, `#5838`, or `#5839`.
- Do not hide `#237` inside `#5836`; `#237` owns the cross-component authority correction and must merge first.
- Do not hand-edit C-SDLC cards; use typed v2 editor routes after resume.
- Do not publish or merge before a fresh exact-head review with no unresolved actionable findings.
- Do not run optional CI or more than one large runner per issue.
- Do not treat v0.93 planning prose as accepted governance authority or implement citizenship, rights, duties, standing, or polis governance in Sprint 5.
- Do not execute WP-20 `#5840` here; it belongs to Sprint `#5856`.
- Do not coordinate WP-24A `#5845`; it is out of band.
- Do not claim release readiness, external publication, or umbrella completion from child-local green tests.

## Source Evidence For A Fresh Task

- `AGENTS.md`
- `.csdlc/prepared/issues/5854/sprint-execution-packet.md` from current `origin/main`
- `.csdlc/issues/5854/cards/spp.md` from current `origin/main`
- `.csdlc/issues/237/index.json`
- `.csdlc/issues/237/cards/srp.md`
- `.csdlc/issues/237/cards/sor.values.json`
- `.csdlc/prepared/issues/237/design.md`
- `.csdlc/evidence/237/`
- `/Volumes/FastWork/adl-worktrees/adl-issue-5836-wp18-first-birthday-demo/.csdlc/issues/5836/index.json`
- `/Volumes/FastWork/adl-worktrees/adl-issue-5836-wp18-first-birthday-demo/.csdlc/prepared/issues/5836/design.md`
- `docs/milestones/v0.93/WBS_v0.93.md`
- `docs/milestones/v0.93/CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md`

Evidence not available in this documentation-only turn: a fresh live GitHub API
observation. PR and merge facts above are retained typed-finish/session facts
cross-checked against the exact commits currently present in `origin/main`.
