# Issue #5360 WP-17 Documentation And Release-Truth Alignment Design

## Decision

Prepare an evidence-driven documentation alignment pass that begins only after
WP-16 issue #5351 is merged, typed `closed_out`, claim-free, backed by its
retained terminal receipt, and ancestral to the exact #5360 execution revision.
The implementation will reconcile existing source-of-truth documents; it will
not create another release database, runtime, deployment system, or planning
authority.

## Preparation Boundary

Preparation owns exactly these paths:

- `.csdlc/issues/5360`
- `.csdlc/locks/5360.lock`
- `.csdlc/prepared/issues/5360`
- `.csdlc/evidence/5360`

No product source, shared documentation, deployment state, issue, PR, or release
surface may change during preparation. Before implementation, a fresh collision
check and typed claim amendment must protect the exact approved documentation
paths below.

The preparation baseline is exact revision
`fbf96beac1cb61c85bf7889e9c08729916c0796b`. Zero-change proof evaluates both
committed and uncommitted paths relative to that revision, so a clean worktree
cannot hide an out-of-scope preparation commit.

## Future Implementation Paths

The proposed implementation path set is closed and exact:

- `README.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.91.8/WBS_v0.91.8.md`
- `docs/milestones/v0.91.8/SPRINT_v0.91.8.md`
- `docs/milestones/v0.91.8/SPRINT_PLAN_v0.91.8.md`
- `docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md`
- `docs/milestones/v0.91.8/MILESTONE_CHECKLIST_v0.91.8.md`
- `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml`
- `docs/milestones/v0.91.8/RELEASE_PLAN_v0.91.8.md`
- `docs/milestones/v0.91.8/RELEASE_NOTES_v0.91.8.md`
- `docs/milestones/v0.91.8/BASELINE_AND_OWNERSHIP_v0.91.8.md`
- `docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md`
- `docs/milestones/v0.91.8/FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md`
- `docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md`
- `docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md`
- `docs/milestones/v0.91.8/features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md`
- `docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md`

Any additional path requires typed claim amendment and bounded re-review before
editing. Generated C-SDLC records and issue-local evidence remain separately
owned by the four preparation paths.

## Source And Claim Model

The alignment packet will inventory each material statement as one of:
`proven`, `planned`, `blocked`, `deferred`, `superseded`, or `explicit_non_claim`.
Only exact retained evidence may support `proven`. Live issue state and the
canonical issue wave determine dependency truth. Aggregate prose, screenshots,
old milestone packets, or successful component checks cannot promote a claim.

For every changed statement, the packet records source path, old classification,
new classification, evidence reference, owning product, and disposition. ADL v2,
Runtime v3, and C-SDLC v2 remain separate products with separate ownership even
where deployment and acceptance evidence converges.

## Dependency Gate

`check-dependencies.rb` fails closed unless all predicates hold:

1. the retained shared-Git receipt `csdlc-v2/closeout/5351.json` exists;
2. the current #5351 typed record exactly matches that receipt;
3. #5351 is `closed_out` and claim-free;
4. typed doctor reports a clean terminal record;
5. terminal disposition and observed state are both `merged` with a PR and SHA;
6. the observed merge SHA is ancestral to the exact #5360 execution revision.

Preparation is permitted while this gate fails. Documentation edits, review for
publication, publication, merge, closeout, and release of WP-18 are forbidden.

## COTS And Architecture

Use repository-native Git, Ruby standard library, typed C-SDLC v2 binaries,
existing Markdown/YAML/JSON structures, and existing focused documentation
checks. Add no crate, gem, package, service, parser framework, workflow engine,
database, deployment manager, telemetry system, or signing layer. Structured
formats must use existing structured parsers or owner tools during execution.

## Budgets

- New dependencies: `0`.
- Product source changes during preparation: `0`.
- Preparation orchestration: at most 1,500 nonblank lines total.
- Individual preparation module: fewer than 500 nonblank lines.
- Focused assertions: fewer than 150.
- Documentation implementation delta: at most 2,500 changed lines across the
  exact protected path set unless an exact reviewed variance is recorded.
- Preparation and dependency gates: 120 seconds each.
- Focused alignment lane: 600 seconds.
- Complete and post-merge exact lanes: 900 seconds each.
- PVF token budgets: 3,500 / 2,000 / 6,000 / 8,000 / 8,000 respectively.

## PVF Plan

`preparation-contract` proves current-registry six-card integrity, reviewed
design and diagram, exact preparation scope, COTS, budgets, clean diff, zero
product changes, and typed doctor truth. `wp16-terminal-gate` is deterministic,
local, network-denied admission proof and is expected to fail during preparation.
`focused-doc-alignment`, `complete`, and `post-merge-exact` are required release
gates but remain unavailable until their declared lifecycle points.

Immediately after typed design approval and before bind, a typed
`csdlc-validate` request runs the `current-registry-card-integrity` lane. Typed
approval atomically refreshes the reviewed design/diagram digests and generated
projections; the lane then verifies all six card pairs against the active native
registry shape and requires typed doctor to report no finding. Final preparation
reruns typed doctor and the required PVF request in the bound phase.

## Review And Release Truth

The bounded preparation review checks bypasses, unsupported claims, path
collisions, product-owner ambiguity, COTS duplication, budget gaps, PVF mapping,
and zero-product-change truth. Every actionable finding is fixed before typed
design approval and bind. Future implementation requires exact-revision review,
green required checks, authorized serialized merge, post-merge proof, and typed
closeout before WP-18 may begin.

## Stop Conditions

Stop without implementation or publication if #5351 is not fully terminal, any
source claim lacks exact evidence, a protected path collides, a required shared
path falls outside the reviewed set, a change would alter product behavior, a
new dependency is required, Runtime v2/AWS/raw `gh`/credentials are requested,
a required lane fails or is deferred, review becomes stale, or release truth
would be represented more strongly than the evidence supports.
