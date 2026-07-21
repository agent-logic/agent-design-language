# Structured Review Prompt

Template: 1.0.0

Issue: 5336

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.github/workflows/ci.yaml
adl/tools/run_authoritative_coverage_lane.sh
adl/tools/merge_coverage_summaries.py
adl/tools/test_ci_runtime_contracts.sh
adl/tools/test_run_authoritative_coverage_lane.sh
adl/tools/test_merge_coverage_summaries.sh
.csdlc/issues/5336
.csdlc/prepared/issues/5336

## Prompts

- Does the plan distinguish fixture/library proof from live process functionality?
- Can every v0.91.7 feature survive Runtime v2 deletion or retain an explicit non-runtime owner?
- Do four lanes maximize parallelism without overlapping source ownership?
- Do acceptance, cutover, and deletion dependencies fail closed?
- Does the plan prevent duplicate Runtime v3 implementations and uncontrolled LoC/test growth?

## Findings

[
  {
    "id": "isolated-workspace-lcov-provenance",
    "severity": "p1",
    "summary": "The workspace worker deletes run-scoped profiles before a later fresh-shell lcov command, allowing missing or stale Codecov evidence.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:35e7e81c54843212c215dee3e1fd956beda76c1d:df225151a2e07871f666313d58ff9e13f4a9c8032f14b2e8a60b4b4c90eae0ad")

Reviewer: Some("subagent:019f832a-9a9e-7331-a0dc-ce6807bd6fb7")

Result: changes_required
