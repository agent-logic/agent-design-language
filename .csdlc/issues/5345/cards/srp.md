# Structured Review Prompt

Template: 1.0.0

Issue: 5345

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-v2/crates/adl-cli
adl-v2/tools/install-adl-v2.sh
.csdlc/prepared/issues/5345

## Prompts

- Does every command remain a typed adapter over exactly one reviewed WP-04 through WP-09 boundary without duplicate domain logic?
- Can any argument, environment value, malformed receipt, stale writer, lock race, symlink, path traversal, interruption, or re-read mismatch bypass exact installation verification or alter prior selector bytes?
- Is rollback explicit, compare-and-swap protected, exact-receipt verified, and free of implicit fallback or cutover authority?
- Are machine-readable stdout, diagnostic stderr, stable exit codes, no-network/no-credential behavior, and host-path/secret redaction proven for every command?
- Are COTS, dependency exclusions, LoC/test/module/time budgets, PVF classification, no-deferral, CI, and exact-revision review complete and executable?

## Findings

[
  {
    "id": "WP10-REVIEW-001",
    "severity": "p1",
    "summary": "Sign and verify remain digest placeholders instead of WP-07 adapters.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:7c832595ff338800ea8b065c2e6c1f2a0da45919:be4429fa4c0e5c84cbe4afdcb2ef2cd76c3b0153523f6537112314d81524bbe8",
    "route": null
  },
  {
    "id": "WP10-REVIEW-002",
    "severity": "p2",
    "summary": "Installer and selector integration/concurrency proof remains limited.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:7c832595ff338800ea8b065c2e6c1f2a0da45919:be4429fa4c0e5c84cbe4afdcb2ef2cd76c3b0153523f6537112314d81524bbe8",
    "route": null
  },
  {
    "id": "WP10-REVIEW-003",
    "severity": "p1",
    "summary": "Dependency receipt and ancestry evidence is observational by explicit operator direction and does not block WP-10.",
    "actionable": false,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "operator-directed policy"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:7c832595ff338800ea8b065c2e6c1f2a0da45919:be4429fa4c0e5c84cbe4afdcb2ef2cd76c3b0153523f6537112314d81524bbe8")

Reviewer: Some("subagent:019f8611-2d02-7492-9c03-7af0fcf6662e")

Result: pass
