# Remaining v0.91.7 Sprint Review Synthesis

Issue: #5403
Status: review packets and canonical register complete; refreshed review pending

## Findings Summary

The ten sprint packets contain 42 packet-level findings:

- P1: 24
- P2: 17
- P3: 1

Current disposition is 39 open, 2 partly fixed, 1 fixed/superseded, and 0
accepted. By severity, the current-risk count is 22 open P1, 16 open P2, and
1 open P3. The partly fixed findings are 1 P1 and 1 P2; the fixed/superseded
finding is P1.

Repeated cross-sprint symptoms are intentionally visible in each owning packet.
They are grouped into shared remediation where one root contract owns the fix.

| Sprint | P1 | P2 | P3 | Result | Remediation |
| --- | ---: | ---: | ---: | --- | --- |
| #4639 WP-12 | 3 | 2 | 0 | changes required | #5404, #5406 |
| #4640 WP-13 | 1 | 3 | 0 | changes required | #5405, #5406 |
| #4648 WP-21 | 1 | 1 | 0 | superseded planning; records finding | #5406 |
| #5036 tools tail | 2 | 2 | 0 | changes required | #5407, #5406 |
| #5045 WP-07 hardening | 3 | 0 | 0 | blocked with findings | #5408 |
| #5121 WP-07A | 3 | 1 | 0 | blocked with findings | #5409 |
| #5174 Runtime v3 parity | 2 | 2 | 1 | changes required | #5410, #5406 |
| #5227 Runtime v3 cutover | 4 | 1 | 0 | changes required; no cutover | #5411, #5413, #5406 |
| #5247 Runtime v3 readiness | 2 | 3 | 0 | changes required | #5412, #5406 |
| #5276 Runtime v3 live parity | 3 | 2 | 0 | changes required | #5413 |

## Highest-Risk Themes

1. Security and authenticity: emergency stop authority, continuity and memory
   authenticity, private-state lineage, and remote Observatory read access.
2. Proof overstatement: static or mocked fixtures labeled integrated, live,
   equivalent, executable, or passed.
3. Runtime architecture divergence: production paths do not execute the
   advertised supervisor/component topology.
4. Readiness and resilience gaps: incomplete component health, unrun final
   soaks, direct-PID guardian shutdown, stale weather, and no real pressure-stop
   serialization.
5. Durable lifecycle truth: ignored historical cards prevent clean-checkout
   audit after the v1 command surface was correctly sunset.

## Release Boundary

- Runtime v3 remains explicit opt-in.
- Runtime v2 remains the default and rollback target.
- Runtime v2 deletion and decommission remain unauthorized.
- Runtime v3 Observatory access is restricted to loopback or a trusted local
  boundary. Remote exposure is prohibited until #5413 adds reviewed read
  authentication and proves the external-access path.
- WP-07, WP-07A, WP-12, WP-13, and the reviewed Runtime v3 cutover surfaces are
  not review-clean while their P1 findings remain open.
- Current v0.91.8 planning supersedes direct consumption of the old #4648
  v0.92 candidate package.

## Remediation Issues

- #5404: WP-12 security/protocol proof and gate findings.
- #5405: WP-13 guild/Godel/economics findings.
- #5406: typed-v2 lifecycle and closeout retention after v1 sunset.
- #5407: #5036 tools reliability-tail findings.
- #5408: #5045 WP-07 hardening findings.
- #5409: #5121 WP-07A implementation completion findings.
- #5410: #5174 Runtime v3 live-kernel assembly findings.
- #5411: #5227 selector/guardian/pressure-stop findings.
- #5412: #5247 Runtime v3 state-authenticity/readiness findings.
- #5413: #5276 live parity and Observatory findings.

## Validation Account

- Every declared child issue and closing PR was reconciled against live GitHub
  state; `CHILD_PR_REVISION_MATRIX.md` retains the child, PR, and merged
  revision chain.
- `SPECIALIST_COVERAGE.md` retains the applicable specialist lanes, reviewed
  revision and scope, packet artifact, and disagreement disposition.
- `docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md` now records
  all ten packets, exact finding counts, remediation owners, and release
  boundaries after #5383 terminally released the path.
- Focused Runtime v2 suites reported 114 `adl-runtime` tests plus 40 integrated
  CSM API tests passing in the WP-07A lane.
- Runtime v3 specialist validation reported 151 passed, 0 failed, and 8
  ignored; ignored tests include the only live v2/v3 process comparison and the
  real 100-cycle soak.
- Documentation integrity uses `git diff --check`.
- Advisory-database coverage is not claimed because `cargo-audit` and
  `cargo-deny` were unavailable.

## Pending Closeout Work

1. Complete refreshed independent review after the current-main merge and
   canonical register update.
2. Republish and close out #5403 after green integration checks.
