# Structured Review Prompt

Template: 1.0.0

Issue: 693

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/telemetry.rs
.csdlc/evidence/693

## Prompts

- Does any path still require exact model-authored JSON?
- Can model prose alone cause dispatch?
- Does the Runtime preserve existing Layer8 admission replay cancellation and correlation authority?
- Does the acceptance enter through production conversation ingress and use non-perfect model output?
- Are A2A results distinct from operator replies and observable?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Behavioral validation used an isolated Ollama-compatible wire fixture rather than the permanent Wuji Runtime or a paid provider.

## Review Result

Revision: Some("git-blake3:7691eb985533a302c70ea4f02bf2351f4619af78:724b63c1fe1adeec07486ceca750980def40ea8d5527aab4c6c59e2e8e521cd4")

Reviewer: Some("codex:/root/a2a_reliability_fix/review_693_prepr")

Result: pass
