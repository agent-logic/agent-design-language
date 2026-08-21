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

Revision: Some("git-blake3:238dacac15939362740f769cf4236506afa7c16b:43fb9b1a3b27b82f5f29a33f89d14eb6db6c9c248361aecc6463bf4553efea6e")

Reviewer: Some("fresh-session:e51eef1c-8183-41ca-9e4d-8002de2069df")

Result: pass
