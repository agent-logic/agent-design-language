# v0.93 Milestone README

## Current planning sources

Start with this milestone package and the [candidate catalog](PLANNED_ISSUE_CATALOG_v0.93.md). The current CodeFriend product source is [CodeFriend Beta 1 Plan — Second-pass working draft](https://docs.google.com/document/d/1_CDM8QydW8_Lmue82j5tTHRJpi-hWzOo2u3y6M312b0/edit), last edited August 23 and inspected September 16. Its successor launch work is mapped in [CodeFriend Launch](features/CODEFRIEND_LAUNCH_v0.93.md). The website-only issue is an input, not the product plan.

This first pass contains 83 candidate results. The opening repository split precedes feature development. [Runtime v4](features/RUNTIME_V4_PLUGIN_SYSTEM_v0.93.md) must be completed in v0.93. This package remains under review and does not open the milestone.

## Metadata

- Milestone: `v0.93`
- Version: `v0.93`
- Owner: ADL maintainers
- Status: `forward_planning_candidate`
- Planning template set: `docs/templates/planning/1.0.0`

## Status

Forward planning. v0.93 is not yet an active implementation milestone and has
a candidate issue-wave package but no final opened issue wave. Its boundary was rechecked during the `v0.90.4` WP-19
handoff pass so reputation and social-cognition work stay here instead of
turning into loose `v0.90.4` follow-on debt.

## Purpose

v0.93 is the planned constitutional citizenship, social-cognition, and
polis-governance milestone. It should convert earlier citizen-state,
moral-trace, identity, and standing work into a bounded governance layer for the
ADL polis.

The canonical allocation is recorded in
[CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md](CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md).

## Milestone Role

v0.93 should establish:

- constitutional citizenship as a trace-grounded policy model
- bounded Theory of Mind, relationship, reputation, and shared social memory as
  distinct social-cognition surfaces
- rights, duties, standing, challenge, appeal, delegation, upstream delegation,
  and IAM semantics
- guilds and collective organization as a governed MVP-scoped collaboration
  surface
- enterprise-security foundations for zero-trust polis operation
- cryptographic trust, secrets/key lifecycle, isolation, audit, and incident
  evidence as first-class governance surfaces
- reviewer-facing governance evidence that does not expose raw private state
- a clear boundary between CSM citizen identity and human/operator action

v0.93 should not claim legal personhood, production constitutional authority,
or complete social-contract theory.

## Dependency Boundary

v0.93 depends on:

- v0.90.3 for citizen state, standing, access control, projection, lineage,
  challenge, sanctuary, and quarantine
- v0.91 for Freedom Gate moral events, moral trace, validation, outcome
  linkage, trajectory review, wellbeing, moral resources, and anti-harm
  evidence
- v0.92 for durable identity, names, continuity, memory grounding, capability
  envelopes, and the first true Gödel-agent birthday
- v0.90.4 and v0.90.5 only where economics or governed-tool authority has
  landed before v0.93

## Parallel Python Reduction Tranche

PY-01 owns the bounded tranche from the
[Python Elimination Staged Plan](../../planning/PYTHON_ELIMINATION_STAGED_PLAN.md).
Record start/end tracked file and LOC counts across the split repository set;
moving a file between repositories is not a reduction. After RD-11, remove or
replace one coherent remaining public ADL helper family with behavior parity.
If no suitable family remains, retain an explicit reviewed justification for
no reduction. Preserve the no-new-Python rule; enable a zero-Python CI gate only
where the measured footprint supports it. QUALIFY consumes this disposition.

## Scope Summary

### In scope

- Constitutional citizenship contract.
- Citizen, guest, human-provider, service-actor, and operator boundary.
- Rights and duties model.
- Bounded Theory of Mind and shared social memory.
- Reputation as a redacted, challengeable projection distinct from private ToM.
- Standing maintenance, degradation, restoration, suspension, and revocation.
- Constitutional review packet shape.
- Challenge and appeal flow.
- Delegation, upstream delegation, and IAM policy model.
- Guilds and collective organization.
- Upstream delegation for governed escalation from local cognition to polis,
  trusted external, or frontier cognition providers while preserving identity,
  policy, provenance, and verification boundaries.
- Zero-trust architecture and trust-boundary model.
- Policy enforcement, authorization, and least-privilege checks.
- Secrets, key lifecycle, signing, encryption, and rotation boundaries.
- Tamper-evident audit, compliance, and incident evidence.
- Tenant/polis isolation, data governance, retention, and privacy controls.
- Security operations, adversarial regression, provenance, and runtime
  hardening.
- Bounded social-contract representation.
- Reviewer-facing governance proof candidates.

### Out of scope

- Runtime implementation in this planning pass.
- Legal personhood.
- Production citizenship or complete constitutional authority.
- Full economics, payments, or markets.
- Replacing v0.90.3 citizen-state/security work.
- Replacing v0.91 moral trace.
- Replacing v0.92 identity and birthday semantics.
- Claiming complete enterprise certification, SOC 2, ISO 27001, FedRAMP,
  HIPAA, or other external compliance approval.
- Production cross-polis networking before the required transport-security
  prerequisites exist.
- Treating private Theory of Mind as public reputation or constitutional
  verdict.

## Document Map

- Vision: [VISION_v0.93.md](VISION_v0.93.md)
- Design: [DESIGN_v0.93.md](DESIGN_v0.93.md)
- WBS: [WBS_v0.93.md](WBS_v0.93.md)
- Sprint plan: [SPRINT_v0.93.md](SPRINT_v0.93.md)
- Decisions: [DECISIONS_v0.93.md](DECISIONS_v0.93.md)
- Demo matrix: [DEMO_MATRIX_v0.93.md](DEMO_MATRIX_v0.93.md)
- Milestone checklist: [MILESTONE_CHECKLIST_v0.93.md](MILESTONE_CHECKLIST_v0.93.md)
- Release plan: [RELEASE_PLAN_v0.93.md](RELEASE_PLAN_v0.93.md)
- Release notes: [RELEASE_NOTES_v0.93.md](RELEASE_NOTES_v0.93.md)
- Candidate issue wave: [WP_ISSUE_WAVE_v0.93.yaml](WP_ISSUE_WAVE_v0.93.yaml)
- Feature plans: [features/README.md](features/README.md)
- Constitutional citizenship and polis governance allocation:
  [CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md](CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md)
- Theory of Mind and social cognition:
  [THEORY_OF_MIND_AND_SOCIAL_COGNITION_v0.93.md](features/THEORY_OF_MIND_AND_SOCIAL_COGNITION_v0.93.md)
- Social relationship, reputation, and shared memory:
  [SOCIAL_RELATIONSHIP_REPUTATION_AND_SHARED_MEMORY_v0.93.md](features/SOCIAL_RELATIONSHIP_REPUTATION_AND_SHARED_MEMORY_v0.93.md)
- Citizenship, rights/duties, and social contract:
  [CITIZENSHIP_RIGHTS_DUTIES_AND_SOCIAL_CONTRACT_v0.93.md](features/CITIZENSHIP_RIGHTS_DUTIES_AND_SOCIAL_CONTRACT_v0.93.md)
- Delegation, upstream delegation, IAM, standing, and appeal governance:
  [DELEGATION_IAM_STANDING_AND_APPEAL_GOVERNANCE_v0.93.md](features/DELEGATION_IAM_STANDING_AND_APPEAL_GOVERNANCE_v0.93.md)
- Guilds and collective organization:
  [GUILDS_AND_COLLECTIVE_ORGANIZATION_v0.93.md](features/GUILDS_AND_COLLECTIVE_ORGANIZATION_v0.93.md)
- Enterprise security:
  [ENTERPRISE_SECURITY_v0.93.md](features/ENTERPRISE_SECURITY_v0.93.md)
- Enterprise-security WPs:
  [SECURITY_WP_S1_ZERO_TRUST_ARCHITECTURE_v0.93.md](features/SECURITY_WP_S1_ZERO_TRUST_ARCHITECTURE_v0.93.md),
  [SECURITY_WP_S2_POLICY_ENFORCEMENT_AUTHORIZATION_v0.93.md](features/SECURITY_WP_S2_POLICY_ENFORCEMENT_AUTHORIZATION_v0.93.md),
  [SECURITY_WP_S3_SECRETS_KEYS_CRYPTOGRAPHIC_TRUST_v0.93.md](features/SECURITY_WP_S3_SECRETS_KEYS_CRYPTOGRAPHIC_TRUST_v0.93.md),
  [SECURITY_WP_S4_AUDIT_COMPLIANCE_INCIDENT_EVIDENCE_v0.93.md](features/SECURITY_WP_S4_AUDIT_COMPLIANCE_INCIDENT_EVIDENCE_v0.93.md),
  [SECURITY_WP_S5_ISOLATION_DATA_GOVERNANCE_PRIVACY_v0.93.md](features/SECURITY_WP_S5_ISOLATION_DATA_GOVERNANCE_PRIVACY_v0.93.md),
  [SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md](features/SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md)
- Red/blue adversarial security issue wave:
  [RED_BLUE_ADVERSARIAL_SECURITY_ISSUE_WAVE_v0.93.md](RED_BLUE_ADVERSARIAL_SECURITY_ISSUE_WAVE_v0.93.md)

## Execution Model

Later WP planning should preserve the standard milestone rhythm:

- WP-01: promote reviewed milestone docs and issue wave
- feature WPs: implement constitutional citizenship, ToM/social-cognition,
  standing, review, delegation, IAM, and social-contract surfaces
- security WPs: implement the six enterprise-security foundations for
  zero-trust polis operation
- demo WP: build constitutional/polis proof demos
- quality/review WPs: validate docs, tests, demo evidence, and review packets
- release WP: close the milestone under the normal ceremony pattern

The first-pass logical sequence is now defined in EXECUTION_PLAN_v0.93.json; issue creation still requires opening authorization.

## Success Criteria

v0.93 is ready to execute when:

- every governance feature consumes earlier substrate instead of redefining it
- the human/citizen boundary is explicit in docs, fixtures, and tests
- constitutional review packets are trace-grounded and privacy-preserving
- ToM, reputation, standing, and constitutional review remain distinct
- delegation, upstream delegation, and IAM decisions are reviewable
- demo candidates prove behavior rather than merely describing policy
- philosophical claims remain separated from implemented engineering behavior

## Opening Phase

Repository decomposition opens v0.93 after accepted v0.92.2 closure, before all feature development. See [migration plan](REPOSITORY_MIGRATION_v0.93.md). Public ADL and six private repositories include separate CodeFriend software and website owners. Runtime v4 is retained from the successor handoff with explicit design and breadth gates.

## First-Pass Execution Map

[Canonical graph](EXECUTION_PLAN_v0.93.json), [issue catalog](PLANNED_ISSUE_CATALOG_v0.93.md), [readiness](WP_EXECUTION_READINESS_v0.93.md), [quality gate](QUALITY_GATE_v0.93.md), [source accounting](TBD_SCHEDULING_RECONCILIATION_v0.93.md), [review](planning-review/REVIEW.md), and [successor handoff](NEXT_MILESTONE_HANDOFF_v0.93.md) complete this package. No GitHub milestone or issue wave has opened.


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

The [numbered sprint plan](SPRINT_v0.93.md) allocates all 83 core candidates across **16 proposed sprints**. Sprints 1–3 finish and accept the repository split before feature work; CodeFriend Beta 1 launch is allocated to Sprint 10 and the release tail to Sprints 14–16. Existing #875/#671 keep separate identities and gated windows. Sprint numbering creates neither calendar commitments nor execution authority.
