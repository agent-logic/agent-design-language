# Structured Intent Prompt

Template: 1.0.0

Issue: 762

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Authenticate invoking issue worktree, branch, exact HEAD and common Git directory before mutation.

## Required Outcome

Repair A520-ARCH-005 and C520-CODE-005: executable location supplies provenance only; authenticated issue context owns writes.

## Scope

- csdlc-v3/src/main.rs
- csdlc-v3/src/commands/proof.rs
- csdlc-v3/tests/proof_parity_install_commands.rs
- csdlc-v3/tests/proof_worktree_binding.rs
- docs/csdlc-v3/PROOF_WORKTREE_BINDING.md

## Authority

- Native v3 remains default. Operator explicitly authorized typed v2 for issue 762 lifecycle remediation on 2026-09-09.

## Assumptions

- none

## Operator Constraints

- Keep main inspection-only. Work beneath FastWork policy parent. Preserve other sessions.
