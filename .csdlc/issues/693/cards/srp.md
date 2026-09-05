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
- External agent communication remains out of scope for #693; this issue covers Runtime-owned communication among agents registered on the same local runtime.

## Review Result

Revision: Some("git-blake3:57f3cfc44360a4d51132ffeafb68bca0d05a79af:030f9b35b3bdff6b0440a151e670ad2e2b317236487c53d303750a808d5b60c6")

Reviewer: Some("codex:/root/issue_693_postfix_readonly_review")

Result: pass
