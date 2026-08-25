# v0.92 WP-25 Internal Review Synthesis

## Metadata

- Skill: `repo-review-synthesis`
- Reviewer: Codex synthesis specialist (`/root/review_313_code`)
- Exact product target: `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Packet: `docs/reviews/v0.92/internal-review-5846`
- Date: 2026-08-25 UTC
- Severity policy: preserve highest specialist severity and all source-role provenance
- Specialist artifacts: 9 present, 0 missing
- Raw specialist findings: 20
- Deduplicated register: 11 findings — 9 open, 2 resolved for this packet
- Verdict: `CHANGES_REQUESTED`
- Release/publication posture: blocked; internal-only packet

## Findings

### SYN-001 — P1 — Canonical review and release authority projects incompatible gate states

- Status: open
- Source roles: architecture (`ARCH-001`), docs (findings 1 and 3), release/publication (`REL-001`)
- Files: `.csdlc/evidence/312/validation.json`; `docs/milestones/v0.92/QUALITY_GATE_v0.92.md`; `docs/reviews/v0.92/docs-release-truth-312/release-truth-diff.md`; `docs/milestones/v0.92/review/THIRD_PARTY_REVIEW_HANDOFF_v0.92.md`
- Scenario: WP-25, an external reviewer, or a release owner asks for the canonical WP-22/#467 gate result at the frozen target.
- Impact: The same revision supports both blocked/locked and zero-blocker/unlocked interpretations, so review ordering, release credit, and downstream authority are nondeterministic.
- Evidence: Retained #312 validation records `status: blocked`, 33 blocked rows, and `downstream_unlock: false`; the quality-gate corrective appendix, canonical inventory, and review packet state #467 supersedes that history with 30 accepted, 3 scoped out, 0 blockers, and unlock true. Other current-tense docs still instruct WP-23 to preserve the blocked result.
- Required disposition: choose one immutable canonical #467 result identity and propagate it across retained validation, quality gate, handoff, release-truth diff, review packet, and downstream validators while retaining #311 only as explicitly historical provenance.

### SYN-002 — P1 — Engineering-complete claims exceed reconciled checklist and security evidence

- Status: open
- Source roles: docs (finding 2), security (finding 1), release/publication (`REL-002`)
- Files: `docs/milestones/v0.92/MILESTONE_CHECKLIST_v0.92.md`; `docs/milestones/v0.92/README.md`; `docs/milestones/v0.92/RELEASE_NOTES_v0.92.md`; `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`
- Scenario: A consumer accepts “engineering milestone complete” and `implemented_with_evidence` claims as proof that engineering acceptance and security claim boundaries are closed.
- Impact: Unreconciled dependency, lifecycle, validation, demo, feature-proof, and ACIP/security denial requirements can be promoted into release narrative without canonical evidence.
- Evidence: The checklist asserts engineering completion while leaving engineering dependency/scope, lifecycle truth, formatting/lint/tests, demo runnability, exact-revision feature proof, anti-substitution rules, and ACIP positive/denial gates unchecked. Feature coverage labels ACIP/A2A transport implemented with evidence.
- Required disposition: reconcile every engineering checkbox to exact retained evidence or narrow the completion claim. Later internal/external review, ceremony, publication, and other intentionally downstream release-tail items may remain open and must not be misclassified as engineering defects merely because they are unfinished.

### SYN-003 — P1 — The demo/proof register cannot represent or execute the corrective state

- Status: open
- Source roles: demos/integration (findings 1 and 2)
- Files: `adl/tools/validate_v092_demo_proof_coverage.py`; `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`; `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`; `docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md`; `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- Scenario: Run D9 or use the four declared proof-register surfaces after #467 hydration.
- Impact: There is no passing local register integration proof. D9 and its negative harness fail at the positive control; the nine other declared rows have no concrete commands, revisions, reviews, or artifacts and therefore cannot be executed from the register.
- Evidence: D9 rejects `implemented_with_evidence` as unsupported. Matrix/index rows remain blocked or pending while feature coverage uses new corrective statuses; the validator's required cross-table equality cannot pass even after adding the missing vocabulary.
- Required disposition: define one shared status contract, hydrate or truthfully downgrade every row with immutable positive/negative/review/command evidence, rerun D9 and its negative harness, and retain exact-target outputs.

### SYN-004 — P1 — Remote validation executes mutable, unverified bootstrap artifacts

- Status: open
- Source role: dependency (finding 1)
- File: `tools/aws_remote_validation/scripts/remote_validation_runner.sh`
- Scenario: A remote host bootstraps missing sccache, cargo-nextest, Rust, or cached tooling.
- Impact: Mutable GitHub `latest`, arbitrary URLs, mutable S3 keys, and an unverified rustup pipeline can execute bytes not bound to the reviewed revision, invalidating remote-proof provenance and creating a supply-chain code-execution boundary.
- Evidence: Direct downloads and S3 cache installation lack required digest/signature/VersionId verification; version-output checks prove executability only.
- Required disposition: require immutable version/source/architecture plus SHA-256 or signature and S3 VersionId where applicable; verify before extraction/execution and retain provenance in the result contract.

### SYN-005 — P1 — The reusable builder image is not reproducible or integrity-pinned

- Status: open
- Source role: dependency (finding 2)
- File: `adl/docker/adl-builder/Dockerfile`
- Scenario: Rebuild the builder image later under the same source revision or nominal image contract.
- Impact: Mutable base, apt resolution, Rust `stable`, rustup, AWS CLI, sccache, and cargo-llvm-cov inputs can change or be compromised without a source change, making validation non-reproducible.
- Evidence: Several inputs are mutable or direct-downloaded without checksum/signature, while Ruby and cargo-nextest show that digest pinning is already feasible.
- Required disposition: pin the base digest and toolchain, verify all direct downloads, and retain a resolved machine-readable image/tool manifest.

### SYN-006 — P1 — Three independent Cargo graphs lack change-triggered CI ownership

- Status: open
- Source roles: tests (finding 1), dependency (finding 3, raised as P2 and promoted to the test lane's P1)
- Files: `.github/workflows/ci.yaml`; `adl/tools/ci_path_policy.sh`; `adl/config/validation_lane_selector.v0.91.6.json`
- Scenario: A PR changes only `adl-characterization/**`, `adl-resilience/**`, or `tools/remote_validation/**`.
- Impact: Required CI may stay green without compiling, resolving, formatting, linting, or testing the changed independent graph.
- Evidence: Existing Rust jobs own other manifests; selectors contain no route for these roots. Focused exact-target local tests passed (35, 4, and 12 respectively), proving current behavior but not future change-triggered ownership.
- Required disposition: add explicit selector ownership, package-local locked metadata/build/test/format/Clippy lanes, and routing regression fixtures.

### SYN-007 — P2 — Dependency review scaffolding still consumes stale lifecycle-lock evidence

- Status: open tooling/review-quality finding
- Source role: dependency (finding 4)
- Files: `docs/reviews/v0.92/internal-review-5846/evidence_index.json`; dependency-review scaffold tooling
- Scenario: A dependency reviewer uses the deterministic scaffold after the specialist assignments were repaired.
- Impact: The scaffold continues to route empty lifecycle lock sentinels instead of Cargo/container/bootstrap surfaces, so a later dependency review can silently regress to a non-representative denominator.
- Evidence: The scaffold consumes stale/misclassified evidence-index entries; the repaired dependency assignment correctly lists 61 material paths but is not its authority.
- Required disposition: classify dependency manifests by ecosystem context, exclude `.csdlc/locks/*.lock`, consume the explicit repaired assignment, and fail on index/assignment disagreement.

### SYN-008 — P2 — The documented diff-hygiene command is non-proving after merge

- Status: open
- Source role: docs (finding 4)
- File: `docs/milestones/v0.92/review/THIRD_PARTY_REVIEW_HANDOFF_v0.92.md`
- Scenario: A reviewer checks out the merged target and runs `git diff --check origin/main...HEAD`.
- Impact: With `origin/main == HEAD == c6792e54...`, the command checks an empty range while appearing to validate the #312 candidate.
- Evidence: Live `origin/main` resolved to the exact target during review.
- Required disposition: name an immutable candidate base or another immutable comparison range.

### SYN-009 — AMBIGUOUS — Derived-terminal receipts and typed doctor disagree on prerequisite terminal state

- Status: open ambiguity; not independently severity-promoted
- Source role: lifecycle/evidence (sole finding)
- Files: `.git/csdlc-v2/derived-terminal/312.json`; `.git/csdlc-v2/derived-terminal/10.json`; `.csdlc/issues/313/cards/stp.md`
- Scenario: WP-25 claims #312 and #10 are terminal and reconciled.
- Impact: Live GitHub, ancestry, cleanup, and immutable derived-terminal receipts support the prerequisite, while typed doctor reports both in `published` phase with blocking findings.
- Evidence: Receipts and GitHub agree on merged/closed ancestry; doctor reports `review_publication_dead_end` and, for #10, `issue_specific_denominator_missing`.
- Required disposition: obtain a typed terminal read that recognizes derived-terminal authority or repair stale typed projections. If doctor is intentionally inapplicable after derived terminalization, encode that rule in packet validation.

### SYN-010 — P1/P2 — Initial specialist assignment denominator was invalid, but this packet has been repaired

- Status: resolved for this packet; generic builder defect remains open outside the packet
- Source roles: code (findings 1 and 2), security (finding 2), tests (finding 2)
- Files: original packet-builder output; repaired `docs/reviews/v0.92/internal-review-5846/specialist_assignments.json`; `docs/reviews/v0.92/internal-review-5846/packet_assignment_recheck.md`
- Original impact: Code/security assignments contained lifecycle lockfiles, tests was empty, and worktree metadata was false.
- Resolution evidence: The replacement assignment is exact-target-derived, byte-reproducible, records portable checkout/output roles, contains all nine nonempty lanes, and assigns 1,442 code, 1,756 security, and 834 test paths. The recheck passed with zero findings.
- Residual: `adl/tools/skills/repo-packet-builder/scripts/build_repo_packet.py` still globally truncates before per-lane routing and detects worktrees by a literal `.worktrees` path component. Do not treat the generic tool as repaired by this issue-local replacement.

### SYN-011 — P2 — Initial dependency lane routing was repaired, but stale evidence-index consumers remain

- Status: partially resolved for this packet
- Source role: dependency (finding 4)
- Resolution evidence: The repaired assignment contains 61 actual dependency surfaces and no longer consists of lifecycle sentinels.
- Residual: This is not a duplicate of SYN-007's remaining behavior: the issue-local assignment is fixed, but the dependency scaffold still consumes the stale evidence index. SYN-007 remains open until consumers use the repaired authority.

## Coverage Matrix

| Lane | Artifact | Status | Findings | Important limitation |
|---|---|---|---:|---|
| Code | `specialists/code.md` | present | 2 | Targeted runtime/code inspection; original assignment invalid, later repaired |
| Security/privacy | `specialists/security.md` | present | 2 | Representative high-risk surfaces; full repository redaction/security audit not run |
| Tests/PVF/CI | `specialists/tests.md` | present | 2 | Three focused package suites; no broad workspace/provider/soak execution |
| Documentation/claims | `specialists/docs.md` | present | 4 | Canonical release surfaces prioritized; not all 6,349 assigned docs read line by line |
| Architecture | `specialists/architecture.md` | present | 1 | Static bounded map; no full dependency graph generation |
| Dependencies | `specialists/dependencies.md` | present | 4 | No live advisory/license service or image rebuild |
| Lifecycle/evidence | `specialists/lifecycle.md` | present | 1 ambiguous | Bounded to #313 dependencies and typed state |
| Demos/integration | `specialists/demos.md` | present | 2 | D1-D8 register rows gated; no cloud/provider actions |
| Release/publication | `specialists/release_publication.md` | present | 2 | Evidence assembly only; no tag/release/publication action |

All required lane assignments are nonempty after the deterministic repair: architecture 736, code 1,442, dependencies 61, docs 6,349, security 1,756, tests 834, lifecycle 6,050, demos 1,216, and release/publication 4,909 paths. Counts are routing denominators, not claims that every path received equal-depth inspection.

## Dedupe Notes

- Architecture `ARCH-001`, docs findings 1/3, and release `REL-001` describe the same incompatible gate-authority projection and are merged into SYN-001 without dropping any role.
- Docs completion finding, security's unchecked ACIP gate, and release `REL-002` share the same completion-versus-checklist risk and are merged into SYN-002. The security-specific positive/denial requirement remains explicit.
- Demo vocabulary failure and unhydrated register are combined in SYN-003 because both prevent the single register integration contract from running; both causal repairs remain stated.
- Tests P1 and dependency P2 describe the same missing CI ownership for three Cargo graphs. SYN-006 preserves the higher P1 severity and both behavioral and dependency consequences.
- Code assignment starvation, security assignment omission, empty tests assignment, and false worktree labeling were all initial packet-generation failures. They are consolidated as SYN-010 and marked resolved only for this packet by the deterministic replacement/recheck.
- Dependency assignment repair and the still-broken scaffold consumer are deliberately split into SYN-011 (packet repair) and SYN-007 (open consumer defect).

## Disagreements And Ambiguities

- Architecture and release reports characterize the handoff itself as asserting a 33/33 blocked result. The handoff's current status text instead points to #467 and its known-risk section states zero blockers; however, retained validation and other current-tense documents still expose the blocked state. SYN-001 preserves the proven dual-authority problem without adopting the over-broad characterization of every handoff passage.
- Release-tail checklist items such as internal/external review, remediation, ceremony, and publication are expected to remain incomplete during WP-25. SYN-002 therefore relies only on unchecked engineering/security acceptance items when challenging “engineering complete”; it does not treat unfinished downstream ceremony as an engineering defect.
- The lifecycle lane intentionally classified its sole finding as ambiguous rather than P0-P3. Synthesis preserves that classification and does not infer severity.
- D1-D8 (nine rows including D7A) are `skipped`, not failed runtime behavior. D9 is the only declared locally executable demo and is failed.
- Passing focused source tests do not resolve missing CI routing, and passing packet-validator execution does not reconcile stale retained evidence.

## Validation Performed Across Roles

- Exact-target inspection used `git show`, `git grep`, and `git ls-tree` against `c6792e54df1db5969fa28c59b6dfe4c714ed5559`.
- Assignment generator syntax, exact-target regeneration, byte equality, lane nonemptiness, material path inclusion, source checkout identity, and worktree registration all passed.
- Documentation packet validator currently passed on surfaces proven unchanged from the target; retained validation evidence remains stale/blocked.
- D9 positive validator and negative harness failed on unsupported corrective status vocabulary.
- Focused tests passed: 13 runtime API authentication tests, 35 `adl-characterization` tests, 4 `adl-resilience` tests, and 12 `tools/remote_validation` tests.
- One observability/redaction filter selected zero tests and was correctly classified non-proving.
- Locked Cargo metadata succeeded for nine principal graphs.
- GitHub Action `uses:` references in inventoried workflows were full commit SHAs.
- Deterministic packet redaction audit passed for the seven then-current internal packet files only; it does not authorize publication of this synthesized packet.
- Release-evidence assembly classified the release packet blocked.
- No cloud, provider, paid, deployment, release, publication, merge, or lifecycle mutation occurred.

## Residual Risk

- The repository-scale denominator is large; specialists used risk-selected depth rather than exhaustive line-by-line inspection.
- No broad workspace suite, soak, provider, AWS, container rebuild, vulnerability feed, transitive license audit, external-link crawl, or full architecture graph was run.
- The final synthesized artifacts require a new redaction/evidence audit before any audience beyond internal review.
- Review artifacts live in the issue-313 worktree above the frozen product target; they are not evidence that target `c6792e54...` already contains remediation.
- No absence-of-defects claim is supported while the nine open findings remain.

## Recommended Follow-up Order

1. Reconcile the canonical #467 gate state and engineering/security checklist truth (SYN-001, SYN-002).
2. Repair and hydrate the demo/proof integration contract, then rerun D9 positive and negative proof (SYN-003).
3. Eliminate mutable executable bootstrap and builder inputs (SYN-004, SYN-005).
4. Add CI ownership for independent Cargo graphs (SYN-006).
5. Repair stale packet/dependency consumers and the handoff diff command (SYN-007, SYN-008).
6. Reconcile typed terminal/doctor authority for prerequisites (SYN-009).
7. Preserve the issue-local assignment repair while routing generic tooling residuals separately (SYN-010, SYN-011).
