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

Revision: Some("git-blake3:ab97441ddcc4b3303d1e7c5e23c3565d53391a43:774adf7be7e71ee0938467385abd6efceffcc451367c0861730e168b79cd74bb")

Reviewer: Some("codex:/root/a2a_reliability_fix/review_693_exact_head_after_recovery")

Result: pass
