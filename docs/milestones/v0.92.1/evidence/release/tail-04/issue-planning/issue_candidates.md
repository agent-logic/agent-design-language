# #520 Finding-to-Issue Plan

## Metadata

- Source review: issue #520 internal review
- Remediation umbrella: issue #522
- Exact base: `f0a011a5c59d46c763d669f69a10308b3f870ba4`
- Exact candidate: `c24f8fa65ce445b03ce6cd69007307291d78b60c`
- Source roles: architecture, code, dependencies, documentation, security, tests
- Status: `partial`
- Ready candidates: 16
- Confirmation-required findings: 1
- Resolved/non-actionable historical findings: 2
- Grouping policy: group only findings with the same root cause and remediation target; preserve every source finding and the highest severity

Synthetic source IDs were assigned to specialist findings that did not carry an
ID: `A520-ARCH-*`, `C520-CODE-*`, `D520-DOC-*`, and `S520-SEC-*`. The tests and
dependency IDs are preserved verbatim.

## Scope

This plan covers every finding in the six completed specialist reports under
`tail-04/specialists/`. It prepares bounded GitHub issue candidates under #522.
It does not create tracker items, change product code, resolve lifecycle state,
or claim that any remediation is complete.

## Issue Candidates

### R520-001 — [P1] Make dynamic-agent removal crash-consistent across roster and checkpoint state

- Source findings: `A520-ARCH-001`, `C520-CODE-001`
- Source roles: architecture, code
- Confidence: high
- Affected paths: `adl-runtime-kernel/src/control.rs`, `adl-runtime-kernel/src/agent_partial_checkpoint.rs`
- Evidence: removal persists the authoritative tombstone before the reduced roster. A later persistence failure can leave the agent resident while restart suppresses its checkpoint.
- Problem: dynamic-agent removal has no recoverable atomic authority across roster, orientation, checkpoint/tombstone, and derived in-memory projections.
- Acceptance criteria:
  - Define one durable lifecycle state machine and authoritative recovery rule.
  - Make every interrupted persistence boundary converge to a resident agent with recoverable state or an absent agent with an authoritative tombstone.
  - Reconcile all durable and in-memory projections idempotently on restart.
  - Never report failed removal while leaving an unrecoverable resident identity.
- Validation: inject failure at every persistence boundary, reopen after each failure, assert one coherent terminal state, then run focused removal/checkpoint/restart suites.
- Non-goals: write-order reversal alone; unrelated checkpoint serialization or provider redesign.
- Dependencies: #522; land before regenerating dynamic-agent continuity proof.
- Approval status: operator approved for separate typed issue creation.

### R520-002 — [P1] Persist and recover admission-triggered A2A initiation

- Source findings: `A520-ARCH-002`, `C520-CODE-002`
- Source roles: architecture, code
- Confidence: high
- Affected path: `adl-runtime-kernel/src/control.rs`
- Evidence: admission succeeds before Beacon's greeting has durable ownership; the spawned result is discarded, and `already_present` replay cannot resume scheduling.
- Problem: restart or transient provider failure can permanently suppress the autonomous greeting while transport remains healthy.
- Acceptance criteria:
  - Persist an idempotent greeting/outbox intent with admission.
  - Resume pending work on startup and repeated admission.
  - Use bounded retry and a stable conversation-ledger idempotency key.
  - Expose pending, retrying, completed, and terminal-failure state.
- Validation: test provider refusal, task interruption, reopen, replay, and duplicate suppression; demonstrate autonomous initiation without an operator prompt.
- Non-goals: promising exactly-once provider delivery; building a general workflow engine.
- Dependencies: #522; coordinate with conversation-ledger ownership.
- Approval status: operator approved for separate typed issue creation.

### R520-003 — [P2] Continue dynamic-agent health refresh after an individual task failure

- Source finding: `C520-CODE-003`
- Source role: code
- Confidence: high
- Affected path: `adl-runtime-kernel/src/control.rs`
- Evidence: the JoinSet loop exits on the first `JoinError`; dropping the populated set aborts remaining checks.
- Problem: one failed health task leaves unrelated agent readiness stale.
- Acceptance criteria: explicitly handle success, failure, and exhaustion; drain all remaining checks; preserve failed-agent identity; retain successful peer updates.
- Validation: deterministic multi-agent test with one failed task and multiple successful tasks, plus focused health tests.
- Non-goals: changing provider health semantics; hiding task failures.
- Dependencies: #522.
- Approval status: operator approved for separate typed issue creation.

### R520-004 — [P1] Align every current C-SDLC v3 authority surface with the completed cutover

- Source finding: `A520-ARCH-003`
- Source role: architecture
- Confidence: high
- Affected paths: `AGENTS.md`, `csdlc-v3/AGENTS.md`, `docs/csdlc-v3/TOOLING_CHANGEOVER_NOTICE.md`, C-SDLC v3 help/module/package/test surfaces.
- Evidence: root policy says PR #591 completed cutover, while linked notice, CLI help, module docs, package metadata, and tests still require pre-cutover v2 wording.
- Problem: operators have two contradictory routing truths and can be sent to retired authority surfaces.
- Acceptance criteria: establish one post-cutover statement everywhere current; preserve old records only as historical evidence; remove misleading current links; add a cross-surface fitness check.
- Validation: focused command/help tests, complete obsolete-guidance scan, and selector/receipt validation.
- Non-goals: rewriting immutable history; restoring v2 default authority.
- Dependencies: #522; coordinate with R520-006 and R520-007.
- Approval status: operator approved for separate typed issue creation.

### R520-005 — [P1] Make repository-review assignments derive from complete per-lane denominators

- Source findings: `A520-ARCH-004`, `C520-CODE-004`
- Source roles: architecture, code
- Confidence: high
- Affected path: `adl/tools/skills/repo-packet-builder/scripts/build_repo_packet.py`
- Evidence: global truncation happens before lane routing; #520 assigned the same lock files to unrelated roles and zero files to tests.
- Problem: a mandatory review can look complete without reviewing the product change.
- Acceptance criteria: classify the complete inventory first; retain source/selected denominators, exclusions, and digests per lane; fail on materially empty/category-invalid lanes; distinguish full scans from samples; add the #520 shape as a regression fixture.
- Validation: exact #520 fixture plus negative empty/category-invalid/digest-mismatch fixtures and packet-completeness validation.
- Non-goals: treating classification as semantic proof; forcing every manual reviewer to read every repository file.
- Dependencies: #522; use the repair for subsequent internal reviews.
- Approval status: operator approved for separate typed issue creation.

### R520-006 — [P2] Bind C-SDLC proof and install routes to authenticated issue-worktree identity

- Source findings: `A520-ARCH-005`, `C520-CODE-005`
- Source roles: architecture, code
- Confidence: high
- Affected paths: `csdlc-v3/src/main.rs`, `csdlc-v3/src/commands/proof.rs`
- Evidence: proof routing chooses the repository from the stable binary path even though issue authority belongs to the bound worktree.
- Problem: valid issue work can be rejected or mutations can target the inspection-only primary checkout and wrong head.
- Acceptance criteria: select the repository from authenticated bound context; validate branch, registration, exact head, shared Git directory, and containment; use executable path only for provenance; explicitly classify every proof route as operational or historical.
- Validation: one stable binary exercised from primary and linked worktrees, proving correct write placement and fail-closed identity mismatch.
- Non-goals: copying binaries into every worktree; weakening head or containment guards.
- Dependencies: #522; coordinate with R520-004 and R520-007.
- Approval status: operator approved for separate typed issue creation.

### R520-007 — [P1] Repair the required detached-head C-SDLC v3 cutover and rollback suite

- Source finding: `T520-TEST-001`
- Source role: tests
- Confidence: high
- Affected paths: `.github/workflows/ci.yaml`, `csdlc-v3/tests/terminal_cleanup_cutover_commands.rs`
- Evidence: the full locked suite exits 101 at the candidate; ten tests fail because the fixture combines a real checkout with historical #505 branch state.
- Problem: the required C-SDLC v3 lane has no reliable detached-head baseline.
- Acceptance criteria: make fixtures self-contained; retain production fail-close behavior; pass all ten regressions from a detached head; retain a full locked result.
- Validation: complete locked C-SDLC v3 suite locally from detached head and required hosted standalone lane.
- Non-goals: disabling tests; weakening production authority guards.
- Dependencies: #522; coordinate with R520-004 and R520-006.
- Approval status: operator approved for separate typed issue creation.

### R520-008 — [P1] Close 198 current retained-predecessor candidate-bound proof gaps

- Source finding: `T520-TEST-002`
- Source role: tests
- Confidence: high
- Affected artifacts: TAIL-01 `quality-gate.json` and `blockers.json`.
- Evidence: exact-current reconciliation retains 143 `non_proving` rows, 51 source-supported rows without execution proof, and four current obligations: 198 unproved rows. The other 29 original rows are 10 proved, eight amended, and 11 later-stage obligations and must retain those dispositions. Ownership/accounting alone is not behavior proof.
- Problem: required predecessor behavior remains unproved for the release candidate.
- Acceptance criteria: preserve the 198-row denominator and all 29 non-gap dispositions; reconcile every retained criterion to successor authority; bind every unproved row to candidate proof or governed amendment/removal; leave no current required retained row non-proving; regenerate the exact release decision.
- Validation: retained inventory and semantic-mapping validators, replayed or exact retained proving lanes, then quality gate with zero required non-proving rows.
- Non-goals: using ownership, closure, or document presence as proof; silently removing requirements.
- Dependencies: #522; use `proof-gap-reconciliation.md` as exact-current authority; likely requires criterion-level child decomposition; must precede release authorization.
- Approval status: operator approved for separate typed issue creation.

### R520-009 — [P2] Bound XCL Terraform provider constraints to supported major versions

- Source finding: `T520-DEP-001`
- Source role: dependencies
- Confidence: high
- Affected paths: AWS and GCP XCL `versions.tf` and provider lockfiles.
- Evidence: both roots use only `>= 5.0`; current locks select AWS 6.62.0 and Google 8.0.0, while a future breaking major remains eligible.
- Problem: routine lock regeneration can silently select an unreviewed provider major.
- Acceptance criteria: declare compatible supported-major ranges; keep locks consistent; document the support policy.
- Validation: read-only init, validate, bounded plan, and static rejection of open-ended major floors.
- Non-goals: opportunistic major upgrades; unrelated Terraform roots.
- Dependencies: #522.
- Approval status: operator approved for separate typed issue creation.

### R520-010 — [P2] Pin the required Rust CI toolchain

- Source finding: `T520-DEP-002`
- Source role: dependencies
- Confidence: high
- Affected paths: `.github/workflows/ci.yaml`, repository toolchain/MSRV surfaces.
- Evidence: the action commit is pinned but requests floating `stable`; no tracked toolchain version or manifest compatibility floor exists.
- Problem: identical commits can compile, lint, or format differently after a stable release.
- Acceptance criteria: pin an explicit compiler; declare intended MSRV where appropriate; make upgrades reviewed changes; align local and hosted required lanes.
- Validation: full required tests, Clippy, and formatting with recorded pinned version; static proof that CI does not float.
- Non-goals: Rust edition/dependency upgrades; exhaustive optional features.
- Dependencies: #522; coordinate with R520-007.
- Approval status: operator approved for separate typed issue creation.

### R520-011 — [P2] Project current delivery and proof status into canonical v0.92.1 release documents

- Source finding: `D520-DOC-001`
- Source role: documentation
- Confidence: high
- Affected paths: feature-proof coverage, demo matrix, release notes, and milestone checklist.
- Evidence: canonical documents remain planning surfaces and the checklist is wholly unchecked rather than showing implemented, proved, deferred, or blocked state.
- Problem: a reviewer cannot derive release status from canonical documents alone.
- Acceptance criteria: project one current per-lane disposition; distinguish delivery from proof and debt; bind every claim; keep all four surfaces consistent.
- Validation: cross-document validators and independent reconstruction of the release decision from canonical documents.
- Non-goals: converting plans into proof; deleting historical context.
- Dependencies: #522; refresh after substantive fixes land.
- Approval status: operator approved for separate typed issue creation.

### R520-012 — [P2] Provide a candidate-safe reproduction route for the TAIL-02 handoff

- Source finding: `D520-DOC-002`
- Source role: documentation
- Confidence: high
- Affected paths: TAIL-02 README and issue #518 handoff validator.
- Evidence: the first documented command exits 1 at the candidate because it compares frozen historical inventory to a grown repository.
- Problem: the advertised handoff procedure is not reproducible from the publication candidate.
- Acceptance criteria: separate source-head historical verification from current-candidate verification; make each documented command prove its claimed boundary; preserve historical hashes; document post-growth behavior.
- Validation: run both routes from clean detached checkouts and test later-file growth.
- Non-goals: rewriting historical evidence; claiming TAIL-02 supplies product proof.
- Dependencies: #522.
- Approval status: operator approved for separate typed issue creation.

### R520-013 — [P2] Redaction-scan every document referenced by the publication manifest

- Source finding: `S520-SEC-001`
- Source role: security
- Confidence: high
- Affected paths: issue #519 publication validator, TAIL-03 README, and #519 lifecycle claims.
- Evidence: validator scans only TAIL-03 files, not 791 referenced documents. A complete scan found 35 machine-local path occurrences, including operator paths and an exact local credential location, while `--all` passes.
- Problem: publication can pass while exposing operator identity, layout, credential locations, or unrecognized private values.
- Acceptance criteria: scan 791/791 referenced bytes; remove/generalize all confirmed local paths; fail on unreadable or unsafe content; report findings without sensitive values; align lifecycle claims to the real denominator.
- Validation: complete manifest scan and negative fixtures for paths, credential locations, private-key blocks, bearer values, and provider token shapes.
- Non-goals: printing detected values; claiming pattern scans detect every possible secret.
- Dependencies: #522; required before external publication.
- Approval status: operator approved for separate typed issue creation.

### R520-014 — [P2] Require SSH recovery access for the public Spot Runtime root

- Source finding: `S520-SEC-002`
- Source role: security
- Confidence: high
- Affected paths: public Spot Runtime module/root variables, main configuration, example, and README.
- Evidence: the public root defaults to no key and no SSH ingress while assigning a public IP.
- Problem: a documented apply can create a node without the required independent recovery path.
- Acceptance criteria: reject a public node without key or authorized SSH ingress; use one approved key and explicit CIDRs; fix example/docs; preserve current metadata, disk, and application-port controls.
- Validation: Terraform negative tests plus validate/plan and bounded `/32` SSH/application-isolation proof.
- Non-goals: a second key pair; forcing public SSH on private-only roots; treating optional SSM as a replacement.
- Dependencies: #522; coordinate paid proof with budget and disposal controls.
- Approval status: operator approved for separate typed issue creation.

### R520-015 — [P1] Refresh exact-head review proof for the four V3-F rows

- Source finding: `T520-TEST-003`
- Source role: tests
- Confidence: high
- Affected paths: `csdlc-v3/tests/terminal_cleanup_cutover_commands.rs` and exact-current reconciliation evidence.
- Evidence: three historical CORP-A rows are resolved, but V3-F-ac-1 through V3-F-ac-4 rely on a review of blob `c5d67168`; the exact candidate has blob `556cb495` after fixture registry drift from 1.0.3 to 1.0.4. That file is also inside the failing suite reported by `T520-TEST-001`.
- Problem: four V3-F rows are not bound to current exact-head review and passing proof.
- Acceptance criteria: independently review the current blob and substantive V3-F scope; bind all four rows to the immutable review receipt and a passing exact-candidate suite; preserve the three resolved CORP-A dispositions.
- Validation: retain the scoped blob diff, complete assigned exact-head review after R520-007, and validate the four mappings against the same SHA.
- Non-goals: reopening resolved CORP-A rows; substituting test success for independent review.
- Dependencies: #522; should follow or combine with R520-007.
- Approval status: operator approved for separate typed issue creation.

### R520-016 — [P2] Prove the GCP-B audit and log posture at the exact candidate

- Source finding: `T520-TEST-004`
- Source role: tests
- Confidence: high
- Affected artifacts: GCP-B proof packet and exact-current semantic reconciliation.
- Evidence: six of the historical 11 rows are proved and four have accepted amendments. Only `GCP-B-ac-1` remains non-proving because #740 contains no audit configuration or log readback; reauthentication blocked the current read-only check.
- Problem: the exact candidate does not retain evidence for the required GCP-B audit/log posture.
- Acceptance criteria: define the exact audit/log assertions; obtain authorized read-only or bounded live evidence; bind sanitized configuration and readback to the candidate and provider identity; keep the other 10 rows closed.
- Validation: use approved reauthentication, capture audit configuration and representative log readback, then run the GCP-B proof and semantic-mapping validators.
- Non-goals: rerunning unrelated paid proofs; reopening proved/amended criteria; recording credentials, local key paths, or sensitive logs.
- Dependencies: #522; requires authorized GCP read access and R520-013 redaction compliance.
- Approval status: operator approved for separate typed issue creation.

## Deferred Findings

`D520-DOC-003` remains confirmation-required. PR #756 and issue #519 are still
open at planning time, so post-merge claims cannot yet be judged. However, the
current records are already internally inconsistent: the publication packet is
`final`, SRP records a passing review while its status is `draft`, and SOR
remains `pre_phase`. Recheck immediately after native finish and terminal
reconciliation; create remediation if the terminal projection does not resolve
the contradiction.

## Resolved Findings

- `T520-TEST-005` — **resolved/non-actionable.** Exact-current reconciliation marks all WP-01, GCP-E, HOT-01, and OBS-B rows synchronized; canonical fields validate and no live/spec row remains.
- `T520-TEST-006` — **resolved/non-actionable.** Independent bounded reviews accepted the exact candidate objects for all three shared surfaces and the objects match `current-exceptions.json`.

## Approval Boundary

The operator authorized subissues under #522 and remediation of all confirmed
findings. This planning task is still mutation-bounded: it created no GitHub
issues, branches, PRs, tests, or product fixes. The 16 ready candidates may be
handed to the typed C-SDLC v3 issue-creation route. `D520-DOC-003` must be
rechecked after merge; the two resolved findings remain audit history only.

## Validation Notes

- Every one of the 23 specialist source findings appears exactly once in a ready candidate, the confirmation-required register, or the resolved register.
- Duplicate code/architecture findings were grouped only where they share the same root cause and repair boundary.
- Grouped severity uses the highest source severity.
- The packet-builder scaffold defect remains a product-tooling finding because it affects future review truth; the provisional #520 scaffold itself is not treated as release-candidate coverage evidence.
- No actionable security finding was deferred.

## Residual Risk

- R520-008 is deliberately broad because the exact-current source finding covers 198 unproved retained criteria. It should be decomposed by authoritative criterion groups during typed issue preparation without dropping the umbrella acceptance condition or misclassifying the other 29 rows.
- Live cloud, GPU, provider, and browser behavior was not rerun by the planning step.
- The single merge-pending finding can become a real issue only after terminal evidence is obtained; its present internal contradiction must remain visible meanwhile.
