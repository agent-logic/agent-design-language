# v0.91.8 Work Breakdown Structure

## Metadata
- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Date: `2026-07-14`
- Owner: ADL maintainers
- Status: planned; issue wave opened and pending setup review

## Status

Planned. No implementation, parity, cutover, or deletion claim exists yet.

## How To Use

Execute in dependency order. Each WP is independently reviewable and follows
the full C-SDLC lifecycle. Parallel work is allowed only where the sprint
packet names disjoint write sets.

## WBS Summary

The milestone moves through contract/baseline, clean-room construction,
integration/parity, reversible cutover, deletion, and release convergence.

## Candidate WP Sequence

| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Planning and issue-wave setup | Create the reviewed milestone package and tracker topology. | Planning package and seeded wave | Runtime v3 and C-SDLC v2 evidence | #5335 |
| WP-02 | Baseline and architecture gate | Pin source/test denominator, owners, budgets, contracts, and architecture. | Hashed baseline and approved architecture | WP-01 | #5336 |
| WP-03 | Characterization corpus | Capture normalized positive/negative behavior and determinism fixtures. | Versioned corpus and normalizer | WP-02 | #5337 |
| WP-04 | Clean-room language core | Implement six primitives, parsing, schema, canonicalization, and validation. | `adl-language` proof | WP-02, WP-03 contracts | #5339 |
| WP-05 | Deterministic compiler | Implement resolution, composition, patterns, stable IDs, and plans. | `adl-compiler` proof | WP-04 | #5338 |
| WP-06 | Portable execution engine | Implement bounded scheduling, retry/failure, joins, and resume. | `adl-engine` proof | WP-05 | #5340 |
| WP-07 | Portable records and trust | Implement error, artifact, trace, result, signing, and verification contracts. | Versioned record package | WP-02, WP-03 | #5342 |
| WP-08 | Runtime v3 adapter | Connect plans and engine events to Runtime v3 without authority leakage. | Runtime adapter proof | WP-06, WP-07 | #5341 |
| WP-09 | Provider and tool adapters | Implement mock, HTTP, governed-tool, and bounded compatibility ports. | Adapter proof matrix | WP-06, WP-07 | #5349 |
| WP-10 | Thin CLI and selector | Implement validate/schema/plan/run/inspect/sign/verify and generation selection. | Owner CLI and selector | WP-04 through WP-09 | #5345 |
| WP-11 | Shadow parity | Compare normalized v1/v2 behavior and classify every mismatch. | Exact-revision parity packet | WP-10 | #5350 |
| WP-12 | Soak, rollback, and reversible default switch | Run representative opt-in scenarios, prove v1 restoration, then execute the reviewed selector change. | Soak, rollback, and selector evidence | WP-11 | #5344, #5343 |
| WP-13 | ADL legacy deletion | Delete externally owned bands, then remove replaced language/compiler/engine/CLI code after the rollback window. | At least 80% deletion and retained manifest | WP-12 | #5347, #5346 |
| WP-14 | Integrated platform acceptance, deployment, and v0.92 handoff | Fully accept and deploy ADL v2, Runtime v3, and C-SDLC v2; consume the moved launch/activation/Memory Palace/capability/witness/birthday/ADR, Unity Observatory, and Adaptive Learning DAG families. | Three-product deployment and acceptance packet plus exact v0.92 handoff | WP-13 | #4641, #5358, #5361, #5352, #4758-#4763, #5007, #4739, #4741, #5332, #5107 |
| WP-15 | Demo convergence | Demonstrate the deployed three-product stack and reconcile public claim boundaries. | Integrated demo convergence packet | WP-14 | #5354 |
| WP-16 | Quality gate | Run focused and integrated product, deployment, rollback, deletion, docs, and release-readiness checks. | Quality-gate packet and blocker list | WP-14, WP-15 | #5351 |
| WP-17 | Documentation alignment | Align product ownership, deployment, README, feature, WBS, sprint, checklist, issue wave, handoff, and release truth. | Documentation alignment packet | WP-16 | #5360 |
| WP-18 | Internal review | Review code, deployments, docs, feature coverage, proof, issue topology, and release-tail packets. | Internal review packet and finding register | WP-17 | #5356 |
| WP-19 | External review | Run independent third-party review after internal review is consumable. | External review handoff and finding register | WP-18 | #5357 |
| WP-20 | Remediation and preflight | Fix accepted findings, rerun proving checks, and converge final checklist truth. | Remediation PRs, preflight packet, final checklist | WP-19 | #5363 |
| WP-21 | Feature-list and v0.92 planning truth alignment | Prepare v0.92 inputs from reviewed deployed product truth. | v0.92 planning seed and feature-list disposition | WP-20 | #5362 |
| WP-21A | Next-milestone closeout planning | Prepare the canonical v0.92 closeout-planning packet. | Review-ready v0.92 closeout plan | WP-21 | #5355 |
| WP-22 | Next-milestone review | Review v0.92 inputs for blockers, stale assumptions, and overclaims. | v0.92 planning review packet | WP-21A | #5359 |
| WP-23 | Release ceremony | Finalize release evidence, closeout truth, notes, tag, publication, and lifecycle reconciliation. | Release ceremony and final closeout record | WP-22 | #5348 |

## Work Packages

The sequence above is authoritative for topology. The issue-wave YAML carries
machine-readable deliverables, proof roles, write sets, and release gates.

## Sidecar Work

- Scope: control-plane bugs discovered while setting up or executing the wave.
- Boundary: bugs receive separate issues and do not widen implementation WPs.
- Proof surface: issue-local C-SDLC records and remediation PRs.

## Sequencing

- Phase 1: WP-01 through WP-03—contract, baseline, and characterization.
- Phase 2: WP-04 through WP-10—clean-room construction and integrations.
- Phase 3: WP-11 through WP-14—parity, soak, switch, deletion, and integrated deployment acceptance.
- Phase 4: WP-15 through WP-23—exact demo, quality, documentation, review, remediation, next-milestone, and ceremony closeout sequence.

## Sequencing Notes

WP-04 and WP-07 may start in parallel after WP-03 freezes their shared
contracts. WP-08 and WP-09 may run in parallel after WP-06/WP-07. WP-11
through WP-23 are serial gates. WP-14 remains open until all three products are
accepted and deployed and every moved WP-14 child is closed or explicitly
blocked with evidence and operator approval.

## Acceptance Mapping

- WP-02 -> exact denominator, architecture, owner map, and budgets.
- WP-03 -> compact behavioral and negative-case corpus.
- WP-04 through WP-10 -> independently validated replacement product.
- WP-11 through WP-13 -> parity, rollback, reviewed default selection, and at least 80% ADL deletion.
- WP-14 -> full ADL v2, Runtime v3, and C-SDLC v2 acceptance/deployment plus v0.92 handoff.
- WP-15 through WP-23 -> the canonical demo, quality, docs, internal review, external review, remediation, next-milestone planning/review, and ceremony sequence.

## Exit Criteria

- Every WP has one concrete issue, owner, dependencies, and proving output.
- No deletion begins before selector rollback proof.
- No release closes below the deletion minimum.
