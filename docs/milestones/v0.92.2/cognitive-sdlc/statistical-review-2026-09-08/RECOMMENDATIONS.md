# A bounded program to make the lifecycle converge

Promotion note: this is the retained statistical study, not a new census or independent recomputation. Detailed study datasets and logs remain local; their hashes are recorded in `../source-promotion-manifest.json`. The tracked plan is self-contained for implementation requirements; reproducing the historical statistics requires the retained study data.

This supplements [Planning #5’s simplification plan](../C_SDLC_V3_SIMPLIFICATION_PLAN.md). It does not replace its SIM-01 → SIM-07 ownership sequence or authorize implementation. Reconcile proposed work with current issues before creating any new ones.

## Priority 1: prove complete journeys and make broken transitions reproducible

Use the real public command boundary and installed candidate, not only the pure state model. Start with a small historical fixture corpus:

| Journey | Historical evidence | Required property |
| --- | --- | --- |
| Prepare with explicitly deferred future validators → ready → bind | #349 | An unchanged admitted contract remains admissible through the recommended transition |
| Implement → review finding → semantic amendment → revalidation → fresh review | #387, #400, #404, #407 | Every supported semantic correction has a legal, auditable recovery path with correct invalidations |
| Source proof → receipt retention → review → publish → finish | #53, #353 | Finite completion with distinct source/evidence/publication identities; no self-referential HEAD requirement |
| Existing valid binding plus unrelated stale historical worktree | #74 | Relevant collision guards remain; unrelated obsolete records do not block valid work |
| Draft publication → ready → uncertain response → reconciliation | #604 | A documented operation exists and reconciles the same remote intent without duplicate effects |
| Terminal readback → receipt → truthful result → cleanup eligibility | #721 | Machine output agrees with effects and terminal evidence |
| Cold install → help/schema → prepared issue start | Current root alias mismatch | Installed entrypoint, selector, guidance, schema, and command behavior agree |

Keep each original issue’s repair as historical evidence. A fixture should encode the invariant and public behavior rather than depend on that issue number or local path.

Define five properties across these journeys:

1. **Recommendation coherence:** if status recommends an action for version V, that action against unchanged relevant facts either succeeds or identifies the precise changed external fact. It cannot discover a contradictory phase-local rule.
2. **Recovery reachability:** valid interrupted and amended states have a finite supported route to a stable state, or an explicit operator decision. “No legal operation can repair the truth required by the next gate” is a defect.
3. **Idempotent effects:** retrying the same intent does not create another remote effect or needless generation. Uncertain outcomes are reconciled before retry.
4. **Evidence coherence:** source, issue version, review, and publication identities are distinct and linked by verified relations. The final publishable candidate still satisfies exact-head review.
5. **Observation purity:** status and validation do not acquire a mutation path that repairs state, changes registrations, or creates hidden lifecycle records.

These belong in SIM-01 through SIM-05, with full application proof in SIM-07. A matrix of helper-test passes is insufficient if no public journey reaches finish.

## Priority 2: remove repeated interpretation from operator input

SIM-02 and SIM-03 should make the authoritative route discoverable in one call. Resolve repository identity, current issue version, bound worktree, registry, and evidence references from canonical state. Users should supply intent and changed semantic content, not restate independently derived facts.

Keep generated typed requests for machine callers. A serialized stale request must still fail; automatic discovery is not permission to silently refresh and execute an old intent. Distinguish:

- malformed or unsupported intent, with the exact accepted schema/action;
- an expected wait, with the dependency or remote operation that will change it;
- stale facts, with the affected version/evidence identity;
- an unexpected implementation failure, with a stable incident code;
- a successful no-op versus a committed transition.

Move ordinary argument discovery out of the success path. The many `--root` versus `--repo`, positional versus named issue, and unavailable command examples should become contract tests against the installed interface. A descriptor table must generate or verify help, schema, dispatch, documentation, and package entrypoints together.

## Priority 3: one semantic record, explicit amendment policy, precise invalidation

SIM-04 and SIM-05 should eliminate separately maintained card truths. Keep six human-facing projections if they are useful; do not make six projections into six independent authorization stores.

The amendment contract should classify changes by semantic effect: scope/acceptance, execution plan, proof requirement, binding, implementation, review, and display-only projection. Each class declares allowed source states, prerequisites, evidence invalidations, and resulting state. This is more maintainable than adding a new exceptional field-specific repair every time a reviewer finds another stale sentence.

Use an invalidation dependency graph. Every invalidation records what changed and why each evidence object is affected. Formatting-only projection drift can require an explicit rebuild without pretending that implementation changed. A new Git commit still requires the exact review relationship specified by policy; do not conflate display independence with a review exemption.

SIM-04 must also own remote outcome incorporation. Keep durable external intent and reconciliation outside a fictitious single filesystem/network transaction. Test crashes on both sides of remote success and local receipt commit. No automatic replay of a possibly completed mutation.

## Priority 4: instrument before claiming a speedup

Add a measurement work item to the simplification program before baseline capture. This is an observability requirement, not another approval gate. The current audit events cannot measure dwell times.

Use the event contract in [METHODS.md](METHODS.md#what-a-real-state-machine-model-requires). At minimum distinguish `transition_applied`, `successful_noop`, `expected_wait`, `invalid_intent`, `stale_request`, `expected_guard_denial`, `unexpected_fault`, and `recovery_required`. Emit the attempted action, before/after issue versions, reason code, and timing even when the action is denied.

For waiting, record the start and end condition separately from active command execution. Required classes are dependency, human decision, independent review, CI, remote service, resource provisioning, and recovery. Do not estimate active work from arbitrary gaps in a conversation. Keep sensitive content out of events and preserve stdout JSON/stderr human-observability separation.

## Acceptance scorecard

The targets below are proposed, not measured achievements. Record them with the promoted plan and qualify the environment.

| Measure | Baseline here | Proposed acceptance |
| --- | --- | --- |
| Prepared issue to first useful work | Historical denominator not identified | Existing target: ≤3 minutes on the qualified environment; report distribution, cold/warm conditions, and all exceptions |
| Healthy deterministic public journeys | Historical counterexamples documented | Every declared healthy fixture completes through terminal reconciliation with no undocumented manual state edits |
| Recovery reachability | Repeated blocked-repair issue families | All declared interrupted/amended fixtures have a finite supported recovery or an explicit required operator decision |
| Diagnostic purity | Static shared recovery path identified | Zero lifecycle/registration/projection changes across healthy, missing, corrupt, and interrupted diagnostic fixtures |
| Duplicate remote effects | Not estimated retrospectively | Zero duplicates in the declared crash/uncertainty corpus; same operation identity after reconciliation |
| Operator-authored context fields | Must capture exact baseline | Zero redundant authority/receipt/context assertions in the ordinary intent interface; retain explicit semantic inputs |
| Avoidable retry attempts | Historical counts mix useful polling and faults | At least 50% reduction on the same adjudicated fixture corpus; preserve useful denials and report added steps elsewhere |
| Unexplained evidence invalidation | Not distinguishable in old audit stream | Every invalidation has a machine-readable cause and affected evidence identity |
| Outcome/timing observability | Untimed audits; many unknown log outcomes | 100% of declared public command attempts emit a correlatable outcome; missing events fail the telemetry contract test |
| Acceptance and security quality | Not reducible to closure counts | No regression in negative authorization, stale review, topology, corruption, zero-test, or cleanup-denial fixtures |

Do not use a falling rejection rate as the sole success measure: deleting guards would improve that number while damaging correctness. Report unexpected faults and legitimate denials separately, along with completed journeys and downstream acceptance failures.

## Evaluation design

1. Freeze the current installed baseline and a representative fixture corpus before changes. Include easy issues, dependency-bound issues, dirty bound worktrees, reviewed/published issues, and interrupted remote effects. Retain command/effect/timing traces and environment identity.
2. Run old and candidate implementations on isolated copies with the same deterministic inputs and fake remote responses. This is a comparison of complete journeys, not two live writers for one issue. Count manual inputs, attempts, invalidations, and effects, and inspect adverse cases.
3. Review every unexpected denial in the fixture corpus. Classify the cause, distinguish an intended guard, and fix the owning invariant rather than relaxing the test to fit the output.
4. After the separately authorized fenced conversion, collect prospective telemetry on consecutive eligible issues, including all failures and abandoned work. Stratify by outcome type, dependency load, proof profile, and code-change scale. Start with at least 30 consecutive journeys as a pilot, report wide intervals, and determine the required larger sample from observed variance; 30 is not statistical proof of a small error rate.
5. Compare matched categories and time trends. A pre/post difference remains observational. If traffic and ownership permit a later controlled evaluation, vary operator-interface presentation on isolated eligible work without changing authority or removing safeguards. Never create competing writers to obtain an experiment.
6. Publish the denominator: eligible, attempted, completed, abandoned, censored, and excluded journeys. Retain reasons. Report median and P90 active/setup/wait time, first-pass yield conditional on valid prerequisites, recovery visits, and escaped acceptance defects.

Rare-event claims require enough exposure. For example, observing zero unexpected failures in 30 independent comparable journeys still leaves a one-sided 95% binomial upper bound of approximately 9.5%; it does not establish 99% reliability. With zero failures, approximately 299 independent comparable journeys are needed to put that bound below 1%. Correlation or changing scope weakens that calculation. This is a sample-planning illustration, not an assertion that the journeys will be independent.

## Crosswalk to Planning #5

| Existing package | Evidence contribution from this study | Addition to acceptance |
| --- | --- | --- |
| SIM-01 diagnostics | Current shared lock/recovery source path; inability to distinguish attempted vs hidden repair | Snapshot effects plus attempt/outcome telemetry, from root and genuine non-primary worktree |
| SIM-02 command contract | Historical #604/#721 and current installed alias discrepancy | Verify installed package/help/schema/guidance parity, not source descriptors alone |
| SIM-03 typed context and intent CLI | Repeated malformed owner arguments and discovery effort | Ordinary journeys require no manually reconstructed receipt/context chain |
| SIM-04 canonical transaction owner | #349 reachability contradiction; #53/#353 evidence cycles | Full transition postconditions, amendments, finite recovery, and remote-intent reconciliation |
| SIM-05 projections | #387/#400/#404/#407 repair cascade | One semantic truth, explicit change classes, causal invalidation records |
| SIM-06 conversion | Separate repository histories, preserved audits, active worktree scope | Exact census and identity mapping; no erased recovery or terminal history |
| SIM-07 deployment proof | Low confidence from helper-only proof; immature post-v3 population | Installed public journey benchmark and prospective scorecard, settled checks and independent review |

Keep the existing coordinated writer pause, old-writer fencing, conversion census, restore boundary, and exact-head approval requirements. Do not stop Runtime/provider services as part of this program. Do not create a new C-SDLC generation to avoid proving the current one.

## What not to prioritize from these statistics

- A blanket reduction of review requirements: the data cannot distinguish all legitimate findings from tool-induced review churn, and several cases show useful reviewers catching false readiness.
- Another broad card rewrite without a coherent amendment model: it moves representations without fixing their contradictory lifecycle rules.
- Removing guards merely because bind or finish reports many adverse observations.
- Treating all pre-PR hours as waste, all repeated commands as pointless, or all closed issues as delivered.
- Claiming savings in dollars or model tokens from session totals: the study has no reliable attributable billing denominator.

The bounded next planning decision is to add the baseline/telemetry and journey contracts to SIM-01 through SIM-07, then promote them through the current typed planning route. The evidence supports that direction now; implementation and activation remain separate work.
