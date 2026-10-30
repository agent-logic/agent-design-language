# Demo And Integration Specialist Review

## Metadata

- Skill: `demo-operator`
- Reviewer identity: Codex demo/integration specialist (`/root/review_313_code`)
- Subject: v0.92 declared demo/proof register
- Target revision: `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Date: 2026-08-25 UTC
- Output Location: `docs/reviews/v0.92/internal-review-5846/specialists/demos.md`
- Finding count: 2 (`P1`: 2)

## Findings

- P1: The only runnable registered demo fails on the canonical corrective status vocabulary
  File: `adl/tools/validate_v092_demo_proof_coverage.py:12`
  Role: demos/integration
  Scenario: Run D9 exactly as declared in `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md` after #467 corrective hydration.
  Impact: D9 cannot validate the current canonical feature-proof document, so the milestone has no passing locally runnable demo/proof-register integration surface. Its negative harness also exits at the initial positive-control invocation and therefore proves none of its advertised rejection cases at this target.
  Evidence: The validator accepts only `accepted`, `blocked_with_evidence`, `deferred_non_claim`, and `planned` at lines 12-17. `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md:20-22` introduces `implemented_with_evidence`, `operator_scoped_out`, and `downstream_stage_gate`, and uses the first of those beginning at line 28. Both `python3 adl/tools/validate_v092_demo_proof_coverage.py --root .` and the documented `adl/tools/test_v092_demo_proof_coverage.sh` fail immediately with `coverage row 'Canonical milestone and version truth' has unsupported status 'implemented_with_evidence'`.

- P1: The demo register and artifact index were not hydrated with the corrective evidence state
  File: `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md:35`
  Role: demos/integration
  Scenario: A release or review consumer uses the declared matrix, feature coverage, bridge ledger, and artifact index as one integrated proof register.
  Impact: The surfaces cannot agree on demo disposition or provide executable proof routing. D1-D8 remain `blocked_with_evidence` with `pending-owner-evidence`, `pending-owner-command`, and no immutable revisions, while feature coverage claims the corresponding outcomes are `implemented_with_evidence` under #467. Even after teaching the validator the new vocabulary, its cross-table equality checks would reject these mismatches. No D1-D8 demo can be executed from the register as written.
  Evidence: `DEMO_MATRIX_v0.92.md:35-44` leaves D1-D8 blocked with pending commands and revisions. `docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md:28-44` likewise leaves AEE-001 through AEE-017 blocked with pending proof/review/command fields. `FEATURE_PROOF_COVERAGE_v0.92.md:28-49` marks the corresponding product rows `implemented_with_evidence` (with scoped-out/downstream exceptions). The D9 validator explicitly requires coverage status to equal artifact-index status in `adl/tools/validate_v092_demo_proof_coverage.py` after table parsing. Hydrate the matrix/index/ledger with concrete exact-revision positive, negative, review, command, and artifact evidence, or narrow the feature-coverage statuses so all four surfaces tell one fail-closed truth.

## Target

- Mode: `operate_demo_doc`
- Demo Target: `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`, rows D1 through D9
- Intended Proof Surface: truthful agreement among demo matrix, feature/proof coverage, activation bridge ledger, AEE artifact index, exact revisions, commands, positive/negative artifacts, and review state
- Selected executable demo: D9, WP-20 coverage validator

## Prerequisites

- Demo Entry Surface: `python3 adl/tools/validate_v092_demo_proof_coverage.py --root .`
- Negative Harness: `bash adl/tools/test_v092_demo_proof_coverage.sh`
- Required repository surfaces: demo matrix, feature/proof coverage, activation bridge ledger, AEE artifact index, and retained predecessor receipts under `.csdlc/evidence/308/`
- Provider or Credential Requirements: none
- Network, cloud, or paid requirements: none
- D9 Gate Status: `pass` (Python 3 and every required local entry path available)
- D1-D8 Gate Status: `gated` (nine declared rows because D7A is separate; the authoritative register declares pending owner evidence, immutable revision, review, and command rather than a concrete local entrypoint)
- Exact-target posture: the selected scripts, register documents, and retained #308 evidence have no diff from the requested target in the shared review worktree

## Execution And Classification

### D1 — First birthday proof

- Command Run: none; register value is `pending-owner-command`
- Produced Artifacts: none
- Result: `NOT_RUN`
- Classification: `skipped`
- Reason: explicitly gated by missing concrete owner evidence, exact revision, and command in the declared register; no provider or improvised substitute was authorized

### D2 — Not-a-birthday negative suite

- Command Run: none; register value is `pending-owner-command`
- Produced Artifacts: none
- Result: `NOT_RUN`
- Classification: `skipped`
- Reason: explicitly gated by the same pending owner proof fields; the lane did not invent an ad hoc substitute

### D3 — Continuity across bounded cycles

- Command Run: none; register value is `pending-owner-command`
- Produced Artifacts: none
- Result: `NOT_RUN`
- Classification: `skipped`
- Reason: no concrete command or immutable proof revision is declared

### D4 — Memory grounding proof

- Command Run: none; register value is `pending-owner-command`
- Produced Artifacts: none
- Result: `NOT_RUN`
- Classification: `skipped`
- Reason: no safe executable entrypoint is declared; raw/private memory inspection is outside the lane

### D5 — Capability envelope proof

- Command Run: none; register value is `pending-owner-command`
- Produced Artifacts: none
- Result: `NOT_RUN`
- Classification: `skipped`
- Reason: no concrete local command or exact revision is declared

### D6 — ACP/cognitive-profile proof

- Command Run: none; register value is `pending-owner-command`
- Produced Artifacts: none
- Result: `NOT_RUN`
- Classification: `skipped`
- Reason: no concrete local command or exact revision is declared

### D7A — Adaptive Learning DAG boundary proof

- Command Run: none; register value is `pending-owner-command`
- Produced Artifacts: none
- Result: `NOT_RUN`
- Classification: `skipped`
- Reason: no concrete local command or exact revision is declared

### D7 — ACIP binary schema and WebSocket carrier proof

- Command Run: none; register value is `pending-owner-command`
- Produced Artifacts: none
- Result: `NOT_RUN`
- Classification: `skipped`
- Reason: no concrete local command is declared and this lane prohibited provider/remote execution

### D8 — Birthday-to-governance handoff

- Command Run: none; register value is `pending-owner-command`
- Produced Artifacts: none
- Result: `NOT_RUN`
- Classification: `skipped`
- Reason: documentation-only handoff has no executable proof command in the register

### D9 — WP-20 coverage validator

- Command Run: `python3 adl/tools/validate_v092_demo_proof_coverage.py --root .`
- Intended Proof: positive agreement among the four canonical register surfaces and predecessor evidence
- Produced Artifacts: none; command emitted a bounded failure diagnostic
- Result: `FAIL`
- Classification: `failed`
- Reason: the positive validator rejects canonical `implemented_with_evidence` status before cross-surface validation

### D9 negative harness

- Command Run: `TMPDIR=<repo-local-ignored-temp> bash adl/tools/test_v092_demo_proof_coverage.sh`
- Intended Proof: positive control plus rejection of missing artifacts, duplicate owners, planned-as-passed/substitution, status/owner/command mismatches, and related negative fixtures
- Produced Artifacts: temporary fixtures were cleaned by the harness; no retained proof artifact
- Result: `FAIL`
- Classification: `failed`
- Reason: the harness exits during its initial positive-control validator call on the same unsupported status, before reaching the negative cases

## Aggregate Classification

- Proving: 0
- Non-proving: 0
- Skipped: 9 (D1-D8, including D7A as its own declared row)
- Failed: 1 named executable demo (D9); its companion negative harness also failed
- Overall demo/integration lane: `failed`

## Artifacts

- Review artifact: `docs/reviews/v0.92/internal-review-5846/specialists/demos.md`
- Retained demo output: none produced by D9
- Existing retained proof inspected: `.csdlc/evidence/308/exact-base-revisions.txt` and predecessor receipts referenced by the D9 validator
- Temporary harness files: repo-local ignored temporary directory only; harness cleanup applied

## Limitations

- This lane intentionally performed no provider, cloud, paid-runner, deployment, external network, or remote-host action.
- D1-D8 were not reconstructed from implementation tests because the task was to operate the declared register, not invent replacement demo commands.
- A passing unit or integration test outside the declared matrix would not repair missing exact command/artifact routing in the register.
- The register contains no runnable proof for D1-D8 at the exact target, so runtime claims represented by those rows remain unexecuted in this lane.
- Issue `#269` was excluded and not inspected.

## Follow-up

- Recommended Next Step: repair the D9 vocabulary/cross-surface contract, hydrate or truthfully downgrade every matrix/index/ledger row, rerun the D9 positive and negative commands, and retain their exact-target outputs before any synthesis claims demo coverage.
