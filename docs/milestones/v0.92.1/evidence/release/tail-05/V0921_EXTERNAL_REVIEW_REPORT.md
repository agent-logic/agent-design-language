# ADL v0.92.1 External Review Report (#833)

Candidate commit: 9c7e57d412d61898bd44ab00d53e31afbb779e5c

> Convenience copy at repository root. The canonical artifact required by the
> handoff is `docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXTERNAL_REVIEW_REPORT.md`.
> Both files are byte-identical apart from this note. If one is edited, they diverge.

## 1. Reviewer identity and independence

- Reviewer: Claude (Anthropic), chat lane, invoked interactively by the operator.
- Provider/model: Anthropic; conversational session, not a C-SDLC-dispatched API lane.
- Independence: **partial.** This lane is independent of the C-SDLC pipeline — it
  was not dispatched, prompted, budgeted, or filed by the system under review.
  It is **not** independent of the operator: the operator selects the inputs,
  supplies the assignment, and receives the output. No review recorded in this
  milestone is performed by a party the operator does not control. This is
  unchanged from the prior lane and is stated so it is not mistaken for
  third-party attestation.

## 2. Checkout and drift result

- Required detached worktree: **not established.** This environment has no Git
  execution. `git fetch`, `git worktree add --detach`, `git rev-parse HEAD`, and
  `git status --short` could not be run.
- Revision binding achieved by **file read**, not Git command:
  `.git/HEAD` contains `ref: refs/heads/main`; `.git/refs/heads/main` contains
  `9c7e57d412d61898bd44ab00d53e31afbb779e5c`. The ref therefore equals the
  candidate commit.
- Checkout is **attached to `main`**, not detached. Worktree cleanliness is
  **unknown**; uncommitted modifications would be invisible to this lane.
  Candidate drift during review is **undetectable** by this lane.
- Consequence: findings below are bound to the candidate ref by inspection, but
  this does not satisfy the handoff's required checkout procedure. Per the
  handoff's own rule, the *procedure* is non-proving even though the *ref*
  matches.

## 3. Findings

Ordered P0–P3. No P0 issued.

### P1 — Mandated coverage denominator is substantially unexercised by this lane

- **Evidence:** the handoff requires coverage reported separately for
  implementation, tests, documentation, security, dependencies, retained proof,
  Runtime, provider/cloud, C-SDLC v3, and release truth, with per-lane paths
  inspected and omissions stated. This review inspected nine files and five
  directory listings (§5). Of the fourteen internal findings in
  `docs/milestones/v0.92.1/evidence/release/tail-04/findings.json`, two were
  verified at the candidate.
- **Impact:** this lane cannot discharge the #833 review denominator. A `PASS`
  from this coverage would be manufactured proof of exactly the kind the handoff
  prohibits. This is a limitation of the reviewing lane, not a defect in the
  candidate.
- **Remediation:** dispatch a review lane with Git execution, build/test
  execution, and budget to traverse the execution specification and evidence map
  as denominators; or accept this report as a bounded partial with the coverage
  table in §5 recorded as its scope.

### P2 — Predecessor reconciliation is bound to a superseded source candidate

- **Evidence:**
  `docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/README.md` binds
  its finding-disposition map to source candidate
  `64a99fd71b9770e15cb0dc393d669450d3a5f5b4`, and states *"Revalidate before
  using this packet on a later candidate."* The review candidate is
  `9c7e57d412d61898bd44ab00d53e31afbb779e5c`.
- **Impact:** the fourteen-finding disposition ledger relied on for TPR-002
  closure is not bound to the reviewed bytes. Its own text anticipates this.
- **Remediation:** run
  `python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/validate.py`
  at `9c7e57d4` and retain the result, or reissue the packet bound to the current
  candidate.

### P3 — Merged removal-approval packets retain pre-merge `pending_operator_review` fields

- **Evidence:** `tail-06/issue-834/README.md` records that PRs #832/#827/#828
  *"explicitly make their merges approval of the exact removal proposal digests,
  without asserting behavioral proof"* and that *"their immutable packets still
  contain pre-merge `pending_operator_review` fields."*
- **Impact:** a downstream consumer reading those packets directly, without the
  #834 narrative, could read `pending_operator_review` as current state. The
  condition is disclosed and owned by #835, so this is recorded rather than
  escalated.
- **Remediation:** ensure the #835 reconciliation emits a terminal projection
  that supersedes those fields explicitly, as #834 already schedules.

## 4. Prior-finding dispositions (TPR-001 … TPR-005)

Against `docs/milestones/v0.92.1/evidence/release/tail-05/findings.json`:

| ID | Prior severity | Disposition at this candidate |
| --- | --- | --- |
| TPR-001 | P1 | **Resolved.** Handoff supplies immutable candidate `9c7e57d4…`; `.git/refs/heads/main` matches it exactly. Residual: required detached-checkout procedure unmet by this lane (§2). |
| TPR-002 | P1 | **Resolved, with §3 P2 caveat.** #520 closed by merged PR #831, merge `e0584126…` recorded ancestral. All fourteen internal findings carry owners and merged PRs in `tail-06/issue-834/reconciliation.json`. |
| TPR-003 | P1 | **Correctly reframed, still open and owned.** `tail-06/issue-834/README.md` states *"Release readiness remains false in this bounded reconciliation."* #835 owns the final projection and the 198-row gate. The prior finding was that closure was requested while every row was blocked; the request framing is now aligned with the artifacts. |
| TPR-004 | P2 | **Not verified this pass.** Owned by #836. No recursive before/after measurement was inspected. |
| TPR-005 | P2 | **Not verified this pass.** Owned by #837. Workspace still contains `adl/`, `adl-v2/`, `csdlc-v2/`, `csdlc-v3/`, `adl-runtime/`, `adl-runtime-kernel/`; no boot-path table was located or sought. |

## 5. Validation performed

All by file read; **no command was executed** in this environment.

| Lane | Inspected | Coverage |
| --- | --- | --- |
| Release truth | `V0921_EXTERNAL_REVIEW_HANDOFF.md`; `tail-05/findings.json`; `tail-06/issue-834/README.md`; `tail-04/SECOND_REVIEW_SUMMARY.md` (prior pass); `MILESTONE_CHECKLIST_v0.92.1.md` (prior pass) | partial |
| Retained proof | `tail-04/findings.json` (14 findings, full read); directory listings of `evidence/release/`, `tail-04/`, `tail-05/`, `tail-06/`, `tail-06/issue-834/` | partial |
| Security | `.csdlc/prepared/issues/512/validate-obs-b-redaction.sh` (full); `adl-runtime/tests/shepherd_local_model.rs` (full) | 2 of 4 SEC findings |
| Tests | `adl-runtime/tests/shepherd_local_model.rs` | partial |
| Candidate integrity | `.git/HEAD`; `.git/refs/heads/main` | complete for ref equality only |
| Implementation | — | **none** |
| Documentation | — | **none this pass** |
| Dependencies | — | **none** |
| Runtime | attempted `adl-runtime-kernel/src/control.rs` ~2400–2530; read failed on tool size limits | **none** |
| Provider/cloud | — | **none** |
| C-SDLC v3 | — | **none** |

### Verified fixes at this candidate

**D520-SEC-004 — fixed, and exceeds the reported defect.**
`adl-runtime/tests/shepherd_local_model.rs:23` replaces the
`starts_with("http://localhost:")` check with `validate_loopback_ollama_origin`,
which rejects userinfo/path/query/fragment/backslash in the authority, requires
an explicit nonzero `u16` port, handles bracketed IPv6, and requires the host to
be `localhost` (case-insensitive) or parse to a loopback `IpAddr`. The test
rejects the exact reported attack `http://localhost:11434@evil.example:80`, plus
`http://2130706433:11434` (decimal IP), `http://%31%32%37.0.0.1:11434`
(percent-encoded), `http://localhost.evil.example:11434` (suffix domain),
`http://[::2]:11434`, and leading/trailing whitespace — seventeen rejection cases
against five acceptance cases. A new `local_model_runner_denies_redirects` test
stands up a real redirect server and asserts the redirect target was never
reached, closing an adjacent exfiltration vector not named in the finding.

**D520-SEC-003 — fixed, and exceeds the reported defect.**
`.csdlc/prepared/issues/512/validate-obs-b-redaction.sh` no longer greps a
planning document for the word "redacted". It now runs the production projection
test (`cargo test --test distributed_projection
coherent_projection_is_deterministic_redacted_and_openapi_aligned`), fails if the
emitted Runtime JSON is empty, validates it against
`adl.distributed.projection.v1`, and scans for six secret classes (machine-local
paths, private-key blocks, GitHub tokens, AWS access keys, bearer tokens,
provider credential assignments). It enforces denominator floors
(`runtime >= 1`, `ui >= 4`, `evidence >= 9`), which structurally prevents the
vacuous-pass mode the finding described. It then requires nine negative fixtures
to be **rejected with a matching error classification**, plus a manifest-path
leak negative and an omitted-declared-artifact negative. This is a proof surface
capable of failing.

## 6. Validation omitted, and why

- **All commands.** No Git, build, `cargo test`, validator, doctor, or cloud
  execution — this lane has no execution access to the repository host. Every
  conclusion is from reading tracked bytes.
- **Twelve of fourteen internal findings.** D520-RUNTIME-001/002 could not be
  read: `control.rs` region ~2400–2530 exceeded the reading tool's size limits.
  D520-SEC-001/002, D520-TEST-001, D520-RET-001, D520-V3F-001, D520-REL-001,
  D520-DOC-003/004, D520-EVID-001/002 were not inspected for reasons of scope
  and session budget, not because evidence was unavailable.
- **The mandated denominators.** `WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml`,
  `FEATURE_PROOF_COVERAGE_v0.92.1.md`, `EVIDENCE_MAP.md`, `AGENTS.md`, and
  `tail-05/packet-manifest.json` were not read this pass.
- **The 198-row retained-proof gap.** Not inspected; owned by #835.

## 7. Assumptions, limitations, residual risks

- Assumes `.git/refs/heads/main` is an accurate, unpacked ref and that no
  uncommitted working-tree modification diverges from it. Neither was verified.
- The two verified fixes are verified **as source text**, not as passing tests.
  Neither validator nor test suite was executed. `D520-SEC-003`'s script invokes
  `cargo test`; whether that test passes at this candidate is unknown here.
- Residual risk: two well-executed fixes are weak evidence about the other
  twelve. Quality observed in a sample does not transfer to the denominator.
- Residual risk: reviewer independence is partial (§1), so this report should not
  be cited as third-party attestation.

## 8. Verdict

**FAIL — changes required**

Stated precisely, because the reason matters:

1. **No new release-blocking defect was found in the candidate.** Everything
   verified at `9c7e57d4` was sound, and the two security fixes inspected are
   materially better than the findings required.
2. The verdict is driven by (a) the coverage gap in §3-P1 — this lane inspected a
   fraction of the mandated denominator and cannot certify what it did not read;
   (b) the §2 checkout procedure being unmet; and (c) release-blocking work that
   the candidate's own artifacts record as open and owned — #835's final
   projection, the 198-row gate, and `tail-06/issue-834`'s explicit
   *"Release readiness remains false."*
3. On the handoff's actual question — whether the candidate is **suitable to
   proceed through release-tail remediation** — the evidence read here is
   affirmative. Remediation is functioning: the predecessor is finalized and
   ancestral, all fourteen findings carry owners and merged PRs, and the two
   fixes sampled are thorough. Nothing found here argues for halting remediation.
   `FAIL` records that this lane cannot certify the candidate, not that the
   candidate is unsound.

Per the handoff, #522 remains open, and a failed review keeps it open regardless
of this report's reasoning.
