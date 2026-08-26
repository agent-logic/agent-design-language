# Structured Intent Prompt

Template: 1.0.0

Issue: 550

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make CSM and the HTML Observatory use the exact browser-trusted Runtime and Observatory origins without changing TLS, continuity, or bind authority.

## Required Outcome

A reviewed repair that emits the trusted Wuji Observatory origin, makes the HTML Observatory trust the configured Wuji Runtime API host instead of stale runtime.dev.agent-logic.ai, and applies Runtime health-route CORS so localhost:8000 can complete its real three-endpoint browser connection.

## Scope

- CSMctl
- docs/tooling/CSMctl.conf.example
- adl-runtime/tests/runtime_api_wss.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/control.rs
- adl/tools/test_csmctl_observatory_origins.sh
- adl/tools/run_owner_validation_lane.sh
- demos/html-observatory/app.js
- demos/html-observatory/runtime-v3.config.json
- demos/html-observatory/tests/security_privacy_adversarial.test.mjs
- .csdlc/prepared/issues/550/validate-csm-origin-generation.sh
- .csdlc/prepared/issues/550/validate-shell-and-diff.sh
- .csdlc/prepared/issues/550/validate-live-wuji.sh
- .csdlc/prepared/issues/550
- .csdlc/evidence/550

## Authority

- Issue #550 owns only boot-time CSM origin wiring
- HOT-01 #510 owns dynamic config hot reload
- Existing Let's Encrypt, Caddy, DNS, router, and TLS key authority does not move
- Merged #540 and PR #543 remain immutable predecessor evidence

## Assumptions

- none

## Operator Constraints

- Never use a self-signed certificate
- Do not rewrite merged PR #543 history
- Use current main and a FastWork issue worktree
- Obtain fresh exact-head review before publication
- Stop before merge until required checks are green
