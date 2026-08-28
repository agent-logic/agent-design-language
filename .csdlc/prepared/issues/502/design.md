# Issue #502 Design: V3-C C-SDLC v3 lifecycle kernel

## Purpose

Produce the transactional C-SDLC v3 lifecycle-kernel slice that can model
capability-checked lifecycle transitions, atomic state commits, recovery replay,
and typed Git/process adapters without becoming operational lifecycle authority.

## Authority boundary

C-SDLC v2 remains the sole operational lifecycle authority. The #502 kernel is a
non-authoritative construction slice: it may model transition decisions,
transactions, recovery plans, and adapter results, but it must not perform
GitHub mutation, live lifecycle writes, publication, finish, cleanup, v2
migration, or authority cutover.

Because #501 is not yet merged to `main`, #502 is planned as a stacked execution
on the #501 V3-B foundation branch. Publication and final merge must preserve
the dependency ordering.

## Inputs

- `agent-logic/agent-design-language#502`
- `agent-logic/agent-design-language#501`
- retained predecessor issues `#168`, `#169`, and `#170`
- `docs/csdlc-v3/CONTRACT.md`
- `docs/csdlc-v3/predecessor-coverage.json`
- `docs/csdlc-v3/proportional-lifecycle.json`
- `docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#V3-C`
- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml`
- `docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md`
- planned predecessor packets for issues `#168`, `#169`, and `#170`

## Deliverables

- `csdlc-v3/src/lifecycle/**`: pure transition and capability-checking model
  with explicit atomic invalidation semantics.
- `csdlc-v3/src/storage/**`: deterministic in-memory transaction store,
  staged commit model, recovery journal, and replay classifier.
- `csdlc-v3/src/adapters/**`: typed Git/process adapter traits and fake
  implementations proving argv, status, stdout, stderr, timeout, cancellation,
  and redaction boundaries.
- `csdlc-v3/tests/transactions.rs`: focused transition, failure-injection,
  recovery-replay, and adapter-boundary tests for requirements `#168` through
  `#170`.
- `csdlc-v3/AGENTS.md`: crate-local agent guidance for the v3 construction
  surface, making the non-authoritative/v2-authority boundary and fast issue
  start expectations explicit for future workers.

## Design decisions

1. Transition decisions are pure data: every command/state pair is accepted or
   rejected by explicit capability rules.
2. Transaction commits are atomic at the modeled state boundary; projections and
   evidence are repairable outputs, not authority.
3. Recovery replay preserves audit provenance and classifies interrupted writes
   as prior-state, new-state, or repair-required outcomes.
4. Git/process adapters are typed boundaries with argv-only invocation records
   and separated exit/status/output/error/cancellation outcomes.
5. Branch/worktree observation alone never authorizes lifecycle work.
6. The stack depends on #501 foundation code but does not widen #501 or start
   V3-D.
7. `csdlc-v3/AGENTS.md` is documentation for humans and agents only; it must
   not redefine lifecycle authority or claim the v3 kernel is operational.

## Validation lanes

- `transition-matrix`
- `transaction-failure`
- `recovery-replay`
- `adapter-boundary`
- `strict-clippy`
- `diff-hygiene`

## Stop conditions

- A partial write can become authoritative.
- Recovery loses or rewrites audit provenance.
- A lifecycle decision depends on branch-name observation alone.
- A Git/process adapter accepts shell strings or ambient credentials.
- The slice performs live GitHub or lifecycle mutation.
- Work expands into V3-D or v2 migration.
- The crate-local `AGENTS.md` conflicts with root `AGENTS.md` or implies v3
  authority before an approved cutover.
