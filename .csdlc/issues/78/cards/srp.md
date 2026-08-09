# Structured Review Prompt

Template: 1.0.0

Issue: 78

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/78
.csdlc/issues/78
.csdlc/prepared/issues/78
csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs

## Prompts

- Can any issue without typed review recovery invoke the operation?
- Can the operation modify any field other than STP deliverables?
- Does audit evidence retain both exact previous and replacement values?
- Do negative tests cover every phase, wrong card, stale CAS, malformed input, and projection drift?
- Does the design avoid weakening review and publication authority?

## Findings

[
  {
    "id": "F-78-1",
    "severity": "p2",
    "summary": "SPP steps S1-S3 remained pending after implementation and validation completed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d183351097dc50ed5bdf2adf359f8cfd4fa95113:d7a18aaab08820fb561ff1d225b3065f459b58be6a6f84cdba053caa513f5f5d",
    "route": null
  },
  {
    "id": "F-78-2",
    "severity": "p2",
    "summary": "Focused proof covered stale generation but not stale digest and unchanged durable state.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d183351097dc50ed5bdf2adf359f8cfd4fa95113:d7a18aaab08820fb561ff1d225b3065f459b58be6a6f84cdba053caa513f5f5d",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The operation is intentionally narrow and remains unavailable until this exact reviewed binary is installed; issue #73 consumption is the final pre-publication integration proof.

## Review Result

Revision: Some("git-blake3:d183351097dc50ed5bdf2adf359f8cfd4fa95113:d7a18aaab08820fb561ff1d225b3065f459b58be6a6f84cdba053caa513f5f5d")

Reviewer: Some("codex-subagent:Volta")

Result: pass
