# Structured Review Prompt

Template: 1.0.0

Issue: 252

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime/src/guardian.rs test-only serialization correctness
both hosted SpawnFailed regressions and missing-program fail-closed semantics
typed validation evidence and all issue #252 acceptance criteria

## Prompts

- Does the correction address the shared deterministic cause rather than mask SpawnFailed?
- Can parallel tests still collide on a child executable or path?
- Do missing programs retain fail-closed behavior?
- Do focused and full Runtime proofs cover both hosted failures?

## Findings

[
  {
    "id": "P1-repeated-proof-not-retained",
    "severity": "p1",
    "summary": "AC-1 repeated proof was claimed but not retained in exact-head evidence.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "Retain ten-run eight-thread proof from two caller working directories and rereview."
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted Linux confirmation remains CI evidence after publication.

## Review Result

Revision: Some("git-blake3:b1254e21135efd2dc5b3c3b26744548f6253128b:250e95dceea945e17da5091056c02bf39c7c07522850ebe5240bc85bbab6296d")

Reviewer: Some("fresh-agent:issue-252-exact-head")

Result: changes_required
