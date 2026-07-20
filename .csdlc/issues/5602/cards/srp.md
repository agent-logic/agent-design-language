# Structured Review Prompt

Template: 1.0.0

Issue: 5602

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/run_authoritative_coverage_lane.sh
adl/tools/test_run_authoritative_coverage_lane.sh
adl/tools/test_ci_runtime_contracts.sh
.csdlc/issues/5602
.csdlc/prepared/issues/5602

## Prompts

- Does every partition collect profiles with --no-report?
- Are the explicit combined reports and gates unchanged?
- Does the contract prove partition failures still fail closed?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The full hosted instrumented workspace remains the integration proof for the llvm-cov renderer and profile set.

## Review Result

Revision: Some("git-blake3:4aad6e49e6c9c7651d4890fb5a6da43027916187:734095977026a143ff4755c825b6983f5b93b7b92fc647f555f63295d14c8cae")

Reviewer: Some("codex-task:019f81ab-eadd-7430-be8a-b92f560b7f41")

Result: pass
