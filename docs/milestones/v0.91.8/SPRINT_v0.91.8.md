# v0.91.8 Sprint Plan

## Metadata
- Sprint: `v0.91.8-adl-core-rearchitecture`
- Milestone: `v0.91.8`
- Start date: after setup PR review and merge
- End date: after WP-23 release ceremony and closeout
- Owner: ADL maintainers
- Status: planned

## Status

Planned. Child execution must not start from this draft alone.

## How To Use

The milestone umbrella owns ordering and blocker truth. Each child issue owns
its implementation and validation. Watchers may observe wait states but may not
advance serial gates without evidence.

## Sprint Overview

Replace the incumbent ADL monolith through a contract-first clean-room wave,
then prove and delete the old ownership.

Planned scope:

- exact baseline and characterization;
- language/compiler/engine/CLI and adapter construction;
- shadow parity, soak, rollback, default switch, and deletion;
- full ADL v2, Runtime v3, and C-SDLC v2 acceptance/deployment;
- exact demo, quality, docs, internal review, external review, remediation,
  next-milestone planning, next-milestone review, and release closeout.

## Sidecar Sprint

- Scope: C-SDLC/runtime/tooling defects that block the wave.
- Boundary: separate issue and PR; no hidden repair inside product WPs.
- Proof surface: issue-local diagnosis, focused validation, and closeout.

## Sprint Goals

- Make the ADL product boundary small and explicit.
- Preserve declared behavior with fewer, stronger contract tests.
- Delete at least 80% of the pinned incumbent source surface.

## Sprint Goal

Deliver a reviewed small ADL default product and remove replaced legacy
ownership without changing Runtime v3, C-SDLC v2, or v0.92 semantics.

## Planned Scope

- WP-01 through WP-23, including WP-21A, in [WBS_v0.91.8.md](WBS_v0.91.8.md).

## Work Plan

| Order | Items | Execution | Status |
|---|---|---|---|
| 1 | WP-01 to WP-03 | serial contract gate | planned |
| 2 | WP-04 and WP-07 | parallel after shared contracts freeze | planned |
| 3 | WP-05 then WP-06 | serial compiler/engine core | planned |
| 4 | WP-08 and WP-09 | parallel disjoint adapters | planned |
| 5 | WP-10 to WP-14 | serial integration, cutover, deletion, and platform deployment gates | planned |
| 6 | WP-15 to WP-23 | exact canonical closeout sequence | planned |

## Execution Policy

- Every issue uses `SIP -> STP -> SPP -> VPP -> SRP -> SOR`.
- Every implementation session creates an issue-bound goal after bind.
- Exact revision review precedes publication.
- Focused validation runs locally; CI integration proof is recorded separately.
- Findings outside scope become routed issues rather than hidden additions.

## Sprint Execution Packet

- Execution mode: hybrid.
- SEP artifact: this section plus the issue-wave YAML.
- Recommended order: WP-01 through WP-23 dependency order, including WP-21A.
- Candidate parallel lanes: WP-04/WP-07 and WP-08/WP-09.
- Safe parallel lanes: only disjoint crates, contracts already frozen, and no shared selector/docs writes.
- Serial gates: architecture approval, plan contract, CLI integration, parity,
  soak, switch, deletion, three-product acceptance/deployment, demos, quality,
  docs, internal review, external review, remediation, next-milestone planning,
  next-milestone review, and ceremony.
- PVF notes: each WP selects the smallest proving lane; full workspace proof belongs to WP-16.
- Planned versus actual parallelism: actual execution must be recorded in SORs.
- Residual routing: defects and missing capabilities become separate issues with owner and milestone disposition.

## Cadence Expectations

- Keep at most one serial-gate issue active.
- Parallel lanes may proceed only after predecessor merge/consumption is verified.
- Watchers own external wait states; schedulers do not gain merge authority.

## Risks / Dependencies

- Dependency: exact Runtime v3 and C-SDLC v2 owner boundaries remain stable.
- Risk: active v0.91.7 changes alter the denominator.
- Mitigation: pin the denominator revision and report later additions separately.
- Risk: v0.92 consumes an unreviewed intermediate state.
- Mitigation: WP-14 records the initial exact handoff, and WP-21 through WP-22
  reconcile it against reviewed release truth.

## Demo / Review Plan

- Demos are defined in [DEMO_MATRIX_v0.91.8.md](DEMO_MATRIX_v0.91.8.md).
- Each implementation WP receives bounded review before PR publication.
- WP-18 performs internal review and WP-19 performs independent external review.

## Closeout Bar

- At least 80% pinned incumbent deletion.
- Full accepted deployment evidence for ADL v2, Runtime v3, and C-SDLC v2.
- No unclassified parity mismatches.
- Default selector and rollback window truth reconciled.
- v0.92 handoff reviewed.

## Exit Criteria

- Every WP is closed with evidence, explicitly deferred, or blocked with operator-approved evidence.
- WP-15 through WP-23 execute in the exact closeout order without combining roles.
- Release ceremony contains no hidden implementation work.
