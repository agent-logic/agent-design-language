# WP-12 Security And Protocol Sprint Review

Issue: #4639
Review issue: #5403
Status: changes required
Remediation: #5404; shared records issue #5406

## Findings

### P1: CAV evidence claims integrated runtime execution that never occurs

Every generated scenario is marked `integrated_csm_path: true` in
`adl/src/csm_cav_red_blue.rs:393`, but the purported end-to-end scenario only
constructs and validates a static security-boundary contract at
`adl/src/csm_cav_red_blue.rs:434`. The retained validator trusts the generated
integration booleans at `adl/tools/validate_wp12_cav_red_blue_4914.py:71`
instead of proving that a runtime API, command path, restore path, telemetry
pipeline, or AWS admission boundary was exercised.

Impact: #4914 can be classified `integrated_proven` without crossing the CSM
runtime boundary required by its acceptance criteria. Consumers #4656, #4660,
and #4906 can therefore inherit unsupported integration truth.

Disposition: open. Route a security/runtime remediation issue and downgrade
the consuming rows until real boundary-crossing proof is retained.

### P1: The activation gate and validator require obsolete GitHub state

`adl/tools/validate_wp12_access_activation_gate_4660.py:89` requires #4659 to
remain `pr_open_pending_ci_review` and the only blocker. The retained gate
repeats that state at
`docs/milestones/v0.91.7/review/security/wp12_access_activation_gate_4660.json:136`,
although PR #5146 merged and #4659 closed. The WBS also preserves the stale
open/blocked account at `docs/milestones/v0.91.7/WBS_v0.91.7.md:70`.

Impact: the canonical gate cannot accept current truth without changing code
that is designed to reject it, and downstream activation decisions receive
contradictory blocker information.

Disposition: open. Route a #4660 gate-reconciliation issue covering the parent
#4656 gate, WBS, v0.92 consumers, and validator expectations.

### P1: WP-12 lacks durable merged sprint-review and closeout truth

The local sprint review named by the umbrella closeout is under ignored
`.adl/` state (`.gitignore:3`). No tracked WP-12 sprint state, activity log,
review synthesis, or closeout packet is retained on `main`. The canonical
register still records WP-12 and its children as open with no review packet at
`docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md:74`.

Impact: live GitHub closure is not backed by release-visible sprint review and
closeout truth. A clean checkout cannot reconstruct the closeout record named
by the issue comment.

Disposition: partly fixed by this #5403 packet. Route typed-v2 closeout
normalization separately and reconcile the canonical register after its
current #5383 path claim is released.

### P2: Registered CI lanes compile two security validators without running them

The #4657 lane at
`adl/config/validation_lane_selector.v0.91.6.json:580` runs `py_compile` and
selector/validation-manager tests but omits the focused SSM validator. The
#4660 lane at the same file's line 605 likewise omits the access-gate
validator.

Impact: semantically invalid SSM or activation evidence can pass the selected
CI lane. The stale #4659 state currently passes when its validator is invoked
manually.

Disposition: open. Route a PVF/validation-manifest issue that executes the
focused validators and adds negative state-transition fixtures.

### P2: Credential-policy simulations emit operationally ambiguous audit events

The retained documentation identifies the #4920 cases as simulations at
`docs/milestones/v0.91.7/review/security/WP12_CSM_CREDENTIAL_POLICY_4920.md:35`.
`prove_credential_policy` nevertheless emits each case through the normal
observability route at `adl/src/csm_credential_policy.rs:73`, including
unconditional `break_glass_started` and `break_glass_revoked` events around
line 288, without a proof or simulation classification.

Impact: retained or exported telemetry can be mistaken for a real credential
incident or emergency-access session.

Disposition: open. Route an observability/security issue that marks synthetic
events explicitly and keeps them out of operational audit streams.

## Child Coverage

| Child | PR | Reviewed surface | Result |
| ---: | ---: | --- | --- |
| #4656 | #5129 | Security gate and readiness documents | stale consumer truth; P1 finding |
| #4657 | #5132 | SSM proof, validator, and tests | retained proof; P2 CI-lane finding |
| #4658 | #5137 | ACIP projection architecture and tests | no new active finding |
| #4659 | #5146 | WebSocket code, proof, and dependency | bounded loopback proof is truthful; gate consumer is stale |
| #4660 | #5151 | Access gate and v0.92 consumers | P1 stale-gate and P2 CI-lane findings |
| #4914 | #5160 | CAV code, validator, and retained evidence | P1 false-integration finding |
| #4917 | #5139 | Custody manifest, P-256 trust, and tamper checks | no new active finding |
| #4920 | #5144 | Credential-policy CLI, events, tests, and docs | P2 event-classification finding |

All eight child issues and closing PRs are closed and merged. GitHub reports no
formal PR reviews on the child PRs.

## Testing-Discovered Defects

The #4658 retained packet records incomplete enum coverage and insufficient
schema-drift checks discovered during testing and fixed before merge. It also
records a transient ACIP/AEE failure that passed in isolation and on final
rerun. Those are not counted among this review's five findings. All five
findings above are review-discovered.

## Validation And Limits

- The retained #4657, #4660, and #4914 validators were executed read-only and
  passed, proving internal consistency rather than claim correctness.
- `git diff --check` passed before this packet was authored.
- Rust tests and proof generators were not rerun during the read-only
  specialist pass.
- `cargo-audit` was unavailable, so this review does not claim an
  advisory-free dependency graph.
- Ignored local `.adl` lifecycle artifacts unavailable in this clean worktree
  were not treated as durable release evidence.

## Review Result

Changes required. WP-12 must not be consumed as review-clean until the three P1
findings are repaired or explicitly accepted by the operator and the two P2
findings are routed with truthful release boundaries.
