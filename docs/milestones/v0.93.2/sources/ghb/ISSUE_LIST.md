# GHB sprint umbrella and issue list

Status: drafted locally; no GitHub issues created or updated. Local IDs must be replaced with fully qualified GitHub issue links after creation. Source: [implementation plan](IMPLEMENTATION_PLAN.md), [design/research](DESIGN_AND_RESEARCH.md), [estimate](ESTIMATE.md).

## Count and routing

**14 issues in the program: one new sprint umbrella, 12 new child issues, and existing Runtime #112.** This means 13 new tracker items if the mapping survives the initial duplicate/ownership check. The umbrella lives in Runtime. Two child issues live in ADL for specification and contract distribution; ten new children live in Runtime. Existing Runtime #112 is the thirteenth child. WP12 coordinates paper and ADL documentation from a Runtime evidence issue, without inventing an unverified paper tracker name.

All issues target working installed Runtime behavior. Local WP IDs are not issue numbers. Estimates below are base allocations before contingency, not spending authorization. Issue #112 was open at the research snapshot; refresh status, owner and delivered evidence before scheduling it.

## Sprint umbrella — GHB-SPRINT

**Proposed title:** [GHB][Sprint] Deliver governed recursive improvement in the installed Agent Logic Runtime

**Repository:** `agent-logic/agent-logic-runtime`  
**Type:** sprint/program umbrella  
**Priority:** P1 proposed scheduling priority, not a vulnerability severity  
**Owner:** program owner; implementers and independent reviewers assigned per child

### Objective and problem

Existing GHB proof artifacts and Runtime identity/continuity components do not yet establish the full installed resident improvement loop. Deliver an authentic resident that observes failures, creates hypotheses and candidate changes, executes authorized experiments, updates beliefs from measured outcomes, adopts an independently evaluated successor, and continues using it after restart. Then demonstrate bounded recursive improvement of the improvement process.

### Required acceptance

- [ ] All 13 child work packages are linked with verified repository identities, owners, dependencies and current status; existing #112 is reused.
- [ ] Gate A passes through normal installed Runtime entrypoints: resident wake/task → GHB → actual provider/tool calls → admitted experiment → measured result → belief update → authorized successor activation → subsequent useful task → service restart → useful continuation under the same identity.
- [ ] Installed acceptance needs no ADL source checkout, development binary, fixture-generated authority, manual receipt transfer or manual branch switch.
- [ ] Gate B demonstrates an admitted change to a bounded improvement component and a subsequent cycle using it, with independent held-out/transfer evaluation and retained lineage.
- [ ] Authority, evaluator integrity, capability limits, concurrency, interruption recovery and rollback negative tests pass at the qualified revisions.
- [ ] Qualification covers the declared first domain and named Runtime/provider/body profiles; each intended fleet profile has an explicit qualified/pending/excluded disposition.
- [ ] Required local validation, CI, independent review, installed artifact provenance, costs and source-to-caller evidence are reconciled.
- [ ] Operator documentation and paper claim ledger match measured results; unsupported claims remain unclaimed.

### Non-goals

Foundation-model training, a proof of global optimality, consciousness/personhood claims, unrestricted self-modification, a replacement Runtime, an unbounded fleet rebuild, and public scholarly submission. Fleet deployment requires its own concrete rollout authorization. No experimental improvement result is guaranteed by finishing source code.

### Execution and closure

Use the child checklist below as the authoritative draft roster. Track Gate A and Gate B separately. Implement via bound FastWork worktrees, current typed lifecycle, required issue cards and independent pre-PR review. Cross-repository dependencies must use full URLs once created. Implementation PRs close their exact child issue; do not automatically close the umbrella with the first child PR. Umbrella closure requires all release gates and truthful terminal reconciliation. If empirical improvement is unproven, report that explicitly and keep the corresponding gate open.

The umbrella is organizational scope, not an extra implementation estimate. Coordination and review are included in the program contingency. Base full program: 77 engineering days and 8.45M development tokens before contingency; approximately 96 days and 10.6M development tokens with contingency, plus an illustrative 33M runtime-experiment tokens. Three-lane elapsed planning window: 10–12 weeks; see ESTIMATE.md for uncertainty and staged budget release.

## Child roster

| ID | Target repository | Proposed child title | Hard completion dependencies | Base days | Dev tokens M | Tracker |
| --- | --- | --- | --- | ---: | ---: | --- |
| WP00 | `agent-logic-runtime` | [GHB][WP00] Source-to-caller audit and baseline | None | 3 | 0.25 | Not created |
| WP01 | `agent-design-language` | [GHB][WP01] Normative specification and invariants | WP00 | 3 | 0.25 | Not created |
| WP02 | `agent-design-language` | [GHB][WP02] Schemas and installed contract parity | WP01 | 4 | 0.45 | Not created |
| WP03 | `agent-logic-runtime` | [GHB][WP03] Belief update and experiment selection | WP02 | 7 | 0.80 | Not created |
| WP04 | `agent-logic-runtime` | [GHB][WP04] Candidate generation and archive | WP02, WP03 | 6 | 0.65 | Not created |
| WP05 | `agent-logic-runtime` | [GHB][WP05] Isolated body builder and experiment executor | WP02 | 7 | 0.80 | Not created |
| WP06 | `agent-logic-runtime` | [GHB][WP06] Independent evaluator and promotion decision | WP00, WP02, WP03, WP05 | 8 | 0.90 | Not created |
| WP07 | `agent-logic-runtime` | [P1] Complete the governed resident-to-birthday activation path | WP00 | 5 | 0.55 | [Runtime #112](https://github.com/agent-logic/agent-logic-runtime/issues/112) |
| WP08 | `agent-logic-runtime` | [GHB][WP08] Successor activation and rollback | WP02, WP05, WP06, WP07 | 7 | 0.80 | Not created |
| WP09 | `agent-logic-runtime` | [GHB][WP09] Live GHB resident loop and Gate A | WP03, WP04, WP05, WP06, WP07, WP08 | 6 | 0.65 | Not created |
| WP10 | `agent-logic-runtime` | [GHB][WP10] Bounded recursive improvement | WP09 | 8 | 0.90 | Not created |
| WP11 | `agent-logic-runtime` | [GHB][WP11] Longitudinal qualification, fleet adoption and Gate B | WP10 | 10 | 1.10 | Not created |
| WP12 | `agent-logic-runtime` | [GHB][WP12] Research evidence and operator documentation | WP01, WP11 | 3 | 0.35 | Not created |

Dependency notes: WP04 can start archive work after WP02 but completes after WP03 integration. WP12 can start bibliography/runbooks after WP01 but final evidence closure requires WP11. WP07 retains its existing acceptance scope and owner; WP00 is a coordination prerequisite, not grounds to stop work already proceeding on #112. The installed loading/invocation smoke slice starts in WP02 and completes in WP09; do not defer Runtime integration discovery to the last wave.

### Sprint checklist

- [ ] WP00: [GHB][WP00] Source-to-caller audit and baseline
- [ ] WP01: [GHB][WP01] Normative specification and invariants
- [ ] WP02: [GHB][WP02] Schemas and installed contract parity
- [ ] WP03: [GHB][WP03] Belief update and experiment selection
- [ ] WP04: [GHB][WP04] Candidate generation and archive
- [ ] WP05: [GHB][WP05] Isolated body builder and experiment executor
- [ ] WP06: [GHB][WP06] Independent evaluator and promotion decision
- [ ] WP07: [P1] Complete the governed resident-to-birthday activation path — reuse [Runtime #112](https://github.com/agent-logic/agent-logic-runtime/issues/112)
- [ ] WP08: [GHB][WP08] Successor activation and rollback
- [ ] WP09: [GHB][WP09] Live GHB resident loop and Gate A
- [ ] WP10: [GHB][WP10] Bounded recursive improvement
- [ ] WP11: [GHB][WP11] Longitudinal qualification, fleet adoption and Gate B
- [ ] WP12: [GHB][WP12] Research evidence and operator documentation

## Issue bodies

All new children inherit proposed P1 scheduling priority, the umbrella Runtime delivery contract, exact-revision evidence, and the normal review/validation lifecycle. No source issue/card/PR has been created by this document.

### WP00 — [GHB][WP00] Source-to-caller audit and baseline

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** None  
**Base estimate:** 3 engineering days; 0.25M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** DESIGN_AND_RESEARCH.md §2; Runtime #112; installed release and production callers to be pinned.

Owner: program + Runtime. Dependencies: none.

Inventory all actual callers of GHB, adaptive learning, provider execution, memory, body/config changes, continuity and birthday. Map every required behavior to an existing owner, missing integration, or new implementation. Refresh Runtime #112 and coordinate with its current owner. Record the actual supported Runtime release, deployment topology, installed package versions, service entrypoints and durable state paths, and baseline tasks/results. Select the first domain, fixed task split, mutation permissions and cost envelope.

Acceptance: source-to-caller matrix at pinned revisions; no duplicated owner planned; runnable unchanged-agent baseline; measurable Gate A scenario; approved evaluation protocol and task/data licenses. A missing existing owner is an explicit gap rather than assumed absence.

**Validation:** Reproduce a baseline through the installed resident path; inspect production call sites and installed artifact versions; review task split, licenses and measurable acceptance protocol.

**Non-goals:** No implementation rewrite, new execution owner, deployment, or benchmark performance claim.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP01 — [GHB][WP01] Normative specification and invariants

**Repository:** `agent-logic/agent-design-language`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP00  
**Base estimate:** 3 engineering days; 0.25M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** ADL docs/milestones/v0.86/features/GODEL_HADAMARD_BAYES_ALGORITHM.md and docs/milestones/v0.89/ideas/GHB_EXECUTION_MODEL.md have conflicting phase semantics.

Owner: ADL. Dependencies: WP00.

Resolve phase semantics, task/improvement loops, termination, replay, evidence provenance, mutable/protected surfaces, identity/body distinction and conformance profiles. Write a proposed ADR and migration mapping from existing proof packets. Define typed rejection and recovery conditions.

Acceptance: every transition has inputs, outputs, authority, side effects and stop conditions; fresh inference versus replay is explicit; historical docs link to the normative successor after review; conformance labels remain proposed until accepted.

**Validation:** Review every transition against source contracts; walk positive, denied, deferred and interrupted scenarios; check historical terminology mappings and replay semantics.

**Non-goals:** No production algorithm implementation or claim of mathematical optimality.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP02 — [GHB][WP02] Schemas and installed contract parity

**Repository:** `agent-logic/agent-design-language`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP01  
**Base estimate:** 4 engineering days; 0.45M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** Existing installed Gödel contract distribution and ADL adl/src/godel/ghb_loop.rs packet types; DESIGN_AND_RESEARCH.md §§4–7.

Owner: ADL. Dependencies: WP01.

Implement observation, belief, hypothesis, candidate, body, experiment, trial, posterior, disposition, activation and lineage contracts. Reuse existing record types when compatible. Version semantic changes explicitly; distribute through the established installed contract path.

Acceptance: canonical digest/round-trip tests, malformed and stale-reference rejection, schema examples, installed Runtime compatibility test, and a migration policy for existing artifacts. New tests are classified in the validation inventory when authored.

**Validation:** Run schema/serde round trips, digest stability, negative compatibility cases and installed Runtime consumption without an ADL checkout. Register new test lanes.

**Non-goals:** No separate Runtime engine in ADL; no silent incompatible schema replacement.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP03 — [GHB][WP03] Belief update and experiment selection

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP02  
**Base estimate:** 7 engineering days; 0.80M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** ADL adl/src/godel/promotion.rs currently uses a fixed score; DESIGN_AND_RESEARCH.md §5 defines the proposed statistical replacement.

Owner: Runtime algorithm lane; exact module selected by WP00. Dependencies: WP02.

Implement tractable posterior models, likelihood adapters, calibration metrics, paired baseline comparison, unknown-evidence handling, information gain/value-of-information selection, resource costs and stopping rules. Keep model-based confidence separate from observed outcomes.

Acceptance: analytically known posterior fixtures; zero/impossible likelihood handling; correlated evidence cannot be silently double-counted; replay reproduces updates; informative experiments can differ from high-utility candidates; sparse evidence defers promotion.

**Validation:** Run analytical posterior cases, numerical edge cases, correlated/missing evidence cases, calibrated synthetic trials and deterministic evidence replay.

**Non-goals:** No invented probabilities for qualitative evidence; no candidate activation.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP04 — [GHB][WP04] Candidate generation and archive

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP02, WP03  
**Base estimate:** 6 engineering days; 0.65M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** ADL adl/src/godel/stage_loop.rs and ghb_loop.rs; DESIGN_AND_RESEARCH.md §6; archive and live-provider gaps in the inspected path.

Owner: Runtime algorithm lane. Dependencies: WP02; integrate WP03 selection after initial archive support.

Add proposer interfaces, typed interventions, predicted effects, falsification tests, candidate diversity/deduplication, parent selection, archive DAG, and bounded search. Retain rejected and inconclusive variants with provenance. Provider adapters supply actual outputs in separately budgeted trials.

Acceptance: candidate changes match the declared mutation class; exact parent and proposal provenance retained; archive cannot change production head; budget exhaustion terminates cleanly; at least two distinct useful candidate strategies are exercised.

**Validation:** Test DAG provenance, deduplication, parent selection, bounded candidate count and budgets; retain evidence of actual provider-generated distinct candidates separately from fixtures.

**Non-goals:** No production-head mutation or unbounded search; no evaluator access.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP05 — [GHB][WP05] Isolated body builder and experiment executor

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP02  
**Base estimate:** 7 engineering days; 0.80M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** DESIGN_AND_RESEARCH.md Runtime delivery contract and §7; Runtime adapter/capability owners to be mapped by WP00.

Owner: Runtime/execution. Dependencies: WP02.

Build candidate bodies from immutable parents in isolated worktrees/containers; use explicit capability and resource admission; compile/package as needed; run tasks with captured outputs and declared effects. Protect production credentials, evaluator data and production state.

Acceptance: denied network/filesystem/capability escapes; killed and timed-out experiments leave no production changes; exact candidate digest in results; measured resource accounting; controlled-adapter and live-provider execution classifications remain distinct.

**Validation:** Test prohibited effects, termination, timeout, resource exhaustion, candidate digest binding and interrupted cleanup using isolated state; separately prove live adapter integration.

**Non-goals:** No production deployment or credential access by candidates; no duplicate provider subsystem.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP06 — [GHB][WP06] Independent evaluator and promotion decision

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP00, WP02, WP03, WP05  
**Base estimate:** 8 engineering days; 0.90M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** ADL adl/src/godel/promotion.rs fixed threshold and DESIGN_AND_RESEARCH.md §5; independent measured promotion remains required.

Owner: evaluation lane. Dependencies: WP00 protocol, WP02 contracts, WP03 statistics, WP05 executor.

Implement development, selection and untouched final test splits; paired trials; quality/cost/reliability metrics; regression invariants; sequential-selection controls; evaluator versioning; and commit/reject/quarantine/defer decisions. Add candidates that intentionally alter tests, suppress failures, leak hidden tasks or falsify metrics.

Acceptance: these malicious/invalid candidates fail; raw outcomes can be independently recomputed; unseen final tasks are excluded from candidate context; thresholds declared before trials; non-improvement is a legitimate terminal outcome. Promotion recommendation cannot itself activate a body.

**Validation:** Independently recompute outcomes; test hidden-data access, test tampering, forged scores and suppressed failure attempts; verify predeclared statistical protocol and all four dispositions.

**Non-goals:** No self-scoring authority or hidden-test exposure; no body activation.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP07 — [P1] Complete the governed resident-to-birthday activation path

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Reuse existing Runtime #112; do not replace its body or widen scope automatically.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP00  
**Base estimate:** 5 engineering days; 0.55M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** https://github.com/agent-logic/agent-logic-runtime/issues/112; Runtime adl-runtime-kernel/src/production_birthday.rs.

Owner: Runtime. Dependencies: WP00 audit; coordinate/reuse Runtime #112 rather than opening a duplicate.

Complete production resident construction/consumption of identity, continuity, memory, capabilities, profile, adaptive history and witness authorities. Preserve the existing transaction store. Add explicit activation and useful continuation after restart.

Acceptance: Runtime #112's actual-resident positive/negative criteria pass without test-support authority substitutes; no implicit birthday on startup; one birthday under retry/concurrency/interruption; authenticated restored resident performs a useful task.

**Validation:** Use the existing issue’s actual-resident integration tests, forged/stale/missing prerequisite cases, duplicate/concurrent activation, interrupted recovery and useful restored continuation.

**Non-goals:** No duplicate birthday store or implicit birth on startup; no expansion of the existing issue without owner coordination.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP08 — [GHB][WP08] Successor activation and rollback

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP02, WP05, WP06, WP07  
**Base estimate:** 7 engineering days; 0.80M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** Runtime continuity and production_birthday owners; DESIGN_AND_RESEARCH.md §7 requires successor activation distinct from birthday.

Owner: Runtime. Dependencies: WP02, WP05, WP06, WP07.

Bind a promotion proposal to exact parent, candidate, evaluation version and capability envelope. Reuse continuity transactions for durable activation and recovery. Record rollback as a forward lineage transition. Define memory/body migration compatibility and staged cleanup.

Acceptance: failpoints at durable boundaries; stale parents and concurrent conflicting activations reject; crash recovery yields one authoritative head; accepted successor is actually loaded; rollback restores compatible execution without erasing history or replaying external effects.

**Validation:** Inject interruption at each durable boundary; test stale-parent/concurrent proposals, incompatible migration, actual successor loading and rollback as a retained lineage event.

**Non-goals:** No identity reset, history deletion, capability expansion, or blind memory rewind.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP09 — [GHB][WP09] Live GHB resident loop and Gate A

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP03, WP04, WP05, WP06, WP07, WP08  
**Base estimate:** 6 engineering days; 0.65M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** ADL ghb_loop.rs proof path does not invoke hosted providers; mandatory installed Runtime chain in IMPLEMENTATION_PLAN.md.

Owner: Runtime resident integration. Dependencies: WP03–WP08.

Wire observation, inadequacy detection, generation, experiments, posterior updates, decision, activation, memory and continuation into the normal installed resident execution path. Package every executable dependency in the Runtime distribution; add configuration compatibility, startup checks, durable migration handling, and version pinning or draining for in-flight work. Add cycle scheduling, budgets, cancellation, structured logs, traces, redaction and offline replay. Prevent runaway improvement triggers.

Acceptance: full Gate A scenario from a clean installed Runtime environment with no ADL source checkout or development binaries; separate controlled-adapter regression and live-provider evidence; successor handles later tasks; replay performs no external mutations; two competing cycles cannot both activate against one parent. Export an exact-version evidence packet.

**Validation:** Install the release in a clean environment; execute Gate A with real provider/tool calls; restart service and continue tasks; verify replay is side-effect free and logs are redacted.

**Non-goals:** No acceptance from a standalone ADL demo, fixture constructor, manual receipt injection or development checkout.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP10 — [GHB][WP10] Bounded recursive improvement

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP09  
**Base estimate:** 8 engineering days; 0.90M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** DESIGN_AND_RESEARCH.md §§7,9; empirical recursion requires a successor actually running the next improvement cycle.

Owner: Runtime algorithm and activation owners. Dependencies: Gate A.

Allow candidates to modify a bounded proposer, search-policy or experiment-allocation component. Define immutable authority/evaluator boundaries and compatibility checks. Retain both the original baseline and successor lineage. Require initial human review of source changes.

Acceptance: at least one admitted successor changes improvement behavior and executes the next cycle using that change; independent tests verify the change did not modify protected evaluation or authority rules. If no improvement emerges within budget, report the capability as implemented but empirical improvement unproven; do not manufacture acceptance.

**Validation:** Run a successor that changes an allowed improvement component and uses it in the next cycle; compare held-out outcomes and verify protected-state integrity independently.

**Non-goals:** No mutation of protected authority, identity roots, hidden evaluators, or capability ceilings.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP11 — [GHB][WP11] Longitudinal qualification, fleet adoption and Gate B

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP10  
**Base estimate:** 10 engineering days; 1.10M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** DESIGN_AND_RESEARCH.md §9 and IMPLEMENTATION_PLAN.md Gate B; current proof packets do not establish fleet qualification.

Owner: evaluation + Runtime. Dependencies: WP10; test design begins during WP06.

Run held-out transfer, multiple model/provider configurations, phase ablations, restart/migration, corrupted input, concurrency, rollback and reward-hacking campaigns. Inventory each resident's compatible body/capability profile. Define opt-in rollout, shadow operation, canary, monitored promotion, abort and rollback criteria.

Acceptance: qualification matrix names each tested domain/configuration; one resident reaches Gate B with useful post-restore continuation; every intended fleet profile is either qualified, pending or excluded with a reason; no blanket claim that all agents qualify from one demonstration. Rollout authorization is separate from local qualification.

**Validation:** Run preregistered held-out/transfer and ablation campaigns, two-week longitudinal observation, restart/concurrency/rollback cases, and a per-profile conformance matrix.

**Non-goals:** No automatic fleet deployment or universal claim from one resident; no threshold relaxation to obtain a pass.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

### WP12 — [GHB][WP12] Research evidence and operator documentation

**Repository:** `agent-logic/agent-logic-runtime`  
**Tracker action:** Create one new child after duplicate/owner check.  
**Parent:** GHB-SPRINT (link pending)  
**Completion dependencies:** WP01, WP11  
**Base estimate:** 3 engineering days; 0.35M development tokens. Runtime experiment consumption is charged to the shared campaign ledger, not multiplied per issue.

**Problem/evidence:** Paper GODEL_HADAMARD_BAYES_SOURCE_PACKET.md, GODEL_HADAMARD_BAYES_CLAIM_AND_CITATION_PACKET.md and provisional refs/sources.bib.

Owner: docs/research, with independent review. Dependencies: WP01 for vocabulary; WP11 for final claims.

Curate bibliography and related-work comparison; update paper claims only from exact source/experiment receipts. Publish internal operator runbooks for start, inspect, defer, quarantine, recover and rollback. Document model/data versions, costs, failed experiments and limitations.

Acceptance: claim-to-evidence ledger; reproducible benchmark instructions; installed command/manual parity where applicable; no unsupported theorem, novelty, personhood or general-performance claim. Scholarly publication is outside this plan's authorization.

**Validation:** Check each claim against source and trial receipts; verify bibliography metadata, runbook commands against installed help/manuals, links and reproducibility inputs.

**Non-goals:** No public paper submission, unsupported novelty claims, or invented implementation evidence.

**Completion record:** exact source and installed package revisions where applicable, changed production callers, validation/CI results, independent review findings and dispositions, measured token/time usage, and links to downstream consumers. Claims remain limited to what these records prove.

## Creation and handoff order

1. Refresh repository/issue identities and collisions; create the Runtime umbrella and WP00.
2. Create WP01 and WP02 in ADL with reciprocal umbrella links; map existing Runtime #112.
3. Create Runtime WP03–WP06 and WP08–WP11; encode dependency links rather than relying on numbering order.
4. Create Runtime WP12 as the evidence/documentation coordination child; link paper artifacts without presuming a separate paper implementation issue is needed.
5. Replace every local ID reference with a fully qualified issue link while retaining WP IDs for traceability. Verify the roster still has 13 children and one umbrella.

If WP00 discovers already-delivered or already-owned work, reuse those issues and evidence before creating duplicates. If a package must split, update the roster, dependency graph and estimate explicitly; the current 14-issue count is a planning count, not a prohibition on justified decomposition.
