# Structured Review Prompt

Template: 1.0.0

Issue: 5590

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5590/audit.jsonl
.csdlc/issues/5590/index.json
.csdlc/prepared/issues/5590/amend-guardian-cli-coverage-claim.json
.csdlc/prepared/issues/5590/record-coverage-impact-rereview-finding.json
adl-runtime/tests/guardian_cli.rs
adl/tools/check_coverage_impact.sh
adl/tools/test_check_coverage_impact.sh

## Prompts

- Does one init model and one Axum/rustls router truthfully cover local and remote access without hard-coded addresses or HTTP?
- Do HTTP and WebSocket Observatory paths share authentication, origin, authority, frame, redaction, and live-state contracts?
- Does discovery report the actual listener and configured public HTTPS base for default, non-default, and ephemeral ports?
- Does the external guardian distinguish intentional stop, invalid config, bounded retry, pressure serialization, and checkpoint restore without sidecars?
- Does Vector own collection/export while Runtime stderr, health, control, and shutdown survive collector absence?
- Is rollback explicit, reviewed, evidence-preserving, and free of Runtime v2 source edits, automatic cutover, AWS, or deployment claims?
- Do S1 through S6 and all lanes cover AC-1 through AC-8 with no deferred or fixture-only parity credit?

## Findings

[
  {
    "id": "guardian-cli-portability",
    "severity": "p2",
    "summary": "The black-box success and invalid-input cases hard-code /usr/bin/true, making the required coverage regression Unix-specific despite a cross-platform guardian.",
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

- Whole-crate all-target Clippy still reports two pre-existing cav.rs lib-test warnings outside the #5590 protected paths; focused guardian_cli Clippy passes.

## Review Result

Revision: Some("git-blake3:c74af8bb1928075f2cb9580ffaf906f7d49c509f:175fa7171c0a07f55b9a1de2d6688cb5edb2d0d6812a380568e1dfdecf6a6221")

Reviewer: Some("subagent:019f8692-79df-7fe0-98bd-8d42df9b5f1a")

Result: changes_required
