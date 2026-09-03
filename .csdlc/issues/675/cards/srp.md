# Structured Review Prompt

Template: 1.0.0

Issue: 675

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/ingress.rs
demos/html-observatory/app.js
.csdlc/prepared/issues/675

## Prompts

- Does the model/shepherd path emit a first-class governed A2A action rather than relying on reply text?
- Are sender, recipient, work, turn, conversation, and correlation identities distinct and observable?
- Can recipient/provider output be confused with the initiating agent's own reply?
- Are Layer8, roster eligibility, replay, cancellation, and failure semantics preserved?
- Does the UI distinguish accepted dispatch from terminal delivery?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The exact-head review did not rerun cargo or node validation because it was explicitly read-only; it relied on the recorded local validation passes.
- No live credential-backed provider inference, AWS, paid runner, or production runtime restart was performed.

## Review Result

Revision: Some("git-blake3:5212c20d09f2aa38c9c5268be14bfac2452df571:76da7409d54bca64f0fd20ab7a70c11d6822ed6594e932569e068541d4c6c033")

Reviewer: Some("subagent:/root/review_675_provider_envelope_exact")

Result: pass
