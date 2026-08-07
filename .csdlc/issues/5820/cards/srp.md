# Structured Review Prompt

Template: 1.0.0

Issue: 5820

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/config.rs
adl-runtime/src/bin/adl-runtime-guardian.rs
adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
adl-runtime/src/guardian.rs
adl-runtime/tests/runtime_guardian_lifecycle.rs
adl/tools/validate_v092_runtime_guardian_lifecycle.sh
adl/tools/validate_v092_runtime_native_receipts.rb
.csdlc/issues/5820
.csdlc/evidence/5820

## Prompts

- Is Guardian the only production process owner and is one init file truly authoritative?
- Can configuration, provider, network time, certificate, Vector, or Observatory failure kill or deadlock the kernel?
- Are restart, backoff, cancellation, drain, checkpoint, state recovery, and terminal states bounded and truthful?
- Do authenticated API/WSS and stdout/stderr logging proofs use production paths?
- Are macOS, Linux, and native Windows claims exact and is WP-04/WP-14/WP-18A scope excluded?

## Findings

[
  {
    "id": "F-5820-1",
    "severity": "p2",
    "summary": "Preflight lifecycle reports were overstated as platform acceptance proof despite acceptance_eligible=false.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5820-2",
    "severity": "p2",
    "summary": "Final validation replaced digest-complete native receipt semantics with a shallow selected-field summary check.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Native Windows lifecycle proof remains explicitly blocked by unavailable execution authority.

## Review Result

Revision: Some("git-blake3:833ace7992c0cfd109777183ca55064d4c3a5a05:011a1577df5c66e35f206df42fc5a3578672dd1008f6c5f3a93f080071d8c2d4")

Reviewer: Some("subagent:Leibniz")

Result: changes_required
