# #316 Design — v0.92.1 Reconciliation and v0.92.2 CodeFriend Beta 1 Planning

## Purpose

#316 reconciles the complete v0.92.1 planning package and authors the complete
v0.92.2 CodeFriend Beta 1 planning package before either issue wave is opened.
It updates planning documents only. It does not create, label, move, or close
execution issues.

## Authority and timing

- The operator explicitly makes #314 and #315 non-gating for this planning run.
- v0.92 remains the active milestone until its closeout is complete.
- Issue creation remains deferred until the operator opens each milestone.
- Existing merged planning is the baseline; local TBD material is read-only
  discovery input and never repository authority.

## Reconciliation denominator

The run compares the canonical v0.92.1 package, the global feature list, live
existing issue routing, the retained planned-packet archive, relevant TBD plans,
and the Google Drive CodeFriend product/build/setup/adapter/security plans. Every
candidate receives exactly one disposition: included in v0.92.1, included in
v0.92.2, explicitly deferred, successor handoff, duplicate, or rejected. Tracked
documents must be context-free and must not depend on `.adl` or Google Drive at
execution time.

Required additions to reconcile are:

1. The Agent Logic AWS and GCP account move-in/normalization plans as distinct,
   coordinated inputs to the corporate-transfer lane. Planning preserves
   provider-specific ownership, billing, IAM, organization, project/account,
   audit, and rollback boundaries and grants no cloud mutation authority.
2. Runtime v2/v3 decoupling as a first-class v0.92.1 planning track, using the
   already-authored planning commit as input without treating it as merged.
3. Provider inference profiles and bounded local-model shadow execution,
   excluding the separately deferred MLX/Metal provider and OCI packaging.
4. GCP six-resident qualification as a portable distributed-Runtime sidecar,
   distinct from AWS/Spot qualification and never a paid-run authorization.
5. A complete CodeFriend Beta 1 delivery plan as the v0.92.2 successor
   milestone. Its technical exit bar includes the product shell, portable
   Adapter v2, evidence core, architecture cognition, executable governance,
   specialist review and synthesis, correctness/security/adversarial/
   constitutional perspectives, human publication controls, longitudinal
   intelligence, local/GitHub/CI inputs, Markdown/PDF/HTML outputs, operator and
   user documentation, sample fixtures, and proof on ADL plus one bounded
   external repository. The v0.92.1 boundary is planning and handoff only;
   v0.92.2 must deliver the usable beta rather than another prototype.
6. Existing routing drift for #251, #84, #122, #345, and historical #457, recorded
   without mutating their labels or issue state in this issue.
7. One-to-one or explicitly shared predecessor routing for #188-#190.
8. Agent Technical English (ATE) as a reviewed deferred research program
   targeted provisionally at v0.94. It is outside the v0.92.1 issue wave,
   outside the CodeFriend Beta dependency chain, and creates no issue here.
   Any later execution must begin with the ATE 0.1 language, Intent IR,
   validator, benchmark, and capability-lift gate before model training or
   Runtime integration is authorized.
9. Explicit Beta 1 deferrals for Jira, Linear, Slack, broad Workspace
   integration, autonomous repository mutation, public customer-scale
   deployment, security tournaments, ATE, and optional modernization automation
   unless separately admitted.

## v0.92.2 canonical package

The v0.92.2 package must contain the standard milestone documents: README,
vision, design, decisions, WBS, sprint plan, machine-readable issue wave,
execution specifications, planned-issue catalog, canonical-document inventory,
demo matrix, checklist, release plan, release notes, quality gate, feature-proof
coverage, execution readiness, ADR plan, next-milestone handoff, feature index,
and one feature document per first-class track. Planned IDs remain number-free
and are created only by the later opening issue after the operator releases the
hold.

The issue wave is organized for parallel execution where ownership permits:

- shell/install/onboarding and operator controls;
- portable Adapter v2 and repository ingestion;
- evidence, redaction, retention, and stable artifact identity;
- architecture cognition and ADR/rationale intelligence;
- executable governance and CI fitness functions;
- specialist review, synthesis, remediation, and test planning;
- longitudinal project memory and second-run comparison;
- integrations, exports, dashboard/report UX, docs, and fixtures;
- ADL self-review and external-repository qualification;
- independent review/remediation followed by the canonical release tail.

## Planned architecture

v0.92.1 remains parallel after its opening gate. Corporate/IP, C-SDLC
v3, distributed Runtime, podcast, hot reload, Observatory redesign, Runtime
v2/v3 decoupling, and provider profiles are independently schedulable where
their owned paths do not overlap. Integration consumes reviewed merged outputs;
administrative closeout is asynchronous and never an execution dependency.
v0.92.2 uses the same merge-based dependency rule: independent Beta tracks may
run concurrently, integration consumes reviewed merged outputs, and closeout
never serializes downstream work.

## Validation

- Parse every touched YAML and JSON surface.
- Validate the planned-ID denominator, uniqueness, dependency references, and
  exact issue-creation count without creating issues.
- Resolve Markdown links and reject tracked `.adl` dependencies.
- Require every relevant TBD candidate to have an explicit disposition.
- Validate the complete v0.92.2 standard-document denominator, number-free work
  packages, feature coverage, proof matrix, and canonical release-tail order.
- Verify every Drive-derived Beta 1 requirement is represented or explicitly
  deferred without a runtime dependency on Drive.
- Run diff hygiene and one bounded exact-head documentation review.

## Stop conditions

Stop before issue creation, milestone activation, label migration, release
approval, Runtime v4 implementation, MLX/Metal implementation, paid cloud
execution, CodeFriend Beta implementation, or ATE implementation/training.
