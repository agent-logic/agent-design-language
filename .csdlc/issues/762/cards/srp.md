# Structured Review Prompt

Template: 1.0.0

Issue: 762

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/762/clippy.log
.csdlc/evidence/762/csdlc-owner.log
.csdlc/evidence/762/proof-worktree.log
.csdlc/evidence/762/repair/clippy.log
.csdlc/evidence/762/repair/csdlc-owner.log
.csdlc/evidence/762/repair/proof-worktree.log
.csdlc/evidence/762/review-remediation.md
.csdlc/prepared/issues/762/design.md
.csdlc/prepared/issues/762/diagram.mmd
csdlc-v3/src/commands/proof.rs
csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
csdlc-v3/tests/operational_cli_commands.rs
csdlc-v3/tests/proof_parity_install_commands.rs
csdlc-v3/tests/proof_worktree_binding.rs
csdlc-v3/tests/terminal_cleanup_cutover_commands.rs
docs/csdlc-v3/PROOF_WORKTREE_BINDING.md

## Prompts

- Review authenticated context, path confinement, dormant routes and zero-write negative assertions at exact head.

## Findings

[
  {
    "id": "R762-001",
    "severity": "p1",
    "summary": "Exact install receipt and binary endpoints are now preflighted as regular-or-absent before replacement; symlink/directory denial tests preserve existing binary and mutation surfaces.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e4898e517eedae4abdf4172e4c555331d4e39b7b:aa5354d7fdc899c8a92d1ca392c0a55197ecd6cc4123a184bacb8efb9a299f02",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Native binder/editor repair and OS sandboxing arbitrary proof executables remain separate scope.

## Review Result

Revision: Some("git-blake3:e4898e517eedae4abdf4172e4c555331d4e39b7b:aa5354d7fdc899c8a92d1ca392c0a55197ecd6cc4123a184bacb8efb9a299f02")

Reviewer: Some("codex:/root/review_762_fixed")

Result: pass
