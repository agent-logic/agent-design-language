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
    "id": "F-5340-1-state-dataflow-unresolved",
    "severity": "p1",
    "summary": "Landed state-dependency outputs are never resolved into downstream requests or request identity.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5340-2-turn-input-unbounded",
    "severity": "p1",
    "summary": "Completion/cancellation cardinality and payload, plan bytes, and policy bytes are unbounded before processing.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5340-3-resume-semantic-truncation",
    "severity": "p1",
    "summary": "Resume accepts canonical but semantically unreachable graph states and truncated or arbitrary completion journals.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5340-4-compiler-fixture-mapping-absent",
    "severity": "p1",
    "summary": "AC-6 lacks a mechanically checked landed compiler-fixture inventory and actual compiler-produced plan coverage.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5340-5-usize-policy-contract",
    "severity": "p2",
    "summary": "JoinPolicy::AtLeast serializes an architecture-width-dependent usize threshold.",
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

Revision: Some("git-blake3:b9ba0c37f4bc381cb2d74589ad0d27dc1b6fe7be:154cca22aa607086e7ec9fb5109c5671db2c2d96dbe850335766f04c59873c72")

Reviewer: Some("subagent:/root/review_5340_exact")

Result: changes_required
