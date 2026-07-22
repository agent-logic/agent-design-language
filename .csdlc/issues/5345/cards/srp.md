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
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "WP10-REVIEW-002",
    "severity": "p2",
    "summary": "Installer and selector integration/concurrency proof remains limited.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
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

- Sign/verify adapter wiring and expanded selector integration proof remain before publication.

## Review Result

Revision: Some("git-blake3:a6cd915b7ff71c2ba158833f0fc52a24217c5c47:c958da011d58ab0e69b83163d1bf02d26a1a944d59b14dd6cbb6636a5263e401")

Reviewer: Some("subagent:019f8611-2d02-7492-9c03-7af0fcf6662e")

Result: changes_required
