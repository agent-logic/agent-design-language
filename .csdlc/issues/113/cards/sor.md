# Structured Output Record

Template: 1.0.0

Issue: 113

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Closed the exact-ID detail, browser cursor authentication, and population-policy allocation defects.

## Artifacts

- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/live_continuity.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- docs/api/runtime-v3/v1/openapi.json
- docs/api/runtime-v3/v1/observatory.openapi.json
- demos/html-observatory/app.js
- demos/html-observatory/index.html
- demos/html-observatory/styles.css
- adl/tools/test_html_observatory.sh
- adl/tools/validate_v092_html_observatory_roster.mjs
- exact candidate ef1238cd9bca0085d21dc74361308804e91d3ae1
- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/control.rs
- demos/html-observatory/app.js
- adl/tools/test_html_observatory.sh
- adl/tools/validate_v092_html_observatory_roster.mjs
- .csdlc/evidence/113/roster-live-proof-2118c05b3
- 2118c05b3fe503a0f7c902dcc766a90b9cd9c246
- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- docs/api/runtime-v3/v1/observatory.openapi.json
- demos/html-observatory/app.js
- adl/tools/test_html_observatory.sh
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/agent_roster.rs
- demos/html-observatory/app.js
- adl/tools/test_html_observatory.sh

## Execution

- Added the production local roster model, deterministic policy filtering, bounded page tokens, presence/freshness transitions, stable identity, and relocation semantics.
- Projected admitted Shepherd truth through Runtime v3 and versioned OpenAPI without claiming distributed or global roster completeness.
- Integrated bounded roster selection, cursor/incarnation reset, accessible persistent navigation, and responsive browser behavior in the HTML Observatory.
- Routed restart proof through signed Guardian-owned control and made the public result explicitly asynchronous restart acceptance.
- Added exact focused, scale, policy, freshness, pagination, reconnect, relocation, status-transition, reincarnation, and browser proof.
- Replaced production allow-all roster projection with an explicit Runtime-owned public policy that omits unauthorized principals and redacts capabilities and location before serialization.
- Bound per-request roster projection memory to page size, imposed an explicit 10000-entry population ceiling, and retained deterministic ID ordering and revision-bound continuation tokens.
- Classified roster revision gaps as machine-readable full-snapshot resynchronizations in the Observatory instead of ordinary updates.
- Persisted one validated Runtime instance identity in the canonical state root so Guardian restart preserves stable identity while changing process incarnation.
- Refreshed qualified Runtime readiness after restart so the live Observatory cannot remain visibly degraded after recovery.
- Retained source-pinned public-TLS JSON proof and desktop/mobile screenshots with a digest manifest under issue evidence.
- Added the versioned policy-filtered /v1/agents/{agent_id} detail route, OpenAPI contract, browser consumption, and negative visibility behavior.
- Added MAC-protected event cursors bound to revision, policy, filter, and page size; exact successor updates pass while replay-at-current-revision, gaps, policy drift, and query drift fail closed for full resynchronization.
- Removed per-request full-population cloning and BTree reconstruction; Runtime now scans the pre-sorted population once under the explicit 10000-entry ceiling and allocates only the requested page projection.
- Made the production public Observatory policy redact capabilities and location before serialization and aligned proof claims to that behavior.
- Detail lookup now resolves the exact policy-visible ID directly, including 65-128 byte IDs and labels or roles that contain another agent ID.
- Every exact-successor browser update returns the prior cursor to Runtime and accepts the snapshot only when Runtime validates the cursor and returns the matching new cursor.
- Serialized page results no longer clone the Runtime-owned population visibility policy.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--test",
      "agent_roster"
    ],
    "purpose": "Prove Runtime-owned policy filtering, stable identity, freshness, pagination, relocation, and production Shepherd admission.",
    "outcome": "passed",
    "evidence_ref": "Exact candidate ef1238cd9bca0085d21dc74361308804e91d3ae1: agent_roster passed 10/10."
  },
  {
    "command": [
      "cargo",
      "test",
      "--test",
      "control"
    ],
    "purpose": "Prove production roster projection, signed restart acceptance, control policy, and Runtime observatory behavior.",
    "outcome": "passed",
    "evidence_ref": "Exact candidate ef1238cd9bca0085d21dc74361308804e91d3ae1: control passed 24/24."
  },
  {
    "command": [
      "cargo",
      "test",
      "--test",
      "openapi_contract"
    ],
    "purpose": "Prove versioned roster and asynchronous restart contracts match production routes.",
    "outcome": "passed",
    "evidence_ref": "Exact candidate ef1238cd9bca0085d21dc74361308804e91d3ae1: openapi_contract passed 6/6."
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject production Runtime library warnings across the bounded roster implementation.",
    "outcome": "passed",
    "evidence_ref": "Exact candidate ef1238cd9bca0085d21dc74361308804e91d3ae1: production library strict Clippy passed with -D warnings."
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Verify all touched Rust surfaces satisfy canonical formatting.",
    "outcome": "passed",
    "evidence_ref": "Exact candidate ef1238cd9bca0085d21dc74361308804e91d3ae1: cargo fmt --all -- --check passed."
  },
  {
    "command": [
      "bash",
      "adl/tools/test_html_observatory.sh"
    ],
    "purpose": "Prove Runtime v3 roster projection, signed control contract, persistent navigation, and static browser bindings.",
    "outcome": "passed",
    "evidence_ref": "Exact candidate ef1238cd9bca0085d21dc74361308804e91d3ae1: HTML Observatory Runtime v3, signed command, and roster projection contract passed."
  },
  {
    "command": [
      "node",
      "adl/tools/validate_v092_html_observatory_roster.mjs"
    ],
    "purpose": "Prove the trusted-TLS Runtime-backed local Shepherd roster, pagination, policy transitions, relocation, cursor handling, accessible navigation, reconnect, and signed Guardian restart through a new healthy Runtime incarnation.",
    "outcome": "passed",
    "evidence_ref": "Managed Chrome proof passed at exact Runtime source ef1238cd9bca0085d21dc74361308804e91d3ae1 using https://wuji.agent-logic.ai:33783 and Runtime port 33983; restart returned explicit accepted outcome, Runtime incarnation changed, and the restored Shepherd was ready without duplicate rows."
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject malformed whitespace and patch artifacts before final independent review.",
    "outcome": "passed",
    "evidence_ref": "Exact product candidate ef1238cd9bca0085d21dc74361308804e91d3ae1 and the lifecycle-only evidence delta both pass git diff --check."
  },
  {
    "command": [
      "focused issue-113 Runtime, roster, control, OpenAPI, browser, managed-Chrome live restart, lint, format, and diff gates"
    ],
    "purpose": "Prove explicit public authorization and redaction, bounded pagination, revision-gap resynchronization, stable Runtime identity, recovered readiness, and retained exact-head evidence.",
    "outcome": "passed",
    "evidence_ref": "git:2118c05b3fe503a0f7c902dcc766a90b9cd9c246; agent_roster 13/13; control 25/25; openapi_contract 6/6; HTML Observatory PASS; strict lib+binary Clippy PASS; fmt PASS; diff PASS; managed Chrome public-TLS live proof 22/22 with stable runtime_instance_id, changed incarnation, ready recovery, desktop/mobile screenshots, and digest manifest at .csdlc/evidence/113/roster-live-proof-2118c05b3"
  },
  {
    "command": [
      "cargo",
      "llvm-cov",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "agent_roster",
      "--test",
      "control",
      "--test",
      "openapi_contract",
      "--summary-only"
    ],
    "purpose": "Measure the issue-owned Runtime roster and control surfaces under the exact proving tests without substituting unrelated workspace tests.",
    "outcome": "passed",
    "evidence_ref": "Exact head 6a1abef9aa44682dec2aa489d0300962f76bc155: 44/44 focused tests passed under instrumentation; agent_roster.rs line coverage 93.72%, control.rs line coverage 64.32%, ingress.rs 91.01%, telemetry.rs 88.89%. The broader authoritative lane passed 176/177 adl-runtime tests but was stopped after the unrelated distributed three-voter transport test exceeded eight minutes; full kernel instrumentation separately found an unrelated guardian_soak JSON parse failure, so neither broader run is claimed as a passing gate."
  },
  {
    "command": [
      "focused issue-113 roster, control, OpenAPI, browser contract, strict Clippy, format, and diff gates"
    ],
    "purpose": "Prove the policy-safe detail route, authenticated cursor semantics, page-allocation bound, population scan ceiling, and production redaction behavior.",
    "outcome": "passed",
    "evidence_ref": "agent_roster 14/14; control 25/25; openapi_contract 6/6; HTML Observatory contract PASS; strict library and binary Clippy PASS; cargo fmt PASS; git diff --check PASS. Long-running distributed and soak-style tests are explicitly out-of-band under #226 and are not claimed or coupled to this focused gate."
  },
  {
    "command": [
      "focused issue-113 roster, control, OpenAPI, browser contract, strict Clippy, format, and diff gates"
    ],
    "purpose": "Prove exact-ID detail lookup, Runtime-authenticated browser cursor exchange, and page-only policy allocation.",
    "outcome": "passed",
    "evidence_ref": "agent_roster 15/15; control 25/25; openapi_contract 6/6; HTML Observatory contract PASS including cursor return; strict library and binary Clippy PASS; cargo fmt PASS; git diff --check PASS."
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
