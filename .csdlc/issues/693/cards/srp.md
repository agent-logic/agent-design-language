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
adl-runtime-kernel/tests/parity_b_live_kernel.rs
adl-runtime-kernel/tests/production_acip_wss.rs

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

Revision: Some("git-blake3:4f7c93031f33a68384072d10d601f6638cec58a9:b9f6f2125d1eeb55d1c84a29318a310f2eb64d0b2dc3729597f55d6a878e1165")

Reviewer: Some("codex:/root/issue_693_post_authority_fix_review")

Result: pass
