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
    "id": "guardian-binary-coverage-selection",
    "severity": "p1",
    "summary": "The guardian binary path maps only to guardian library tests; it omits the binary unit test, leaving adl-runtime-guardian.rs at 0/135 under focused llvm-cov.",
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

Revision: Some("git-blake3:60da76bcee609930a34c1f409be7131aef75e13d:48ee173827b26195151190f46876d1f72b3a4f5b9edc0a0fe9b21ead0b507f94")

Reviewer: Some("subagent:019f8692-79df-7fe0-98bd-8d42df9b5f1a")

Result: changes_required
