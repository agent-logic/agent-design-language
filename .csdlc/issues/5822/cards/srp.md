# Structured Review Prompt

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/5822
.csdlc/prepared/issues/5822
csdlc-v2/src/cards.rs
csdlc-v2/src/estimation.rs
csdlc-v2/src/finish.rs
csdlc-v2/src/lib.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/estimation_contracts.rs

## Prompts

- Does every forecast field preserve provenance, unknowns, cohort rationale, uncertainty, and drift state?
- Can target actuals, sensitive transcript content, or model-era drift contaminate a forecast?
- Is estimate use strictly advisory in SPP and terminal comparison paths?
- Do backtests and cycle-time cohorts justify the claimed improvement without weakening gates?

## Findings

[
  {
    "id": "WP05-R1",
    "severity": "p1",
    "summary": "Retained AC-7 baseline boundary was inconsistent with acceptance truth.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6b1709887de7c76d1ffc6e377730ac36bca728da:6f2f19b333b3a34ad80092187adbba8b73149778d55dea57837eab5be10add21",
    "route": null
  },
  {
    "id": "WP05-R2",
    "severity": "p1",
    "summary": "PR branch did not incorporate current-main split-authority changes.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6b1709887de7c76d1ffc6e377730ac36bca728da:6f2f19b333b3a34ad80092187adbba8b73149778d55dea57837eab5be10add21",
    "route": null
  },
  {
    "id": "WP05-R3",
    "severity": "p1",
    "summary": "GitHub adapter rejected qualified cross-repository closing linkage and did not verify publication authority.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6b1709887de7c76d1ffc6e377730ac36bca728da:6f2f19b333b3a34ad80092187adbba8b73149778d55dea57837eab5be10add21",
    "route": null
  },
  {
    "id": "WP05-R4",
    "severity": "p1",
    "summary": "Session adapter could not emit schema drift for unsupported availability or schema versions.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6b1709887de7c76d1ffc6e377730ac36bca728da:6f2f19b333b3a34ad80092187adbba8b73149778d55dea57837eab5be10add21",
    "route": null
  },
  {
    "id": "WP05-R5",
    "severity": "p2",
    "summary": "Cycle-time comparison added overlapping intervals instead of wall-clock elapsed time.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6b1709887de7c76d1ffc6e377730ac36bca728da:6f2f19b333b3a34ad80092187adbba8b73149778d55dea57837eab5be10add21",
    "route": null
  },
  {
    "id": "WP05-R6",
    "severity": "p1",
    "summary": "Retained validation summary was stale at generation 49 and omitted its claimed format lane.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6b1709887de7c76d1ffc6e377730ac36bca728da:6f2f19b333b3a34ad80092187adbba8b73149778d55dea57837eab5be10add21",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No equivalent terminal candidate cohort exists; WP-05 intentionally makes no cycle-time or reconnection reduction claim.

## Review Result

Revision: Some("git-blake3:6b1709887de7c76d1ffc6e377730ac36bca728da:6f2f19b333b3a34ad80092187adbba8b73149778d55dea57837eab5be10add21")

Reviewer: Some("subagent:Cicero")

Result: pass
