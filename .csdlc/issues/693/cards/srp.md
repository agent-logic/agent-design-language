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

Revision: Some("git-blake3:f3b433ba2a027c04b8bd83c2a68933495328df17:d0f81cc8b1543f77e12509bbbc6c876a4f2a7a0a302cb9df61e6ace635554608")

Reviewer: Some("codex:/root/issue_693_post_authority_fix_review")

Result: pass
