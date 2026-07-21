# Structured Review Prompt

Template: 1.0.0

Issue: 5336

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl/.gitignore
adl/tools/run_authoritative_coverage_lane.sh
adl/tools/test_run_authoritative_coverage_lane.sh
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
    "id": "coverage-first-partition-failure",
    "severity": "p2",
    "summary": "Multiple partition failures overwrite the first causal nonzero status.",
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

Revision: Some("git-blake3:c91a6b8cd0f29566c796e2127be91005093e7d7e:dbc3e8bed12dd5ebc032ebc55f572adbe3821fc54c7d1f9955a83eb8703e0cc0")

Reviewer: Some("task:019f8273-47ae-7d01-85db-cb67c70554ec")

Result: changes_required
