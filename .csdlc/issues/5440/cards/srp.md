# Structured Review Prompt

Template: 1.0.0

Issue: 5440

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/store.rs
csdlc-v2/tests/gate2.rs

## Prompts

- Verify review authority cannot survive a later-phase design change
- Verify both design and diagram digests refresh atomically
- Verify audit and generation truth

## Findings

[
  {
    "id": "P1-projection-drift",
    "severity": "p1",
    "summary": "Reject unrelated card projection drift during design reapproval",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d6ffeba646080b66ed350ec68639e2a9938ada7e:4b918005fbf71b29bfb6c72bb9cddedf240e564f3937c530ef2ffbfd15414385",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:d6ffeba646080b66ed350ec68639e2a9938ada7e:4b918005fbf71b29bfb6c72bb9cddedf240e564f3937c530ef2ffbfd15414385")

Reviewer: Some("codex-subagent-019f6d31")

Result: pass
