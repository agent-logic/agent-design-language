## Metadata

- Skill: `repo-review-docs`
- Target: `f0a011a5c59d46c763d669f69a10308b3f870ba4..c24f8fa65ce445b03ce6cd69007307291d78b60c`
- Reviewed checkout: detached read-only checkout at the exact candidate SHA
- Date: 2026-09-09T03:33:34Z
- Artifact: `docs/milestones/v0.92.1/evidence/release/tail-04/specialists/docs-review.md`
- Depth: deep

## Findings

- **P2: Canonical release surfaces still do not say what was delivered**
  File: `docs/milestones/v0.92.1/FEATURE_PROOF_COVERAGE_v0.92.1.md:3`
  Role: docs
  Scenario: A release reviewer starts from the canonical feature coverage, demo matrix, release notes, or milestone checklist and tries to determine which lanes are implemented, proved, deferred, or still blocked.
  Impact: The reviewer can see requirements and historical accounting but cannot derive a current per-lane release disposition. Completed work can be mistaken for planned work, and unproved work can be mistaken for delivered work. The release candidate therefore remains unusable as a context-free release-status surface.
  Evidence: `FEATURE_PROOF_COVERAGE_v0.92.1.md:3-20` supplies only a historical-accounting preface and required-proof table; `DEMO_MATRIX_v0.92.1.md:3-22` supplies planned demonstrations rather than observed results; `RELEASE_NOTES_v0.92.1.md:3-19` still calls the content “Planned outcomes” and explicitly says the lanes are planning commitments; `MILESTONE_CHECKLIST_v0.92.1.md:5-53` leaves every opening, execution, integration, and release-tail item unchecked, including already merged prerequisites. The retained TAIL-02 register acknowledges the same unresolved debt as D07 with status `mapped_with_explicit_release_proof_debt` in `docs/milestones/v0.92.1/evidence/release/tail-02/finding-dispositions.json`.
  Residual risk: The complete live issue/PR reconciliation lane may provide the missing facts elsewhere, but those facts are not projected into the canonical release documents.

- **P2: The documented TAIL-02 reproduction command fails on the exact publication candidate**
  File: `docs/milestones/v0.92.1/evidence/release/tail-02/README.md:81`
  Role: docs
  Scenario: A reviewer follows the first listed reproduction command at candidate `c24f8fa65ce445b03ce6cd69007307291d78b60c`.
  Impact: The advertised review procedure cannot be reproduced from the release candidate and terminates before checking the documented inventory, links, or claims. This makes the handoff operationally stale even though its historical evidence remains valid.
  Evidence: `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --all` exits 1 with `current inventory denominator changed; refresh required`. The command is prescribed at README lines 83-85. The validator recomputes the current repository denominator at `.csdlc/prepared/issues/518/validate-documentation-handoff.rb:88-100`, so later tracked README/AGENTS/milestone additions necessarily invalidate the frozen TAIL-02 inventory. The README describes earlier records as historical at lines 92-94 but does not distinguish the listed command as expected to fail after candidate growth or provide a candidate-safe verification command for the TAIL-02 claims.
  Residual risk: The TAIL-03 validator passes, but it verifies hashes against the old source head and does not make the TAIL-02 operator instructions runnable on the candidate.

- **P2: The merged #519 lifecycle record still describes pre-merge work**
  File: `.csdlc/issues/519/cards/sor.md:11`
  Role: docs
  Scenario: A lifecycle reviewer reads #519 from the exact merged #756 candidate to establish publication and integration truth.
  Impact: Durable lifecycle evidence contradicts the observed merged candidate. Automation or a human using the SOR/SRP as release truth can conclude that review, merge, or closeout is still pending, while the release packet simultaneously treats #519 as final.
  Evidence: The SOR is `Status: pre_phase` at line 11, says “Final independent review follows” at line 15, retains preparation-era steps at lines 42-58, reports `pr_open` at line 117, `Merge: not_merged` at line 123, and `Closeout: not_started` at line 127. The SRP remains `Status: draft` at `.csdlc/issues/519/cards/srp.md:11` while lines 45-51 record a passing review. The same candidate’s `docs/milestones/v0.92.1/evidence/release/tail-03/candidate.json:5` declares `status: final`. The artifact list is also repeated five times at SOR lines 19-38, obscuring rather than clarifying the exact output set.
  Residual risk: Native terminal reconciliation may be intentionally asynchronous, but the exact internal-review candidate presently contains no terminal projection that resolves the contradiction.

## Documentation Objects

- Complete changed-path denominator: all 5,481 paths from the exact Git diff were enumerated; 947 are under `docs/` or `.adl/docs/`, 783 are v0.92.1 milestone surfaces, 41 are release-tail surfaces, 391 are cloud operations/evidence surfaces, 70 are README/review/runbook/handoff/release/security/credential/redaction documents, 3,100 are lifecycle/preparation records, and 804 are `.csdlc/evidence` records.
- The complete 791-document TAIL-02 handoff was taken from the sorted, content-addressed `handoff-content.json`, not from a sample. Its retained 15-finding disposition register and independent review were checked before relying on prior semantic coverage.
- Every canonical v0.92.1 planning/release document named by `CANONICAL_DOC_INVENTORY_v0.92.1.md` was included through that content-addressed handoff. Current release-status and runnable-claim surfaces were re-inspected directly at the exact candidate.
- All 121 changed issue-record directories were deterministically enumerated for lifecycle-state screening; #519 was inspected in full because it is the publication predecessor and exact candidate owner.
- Cloud/provider runbooks and evidence were included in complete path and sensitive-claim scans; security-specific results are in `security-review.md`.

## Commands Or Claims Checked

- Exact changed-path counts and category membership from `git diff --name-only <base>..<candidate>`.
- TAIL-02 claims for a 791-document denominator, retained finding dispositions, and runnable reproduction commands.
- TAIL-03 claims for exact source/review/merge identity, 14 artifacts, 791 documents, redaction, and explicit non-release authority.
- Canonical release-note, feature-proof, demo, checklist, quality-gate, and release-tail status claims.
- #519 SIP/STP/SPP/VPP/SRP/SOR claims and their agreement with the final candidate packet.

## Validation Performed

- `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --all` — **failed** at the exact candidate with `current inventory denominator changed; refresh required`; this proves the documented reproduction route is stale.
- `ruby .csdlc/prepared/issues/519/validate-publication-candidate.rb --all` — passed with 791 documents and 14 artifacts; this proves hash/linkage checks run, but not the missing current-status mapping or referenced-content redaction.
- Deterministic path census — 5,481 total changed paths and the complete category counts above.
- Deterministic lifecycle census — 121 changed issue-record directories inspected for state/card consistency; no claim is based on a representative subset.

## Packet-Build Correction

The generic preliminary `repo-packet` scaffold was not used as the review denominator. Its `repo_scope.md:61-65` reports a 29,155-file full-repository inventory with docs/tests sampled at 40, while `specialist_assignments.json:131-177` assigns only three docs, only lock files to security, and no tests. The #520 conductor is replacing that scaffold with the issue-specific complete denominator. This is a packet-build correction, not a candidate product finding.

## Residual Risk

- This lane did not execute Runtime, cloud, provider, or browser behavior; those claims require the code/test/cloud proof lanes.
- Historical records were interpreted at their recorded revision and were not rewritten as current truth.
- Bare external URLs and remote service availability were not network-tested; relative-link and inventory behavior is covered by the retained TAIL-02 checks, subject to the reproduction failure above.
