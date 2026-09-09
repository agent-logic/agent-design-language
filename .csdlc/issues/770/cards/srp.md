# Structured Review Prompt

Template: 1.0.0

Issue: 770

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

infra/aws/csm-runtime-spot
infra/aws/modules/csm-runtime-spot
.csdlc/prepared/issues/770
.csdlc/evidence/770

## Prompts

- Check plan-time guards and null handling.
- Verify one-key /32 recovery, isolated application ingress, unchanged private roots, and hardening.
- Confirm live proof and disposal are actual observations, not mocked claims.

## Findings

[
  {
    "id": "R770-001",
    "severity": "p2",
    "summary": "Nonce binds health checks to the launched wildcard listener; fixed and tested.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:843202bd39ad0f3cd34276d0fd77a0f9ed444856:135ad13cb93511f3ee1d729a3f8473bddd7e37f5c70e89c4f755cc100980614f",
    "route": null
  },
  {
    "id": "R770-002",
    "severity": "p2",
    "summary": "Listener cleanup is verified and cleanup failure fails the verdict; fixed and tested.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:843202bd39ad0f3cd34276d0fd77a0f9ed444856:135ad13cb93511f3ee1d729a3f8473bddd7e37f5c70e89c4f755cc100980614f",
    "route": null
  },
  {
    "id": "R770-003",
    "severity": "p2",
    "summary": "Local socket errors cannot prove isolation; only timeout qualifies; fixed and tested.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:843202bd39ad0f3cd34276d0fd77a0f9ed444856:135ad13cb93511f3ee1d729a3f8473bddd7e37f5c70e89c4f755cc100980614f",
    "route": null
  },
  {
    "id": "R770-004",
    "severity": "p3",
    "summary": "Stale live-proof wording corrected in README and local validation evidence.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:843202bd39ad0f3cd34276d0fd77a0f9ed444856:135ad13cb93511f3ee1d729a3f8473bddd7e37f5c70e89c4f755cc100980614f",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live proof uses first-connection host-key trust and a temporary non-TLS responder; it does not prove Runtime/TLS functionality.
- Actual billed cloud cost is not yet measured.

## Review Result

Revision: Some("git-blake3:843202bd39ad0f3cd34276d0fd77a0f9ed444856:135ad13cb93511f3ee1d729a3f8473bddd7e37f5c70e89c4f755cc100980614f")

Reviewer: Some("codex:review_770_static")

Result: pass
