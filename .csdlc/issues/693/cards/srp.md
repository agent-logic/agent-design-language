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

Revision: Some("git-blake3:9ddbac5f54abab65277fe0ac50da865e55bcf841:1d95dc3e0c1a29c7f1a343a453b4dbb9a8ec84aa8717713c0859bc2f83344d6e")

Reviewer: Some("codex:/root/a2a_reliability_fix/review_693_prepr")

Result: pass
