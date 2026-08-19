# Structured Output Record

Template: 1.0.0

Issue: 340

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implement bounded HTML Observatory Runtime v3 launch/start-stop-restart integration with CSMctl-managed local service proof and documented exposed-route coverage.

## Artifacts

- CSMctl
- demos/html-observatory/app.js
- adl/tools/test_html_observatory.sh
- adl/tools/validate_v092_observatory_restart_reconnect.sh
- adl-runtime/tests/runtime_api_wss.rs
- .csdlc/evidence/340/proof-summary.md
- .csdlc/issues/340
- .csdlc/prepared/issues/340

## Execution

- CSMctl static Observatory serving maps '/' to '/index.html' and persists both OBSERVATORY_RUNTIME_BASE and OBSERVATORY_URL so the launched UI points at the configured Runtime API target.
- HTML Observatory Runtime v3 live mode now requires /v1/observatory, /v1/ready, and /v1/health to return HTTP 200 before live data is shown.
- The #340 validator proves static contract mode, live local CSMctl startup, graceful stop cleanup, restart recovery, local TLS probing, and documented exposed route categories.
- Focused Runtime API/WSS tests bind OpenAPI docs, validator route coverage, strict live-data gating, and CSMctl root/runtime-base behavior.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "--",
      "CSMctl",
      "adl/tools/test_html_observatory.sh",
      "demos/html-observatory/app.js",
      "adl/tools/validate_v092_observatory_restart_reconnect.sh",
      "adl-runtime/tests/runtime_api_wss.rs",
      ".csdlc/issues/340",
      ".csdlc/prepared/issues/340"
    ],
    "purpose": "Run git diff hygiene check over #340 touched paths.",
    "outcome": "passed",
    "evidence_ref": "340-diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/validate_v092_observatory_restart_reconnect.sh",
      "--contract"
    ],
    "purpose": "Run the #340 contract validator mode.",
    "outcome": "passed",
    "evidence_ref": "340-html-observatory-contract.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/validate_v092_observatory_restart_reconnect.sh",
      "--live"
    ],
    "purpose": "Run the #340 live validator mode against local loopback services and configured TLS material.",
    "outcome": "passed",
    "evidence_ref": "340-live-start-stop-restart.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_api_wss"
    ],
    "purpose": "Run focused Runtime API/WSS integration tests.",
    "outcome": "passed",
    "evidence_ref": "340-runtime-api-wss-contract.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--check"
    ],
    "purpose": "Run cargo fmt check.",
    "outcome": "passed",
    "evidence_ref": "340-rust-format.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-340-html-observatory-runtime-restart-integration",
      "issue",
      "--issue",
      "340"
    ],
    "purpose": "Run C-SDLC v2 typed issue validation for #340.",
    "outcome": "passed",
    "evidence_ref": "340-typed-validate.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
