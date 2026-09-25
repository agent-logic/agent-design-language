# v0.93.1 — CodeFriend Beta 1 launch

## Metadata

Planning template family 1.1.0; authoring issue #922. Scope split approved by the operator; draft package, not opened, executed or release-approved. Named execution owners and resource limits remain to be assigned.

## Status

Draft planning. Approval covers the scope allocation, including all template foundations and families. It does not open the issue wave or authorize extraction, provider spending, public launch or release.

## How To Use

Start with the decisions and sprint plan. The machine-readable execution plan owns task dependencies; this package describes the corresponding outcomes and proof.

## Purpose

Complete the repository split, deliver all CodeFriend templates, and launch a qualified CodeFriend Beta 1.

## Milestone Role

This launch milestone replaces the launch portion of the original 83-candidate v0.93 package. Remaining platform work moves to v0.93.2; moving scope is not deleting work.

## Dependency Boundary

Accepted v0.92.2 closeout may include the explicit #915 deferral. RD-11 must accept repository portability before feature execution. Beta 1 consumes a pinned, compatible, qualified existing Runtime. Runtime v4 is a v0.93.2 result, not a launch prerequisite. Demonstrated launch-blocking Runtime defects require bounded fixes and qualification.

## Scope Summary

43 core candidates: 14 opening/split, 7 CodeFriend, 10 template, 2 integration/qualification, and 10 release-tail results. All supported template families and the full 4+1 architecture package remain included.

## Milestone-Specific Extensions

Existing #1148 (citation-grounded correctness) and #1149 (uncertain second-run recovery) are additional launch prerequisites. CF-05 reuses existing #1150 for independent qualification after scope alignment; do not seed a duplicate CF-05 issue. The 43 core work packages therefore map to 45 distinct planned issue identities including those two additions: three identities already exist and at most 42 core issues need seeding. Preserve their identities and original #915 failures; do not create duplicate tasks or replay an uncertain request to manufacture a result.

## Source Map

The [original v0.93 package](../v0.93/README.md) retains historical planning context. The [v0.92.2 handoff](../v0.92.2/NEXT_MILESTONE_HANDOFF_v0.92.2.md) supplies accepted evidence and explicit limitations. The latest operator scope decision overrides the old blanket Runtime v4 launch dependency.

## Document Map

- [VISION_v0.93.1.md](VISION_v0.93.1.md)
- [DESIGN_v0.93.1.md](DESIGN_v0.93.1.md)
- [DECISIONS_v0.93.1.md](DECISIONS_v0.93.1.md)
- [WBS_v0.93.1.md](WBS_v0.93.1.md)
- [SPRINT_v0.93.1.md](SPRINT_v0.93.1.md)
- [REPOSITORY_MIGRATION_v0.93.1.md](REPOSITORY_MIGRATION_v0.93.1.md)
- [DEMO_MATRIX_v0.93.1.md](DEMO_MATRIX_v0.93.1.md)
- [MILESTONE_CHECKLIST_v0.93.1.md](MILESTONE_CHECKLIST_v0.93.1.md)
- [RELEASE_PLAN_v0.93.1.md](RELEASE_PLAN_v0.93.1.md)
- [RELEASE_NOTES_v0.93.1.md](RELEASE_NOTES_v0.93.1.md)
- [QUALITY_GATE_v0.93.1.md](QUALITY_GATE_v0.93.1.md)
- [FEATURE_PROOF_COVERAGE_v0.93.1.md](FEATURE_PROOF_COVERAGE_v0.93.1.md)
- [WP_EXECUTION_READINESS_v0.93.1.md](WP_EXECUTION_READINESS_v0.93.1.md)
- [ADR_PLAN_v0.93.1.md](ADR_PLAN_v0.93.1.md)
- [NEXT_MILESTONE_HANDOFF_v0.93.1.md](NEXT_MILESTONE_HANDOFF_v0.93.1.md)
- [features/README.md](features/README.md)

## Execution Model

Four proposed sprints: split; product and templates; independent qualification and launch; evidence acceptance and full release tail. Same-sprint dependencies remain ordered. No calendar duration is promised.

## Demo and Validation Surface

Installed hosted website, local-agent website and CLI journeys; ADL self-review, external repository and PR review; complete artifact catalog; real recovery and privacy checks.

## Success Criteria

A permitted external tester completes an evidence-grounded review and obtains readable branded output using the exact accepted deployment, with explicit limits and tested recovery.

## Exit Criteria

All admitted launch requirements have accepted evidence; actual launch authorization and live verification are retained; the full release tail and successor handoff are accepted.
