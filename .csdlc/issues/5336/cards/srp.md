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

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:656aa4c4170b1461b367f7d964d3e9a1bc4f0292:307cb2adf9a7b2b5738dcbf0f70c09078b5d93cbb484d90a3dfcf66622e815f3")

Reviewer: Some("subagent:019f832a-9a9e-7331-a0dc-ce6807bd6fb7")

Result: pass
