> **Superseded combined plan.** The operator-approved split is planned under #922 in [v0.93.1](../v0.93.1/README.md) (repo split, templates and CodeFriend Beta 1 launch) and [v0.93.2](../v0.93.2/README.md) (remaining platform work). The original combined scope below is retained for traceability; it does not require Runtime v4 before Beta 1 launch or authorize execution.

# v0.93 Release Plan

## Status

Forward release plan. v0.93 has not started implementation.

## Release Readiness Themes

v0.93 should not be released until it has evidence for:

- constitutional citizenship contract
- human/guest/operator/service/tool/citizen boundary
- rights and duties model
- Theory of Mind schema and signed update-event contract
- reputation and shared social memory boundary
- standing transition evidence
- constitutional review packet
- challenge and appeal flow
- delegation, upstream delegation, and IAM authority evidence
- zero-trust trust-boundary model and deny-by-default evidence
- policy-enforcement and least-privilege authorization evidence
- secrets/key lifecycle, signing, encryption, rotation, and revocation evidence
- tamper-evident audit, compliance-evidence, and incident-record surfaces
- tenant/polis isolation, data-governance, retention, projection, and privacy
  evidence
- security operations, adversarial regression, provenance, and runtime-hardening
  evidence
- at least one governance proof demo
- at least one enterprise-security proof demo or integrated security proof row
- privacy-preserving reviewer packet

## Review Gates

- Docs must separate engineering substrate, policy model, and contextual claims.
- Release notes must not claim legal personhood, production citizenship, or
  complete constitutional authority.
- Demo evidence must cite concrete artifacts, not policy prose alone.
- Redaction checks must cover private state, host paths, endpoints, and
  secret-like strings.
- Redaction checks must cover private ToM and reputation projections.
- Security review must check default-deny behavior, authority boundaries,
  key/secrets lifecycle, audit integrity, isolation, provenance, and incident
  evidence.
- Release docs must not claim external certification or production compliance
  approval.
- The final review handoff must identify which v0.90.3, v0.91, and v0.92
  prerequisite surfaces were consumed.

## Closeout Notes For TAIL-10

The release ceremony should follow the exact closeout pattern used by recent
milestones at the time v0.93 closes. If the standard ceremony script or release
tail changes before then, TAIL-10 should point to the then-current canonical
pattern rather than inventing a new one.

## Non-Release Conditions

Do not ship v0.93 if:

- standing changes can be made without evidence
- review packets expose raw private state
- reputation or constitutional findings expose private ToM without authority and
  redaction
- human out-of-band action can masquerade as citizen action
- delegation can bypass policy
- zero-trust trust-boundary decisions can be bypassed or default-allow
- secrets, keys, signatures, encryption, rotation, or revocation remain hidden
  implementation folklore rather than reviewable lifecycle records
- audit, compliance, or incident packets cannot be reviewed without leaking
  private state
- isolation and data-governance negative cases are missing
- adversarial regression, provenance, or runtime-hardening evidence is absent
- constitutional review duplicates or contradicts moral trace
- release notes describe planned social or legal authority as landed behavior

## Metadata

Planning template set: 1.1.0. Target: v0.93. Authoring issue: #1047; current reconciliation: #922 in v0.92.2. Accountable planning role: milestone owner; named implementation owners are assigned before opening.

## Mandatory Runtime v4 Release Gate

Operator direction on 2026-09-16 requires Runtime v4 completion in v0.93. TAIL-01 and the release ceremony must reject a candidate without completed RV-01 through RV-11 and independent installed proof for native, process and WASM adapters, provider integration, fenced generation recovery and rollback. Native-only implementation or a deferred adapter does not meet this gate.

## CodeFriend launch readiness

QUALIFY also depends on CF-07. The release packet must contain the accepted Beta 1 evidence mapping, installed Runtime v4 compatibility, deployment/onboarding/recovery results, independent external-tester rehearsal, and website preview/handoff. Record the actual audience and publication state; launch readiness alone is not proof of public availability.

## How To Use

Use the following sequence with the canonical graph. All steps are future work; record actual evidence and authority when executed.

## 0. Release-Tail Convergence

| ID | Required completed result |
|---|---|
| TAIL-01 | Quality gate |
| TAIL-02 | Documentation review and external-review handoff |
| TAIL-03 | Publication finalization |
| TAIL-04 | Internal milestone review |
| TAIL-05 | External or third-party review |
| TAIL-06 | Accepted-findings remediation or explicit deferral capture |
| TAIL-07 | Next-milestone planning |
| TAIL-08 | Next-milestone closeout planning |
| TAIL-09 | Next-milestone planning review |
| TAIL-10 | Release ceremony and milestone close |

The canonical dependency sequence is TAIL-01 through TAIL-10. Reviews, remediation and next-milestone planning are separate results; publication finalization prepares approved material and does not silently perform the release ceremony.

## 1. Release Readiness

Require QUALIFY and all mandatory Runtime v4 outcomes, the accepted migration lockset, governance/security consumers, CodeFriend launch qualification, current documentation and resolved review findings. Use QUALITY_GATE_v0.93.md and the actual issue denominator.

## 2. Branch And Tag Preparation

Confirm accepted source heads and product versions in each owning repository, clean build/install evidence and intended tag targets. The milestone coordinates compatible product releases; it does not invent a single shared version scheme. RD-02 settles distribution policy before this step. Tag mutation occurs only under release authorization.

## 3. GitHub Release Steps

Use the then-current native lifecycle route in each owner repository to create the authorized release from the verified artifact set. Record exact tag/source/artifact links, sanitized notes and approval. Verify remote state after writes; unresolved mutation results must reconcile before retry. No release is created by #1047.

## 4. Verification

Recheck remote tags/releases, install the distributed artifact set in a clean consumer and exercise the declared smoke/recovery journeys. Confirm public ADL needs no private credentials and CodeFriend website claims match actual availability. Failed verification preserves rollback and blocks ceremony completion.

## 5. Communication

Prepare a public summary plus private operator handoffs with the appropriate evidence access. Send announcements or publish site changes only with explicit authorization naming audience and qualified release. Do not disclose private repository content or imply compliance certification.

## Exit Criteria

All tail outcomes accepted, releases and install verification match the intended lockset, issue/PR closure reconciled separately, evidence retained, rollback ownership explicit and successor handoff reviewed. Plans, green CI and draft publication alone cannot meet this bar.


## Confirmed scope reconciliation — #922

The reconciled graph contains **83 candidate results**, including required
CT-01–CT-10 branded artifact templates/style guides and CM-01–CM-04 citizen
reproduction/migration. CT-05 gates CF-05; CM-04 gates INTEGRATE. All remain
behind the accepted repository split. See [feature coverage](FEATURE_COVERAGE_v0.93.md)
for all 21 inherited v0.93 feature rows and the two confirmed additions.

#922 drafting is authorized early; accepted final #921 residuals and the final
v0.92.2 handoff remain pending reconciliation before final planning acceptance.
This package does not open v0.93, establish implementation proof or approve
publication. Runtime v4, CodeFriend launch and both additions cannot be silently
deferred. Existing #875/#671 retain separate identities in the issue ledger.

## Sprint allocation — #922

The [numbered sprint plan](SPRINT_v0.93.md) allocates all 83 core candidates across **8 proposed sprints**. Sprint 1 finishes and accept the repository split before feature work; CodeFriend Beta 1 launch is allocated to Sprint 4 and the release tail to Sprint 8. Existing #875/#671 keep separate identities and gated windows. Sprint numbering creates neither calendar commitments nor execution authority.
