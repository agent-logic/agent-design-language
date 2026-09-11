# v0.92.1 executable external-review ledger (#833 / #821 AC-2)

Review candidate: `9c7e57d412d61898bd44ab00d53e31afbb779e5c`

Review worktree: isolated detached worktree at the exact candidate (machine-local
path intentionally not retained)

Review posture: read-only review of tracked candidate bytes. This ledger was
produced outside the candidate and is not part of it. No branch, tracked
file, lifecycle record, issue, pull request, workflow, or remote state was
modified.

## Verdict and findings

**FAIL — changes required.**

### P1 — Corporate/Runtime retained proof rejects the exact candidate

Command:

```sh
ruby .csdlc/prepared/issues/818/validate-retained-corporate-runtime-proof.rb
```

Outcome: nonzero; exact output:

```json
{"status":"blocked","error":"current_source_drift"}
```

Evidence inspected:

- `.csdlc/prepared/issues/818/validate-retained-corporate-runtime-proof.rb:50-55`
  loads the tracked plan/receipt and requires the working denominator and source
  mapping to equal the bytes at its expected candidate.
- `.csdlc/prepared/issues/818/validate-retained-corporate-runtime-proof.rb:75-89`
  enforces candidate identity, the complete 17-row denominator, zero claimed
  candidate executions, 17 proposed dispositions, pending operator approval,
  and `release_ready=false`.
- `.csdlc/evidence/818/retained-corporate-runtime/reconciliation.json:6-12`
  is bound to `add8f48867e55be9256244767758abba6f348091`, records zero
  candidate executions, 17 pending approvals, and `release_ready=false`.
- `.csdlc/evidence/818/retained-corporate-runtime/reconciliation.json:36-51`
  demonstrates the retained `pending_operator_review` projection on a concrete
  denominator row.

Impact: #818's packet is truthful historical/proposal evidence, but it is not
executable proof bound to `9c7e57d4...`. It therefore cannot discharge the
current-candidate retained-proof denominator.

Required follow-up: regenerate or explicitly supersede the #818 candidate-bound
projection through #835, then rerun this exact validator at the final candidate.

### P1 — The release projection correctly refuses readiness and is bound to an older candidate

Commands:

```sh
python3 .csdlc/prepared/issues/835/project_release.py --check
python3 .csdlc/prepared/issues/835/project_release.py --require-ready
```

Outcomes:

```json
{"status": "pass", "candidate": "64a99fd71b9770e15cb0dc393d669450d3a5f5b4", "rows": 393, "approved_removals": 143, "negative_cases": 14, "release_decision": "blocked"}
```

The readiness command exited 1 with:

```text
release readiness refused: unresolved final proof and review blockers
```

Evidence inspected:

- `docs/milestones/v0.92.1/evidence/release/current-status/status.json` binds
  the projection to `64a99fd71b9770e15cb0dc393d669450d3a5f5b4`, records
  `release_decision: blocked`, `release_authorized: false`, 393 inventory rows,
  143 approved removals, 51 `execution_refresh_required` rows, and four
  `final_gate_proof_required` rows.
- `.csdlc/issues/835/cards/sor.values.json:40` states that the 51 execution rows
  target an older changed candidate and require refresh.
- `.csdlc/issues/835/cards/sor.values.json:69-80` records the expected false-ready
  refusal and keeps #835 `IN_PROGRESS`.
- `.csdlc/issues/835/cards/sor.values.json:107` records the four final gates and
  51 changed-producer refreshes as unresolved.

Impact: the projection is functioning fail-closed, but it cannot support a PASS
or release-readiness claim at the reviewed candidate.

Required follow-up: refresh the 51 changed-producer executions, discharge the
four #821 final gates, regenerate the projection at the final candidate, and
rerun `--require-ready`.

### P2 — #834 passes as historical reconciliation, not current-candidate execution proof

Commands:

```sh
python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/test_validate.py
python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/validate.py
```

Outcomes:

```text
Ran 2 tests in 2.077s
OK
{"findings": 14, "merged_owners": 8, "observation_time": "2026-09-11T02:52:29.885689+00:00", "release_ready": false, "result": "pass"}
```

Evidence inspected:

- `docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/reconciliation.json:4-11`
  binds the source candidate to `64a99fd...` and explicitly records
  `release_ready=false`.
- `docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/validate.py:43-47`
  checks ancestry rather than rebinding every proof row to the review HEAD.
- `docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/README.md:50-55`
  assigns pending-approval reconciliation and the complete retained gate to
  #835 and states that release readiness remains false.

Impact: TPR-002's predecessor/owner accounting is reproducible and complete,
but it does not certify product or retained proof at `9c7e57d4...`.

## Executed validation ledger

### Candidate integrity and diff hygiene

Commands:

```sh
git rev-parse HEAD
git symbolic-ref -q HEAD || true
git status --short --branch
git diff --check 9c7e57d^1..9c7e57d
```

Observed before creating this ledger:

```text
9c7e57d412d61898bd44ab00d53e31afbb779e5c
## HEAD (no branch)
```

`git symbolic-ref` produced no ref, confirming detached HEAD. `git diff
--check` produced no findings. `git status` showed no tracked or untracked
candidate changes before this ledger was created.

Inspected paths: the candidate merge range contained only the eight #521
TAIL-05 files under `.csdlc/prepared/issues/521/**` and
`docs/milestones/v0.92.1/evidence/release/tail-05/**`.

### TAIL-05 retention and adversarial validation

Commands:

```sh
ruby .csdlc/prepared/issues/521/test-production-validator.rb
bash .csdlc/prepared/issues/521/test-validator.sh
```

Outcome: PASS. Both commands exercised the production positive case and eight
negative classes: invented exact candidate, dropped finding, changed
remediation route, false identity verification, artifact digest mismatch,
changed source digest with recomputed manifest, changed finding with recomputed
manifest, and overstated reviewer independence.

Positive summary:

```json
{"schema":"adl.v0921.external_review_validation.v3","status":"passed","outcome":"failed","non_proving":true,"findings":5,"p1":3,"p2":2}
```

Inspected/executed paths:

- `.csdlc/prepared/issues/521/test-production-validator.rb`
- `.csdlc/prepared/issues/521/test-validator.sh`
- `.csdlc/prepared/issues/521/validate-external-review.rb`
- `docs/milestones/v0.92.1/evidence/release/tail-05/README.md`
- `docs/milestones/v0.92.1/evidence/release/tail-05/findings.json`
- `docs/milestones/v0.92.1/evidence/release/tail-05/packet-manifest.json`

Boundary: this validates faithful retention of the historical failed review;
it does not turn that review into exact-candidate proof.

### Runtime implementation and tests

Commands:

```sh
cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib admission_greeting
cargo test --locked --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model -- --include-ignored --skip real_local_model_smoke
cargo test --locked --manifest-path adl-runtime/Cargo.toml --test config_reload
```

Outcomes:

- Admission greeting: 8 passed, 0 failed, 210 filtered out.
- Shepherd boundary: 2 passed, 0 failed, 1 intentionally filtered (`real_local_model_smoke`).
- Config reload: 8 passed, 0 failed, 0 ignored.

Inspected/executed paths:

- `adl-runtime-kernel/src/control.rs`
- `adl-runtime/tests/shepherd_local_model.rs`
- `adl-runtime/tests/config_reload.rs`
- corresponding manifests and locked dependencies.

The greeting suite proves restart exhaustion and stable logical work identity.
The Shepherd suite proves structural loopback-only URL admission and redirect
denial. The reload suite proves a pending reload is causally observed before
revert/unreadable-file cancellation.

Omission: the live Ollama GPU smoke was intentionally skipped because it is an
ignored provider-availability test, not proof required for these repaired
security boundaries.

### Security: cloud authorization

Command:

```sh
python3 .csdlc/prepared/issues/815/test_cloud_authorization.py
```

Outcome:

```text
PASS: #815 rejected unsigned/forged approval, plan and sidecar mutation, expiry, projection drift, and account/project replay
```

Inspected/executed path: `.csdlc/prepared/issues/815/test_cloud_authorization.py`
and its local fixtures. No cloud mutation or credential access occurred.

### Security and documentation publication integrity

Commands:

```sh
python3 .csdlc/prepared/issues/816/validate-publication-manifest.py . .csdlc/prepared/issues/816/obs-b-publication-paths.txt
bash .csdlc/prepared/issues/512/validate-obs-b-redaction.sh
```

Outcomes:

```text
manifest_integrity:pass:29
{"status":"pass","publication_paths":17,"runtime_paths":1,"ui_paths":5,"evidence_paths":12,"clean_fixtures":1,"negative_fixtures":11,"redaction_findings":0}
```

Inspected/executed paths include:

- `.csdlc/prepared/issues/816/validate-publication-manifest.py`
- `.csdlc/prepared/issues/816/validate-publication-json.py`
- `.csdlc/prepared/issues/816/obs-b-publication-paths.txt`
- `.csdlc/prepared/issues/512/validate-obs-b-redaction.sh`
- `adl-runtime/tests/distributed_projection.rs`
- the 17 declared publication paths and 11 negative fixtures selected by the validator.

### Release truth and C-SDLC v3

Commands:

```sh
python3 .csdlc/prepared/issues/817/validate-release-truth.py
python3 docs/milestones/v0.92.1/evidence/release/current-status/validate.py
python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/validate.py
```

Outcomes:

```text
{"gcp_e_json_readbacks": 18, "github_observation": "receipt_verified", "issue_519_terminal_projection": "closed_out", "ownership_link_tails": 0, "paid_cloud_mutation": false, "status": "passed", "structured_failure_envelopes": 4}
{"status": "pass", "candidate": "64a99fd71b9770e15cb0dc393d669450d3a5f5b4", "rows": 393, "approved_removals": 143, "negative_cases": 14, "release_decision": "blocked"}
```

The V3-F validator completed successfully before the subsequent #818 failure;
it emitted no standalone line. Inspected paths include the release-truth
validator, current-status packet, V3-F mapping/suite/review packet, 18 GCP-E
readbacks, four structured failure envelopes, issue #519 terminal projection,
and feature-proof ownership links.

Boundary: these commands validate packet consistency at their declared source
candidates. The current-status output itself says the release is blocked and is
bound to `64a99fd...`, not the external-review HEAD.

### Retained C-SDLC v3 denominator

Commands:

```sh
ruby .csdlc/prepared/issues/819/validate-retained-v3-proof.rb
ruby .csdlc/prepared/issues/819/test-retained-v3-proof.rb
```

Outcomes:

```text
PASS issue #819 retained-v3 packet: 152/152 unique, 51 candidate-executed, 101 exact removals pending operator review, 0 unclassified, release_ready=false
PASS issue #819 negative matrix: 18/18 invalid receipts rejected
```

Inspected/executed paths: `.csdlc/prepared/issues/819/**` proof plan and
validators plus `.csdlc/evidence/819/retained-v3/reconciliation.json`.

Boundary: 101 rows remain pending operator review and the packet explicitly
reports `release_ready=false`; a passing validator is not behavioral proof for
those rows.

### Distributed Runtime retained denominator

Commands:

```sh
ruby .csdlc/prepared/issues/820/validate-distributed-runtime-proof.rb
ruby .csdlc/prepared/issues/820/test-distributed-runtime-proof.rb
```

Outcomes:

```text
validated issue 820: 25 unique DRT rows, candidate-bound fixture proof, zero behavioral passes, 25 governed dispositions
validated issue 820 negative matrix: 13 fail-closed mutations rejected
```

Inspected/executed paths: `.csdlc/prepared/issues/820/**` proof plan and
validators plus `.csdlc/evidence/820/distributed-runtime/reconciliation.json`.

Boundary: this is disposition/fixture integrity with zero behavioral passes,
not current distributed-runtime qualification.

### Rust source-accounting finding TPR-004

Commands:

```sh
python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measure.py --check docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measurement.json
python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/test_measure.py
```

Outcomes:

```text
PASS recursive inventory, revision identity, all totals and Markdown claims
Ran 3 tests in 1.021s
OK
```

One expected negative fixture prints `fatal: Needed a single revision` while
the test suite still passes. Inspected paths include `measure.py`,
`measurement.json`, `measurement.md`, `README.md`, and `test_measure.py`.

The result resolves the accounting defect: the resilience family grew from
5,278 to 5,995 physical lines (+717), and the full recursively declared scope
grew from 26,911 to 27,628 (+717). The 64-line facade is decomposition evidence,
not code reduction or behavioral proof.

### Active boot paths and control-plane finding TPR-005

Command:

```sh
bash .csdlc/prepared/issues/837/validate-active-boot-paths.sh
```

Outcome: PASS. Ten `csdlc-v3` command-manifest tests passed, followed by six
explicit command-surface checks:

```text
PASS: prepare emits current csdlc-bind command
PASS: prepare emits ready csdlc-doctor command
PASS: prepare emits finalize validation command
PASS: contract text exposes the supported adapter surface
PASS: contract json exposes the supported adapter surface
PASS: sunset start action is unavailable
```

Inspected/executed paths include `.csdlc/prepared/issues/837/**`, the active
boot-path inventory, `csdlc-v3` command manifest tests, current authority
documentation, and supported adapter contracts.

### Dependency lane

Inspected packet paths:

- `docs/milestones/v0.92.1/evidence/release/tail-04/specialist-input/dependency.json`
- the 46 exact dependency denominator references declared in that packet,
  including component `Cargo.toml`/`Cargo.lock` files and other manifest/lock
  surfaces.

Outcome: the retained specialist packet declares no dependency finding and
binds its observations to `fb6cbc...`. This review also rebuilt the focused
Rust targets with `--locked`, successfully resolving the exact lockfiles.

Limitation: no full repository dependency audit, vulnerability database query,
or network-based advisory scan was run. The retained specialist result is
historical candidate-bound evidence, while the focused locked builds are the
only executable dependency observation at `9c7e57d4...`.

### Documentation lane

Inspected paths:

- `docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml`
- `docs/milestones/v0.92.1/FEATURE_PROOF_COVERAGE_v0.92.1.md`
- `docs/milestones/v0.92.1/evidence/release/current-status/EVIDENCE_MAP.md`
- `docs/milestones/v0.92.1/evidence/release/tail-04/findings.json`
- `docs/milestones/v0.92.1/evidence/release/tail-04/specialist-input/README.md`
- `docs/milestones/v0.92.1/evidence/release/tail-04/specialist-input/documentation.json`
- TAIL-05 and issue-834 documentation listed above.

Executable documentation observations were provided by the #817 release-truth
validator: 18 GCP-E JSON readbacks, zero corrupted ownership-link tails, four
structured failure envelopes, and a closed-out #519 terminal projection.

Limitation: the entire historical documentation denominator was not manually
reread byte-by-byte. The review relied on the existing exact-reference packet
for enumeration and reran the remediation validator over current candidate
bytes.

### Provider/cloud lane

Inspected paths:

- `docs/milestones/v0.92.1/evidence/release/tail-04/specialist-input/provider-cloud.json`
- #815 authorization proof and fixtures
- #817 GCP-E readbacks and structured failure envelopes
- provider-local Shepherd boundary test.

Outcome: the bounded no-cloud authorization matrix and local provider boundary
passed. The release-truth validator records `paid_cloud_mutation: false`.

Omissions: no AWS/GCP mutation, paid qualification, live provider invocation,
credential read, or fresh all-region/all-project inventory was authorized or
performed. Historical cloud receipts remain evidence only for their recorded
revision.

## Denominator disposition summary

| Required lane | Executable/current-candidate result | Limitation or rejection |
| --- | --- | --- |
| Candidate integrity | PASS | Detached exact HEAD; clean before ledger |
| Implementation/Runtime | PASS focused | 18 focused tests; no broad Runtime suite |
| Tests | PASS focused | No full repository test denominator |
| Security | PASS focused | No live cloud/provider mutation |
| Documentation | PASS bounded validators | Historical denominator not manually reread in full |
| Dependencies | PASS locked focused builds | No advisory/network audit |
| Retained corporate/Runtime | **FAIL** | `current_source_drift`; 17 stale pending rows |
| Retained C-SDLC v3 | PASS integrity, non-proving | 101 pending removals; release false |
| Distributed Runtime retained | PASS integrity, non-proving | 0 behavioral passes; 25 dispositions |
| C-SDLC v3 boot path | PASS | 10 tests plus six surface checks |
| Provider/cloud | PASS bounded local checks | Paid/live execution omitted |
| Rust accounting | PASS | Quantitative only, not behavior proof |
| Release truth | **FAIL readiness** | older candidate; 51 refreshes + 4 final gates |
| TAIL-05 retention | PASS retention | retained source review remains failed/non-proving |
| #834 predecessor reconciliation | PASS historical accounting | source candidate is older; release false |

## Final cleanliness and mutation boundary

Before this ledger was created, `git status --short --branch` returned only:

```text
## HEAD (no branch)
```

After this ledger is created, the expected only status addition is:

```text
?? V0921_EXECUTABLE_REVIEW_LEDGER.md
```

The untracked ledger is intentionally outside the immutable candidate. Cargo
build products remain ignored. No candidate byte, tracked worktree path,
`main`, lifecycle record, GitHub issue, PR, workflow, or remote ref was changed.
