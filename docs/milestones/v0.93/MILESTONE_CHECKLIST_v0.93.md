# v0.93 Milestone Checklist

## Status

Forward checklist. Items are intentionally unchecked because v0.93 has not
entered active execution.

## Planning

- [ ] Milestone goal reviewed against the constitutional citizenship and
  social-cognition allocation.
- [ ] WBS converted from candidate allocation into concrete WPs.
- [ ] Issue wave authored and opened.
- [ ] Cards reviewed for concrete outputs and non-goals.
- [ ] Dependencies on v0.90.3, v0.91, v0.92, and governed tools checked.
- [ ] Six enterprise-security WPs are present in the WBS and mapped to
  concrete proof surfaces.

## Scope Integrity

- [ ] v0.93 consumes citizen-state and standing work without redefining it.
- [ ] v0.93 consumes moral trace and trajectory review without duplicating it.
- [ ] v0.93 consumes identity/birthday work as prerequisite where citizenship
  depends on durable identity.
- [ ] Private ToM, public reputation, standing, and constitutional review remain
  distinct.
- [ ] Shared social memory is redacted, evidence-grounded, and challengeable.
- [ ] Human, guest, operator, service, tool, and citizen-mode boundaries are
  explicit.
- [ ] Economics and payment rails are excluded or explicitly bridged by
  decision.
- [ ] Zero-trust, IAM, delegation, tool authority, standing, and identity are
  aligned instead of modeled as separate competing policy systems.
- [ ] Secrets/key lifecycle, encryption, signing, rotation, revocation, and
  sealed-state access are explicit.
- [ ] Audit, compliance, and incident evidence is redaction-safe and does not
  claim external certification.
- [ ] Isolation, data-governance, retention, deletion, projection, and privacy
  controls are explicit.

## Quality Gates

- [ ] Formatting, lint, and tests pass for implementation changes.
- [ ] Demo matrix commands are runnable where demos are implemented.
- [ ] Review packets cite evidence and do not depend on raw private-state
  inspection.
- [ ] Review packets do not expose private ToM or convert it into public
  reputation without authority and redaction.
- [ ] Security review covers default-deny behavior, least privilege, key/secrets
  lifecycle, audit integrity, incident evidence, isolation, provenance, and
  adversarial regression.
- [ ] Claim-boundary scan finds no production-law, legal-personhood, or
  completed-constitution overclaims.
- [ ] Claim-boundary scan finds no SOC 2, ISO 27001, FedRAMP, HIPAA, or other
  external certification overclaims.

## Review And Release

- [ ] Internal review completed.
- [ ] Third-party review handoff prepared if the milestone follows the current
  review cadence.
- [ ] Findings resolved or explicitly deferred.
- [ ] Release notes describe landed work only.
- [ ] Release ceremony completed.

## Exit Criteria

- Constitutional citizenship, ToM, reputation boundary, shared social memory,
  standing, review, appeal, delegation, IAM, the six enterprise-security WPs,
  and proof demos are traceable from docs to PRs and evidence.
- The milestone can be audited without reconstructing intent from chat.

## Metadata

Planning template set: 1.1.0. Target: v0.93. Authoring issue: #1047; current reconciliation: #922 in v0.92.2. Accountable planning role: milestone owner; named implementation owners are assigned before opening.

## Purpose

Track accepted delivery, not merely planning-file existence.

## Execution Discipline

- [ ] Accepted v0.92.2 close, break and opening approval recorded.
- [ ] RD-11 product lockset accepted before features.
- [ ] Each work item uses its owning repository and native lifecycle.
- [ ] Runtime v4 RV-01 through RV-11 completed with native, process and WASM adapter proof, provider integration and installed recovery qualification.

## Release Packaging

- [ ] Exact multi-repository artifacts and consumer versions pinned.
- [ ] Public notes contain no restricted proof.
- [ ] Rollback and independent installs verified.

## Post-Release

- [ ] Product tags/artifacts and links verified.
- [ ] Residuals handed off with owners; shared rollback retirement separately authorized.

- [ ] CF-01 through CF-07 qualified against the current Drive Beta 1 plan, including external-tester rehearsal and website handoff; deployment/publication authority recorded separately.


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
