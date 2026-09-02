# Structured Review Prompt

Template: 1.0.0

Issue: 506

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/506/design.md
.csdlc/prepared/issues/506/diagram.mmd
adl-runtime/src/lib.rs
adl-runtime/src/qualification/mod.rs
adl-runtime/tests/distributed_contract/main.rs
adl-runtime/tests/distributed_contract/validate_drt_a.sh
docs/milestones/v0.92.1/evidence/runtime/drt-a/qualification-contract.json

## Prompts

- Verify that #506 owns exactly DRT-A and does not absorb paid AWS/GCP execution, Observatory redesign, DRT-B, DRT-C, DRT-D, provider credentials, or public cloud exposure.
- Verify that the design maps requirements 181 and 182 and includes all four WP-specified PVF lanes.
- Verify that the planned proof denominator includes identity, authority, duplicate-denial, replay, and negative-matrix behavior.

## Findings

[
  {
    "id": "F-506-57BA3DC2-1",
    "severity": "p1",
    "summary": "negative_matrix still does not prove the invalid-input causes for each denied ACIP vector; denial can remain label-driven because expected denied vectors cannot accept.",
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

- Read-only exact-head review only; no live cloud/provider proof was run or claimed.

## Review Result

Revision: Some("git-blake3:d20902de0916b8bf9f45acb77bc0026717bba30a:c6ef1c9cebe00c1ae5daa8229a39b6a5aea769d103a46acbe33fb35ef7a4e5f0")

Reviewer: Some("fresh-session:57ba3dc2-e7d8-463b-8bcf-70963e12dcbb")

Result: changes_required
