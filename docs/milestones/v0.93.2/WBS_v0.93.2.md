> **Current successor routing (2026-10-30):** [#1405 allocation](../v0.93.3/REORGANIZATION_1405.md): Runtime/#1352 v0.93.3; tooling and governance/continuity foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Older placement below is historical. v0.93.2 product scope is unchanged.

# v0.93.2 Wbs

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Setup issue: ADL #1296. Owner of planning: Planning #12. Product execution owners remain unassigned.

## Status

WP-01 #1296 is in progress. Execution is authorized for C-SDLC #296–299 only; other downstream work remains unactivated. No deployment, provider spend, release or Runtime restart is authorized by these documents.

## WBS Summary

31 planned identities across five dependency-based sprints. Preserve all 67 earlier identities through [the allocation register](../v0.93.2/REALLOCATION_1296.json). 21 retained identities plus five explicit Beta 1 product packages and four existing C-SDLC improvements plus independent CI #205; 46 platform identities move to v0.93.3.

## Work Packages

| WP | Sprint | Repository | Outcome |
| --- | --- | --- | --- |
| WP-01 | 1 | agent-design-language | Reconcile the approved smaller milestones and losslessly route all prior identities without downstream activation. |
| CF-B1-01 | 2 | codefriend | Review actual outputs and end-to-end journeys on representative repositories; agree a prioritized gap register and measurable Beta 1 acceptance denominator with the operator. |
| WP-S1 | 2 | agent-logic-enterprise-security | Define and enforce CodeFriend principal, tenant, repository, service and paired-agent trust boundaries against supported Runtime interfaces; identify any missing primitive as a bounded prerequisite. |
| CF-B1-02 | 3 | codefriend | Resolve accepted report-quality gaps against the comprehensive-review PDF reference, using Geist; verify supported export formats, evidence links and human readability. |
| CF-B1-03 | 3 | codefriend | Resolve accepted review correctness, actionable-finding and recovery gaps using the agreed repository denominator. |
| WP-S2 | 3 | agent-logic-enterprise-security | Enforce per-action and per-message least privilege, bounded delegation and revocation for CodeFriend using supported Runtime contracts; preserve denial reasons. |
| WP-S3 | 3 | agent-logic-enterprise-security | Current scoped keys work and rotated/revoked keys fail in signed/encrypted message and checkpoint consumers. |
| WP-S4 | 3 | agent-logic-enterprise-security | Produce tamper-evident, linked and redacted CodeFriend incident evidence accepted by the operator-review consumer; preserve full constitutional-appeal integration for v0.93.3. |
| WP-S5 | 3 | agent-logic-enterprise-security | Enforce tenant/repository isolation, consent, retention, deletion and redacted CodeFriend exports; preserve citizen and social-memory integration for v0.93.3. |
| CF-B1-04 | 3 | codefriend | Prove invited login, repository consent, agent pairing where enabled, first review and results, including revocation and recovery. |
| CF-B1-05 | 3 | codefriend | Resolve accepted resource, diagnostics, retention and support gaps; verify bounded execution and safe disablement on the supported deployment configuration. |
| WP-S6 | 3 | agent-logic-enterprise-security | One bounded red/blue/purple drill replays a seed per control area and produces mitigation, provenance, threat-board and release-risk evidence. |
| DEMO-SEC | 3 | agent-logic-enterprise-security | Execute D9-D12 plus per-message ACIP cases against the private provider and installed Runtime. |
| INTEGRATE | 4 | codefriend | Integrate the exact CodeFriend, site, supported Runtime and enterprise-security lockset; prove complete customer journeys and safe disablement without Runtime v4 or GHB. |
| QUALIFY | 4 | codefriend | Independently qualify the exact Beta 1 candidate against the accepted product and security denominator before launch; prior v0.93.1 evidence is reused only where still applicable. |
| CF-92 | 5 | codefriend | Deploy and launch the separately authorized Beta 1 candidate; verify live onboarding, review/results, waiting list and rollback or safe disablement. |
| TAIL-01 | 5 | agent-design-language | Verify launch outcomes and all required release evidence before the milestone review tail. |
| TAIL-02 | 5 | agent-design-language | Deliver the complete documentation review and external-review handoff result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-03 | 5 | agent-design-language | Deliver the complete publication finalization result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-04 | 5 | agent-design-language | Deliver the complete internal milestone review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-05 | 5 | agent-design-language | Deliver the complete external or third-party review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-06 | 5 | agent-design-language | Deliver the complete accepted-findings remediation or explicit deferral capture result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-07 | 5 | agent-design-language | Deliver the complete next-milestone planning result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-08 | 5 | agent-design-language | Deliver the complete next-milestone closeout planning result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-09 | 5 | agent-design-language | Deliver the complete next-milestone planning review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-10 | 5 | agent-design-language | Deliver the complete release ceremony and milestone close result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |

| CSDLC-296 | 1 | cognitive-sdlc | Preserve edits and historical compatibility while ensuring derived-view drift cannot block valid source publication. |
| CSDLC-297 | 1 | cognitive-sdlc | Preserve applicable proof/review for title/body-only changes; source, head, configuration and linkage changes retain correct freshness and identity checks. |
| CSDLC-298 | 1 | cognitive-sdlc | Use ordinary independent candidate review and authenticated package provenance without mutable PR-body attestation or unrelated lifecycle gates. |
| CSDLC-299 | 1 | cognitive-sdlc | Provide optional safe worktree start and coherent expected-head delivery; confirmed merge is delivery, issue closure is reported separately and cleanup stays optional. |

| CI-205 | 3 | cognitive-sdlc | Complete existing C-SDLC #205: normalize CI using shared conventions and repository-appropriate change-selected checks; remove duplicate proof while preserving required coverage and measuring duration/cost. Paper CI remains out of scope. |

## Sequencing

[Execution graph](EXECUTION_PLAN_v0.93.2.json), [specifications](WP_EXECUTION_SPECIFICATIONS_v0.93.2.yaml), and [issue wave](WP_ISSUE_WAVE_v0.93.2.yaml) carry exact acceptance and dependencies.

## Acceptance Mapping

Each row maps to the complete acceptance and negative cases in the specifications. Existing issue identities must be reused; new work packages are not yet newly created GitHub issues.

## Exit Criteria

No omitted identity, duplicate routing or unaccepted dependency.

## Approved sidecars

The operator explicitly included these two existing issues in v0.93.2. They remain outside the 31 core work packages and numbered sprint counts; total tracked scope is 33 identities. Reuse both issues without duplication. Implementation remains unactivated; inclusion in the plan is not a completion claim. Record each sidecar disposition at milestone review.

- [ADL-1275](https://github.com/agent-logic/agent-design-language/issues/1275): Reconcile post-split repository adapters and native lifecycle admission.
- [CSDLC-304](https://github.com/agent-logic/cognitive-sdlc/issues/304): Preserve C-SDLC installation identity across filesystem remounts.
