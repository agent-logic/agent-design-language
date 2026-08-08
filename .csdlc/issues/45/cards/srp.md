# Structured Review Prompt

Template: 1.0.0

Issue: 45

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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
    "summary": "Absent code_repository is omitted from serialization so pre-field live indexes and retained terminal receipts preserve their canonical digests.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d09397d7a1a9d0413a5cb5c47ad3de1e40a6b65f:9a08cc12a392389a906567b3d440c89c56bdd3823f55b3725b4f47cfed3020bf",
    "route": null
  },
  {
    "id": "P2-explicit-split-missing-origin-evidence",
    "severity": "p2",
    "summary": "Explicit split authority now fails closed when effective origin is absent or not an exact GitHub repository identity.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d09397d7a1a9d0413a5cb5c47ad3de1e40a6b65f:9a08cc12a392389a906567b3d440c89c56bdd3823f55b3725b4f47cfed3020bf",
    "route": null
  },
  {
    "id": "P2-rebind-request-authority-substitution",
    "severity": "p2",
    "summary": "Recorded code authority takes precedence and idempotent bind rejects a retry-supplied repository identity mismatch.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d09397d7a1a9d0413a5cb5c47ad3de1e40a6b65f:9a08cc12a392389a906567b3d440c89c56bdd3823f55b3725b4f47cfed3020bf",
    "route": null
  },
  {
    "id": "P2-ac6-typed-card-schema-evidence",
    "severity": "p2",
    "summary": "VPP, SOR, and retained evidence now include direct current-source typed issue validation, schema proof, and legacy compatibility proof.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d09397d7a1a9d0413a5cb5c47ad3de1e40a6b65f:9a08cc12a392389a906567b3d440c89c56bdd3823f55b3725b4f47cfed3020bf",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Split-publication consistency remains primarily code-inspection and existing Gate 6 coverage rather than a new network-backed publication integration test.
- The compatibility regression proves serialization and digest stability directly but does not exercise every historical terminal receipt in the repository.
- Same-repository operation without an origin retains preexisting behavior; fail-closed unavailable-origin handling is scoped to explicit split authority.

## Review Result

Revision: Some("git-blake3:d09397d7a1a9d0413a5cb5c47ad3de1e40a6b65f:9a08cc12a392389a906567b3d440c89c56bdd3823f55b3725b4f47cfed3020bf")

Reviewer: Some("subagent:issue45_exact_head_review")

Result: pass
