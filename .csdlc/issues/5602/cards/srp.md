# Structured Review Prompt

Template: 1.0.0

Issue: 5602

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/run_authoritative_coverage_lane.sh
adl/tools/test_run_authoritative_coverage_lane.sh

## Prompts

- Does every partition collect profiles with --no-report?
- Are the explicit combined reports and gates unchanged?
- Does the contract prove partition failures still fail closed?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the full instrumented integration proof.

## Review Result

Revision: Some("git-blake3:a2769d8a672e7c7287440d9a2feda8c59a271f80:654b93e829f3fc563dd69457b8282f2a7d84cf997147967d4f17ab8f53f28f0b")

Reviewer: Some("subagent:review-5602")

Result: pass
