# Structured Output Record

Template: 1.0.0

Issue: 695

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented five-minute per-agent partial checkpoints with bounded local retention, asynchronous KMS-encrypted S3 archival, tombstone and local/S3 restart restore, Runtime API continuity fields, Observatory rendering, and a production Terraform archive root. Live AWS apply and Wuji rollout were not performed.

## Artifacts

- adl-runtime-kernel/src/agent_partial_checkpoint.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- demos/html-observatory/app.js
- demos/html-observatory/tests/agent_continuity.test.mjs
- docs/api/runtime-v3/v1/observatory.openapi.json
- infra/aws/runtime/agent-checkpoint-archive
- infra/runtime-v3/runtime-init.toml

## Execution

- Added a default 300-second monotonic skipped-tick coordinator with bounded per-agent concurrency and terminal-turn snapshot isolation.
- Added self-contained checksummed partials, removal tombstones, atomic bounded local retention, coalesced bounded S3 spool, verified uploads, durable receipts/degradation/backoff, and full-checkpoint-lineage local/S3 restore.
- Added provider, model, last snapshot/archive timestamps, sequence, backlog, and explicit continuity states to roster, detail, and Observatory API projections.
- Rendered backing-model and continuity truth in the Observatory agent roster and detail view.
- Added Terraform for a private versioned S3 bucket, rotating CMK, TLS and encryption enforcement, lifecycle retention, and separate least-privilege writer and restore policies.

## Validation

[
  {
    "command": [
      "/bin/bash",
      ".csdlc/prepared/issues/695/validate-implementation.sh"
    ],
    "purpose": "Issue #695 focused implementation validation",
    "outcome": "passed",
    "evidence_ref": "agent-partial-checkpoint.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
