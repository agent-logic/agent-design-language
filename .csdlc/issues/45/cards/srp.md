# Structured Review Prompt

Template: 1.0.0

Issue: 45

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/45
.csdlc/evidence/45
.csdlc/prepared/issues/45
adl/tools/test_install_adl_pr_cycle_skill.sh
csdlc-v2/operator/skills/csdlc-v2-bind/SKILL.md
csdlc-v2/operator/skills/csdlc-v2-doctor/SKILL.md
csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md
csdlc-v2/src/doctor.rs
csdlc-v2/src/finish.rs
csdlc-v2/src/lifecycle.rs
csdlc-v2/src/model.rs
csdlc-v2/src/publication.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate10a.rs
csdlc-v2/tests/gate2.rs
csdlc-v2/tests/gate4.rs
csdlc-v2/tests/gate5.rs
csdlc-v2/tests/gate_cleanup.rs
csdlc-v2/tests/gate_finish.rs
docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
docs/tooling/C_SDLC_V2_ISSUE_CREATION_AND_BINDING_RUNBOOK.md
docs/tooling/adl_pr_cycle_skill.md

## Prompts

- Can any incidental or attacker-controlled remote mismatch be accepted as a split route?
- Are issue and code repository identities sourced from independent explicit authority rather than guessed from origin?
- Do the three focused tests distinguish same-repo success, valid split success, and invalid drift failure?
- Do all active skills and runbooks teach the new contract consistently without reviving sunset concepts?
- Are the time and token estimates sufficient but non-binding?

## Findings

[
  {
    "id": "P1-legacy-record-receipt-digest-compatibility",
    "severity": "p1",
    "summary": "Absent code_repository remains omitted from claim-free current serialization so pre-field index and retained receipt digests stay stable after main reconciliation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:9d2ec58d032722300871b0edbffe58844dc0ddda:7b5b5a386db8bc2a9459e0a8722e96514c31d2b09d2daccfbc85784887977ccc",
    "route": null
  },
  {
    "id": "P2-explicit-split-missing-origin-evidence",
    "severity": "p2",
    "summary": "Explicit split authority fails closed when effective origin is absent or not an exact GitHub identity.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:9d2ec58d032722300871b0edbffe58844dc0ddda:7b5b5a386db8bc2a9459e0a8722e96514c31d2b09d2daccfbc85784887977ccc",
    "route": null
  },
  {
    "id": "P2-rebind-request-authority-substitution",
    "severity": "p2",
    "summary": "Recorded code authority takes precedence and idempotent bind rejects retry identity substitution after claim removal.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:9d2ec58d032722300871b0edbffe58844dc0ddda:7b5b5a386db8bc2a9459e0a8722e96514c31d2b09d2daccfbc85784887977ccc",
    "route": null
  },
  {
    "id": "P2-ac6-typed-card-schema-evidence",
    "severity": "p2",
    "summary": "Current typed cards, schema, compatibility, lint, formatting, and installer proof remain green after main reconciliation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:9d2ec58d032722300871b0edbffe58844dc0ddda:7b5b5a386db8bc2a9459e0a8722e96514c31d2b09d2daccfbc85784887977ccc",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The read-only rereview did not perform live GitHub publication; typed publish and shepherd own that observation.
- The compatibility regression proves representative pre-field record and retained-receipt digest stability rather than loading every historical receipt.

## Review Result

Revision: Some("git-blake3:9d2ec58d032722300871b0edbffe58844dc0ddda:7b5b5a386db8bc2a9459e0a8722e96514c31d2b09d2daccfbc85784887977ccc")

Reviewer: Some("subagent:issue45_exact_head_review")

Result: pass
