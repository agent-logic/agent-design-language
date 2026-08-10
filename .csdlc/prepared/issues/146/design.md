# v0.92.1 milestone planning design

## Status

Design-ready for bounded review. This setup issue creates planning authority only; no child implementation, legal transfer, infrastructure mutation, or live Runtime proof is authorized here.

## Milestone intent

v0.92.1 combines three foundation programs under one release contract while preserving independent execution:

1. **Corporate and IP transfer** moves legal ownership, operational control, and due-diligence evidence to Agent Logic, Inc.
2. **C-SDLC v3** implements the independently reviewed Rust architecture through its declared construction, lifecycle, remote-operation, canary, and cutover phases.
3. **Distributed multi-agent Runtime proof** validates the existing distributed Runtime and Observatory implementation across real nodes, agents, trust boundaries, failure modes, replay, and bounded soak.

The three lanes share a WP-01 milestone-opening gate, final review, and release
closeout. They do not depend on one another for implementation progress. A
delay in counsel review, C-SDLC v3, or Runtime proof must not strand the other
lanes.

## Source authority

- Corporate infrastructure source: `docs/milestones/v0.92.1/sources/CORPORATE_INFRASTRUCTURE_CONSOLIDATION_SOURCE.md`, promoted from the ignored `.adl/docs/TBD/V0.92.5_CORPORATE_MIGRATION.md` draft and rerouted into v0.92.1.
- Legal gap source: `.adl/docs/TBD/planning/GTM_CHECKLIST_v0.1.md` and the absence of a chain-of-title plan.
- C-SDLC v3 source: `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md` and `.adl/docs/TBD/CSDLC_V3_RUST_PLAN_REVIEW.md`.
- Runtime source: existing v0.92 distributed Runtime, CSM, ACIP, identity/authority, resilience, Observatory, and operationalization work, including issue #142.

The package validator pins the accepted C-SDLC v3 source to PR #77 merge commit `413fa9b8588dd25be3785cfe111c4f1df3af36eb` and verifies ancestry at publication. Runtime qualification pins its source authority to the eventual terminal revision and receipt of #142 plus the canonical WP-04.16 contract; it cannot execute against mutable or merely open implementation truth.

The v0.92.5 corporate migration draft is superseded in routing, not deleted. Its infrastructure content moves into v0.92.1 and gains the missing legal chain-of-title lane.

## Lane A: Corporate and IP transfer

The lane begins with an authoritative asset and account inventory. It must distinguish:

- founder-created code, documentation, architecture, product designs, media, brands, domains, datasets, model artifacts, credentials, cloud resources, repositories, and contractual rights;
- company-created or company-purchased assets;
- third-party and open-source materials governed by licenses rather than assignment;
- excluded personal assets and any retained rights;
- contributor-created material requiring separate provenance or assignment evidence.

Execution work then produces counsel-reviewed transfer instruments, asset schedules, corporate approval and acceptance evidence, repository/domain/cloud/SaaS control transfer, infrastructure verification, and a redacted due-diligence index. Secrets and private legal instruments remain outside the public repository; the repo retains redacted receipts, identifiers, hashes, dates, authority, and verification outcomes only.

No planning document may claim that an assignment agreement is legally sufficient. Qualified counsel and authorized corporate actors own that determination.

## Lane B: C-SDLC v3

The milestone preserves the reviewed v3 issue sequence:

- contract freeze and construction spike;
- one binary, command tree, application context, repository context, and v2 read-only importer;
- canonical state/cards, lifecycle kernel, transaction store, and typed effects;
- issue/bind, card/doctor, PVF planning, and PVF execution;
- exact review/publication gates, GitHub observation, PR mutation/foreground watch, finish, and cleanup;
- parity, shadow, canary, writer-fenced cutover, rollback window, and deferred v2 retirement.

V3-R01 remains outside the initial release gate until the rollback window expires and a later reviewed operator decision authorizes retirement. v3 must never gain mutation authority while v2 remains writable for the same issue.

The construction spike is a decision gate. It must measure implementation size, dependencies, clean/warm build time, binary size, test speed, error clarity, and contributor comprehension before downstream estimates become commitments.

The issue wave preserves all eleven required operator decisions in a decision register. Decision 11, the per-platform commit matrix and initial Windows mutation posture, is a separate machine-readable gate after V3-02 and before V3-08. V3-08 cannot start until that decision is approved.

## Lane C: Distributed multi-agent Runtime proof cycle

This lane validates, rather than reimplements, the distributed Runtime. It is
split into explicit preparation, deterministic contract, native topology,
fault/recovery, soak, and synthesis issues. DRT-01 may start only after WP-01
opens and validates the issue wave. Every live topology issue is additionally
hard-gated on terminal #142 production proof and consumes its exact runner,
revision, and receipt contract. The lane tests additional release-candidate
qualification; it does not duplicate or bypass #142 or WP-04.16.

### Minimum proving topology

- exactly three independently started voting Runtime/Guardian/kernel nodes in one polis, each with distinct identity, port, state root, credential set, storage, and interface;
- at least three governed agents with distinct identities and capabilities;
- one human/operator identity through the Observatory or canonical operator surface;
- exactly one quorum-leased Observatory owner and one active non-voting shepherd;
- production transport, certificate, storage, authority, and admission paths;
- retained topology and revision manifests proving which binaries, configurations, certificates, nodes, and agents participated.

An in-process service graph, mocked transport, direct executor call, fabricated counter, or hand-authored success receipt does not prove distribution.

### Required behavior

- identity continuity and authority enforcement across nodes;
- positive collaboration, delegation, messaging, and shared-state flows;
- unauthorized, stale, replayed, malformed, partitioned, duplicated, and reordered operations rejected or reconciled as specified;
- leader/coordinator failure, network partition, process restart, storage restart, certificate failure, and node rejoin behavior;
- quorum continuity, snapshot and committed-index parity, stale-leader and stale-Observatory fencing, and mutation halt when only one voter remains;
- serial proof windows: first three isolated Wuji voters, then one Wuji voter plus two private AWS voters in separate availability zones;
- deterministic replay and receipt verification from retained inputs;
- Observatory visibility that correlates agent, node, trace, authority, state revision, and failure evidence without leaking secrets;
- bounded resource use and a declared soak duration with explicit pass/fail thresholds.

Every machine-readable receipt is derived from producer output and tied to the exact tested revision. Validators recompute counts and outcomes; they do not accept hard-coded success totals.

## Execution topology

After WP-01 is terminal and execution is explicitly authorized:

- Lane A, Lane B, and Lane C may start concurrently.
- Each lane has its own umbrella, sprint sequence, readiness gates, and review/remediation tail.
- Lane C may depend on specific existing Runtime implementation issues being terminal, but not on Lane A or Lane B.
- Final milestone review begins only after each lane publishes its own terminal evidence or an explicit approved deferral.
- INT-01 through INT-06 preserve separate integrated review, release qualification,
  next-milestone planning, next-milestone review, release ceremony, and terminal
  closeout gates.

## Release gates

v0.92.1 is not release-ready until:

1. every asset on the counsel-approved critical-asset schedule has completed transfer and corporate acceptance evidence; only expressly accepted non-critical exclusions may remain, and any deferred in-scope critical asset blocks release;
2. C-SDLC v3 passes parity, canary, single-writer cutover, rollback, and independent review gates;
3. distributed Runtime proof passes real-topology, fault/recovery, replay, observability, and soak gates;
4. security, legal-boundary, architecture, and release reviews have no unresolved blocking findings;
5. the release packet states residual risks and non-claims truthfully.

## Validation posture for issue #146

Issue #146 is documentation-only. It requires focused checks for package completeness, YAML parsing, identifier/dependency integrity, local links, placeholders, planning posture, and diff hygiene. It does not require broad Rust or live Runtime testing.

## Owned Paths

- `.csdlc/issues/146/**`
- `.csdlc/prepared/issues/146/**`
- `docs/milestones/v0.92.1/**`
- `csdlc-v3/README.md`

## Non-goals

- Executing any child work package.
- Drafting final legal instruments in the repository.
- Replacing active Runtime implementation ownership.
- Treating v3 retirement as part of initial cutover.
- Manufacturing proof receipts during planning.
