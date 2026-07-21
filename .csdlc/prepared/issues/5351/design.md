# #5351 WP-16 Integrated Platform Quality Gate Design

## Status

Preparation-only design for v0.91.8 WP-16. It grants no authority to run the
quality gate, edit product code, publish, or merge while WP-15 #5354 is
nonterminal.

## Objective

Produce one exact-revision quality-gate packet that evaluates the integrated
ADL v2, Runtime v3, and C-SDLC v2 platform after the reviewed WP-15 demo has
converged. The packet must expose every failed or unavailable gate as a blocker;
it must never turn a failure into a documentation disposition.

## Authority Boundary

Preparation owns only:

- `.csdlc/issues/5351`
- `.csdlc/locks/5351.lock`
- `.csdlc/prepared/issues/5351`
- `.csdlc/evidence/5351`

No product, runtime, deployment, demo, milestone-document, CI-policy, or release
path is claimed during preparation. Any future path amendment must be typed,
reviewed, collision-free, and occur only after the #5354 dependency gate opens.

## Dependency Gate

Execution is admitted only when all of the following are true for #5354:

1. The retained terminal record reports an actually merged PR.
2. Typed C-SDLC v2 reports `closed_out`.
3. The typed claim is absent.
4. The shared Git terminal receipt exists and verifies.
5. The receipt records a merged disposition, PR number, and observed merge SHA.
6. The observed merge SHA is an ancestor of the exact #5351 execution revision.

The gate fails closed on absent, stale, malformed, contradictory, or
non-ancestral evidence. WP-14A is consumed transitively through #5354 and is not
reopened or re-evaluated by #5351.

## Quality Packet

The future packet will bind exact revisions and retained evidence for:

- product contracts and characterization;
- deterministic compiler and portable engine behavior;
- signing, trust, provider, and governed-tool boundaries;
- Runtime v3 ingress, continuity, supervision, observability, and rollback;
- C-SDLC v2 lifecycle and closeout integrity;
- distributed workcell acceptance;
- deployment, demo convergence, and public claim boundaries;
- deletion eligibility and post-deletion proof;
- docs, YAML, links, feature-proof, and release-blocker truth.

Each row has one of: `pass`, `fail`, `blocked`, `not_applicable`, or
`explicit_non_claim`. Only `pass` satisfies a required gate. `blocked` and
`fail` stop WP-17. Missing evidence is `blocked`, never inferred green.

## COTS And Simplicity

No new dependency is permitted. The gate composes existing repository tools,
typed C-SDLC v2 binaries, Git, Ruby standard library, existing validators, and
existing product test commands. It does not implement a test runner, workflow
engine, signer, telemetry system, deployment manager, or evidence database.

## Budgets

- new issue-local gate orchestration and fixtures: at most 1,500 nonblank lines;
- each new script/module: below 500 nonblank lines;
- focused quality assertions: fewer than 150;
- preparation validation: 120 seconds;
- focused product-contract gate: 600 seconds;
- integrated platform gate: 1,800 seconds;
- complete pre-publication or post-merge gate: 2,280 seconds;
- new third-party dependencies: zero.

Any variance requires an exact-revision review and explicit recorded
disposition; the 2,280-second ceiling is not automatic authorization. The six
lane ceilings total exactly the `large` profile's 7,200-second automatic
validation budget.

## PVF

- `preparation-contract`: deterministic, small, release-planning proof;
- `wp15-terminal-gate`: deterministic, small, execution admission proof;
- `focused-quality`: deterministic, medium, required pre-integration proof;
- `integrated-platform`: deterministic, large, required release-gate proof;
- `complete`: deterministic, large, required pre-publication proof;
- `post-merge-exact`: deterministic, large, required closeout proof.

All future test additions must be classified in the existing tracked PVF
inventory in the same issue before they are credited.

## Failure And Rollback

Any failed, missing, stale, non-ancestral, secret-bearing, host-bound, or
out-of-scope result stops the gate and routes a blocker to its owning issue.
#5351 does not repair unrelated products inside the quality-gate change. Before
publication, rollback is deletion of issue-local generated evidence. After an
authorized merge, rollback follows the accepted platform rollback contract and
requires a new reviewed issue; #5351 never rewrites terminal evidence.
