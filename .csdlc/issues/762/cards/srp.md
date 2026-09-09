# Structured Review Prompt

Template: 1.0.0

Issue: 762

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/762/base-integration/clippy.log
.csdlc/evidence/762/base-integration/csdlc-owner.log
.csdlc/evidence/762/base-integration/proof-worktree.log
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
    "summary": "Exact receipt and binary endpoints are preflighted before writes; zero-write regressions pass after main integration.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:b5dd3c4832f5abbdde5bc77dbf7f15e50972b5ca:2880ad60a8f5e61d486d92d4c51089b1c19d7b50806d0aed5720165677273a54",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Native binder/editor remediation and OS sandboxing arbitrary proof executables remain separate scope.

## Review Result

Revision: Some("git-blake3:b5dd3c4832f5abbdde5bc77dbf7f15e50972b5ca:2880ad60a8f5e61d486d92d4c51089b1c19d7b50806d0aed5720165677273a54")

Reviewer: Some("codex:/root/review_762_fixed")

Result: pass
