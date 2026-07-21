# Structured Review Prompt

Template: 1.0.0

Issue: 5336

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.github/workflows/ci.yaml
adl/tools/test_ci_path_policy.sh
adl/tools/test_ci_runtime_contracts.sh
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

Revision: Some("git-blake3:02ef273a804dbd52dcc0d6ddb3adb0d16d0160ec:2ab4631b728b44094f94af6c70c9df7fb212a1a935e3f1c73a03d8c9ca53fde0")

Reviewer: Some("subagent:019f832a-9a9e-7331-a0dc-ce6807bd6fb7")

Result: pass
