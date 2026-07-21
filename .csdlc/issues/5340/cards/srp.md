# Structured Review Prompt

Template: 1.0.0

Issue: 5340

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-v2/crates/adl-engine

## Prompts

- Does the engine consume only the landed inert #5338 plan and keep ADL plan-level scheduling distinct from Runtime v3 operational scheduling, supervision, recovery, and policy?
- Are readiness, dispatch, joins, completions, retries, cancellation, failures, and saturation fully deterministic and bounded at every limit edge?
- Can completion arrival, map order, retries, duplicate inputs, checkpoint encoding, or fresh-process resume change effects, attempts, snapshots, or final bytes?
- Do provider/tool ports carry stable typed identity and idempotency while keeping production adapters, IO, credentials, policy, and Runtime source outside WP-06?
- Does quiescent-only checkpoint/resume reject every plan, limit, budget, sequence, attempt, identity, state, or encoding mismatch without guessing about in-flight effects?
- Are every #5338 fixture classification, protected path, COTS choice, source/test budget, PVF class, time ceiling, no-deferral acceptance row, rollback action, exact-revision review, and terminal gate explicit and executable?

## Findings

[
  {
    "id": "F-5340-6-resume-journal-reachability",
    "severity": "p1",
    "summary": "Quiescent resume accepts root Ready and cancelled-to-Pending states and does not bind contiguous attempts or terminal outcomes to completion receipts.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5340-7-state-materialization-bound",
    "severity": "p1",
    "summary": "Repeated state references can clone expanded outputs before max_request enforcement and encode_bounded allocates the full serialization first.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5340-8-applicable-fixtures-not-executed",
    "severity": "p1",
    "summary": "The landed applicable fixture inventory is name-checked but none of the six actual files is parsed, compiled, and engine-executed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5340-9-design-truth-drift",
    "severity": "p2",
    "summary": "Approved design/SPP/VPP truth omits the public byte/cardinality bounds and JSON/text state-output contract added during remediation.",
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

Revision: Some("git-blake3:2bed30d6d2a298eb0a3db2d030f06f20901e22de:4b52630fa14ff4ef854bfa4a9485367e01fadbb4623debfab2d866d6085270f2")

Reviewer: Some("subagent:/root/review_5340_exact")

Result: changes_required
