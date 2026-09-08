# v0.92.2 C-SDLC v3 Simplification Plan

Status: tracked planning candidate promoted through #523 / PR #743; implementation and cutover are not authorized by this document.

Authoring context: Planning #5, 2026-09-08; revised by Planning #6 on 2026-09-08 following the statistical review and operator direction.
Target release: **v0.92.2**. Program size: **eight planned issues: one umbrella plus SIM-01 through SIM-07**. This is the C-SDLC program count, not the total milestone count. Promotion is included in #523 / PR #743; no separate plan-update issue is required.
Source baseline: `bf159eb416950dfa3399933829726a7b7e71f897` on ADL `main`.
Audience: milestone planners, C-SDLC maintainers, implementation agents, and independent reviewers.

## Decision and objective

Simplify C-SDLC v3 around one operational lifecycle, one authoritative issue record, and commands that accept operator intent while assembling and verifying their own context and evidence.

The operator selected a **full staged program** and a **coordinated breaking change**. Develop and prove the replacement in bounded slices, then activate it during one explicitly scheduled writer pause. Do not maintain permanent compatibility adapters or two active writers. Existing requests and issue state are converted explicitly; obsolete request formats are rejected after activation with actionable diagnostics.

This is a simplification within C-SDLC v3, not a new generation or permission to repeat the v2-to-v3 authority cutover. Preserve the native selector and authenticated cutover provenance. Any selector/receipt change needed for the new writer revision requires its own reviewed transition evidence.

The controlling acceptance outcome is that ordinary issues complete successfully and supported amendments or interruptions have a finite recovery path.

Success means fewer independently interpreted facts, fewer operator-authored request fields, and fewer opportunities for the same issue to appear in contradictory states. Lines deleted and command count alone are not acceptance criteria.

## Observed problems and evidence

These findings are from static source inspection, not executed failure demonstrations. Reconfirm them at the implementation baseline; ongoing work may have changed them.

| ID | Priority | Observation | Source at baseline |
| --- | --- | --- | --- |
| S1 | P1 | Diagnostic routes enter a shared executor that locks and recovers pending mutations before dispatch. Recovery can register worktrees and replace state directories, while diagnostic results report no mutation. | `csdlc-v3/src/commands/local/mod.rs`: `execute_operational_local_route`, `recover_pending_local_transaction`, `commit_local_transaction`, `diagnose_operational_issue` |
| S2 | P2 | The pure lifecycle and durable transaction store are not the common operational command path. Local commands maintain a separate index/journal; remote mutations and terminal writes have additional persistence paths. | `csdlc-v3/src/lifecycle/mod.rs`, `csdlc-v3/src/storage/mod.rs`, `csdlc-v3/src/commands/local/mod.rs`, `csdlc-v3/src/commands/remote/mod.rs`, `csdlc-v3/src/commands/terminal.rs` |
| S3 | P2 | The local lifecycle digest incorporates all six values files and rendered Markdown files, coupling presentation drift to lifecycle concurrency. | `csdlc-v3/src/commands/local/mod.rs`: `persist_index`, `lifecycle_digest`, `validation_findings` |
| S4 | P2 | Local requests repeat context and require registry/registration inputs; remote requests expose receipt paths, digests, and repeated revision facts. | `csdlc-v3/src/main.rs`: `run_local_report`, `run_remote`; local and remote request types |
| S5 | P2 | Remote `expected_lifecycle_digest` denotes selector evidence, while readiness and approval output fields reuse the selector digest. Local approval and selector fields also duplicate one fact. | `csdlc-v3/src/commands/remote/mod.rs`: `verify_canonical_v3_authority`; local `OperationalLocalContext` and `validate_context` |
| S6 | P1 | Current guidance, historical construction instructions, command manifests, tests, and runtime fallback paths expose conflicting operational interpretations. | `csdlc-v3/AGENTS.md`, `csdlc-v3/README.md`, `csdlc-v3/src/main.rs`, `csdlc-v3/tests/command_manifest.rs`, `docs/csdlc-v3/v3-command-manifest.json` |

The existing `docs/csdlc-v3/proportional-lifecycle.json` already describes collapsed SIP/STP/VPP and derived SRP surfaces. This proposal implements that simplification while retaining the six-card vocabulary and reviewable projections. It does not remove meaningful design or validation gates.

## Statistical evidence and limits

The [statistical review](statistical-review-2026-09-08/REPORT.md), [methods](statistical-review-2026-09-08/METHODS.md), and [recommendations](statistical-review-2026-09-08/RECOMMENDATIONS.md) inform this revision. The accessible census covers 3,538 issues and 3,096 PRs across the legacy and canonical repositories; 82,411 historical command bundles were analyzed. Among 42,743 observable outcomes, 30.8% were adverse observations, including legitimate guard denials and expected waits. This is not a defect rate. Roughly 48% of command outcomes were unknown. Retained audits show review recovery on 336 of 524 issues, including at least three recoveries on 172; neither count establishes that those reviews were avoidable.

The evidence prioritizes transition coherence, recovery, and installed-command behavior. It does not establish a causal tooling tax, wasted labor, monetary savings, or post-cutover v3 effectiveness. GitHub closure and merged linkage do not prove acceptance; pre-PR elapsed time includes useful work and backlog. Historical fixtures encode invariants from repaired issues, not claims that every old defect remains open. Preserve both repository identities in evidence.

## Target architecture and interfaces

### One application path

Use four explicit responsibilities: CLI intent parsing, application orchestration, pure lifecycle decisions, and effect adapters/storage. Every operational transition passes through the same lifecycle decision and issue transaction boundary. Reuse and extend the existing lifecycle/storage code; retire the alternative local transition mechanism after conversion and equivalence proof.

Do not force remote network operations into a local filesystem transaction. Preserve durable remote intent, idempotency, and authenticated reconciliation as an effect protocol. A verified remote outcome becomes input to the issue transaction; a crash between remote success and local commit must reconcile the existing operation rather than repeat the mutation.

### One issue record and separate projections

Use one canonical versioned issue record under `.csdlc/v3/issues/<issue>/state.json`. It contains issue/repository identity, the typed intent and plan, validation requirements, binding identity, lifecycle phase, generation, and references to review, publication, and terminal evidence. The existing transaction model owns generation checks, semantic digest, audit ordering, and recovery.

All six cards remain generated views. Their values derive from the issue record and evidence references rather than six separately maintained sources of lifecycle truth. Store projection digests separately from the semantic state digest. Missing or altered projections must be visible and repairable through an explicit mutating operation; inspection must not repair them.

A formatting-only projection change does not invalidate semantic evidence. A scope, plan, acceptance, validator, binding, or implementation change must trigger the corresponding existing invalidation rules. Exact-head publication remains exact-head: a new Git commit still requires review freshness even when the edit is presentational. This proposal does not create an exemption from that guard.

Evidence receipts remain immutable and independently verifiable. The issue record indexes their identity and disposition; it does not replace or duplicate their contents. Terminal completion belongs to this record with a referenced terminal receipt, replacing the separate operational terminal-state interpretation.

### Typed facts

Introduce distinct validated types for `AuthorityEvidence`, `IssueVersion`, `CheckoutIdentity`, `ReviewEvidence`, and `RemoteObservation`. `IssueVersion` contains generation and semantic digest. Authority evidence binds the canonical selector and receipt; checkout identity binds repository, branch/worktree, and exact revision. Review evidence binds scope, revision, principals, findings, and proof.

Remove mislabeled digest fields and duplicate caller assertions such as a review-present flag accompanying a verified review receipt. Resolve facts once per command and pass validated objects through the application layer. Recheck issue version, binding, and authority freshness at the actual mutation boundary; do not introduce a long-lived authority cache.

### Operator commands

The following are proposed interfaces, not commands currently available:

- `csdlc status <issue> [--json]`: observe issue state, authority, worktree, evidence freshness, blockers, and allowed next operations.
- `csdlc prepare <issue> --plan <path>`: prepare the typed issue contract and projections using the canonical registry and fetched issue identity.
- `csdlc bind <issue>`: resolve the reviewed plan and canonical worktree policy, preview the target in the result, and bind only when all prerequisites pass.
- `csdlc edit <issue> --changes <path>`: apply typed semantic field changes and required invalidations through the issue transaction.
- `csdlc validate <issue>`: check existing evidence and projections without running validators or repairing state. Validator execution remains an explicit `csdlc proof <issue>` action.
- `csdlc review <issue> --evidence <path>`: record externally produced independent review evidence after verifying its identity, scope, findings, and revision.
- `csdlc publish <issue>`: derive the canonical target and review inputs, then execute the governed publication action and reconcile its exact result.
- `csdlc finish <issue>`: observe remote terminal truth and record verified completion.
- `csdlc clean <issue>`: preview exact registered-worktree cleanup; `--execute` consumes a fresh preview and rechecks eligibility before removal.
- `csdlc recover <issue>`: describe pending recovery; `--execute` performs only the classified recovery against the same issue version and transaction identity.

Keep direct GitHub issue operations as a distinct `csdlc github` command family with typed operation inputs; do not conflate creating a remote issue with preparing local execution. Keep simulation, historical foundation imports, and cutover qualification under an explicit proof/administrative namespace rather than automatic operational fallback. Installation remains an administrative operation.

Retain machine-oriented typed requests as an advanced interface, generated by the same intent parser. They use the new schema and invoke the same application path. Discover repository, template registry, worktree registration, and evidence references from canonical sources; fail on ambiguity instead of guessing. A stale serialized request is rejected, never silently refreshed and executed.

One command descriptor table defines public help, input schema, effect classification, result shape, and tests. Use one versioned result envelope with command, issue, status, authority status, issue version, effects performed, findings, and allowed next operations. `blocked` is a diagnostic state, not evidence of successful execution. JSON stays on stdout; human observability stays on stderr, with existing redaction and compatibility logging behavior explicitly tested.

## Ordered work packages

The umbrella owns dependency ordering, the release scorecard, adjudication of failures, and final evidence aggregation. It does not duplicate implementation scope. These are planning identifiers, not created issues or execution assignments. Do not take over currently underway work. Before sprint launch, reconcile each package against then-current issues and merged changes. The native validator, PR readback/identity and ready-transition repairs in PR #743 are baseline fixes, not completion of a SIM package.

| Package | Deliverable and boundary | Acceptance / handoff |
| --- | --- | --- |
| SIM-01 | Reproduce S1 using isolated fixtures, then separate observation from mutation/recovery. Move state-directory creation and mutation locks out of read-only paths. Freeze the installed baseline and historical public-journey corpus; introduce attempt/outcome measurement here. | Diagnostics leave issue files, journals, registrations, and projections unchanged for healthy, interrupted, missing, and corrupt state. They report recovery-required explicitly. Retain baseline traces, environment identity, manual inputs and outcome denominators; exercise root and genuine non-primary worktree entrypoints. |
| SIM-02 | Define the single current command contract and separate historical construction fixtures/guidance from operational discovery. Preserve historical bytes where they are retained proof. | Installed executable provenance, help, schemas, nested guidance, command descriptors, and tests agree about current authority and effects; no automatic construction fallback on operational errors. |
| SIM-03 | Add validated evidence types and canonical context resolution; implement the intent-oriented CLI against the existing operational behavior first. | New callers supply intent and changed content rather than hand-built context/receipt chains; stale requests, ambiguous topology, wrong repository, and stale review are rejected. |
| SIM-04 | Extend the existing lifecycle/transaction owner to hold the complete semantic issue record; route local transitions and verified remote/terminal outcomes through it. | Every mutating journey is visible in one issue generation/audit history; no command writes an independent lifecycle phase or terminal truth. Crash and concurrent-writer proofs cover real application entrypoints. Recommended transitions remain admissible against unchanged relevant facts; supported amendments and interruptions have finite legal recovery. |
| SIM-05 | Derive the six cards and separate semantic versus projection integrity. Provide explicit projection rebuild through the transaction/application owner. | Projection-only drift is classified independently; semantic changes invalidate the right evidence; regeneration is deterministic and cannot invent acceptance or completion. Classify scope/acceptance, plan, proof, binding, implementation, review and display changes; declare source states, prerequisites, resulting state and affected evidence with causal invalidation records. |
| SIM-06 | Implement explicit conversion, transition manifest, writer fencing, and restore tooling. Rehearse the coordinated breaking change on copies of representative active records. | Exact census, lossless semantic mapping, old/new revision evidence, interrupted-conversion recovery, and pre-resume restore all pass. Unsupported records stop conversion with an issue-specific disposition. |
| SIM-07 | Independently review the complete candidate, qualify the journey evidence accumulated since SIM-01, and prepare the deployment runbook and prospective measurement protocol. Activate only in the separately authorized pause. | No unresolved actionable review findings; required aggregate CI settled; conversion/readback and all pre-resume acceptance gates pass before writers resume. After authorized activation, report a consecutive-issue pilot including failures and abandonment; keep reliability claims bounded by exposure. |

Scheduling: this is the first coherent v0.92.2 sprint, authorized to proceed in parallel with Runtime work. Its own issue readiness and disjoint ownership govern startup; it does not wait for WP-01, the CodeFriend build or unrelated prior closeout. SIM-UMBRELLA opens and coordinates the sprint, and completes only after SIM-07.

Order: SIM-01 → SIM-02 → SIM-03 → SIM-04 → SIM-05 → SIM-06 → SIM-07. Preparation may overlap only with disjoint ownership. Earlier correctness repairs may ship separately; the incompatible CLI/state replacement activates once, after the full transition proof. No intermediate deployment may introduce two writers for one issue.

## Journey benchmark and package handoffs

Start in SIM-01 and extend the same corpus in every package; SIM-07 is final qualification, not the first end-to-end test. Use installed executables and real application paths in isolated repositories with deterministic fake remote transport. Preserve exact baseline/candidate versions and compare isolated copies, never two live writers on one issue.

| Journey / historical canonical issue | Owning packages | Required behavior |
| --- | --- | --- |
| Healthy prepare → bind → edit → proof → review → publish → finish → clean | SIM-01 baseline; SIM-03–07 extensions | Complete without undocumented manual state edits |
| Deferred future validators, #349 | SIM-03/04 | Status recommendation and next transition agree on unchanged facts |
| Review finding → semantic correction → revalidation → fresh review, #387/#400/#404/#407 | SIM-04/05 | Finite recovery by amendment class, with precise invalidations |
| Proof/evidence commits → review → publication, #53/#353 | SIM-04/05 | Distinct verified identities; no self-referential HEAD requirement; exact-head review preserved |
| Valid binding with unrelated stale worktree, #74 | SIM-03/04 | Preserve relevant collision guards without unrelated obsolete blockers |
| Draft → ready and uncertain remote response, #604 | SIM-02/04/06 | Supported command, durable intent and reconciliation without duplicate effects |
| Terminal readback → receipt → result → cleanup, #721 | SIM-02/04/07 | Output agrees with effects and verified terminal truth |
| Install → help/schema → prepared start | SIM-01/02/03 | Discoverable authoritative installed route agrees with guidance |

Canonical historical issue references above mean `agent-logic/agent-design-language`; full evidence is in the linked statistical report. Each package owns its fixture additions, baseline/candidate comparison, and disposition of failures. An expected guard denial must remain a passing negative case, not be erased to improve success rates.

Required properties are recommendation coherence, finite recovery, idempotent effects, independently verified evidence identities, and strictly observational diagnostics. A supported recovery may require an explicit operator decision with a named owner and actionable choices; a dead end without a legal repair is a defect.

### Measurement contract

SIM-01 introduces a small attempt-level measurement contract and baseline harness. SIM-02 standardizes its result envelope; SIM-03–06 preserve it; SIM-07 aggregates the scorecard. Record attempts, including denials, with correlation/intent identity, before/after issue version, operation, outcome, stable reason code, effects, evidence invalidation cause, wall time and monotonic duration. Inject clocks at effect boundaries so the semantic kernel remains deterministic.

Distinguish transition applied, successful no-op, expected wait, invalid intent, stale request, expected guard denial, unexpected fault, and recovery required. Record wait start/end and owner separately for dependency, human decision, independent review, CI, remote service, provisioning, and recovery. Missing outcomes remain missing, not successes. Conversation gaps are not active work. Preserve redaction and stdout/stderr contracts.

| Scorecard | Proposed acceptance |
| --- | --- |
| Prepared start | ≤3 minutes to first useful work in the qualified environment; retain cold/warm conditions and exceptions |
| Healthy journeys | Every declared deterministic healthy fixture reaches terminal reconciliation and eligible cleanup |
| Recovery | Every declared supported amendment/interruption has finite supported recovery or an explicit actionable operator decision |
| Diagnostic effects | Zero lifecycle, registration or projection mutations |
| Remote uncertainty | Zero duplicate remote effects in the declared crash corpus |
| Ordinary CLI input | Zero redundant manually reconstructed authority/receipt/context assertions |
| Avoidable retries | At least 50% reduction on the same adjudicated corpus; report useful denials and any added steps separately |
| Observability | 100% correlatable outcomes for declared public command attempts; every invalidation names its cause and affected evidence |
| Preserved safeguards | No regression in authorization, stale review, topology, corruption, zero-test or cleanup-denial cases |

These are targets, not achieved results. Publish attempts, manual interventions, completed journeys, first-pass yield conditional on valid prerequisites, recovery visits, escaped acceptance defects, and median/P90 setup, active and wait durations. Report eligible, attempted, completed, abandoned, censored and excluded denominators with reasons.

After separately authorized activation, observe at least 30 consecutive eligible journeys, including failures and abandoned work, stratified by work type, dependencies, proof profile and scale. Use pilot variance to plan further exposure. Zero failures in 30 independent comparable journeys still permits an approximately 9.5% one-sided 95% upper bound; it does not prove 99% reliability. Pre/post comparisons remain observational, and correlated journeys weaken rare-event claims. This post-resume pilot is a separate qualification step, not a prerequisite that could only be measured after the pre-resume gate.

## Coordinated breaking-change procedure

1. **Inventory:** record exact installed binary provenance, active worktrees, registered branches, issue generations/digests, pending local transactions, remote intents, review/publication receipts, and terminal records. Preserve dirty work; do not clean other sessions. Inventory the state actually used by each checkout, including legacy path layouts.
2. **Rehearse:** convert copies in isolated fixtures. Compare each old record's scope, plan, validation, bindings, and evidence identities with the new record. Keep all receipts and historical projections. Refuse ambiguous or unsupported state; do not infer missing approvals or reviews.
3. **Schedule and fence:** notify active operators through the repository coordination surface and obtain an explicit pause window. Drain or explicitly classify in-flight mutations. Enforce a writer fence respected by the old installed generation before installing the new one; a replacement that only fences itself is insufficient. No live conversion begins until old writers are stopped or demonstrably fenced.
4. **Snapshot and convert:** retain a hash-bound snapshot and source-to-destination mapping for every affected state file. Convert to staging, validate the complete census, then activate the new layout and stable installed binary through a transition journal with restartable steps. Do not delete source records during activation.
5. **Verify while paused:** check per-issue semantic equivalence, live binding identity, evidence digests, canonical authority, command effects, and absence of competing writers. Run read-only inspection from the primary checkout and an actual non-primary worktree. Reject old request schemas with an explicit conversion diagnostic.
6. **Resume:** allow writes only after the full conversion and installed-provenance readback pass. Retain the old executable and snapshots as restore material, not a callable alternate lifecycle route.
7. **Restore boundary:** before any new operational write, restore the snapshot and prior binary under the same pause and writer fence if verification fails. After a new-format write or remote effect, automatic snapshot restoration is forbidden: freeze writers and use a separately reviewed reconciliation/forward-repair plan so new evidence is not lost or remote work repeated.

The pause is for C-SDLC writers. Do not stop or mutate resident Runtime/provider/cloud services as part of this migration. No paid execution, release ceremony, or external publication beyond separately authorized lifecycle work is implied.

## Validation and release criteria

Assign every new test a PVF lane, proof role, determinism/resource posture, and release-gate status. Use focused owner validation during each slice; the final proof must exercise public commands and production application paths rather than only the pure model.

- **Read-only guarantee:** byte snapshots and Git worktree registration snapshots remain unchanged across status/validation of healthy, missing, corrupt, and interrupted states. Also test invocation from a bound FastWork worktree.
- **Recovery:** inject failures before/after intent durability, external effect, state activation, receipt persistence, and projection completion. Restart reconstructs the same outcome without double execution or hidden repair during inspection.
- **Concurrency:** two requests with the same issue version cannot both commit. Authority or checkout changes before mutation are rejected. An old binary cannot write during or after conversion.
- **Evidence:** reject stale review SHA, changed acceptance scope, wrong reviewer identity, altered receipt bytes, wrong PR base/head/linkage, missing terminal evidence, and ambiguous worktree ownership.
- **Projection semantics:** formatting drift produces a projection finding; a changed typed plan invalidates its dependent evidence. Regeneration preserves source facts and exact published artifact provenance.
- **Remote effects:** simulate ambiguous network outcomes and crash after remote success but before issue commit. Reconciliation uses the recorded operation identity and does not repeat the mutation.
- **Conversion:** cover prepared, bound/dirty, implemented, reviewed, published, terminal, and pending-recovery cases. Report exact record counts and retained evidence counts, with zero silent omissions. Rehearse restore before resume and denial of unsafe restore afterward.
- **Operator journey:** prepare → bind → edit → proof → review → publish → finish → clean through the installed candidate in an isolated repository, using deterministic fake remote transport for ordinary CI. Any live GitHub proof requires separately scoped authorization and retained readback.
- **Observability:** validate stdout/stderr separation, honest effect reporting, redaction, and documented compatibility logging. No secrets or account identifiers enter the planning/proof artifacts unnecessarily.
- **Portability:** run required supported-platform checks; preserve explicit denial where mutation is unsupported. This program does not silently claim new Windows operational support.

Measure the same prepared-issue journey before and after: operator-authored fields, distinct request files, required invocations, authority reads/Git subprocesses, and elapsed time. The prepared issue must be inspectable, bindable, and ready for first useful work within the existing three-minute target on the qualified environment. Report the environment and actual measurements; no speed claim is established by this plan.

Final acceptance requires the journey and measurement scorecard above, one operational transition owner, one semantic issue record, strictly observational diagnostics, one current CLI contract, preserved authority/review/cleanup guards, passing conversion/restore rehearsals, independent exact-head review, and settled required aggregate CI.

## Planning status and next boundary

The operator has selected v0.92.2 and the eight-issue program structure. GitHub umbrella and SIM issue numbers remain unassigned. Other v0.92.2 work exists outside this program; reconcile overlaps before assigning owners. This tracked planning document is not a C-SDLC card, execution binding, approved architecture decision, or operational authority. The eight rows are reconciled into the milestone wave; issue numbers and execution owners remain to be resolved at the dedicated sprint launch.

Only the source review and document checks have been performed. Failure reproduction, runtime tests, performance measurements, compatibility conversion, independent implementation review, and transition rehearsal remain future work. The principal residual risks are preserving evidence invalidation semantics, fencing existing writers, and reconciling a remote success across a local crash.

The operator has authorized updating and promoting this plan through one bounded planning issue. That does not authorize implementing the seven SIM packages or activating conversion. The next program action is to reconcile packages with ongoing C-SDLC work and assign bounded owners through the active typed lifecycle. Promotion is carried by existing #523 / PR #743 at the operator’s direction, so no additional plan-update issue is needed. The source document’s blocked issue-creation attempt remains historical evidence; it is not a startup dependency for this sprint. Implementation, conversion activation and terminal delivery remain separate future work.
