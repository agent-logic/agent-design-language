# Structured Review Prompt

Template: 1.0.0

Issue: 400

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/400
.csdlc/evidence/400
.csdlc/prepared/issues/400

## Prompts

- Does the typed recovery path only allow #400 SPP/STP truth repairs and reject unrelated implemented-phase rewrites?
- Do tests prove positive #117-style repairs and negative stale/phase/malformed cases?
- Are review, publication, and terminal gates preserved?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The new implemented-phase repairs remain intentionally limited to post-review recovery of STP dependency truth and SPP status-only plan-step truth.
- Review inspected retained evidence logs and hashes rather than rerunning Cargo tests to preserve read-only review posture.

## Review Result

Revision: Some("git-blake3:7c446e517f6da4e45aef9c84309d5037602cf6f2:414504fffaaafc20bab723f3a08cc3f96f0b99f1510c22936c3ec4845ee444a5")

Reviewer: Some("fresh-session:5fa2f051-8f48-47d5-b65d-cc1a5280a7c9")

Result: pass
