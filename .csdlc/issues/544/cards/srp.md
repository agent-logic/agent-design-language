# Structured Review Prompt

Template: 1.0.0

Issue: 544

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/lifecycle.rs
csdlc-v2/tests/primary_checkout_bootstrap_guard.rs
docs/onboarding.md
csdlc-v2/README.md
.csdlc/issues/544
.csdlc/prepared/issues/544

## Prompts

- Does the guard use Git topology authority rather than branch heuristics?
- Does the guard run before all initialization writes, including design, diagram, issue, prepared, and lock surfaces?
- Does every topology-listing, parsing, canonicalization, and common-dir ambiguity fail before writes?
- Does it preserve non-primary staging checkout bootstrap and idempotent reconciliation?
- Does it leave csdlc-bind FastWork policy unchanged?
- Are tests isolated from the real repository and do they prove zero residue?

## Findings

[
  {
    "id": "REV-544-P1-PRIMARY-SUBDIR-BYPASS",
    "severity": "p1",
    "summary": "Primary-checkout bootstrap can be bypassed by invoking csdlc-issue create with --root set to a subdirectory of the primary checkout because the guard compares the supplied root directly to the primary worktree root instead of Git's resolved top-level.",
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

- none

## Review Result

Revision: Some("git-blake3:22400c62ddd206c08db6331a8d95016cd237fb11:a9f447d829f602057c275d4aaefff590aab30a45ea00588072beb2d86ea6f992")

Reviewer: Some("fresh-session:01a03f33-6211-7253-bf25-d178afac8962")

Result: changes_required
