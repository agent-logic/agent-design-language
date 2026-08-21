# Structured Review Prompt

Template: 1.0.0

Issue: 446

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/446
.csdlc/issues/446
.csdlc/prepared/issues/446/design.md
.csdlc/prepared/issues/446/diagram.mmd
.csdlc/prepared/issues/446/validate_issue446.sh
adl-runtime/src/resident_agent.rs
adl/src/csm_resident_agents.rs
adl/src/csm_runtime_api.rs
adl/src/cli/runtime_v2_cmd/commands.rs
adl/src/cli/runtime_v2_cmd/helpers.rs
adl/src/cli/runtime_v2_cmd/tests.rs
adl/src/governed_executor_parts/logic.rs
adl/src/lib.rs
adl/src/long_lived_agent.rs
adl/src/long_lived_agent/storage.rs
adl/src/long_lived_agent/tests.rs
adl/src/long_lived_agent/types.rs
adl/src/resident_tool_execution.rs
adl/src/runtime_aws_signal.rs

## Prompts

- Can provider output bypass authority?
- Can fixtures enter production?
- Does every proposal have one receipt?
- Are receipts redacted and lineage-bound?
- Is dependency direction acyclic?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The retained live proof is local Ollama gemma4:12b-mlx evidence; Linux/AWS six-resident qualification remains owned by dependent issue #268.

## Review Result

Revision: Some("git-blake3:2f40024e543fb9e64bf25af61c4c0ee91f19e550:31a6556e4aa00d370984477c5b6aa19c4e2e23421d56a605342aa36b1024ed32")

Reviewer: Some("fresh-session:01a0227f-d655-7191-b038-aa8f073dd181")

Result: pass
