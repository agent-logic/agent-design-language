# v0.92 WP-25 Internal Repository Review

- Exact product target: `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Specialist coverage: architecture, code, dependencies, docs, security, tests, lifecycle, demos, release/publication
- Raw findings: 20
- Deduplicated findings: 11
- Internal review result: `changes_requested`
- Product release state: blocked pending remediation

## Executive Summary

The review found no P0 defects. It produced 20 source findings and reconciled
them into 11 register entries: six open P1 findings, two open P2 findings, one
open lifecycle ambiguity, and two packet-routing findings resolved or partly
resolved for this review. The dominant risks are inconsistent release
authority, claims that exceed retained evidence, mutable executable bootstrap
inputs, a non-reproducible builder image, and missing CI ownership for three
independent Rust packages.

The packet is suitable for internal findings-first use. It does not authorize a
v0.92 release or external publication. A live Gemini API meta-review found no
actionable packet defect and independently confirmed the reconciliation of the
20 source findings into the 11-item register.

## Review Scope

The frozen product target was reviewed through nine bounded specialist lanes:
architecture, code/correctness, dependencies and supply chain,
documentation/claims, security/privacy, tests/PVF/CI, lifecycle/evidence,
demos/integration, and release/publication. The deterministic assignment covers
all nine lanes and is byte-reproducible from the exact target.

This was risk-selected repository review, not an exhaustive line-by-line audit
of every inventoried path. No broad workspace soak, cloud execution, container
rebuild, live vulnerability feed, transitive license audit, or external release
operation was performed.

## Top Findings

### Finding SYN-001: [P1] Canonical gate authority has incompatible states

- Evidence: `.csdlc/evidence/312/validation.json` retains a blocked 33-row result while current quality-gate surfaces describe the superseding #467 result as 30 accepted, three downstream-scoped, and zero blockers.
- Impact: review ordering, release credit, and downstream authority can derive different answers from the same target.
- Recommended action: establish one immutable #467 authority identity and propagate its blocker and unlock state across every current entrypoint; label #311 evidence historical.
- Validation gap: no exact-target cross-document assertion currently enforces one state.

### Finding SYN-002: [P1] Engineering-complete claims exceed retained checklist and security evidence

- Evidence: milestone completion language coexists with unchecked engineering, validation, demo, exact-revision feature-proof, and ACIP/security denial rows.
- Impact: incomplete engineering and security boundaries can enter release narrative as completed work.
- Recommended action: link each checked claim to exact positive and negative evidence or narrow the completion language; keep later release-tail tasks explicitly pending.
- Validation gap: no complete claim-to-evidence reconciliation passes at the target.

### Finding SYN-003: [P1] Demo and proof registers cannot represent the corrective state

- Evidence: D9 rejects the canonical corrective status vocabulary, and D1-D8 register rows lack concrete commands, revisions, reviews, or immutable artifacts.
- Impact: the declared cross-table demo proof cannot run successfully or reproduce claimed coverage.
- Recommended action: unify status vocabulary, hydrate or downgrade every row truthfully, and rerun positive and negative D9 paths at one revision.
- Validation gap: D9 and its negative harness do not both pass at the same target.

### Finding SYN-004: [P1] Remote validation executes mutable bootstrap artifacts

- Evidence: remote bootstrap paths accept mutable release aliases, arbitrary URLs, mutable S3 objects, and rustup-delivered executable content without mandatory immutable digest or object-version checks.
- Impact: remote proof can execute bytes not bound to the reviewed source and lose supply-chain provenance.
- Recommended action: require version, architecture, immutable object identity, and digest or signature before extraction or execution; retain bounded provenance.
- Validation gap: negative tests do not reject every absent or mismatched immutable identity.

### Finding SYN-005: [P1] Builder image inputs are not reproducible or integrity-pinned

- Evidence: the builder uses mutable base/toolchain selectors and unchecked direct downloads for several tools.
- Impact: rebuilding the nominally same image can produce different or compromised validation environments without a source change.
- Recommended action: pin the image digest and toolchain, verify direct downloads, and emit a resolved machine-readable tool manifest.
- Validation gap: no reproducibility contract proves all resolved inputs are immutable and verified.

### Finding SYN-006: [P1] Independent Cargo graphs lack change-triggered CI ownership

- Evidence: `adl-characterization`, `adl-resilience`, and `tools/remote_validation` have real tests but are absent from required change-routing ownership.
- Impact: a package-only change can leave aggregate CI green without compiling, formatting, linting, or testing the changed graph.
- Recommended action: add package-root selectors and locked metadata, build, test, format, and Clippy lanes with routing fixtures.
- Validation gap: there are no package-only routing tests for the three roots.

### Finding SYN-007: [P2] Dependency scaffold consumes stale lifecycle-lock evidence

- Evidence: the dependency scaffold reads misclassified evidence-index entries even though the issue-local dependency assignment now contains 61 material paths.
- Impact: later dependency review can silently regress to a non-representative denominator.
- Recommended action: consume the material dependency assignment, exclude lifecycle sentinels, and fail on assignment/index disagreement.
- Validation gap: no deterministic scaffold regression proves material inclusion and sentinel exclusion.

### Finding SYN-008: [P2] Post-merge diff hygiene command is non-proving

- Evidence: the handoff uses `git diff --check origin/main...HEAD` when both names resolve to the same merged target.
- Impact: an empty comparison appears to validate the #312 candidate range.
- Recommended action: bind the command to an immutable candidate base and exact target.
- Validation gap: the documented command does not fail on an injected candidate-range whitespace defect.

### Finding SYN-009: Lifecycle terminal receipts and doctor projections disagree

- Evidence: live GitHub, ancestry, cleanup, and derived-terminal receipts show #312 and #10 terminal, while historical card projection diagnostics remain in a published/recovery posture.
- Impact: consumers can mistake intentionally immutable post-merge cards for live terminal authority.
- Recommended action: encode and validate the rule that immutable derived-terminal receipts plus live GitHub state supersede post-merge card projections.
- Validation gap: packet-local reconciliation exists in `LIVE_STATE.md`, but the generic diagnostic contract does not express the distinction.

### Finding SYN-010: Packet lane assignments were repaired

- Evidence: exact-target regeneration is byte-identical and all nine lanes are nonempty with material code, security, test, and dependency paths.
- Impact: the original packet denominator no longer weakens this review; the generic packet-builder defect remains a separate tooling risk.
- Recommended action: preserve the issue-local repaired assignment and remediate the generic builder under the follow-on owner.
- Validation gap: generic builder scoring remains uncorrected; the worktree-detection claim was removed from this finding because it was not exercised by this packet.

### Finding SYN-011: Dependency assignment is repaired but one consumer remains stale

- Evidence: the repaired dependency assignment has 61 material paths, while the dependency scaffold still consumes the stale evidence index.
- Impact: the current review has a representative dependency lane, but regeneration through the old consumer can lose that coverage.
- Recommended action: close SYN-007 in the generic consumer before relying on future generated scaffolds.
- Validation gap: consumer authority has not yet moved to the repaired assignment.

## Architecture Summary

No new product-layering defect was established in the sampled Runtime and ADL
facade boundaries. The material architecture risk is control-plane truth: the
same frozen revision exposes incompatible current gate states. The broad `adl`
crate remains an explicit integration facade, while `adl-runtime` retains its
declared Runtime-owned boundary in the sampled dependency surface.

## Security And Privacy Notes

Security review covered authentication, TLS, redaction, AWS bootstrap,
ACC/private-state, and network-sensitive surfaces selected from the repaired
assignment. Focused Runtime API authentication tests passed 13/13. The review
does not claim that unchecked ACIP positive and denial boundaries are complete.
The packet retains no provider credential material; the Gemini lane records
only model/request metadata and response digests. A deterministic redaction
audit is required over the final packet before any wider audience.

## Test Recommendations

1. Add package-only CI routing fixtures for the three independent Cargo graphs.
2. Add immutable bootstrap identity and mismatch-denial tests for every remote executable source.
3. Add builder input pinning and digest-contract tests.
4. Make D9 positive and negative paths exercise the shared status vocabulary at one exact target.
5. Add cross-document gate-state and immutable diff-range assertions.
6. Add packet dependency-scaffold regressions for material inclusion and lifecycle-sentinel exclusion.

Focused exact-target package tests passed: 35 characterization, four
resilience, and 12 remote-validation tests. One redaction-named filter selected
zero tests and was retained as non-proving.

## Remediation Sequence

1. Reconcile canonical gate and engineering/security claim truth (SYN-001 and SYN-002).
2. Repair and hydrate demo/proof integration (SYN-003).
3. Pin remote bootstrap and builder supply-chain inputs (SYN-004 and SYN-005).
4. Add CI/PVF ownership for independent graphs (SYN-006).
5. Repair generic packet consumers and diff-hygiene guidance (SYN-007 and SYN-008).
6. Encode terminal-receipt precedence in lifecycle diagnostics (SYN-009).
7. Preserve the repaired packet assignments while closing their generic tooling residuals (SYN-010 and SYN-011).

## Residual Risks

- Nine product/tooling findings remain open and block v0.92 release authority.
- Review depth was risk-selected across a large repository inventory.
- Cloud/provider/runtime soak, container rebuild, vulnerability, licensing, and full architecture-graph surfaces were not executed.
- Passing focused tests do not substitute for missing CI routing or immutable supply-chain provenance.
- The packet is internal review evidence above the frozen product target; it is not evidence that remediation already exists in that target.
- Issue `#269` was excluded and was not inspected or executed.
