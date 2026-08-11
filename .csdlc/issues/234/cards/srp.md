# Structured Review Prompt

Template: 1.0.0

Issue: 234

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.github/workflows
adl/tools
docs/tooling
csdlc-v2
.csdlc/issues/234
.csdlc/prepared/issues/234

## Prompts

- Does any optional, unrelated, retained-proof, soak, demo, provider, nightly, or release workflow still acquire a runner automatically for an ordinary PR?
- Do all required heavy lanes remain path-policy gated and routed to the configured 16-core runner?
- Can two PR objects for one branch and head SHA execute duplicate required fleets?
- Can an unknown or focused shared-path change fan out to optional workflows or full coverage?
- Are long soaks explicitly isolated from normal tests and PR coverage?

## Findings

[
  {
    "id": "P1-heavy-matrix-cardinality",
    "severity": "p1",
    "summary": "The sole heavy-runner job could declare a strategy matrix whose axis plus include expansion was undercounted, violating the one-heavy-runner allocation invariant.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "Remediate exact 810c0b934 by prohibiting strategy.matrix on adl_ci and adding axis-only, include-only, and combined negative regressions."
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
