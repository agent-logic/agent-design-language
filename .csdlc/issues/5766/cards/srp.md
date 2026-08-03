# Structured Review Prompt

Template: 1.0.0

Issue: 5766

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/5766
.csdlc/issues/5766
.csdlc/locks/5766.lock
.csdlc/prepared/issues/5766
adl-runtime/src/runtime_api.rs
adl/src/csm_runtime_api.rs

## Prompts

- Check that advertised availability and mounted routes agree.
- Check that Runtime v3 kernel readiness is not confused with CSM runtime API readiness.
- Check that tests fail on future inventory/router drift.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The exact-head review did not rerun Cargo tests; it reviewed source and retained typed finalize evidence.
- The two Cargo validation logs may include local build paths, so their PVF evidence policy intentionally does not claim relative-only log content.

## Review Result

Revision: Some("git-blake3:5495193ee747e31a8dc15179f5013da17dc08d47:0be84754ef00184e650dc66b1304a43a5be0d2f75f80210d3289a03f1f008c7c")

Reviewer: Some("subagent:019fc928-dc3a-7b11-a3b3-a9627a93d0b6")

Result: pass
