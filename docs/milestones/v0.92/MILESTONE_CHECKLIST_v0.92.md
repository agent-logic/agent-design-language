# v0.92 Milestone Checklist

## Metadata

- Milestone: `v0.92`
- Version: `v0.92`
- Engineering closeout date: `2026-08-24`
- Owner: ADL maintainers
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Status

The v0.92 engineering milestone is complete. This checklist separates that
completed engineering boundary from repository documentation reconciliation,
canonical #467 gate hydration, and independently authorized external
publication. Unity/Observatory work outside the landed HTML consumer slice is
backlogged or scheduled later and is not claimed here.

## Purpose

Track the minimum planning, execution, quality, release, and post-release
checks needed for a truthful v0.92 closeout.

## Planning

- [x] Milestone goal reviewed against the identity, continuity, and birthday
  allocation.
- [ ] `v0.91.5` release-tail closeout, the activation-test map, and the
  `v0.91.6` readiness tranche and `v0.91.7` implementation/proof tranche are consumed.
- [ ] `#3377` first-birthday readiness packet consumed or blocked with evidence.
- [x] WBS converted from candidate allocation into concrete WPs.
- [x] Issue wave authored and opened.
- [x] Every mapped child issue has initialized typed cards and explicit outputs,
  proof surfaces, dependencies, and non-goals.
- [ ] Dependencies on v0.90.3 citizen state and v0.91 moral trace checked.
- [ ] Dependencies on v0.91.1 memory/identity, ToM, intelligence metrics,
  governed learning, and capability/aptitude evidence checked for ACP profile
  inputs.
- [ ] Dependencies on v0.91/v0.91.1 ACIP substrate and hardening evidence
  checked for binary schema/catalog transport readiness.
- [ ] WP-12 access and activation gate
  `docs/milestones/v0.91.7/review/security/WP12_ACCESS_ACTIVATION_GATE_4660.md`
  consumed before ACIP, external-agent, WebSocket, SSM, custody, credential, or
  CAV readiness claims are made.

## Scope Integrity

- [ ] v0.92 consumes citizen-state and standing work without redefining it.
- [ ] v0.92 consumes moral trace and trajectory review without duplicating it.
- [ ] v0.92 reserves constitutional citizenship and polis governance for v0.93.
- [ ] ACP/cognitive profiles stay bounded to evidence-grounded runtime profile
  claims and do not become reputation, rights, personhood, or social standing.
- [ ] Binary ACIP stays inspectable through public schemas while
  message-content access remains governed.
- [ ] Birthday, startup, wake, snapshot, admission, and copied state are
  distinguished.
- [ ] Memory palace and learning-model sources are used within bounded scope.

## Execution Discipline

- [x] Every opened issue has `SIP`, `STP`, `SPP`, `VPP`, `SRP`, and `SOR` cards.
- [ ] `SIP`, `STP`, and `SPP` are design-time ready before execution starts.
- [ ] `SPP` is updated if execution materially diverges.
- [ ] `SRP` records actual review findings and dispositions.
- [ ] `SOR` records actual validation and integration truth.
- [ ] Every issue delivers its complete declared outcome; no placeholder,
  scaffold, partial implementation, or intent-only document is accepted as
  completion.
- [ ] Documentation and planning outputs are source-grounded, decision-ready,
  and executable by the owning follow-on without chat reconstruction.
- [ ] Tooling and cleanup outputs show measured useful value and preserve
  required behavior with focused regressions.

## Quality Gates

- [ ] Formatting, lint, and tests pass for implementation changes.
- [ ] Demo matrix commands are runnable where demos are implemented.
- [ ] Birthday review packets cite evidence and do not depend on raw
  private-state inspection.
- [ ] ACP/cognitive-profile review packets cite trace-backed inputs and mark
  unsupported claims explicitly.
- [ ] ACIP binary transport proof includes JSON/protobuf round-trip,
  public-schema lookup, denied-access, malformed-payload, and event-ordering
  cases.
- [ ] ACIP/security claim-boundary proof rejects live WebSocket runtime API,
  production transport security, x402, and red/blue CAV readiness claims unless
  the #4660 gate row is `integrated_proven` or explicitly scoped out with
  evidence.
- [ ] Claim-boundary scan finds no legal-personhood, production-citizenship, or
  completed-governance overclaims.
- [ ] Every feature listed in `features/README.md` has landed exact-revision
  implementation, validation, review, and integration evidence.
- [ ] No feature receives completion credit from fixtures, demo mode,
  receipt-only behavior, synthetic success, or provider substitution.
- [ ] WP-22 blocks internal review while any v0.92 feature remains merely
  planned or lacks accepted proof.

## Review And Release

- [ ] Internal review completed.
- [ ] Third-party documentation review handoff passes its exact-revision send
  gate. Preparation is owned by WP-23/#312 and does not itself mean external
  review or release approval occurred.
- [ ] Findings resolved or explicitly deferred.
- [x] Release notes describe landed work only.
- [ ] Release ceremony completed.

## Release Packaging

- [ ] WP-23/#312 completes exact-revision review and publishes the reconciled
  release-truth docs; #467 separately updates canonical feature status and gate
  notes from landed evidence before review.
- [x] WP-24 delivered all ten review-ready launch articles.
- [ ] WP-24A podcast publication-media work remains out of band and does not
  gate the v0.92 engineering milestone.
- [ ] WP-27 reflected review-finding remediation in release-facing docs where
  needed.
- [ ] WP-29 assembled the release evidence packet.
- [ ] WP-29 rewrote final release notes from draft planning text to landed
  behavior.
- [ ] Review handoff and remediation records are linked.
- [ ] Birthday evidence packet is included or explicitly linked.

## Post-Release

- [x] v0.93 handoff routed without claiming v0.93 implementation.
- [x] Deferred findings are linked to follow-on issues or backlog entries.
- [ ] Release ceremony notes record evidence-backed blockers and scoped-out surfaces.

## Exit Criteria

- Identity, continuity, memory grounding, capability, ACP/cognitive profile,
  ACIP binary transport readiness, witnesses, receipts, first-birthday proof,
  and negative cases are traceable from docs to PRs and evidence.
- The milestone can be audited without reconstructing intent from chat.
