# Validation Planning Prompt

Template: 1.0.0

Issue: 279

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/279/design.md

Diagram: .csdlc/prepared/issues/279/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Validate #279 issue identity, dependency terminal gates, structured proof/presentation scope, and forbidden sibling/parent ownership before bind or in the dedicated bound topology.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/279/validate_preparation_bundle.py"
    ],
    "parallel_group": "279-serial-01-prep",
    "defer_reason": null
  },
  {
    "lane": "observatory-accessibility-responsive",
    "proof_role": "Run deterministic Node-based accessibility/responsive assertions over the HTML Observatory fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "node",
      "demos/html-observatory/tests/accessibility_responsive.test.mjs"
    ],
    "parallel_group": "279-serial-02-proof",
    "defer_reason": null
  },
  {
    "lane": "observatory-conversation-regression",
    "proof_role": "Rerun the focused HTML Observatory conversation regression touched by the accessibility/responsive presentation slice.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "node",
      "demos/html-observatory/tests/conversation_sessions.test.mjs"
    ],
    "parallel_group": "279-serial-03-conversation-regression",
    "defer_reason": null
  },
  {
    "lane": "observatory-operator-attention-regression",
    "proof_role": "Rerun the focused HTML Observatory operator-attention regression touched by the accessibility/responsive presentation slice.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "node",
      "demos/html-observatory/tests/operator_attention_inbox.test.mjs"
    ],
    "parallel_group": "279-serial-04-operator-attention-regression",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed whitespace or patch artifacts before review.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "279-serial-05-diff",
    "defer_reason": null
  },
  {
    "lane": "fresh-exact-head-review",
    "proof_role": "Require a typed fresh-session exact-head review before publication.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-review",
      "guard",
      "--request",
      "/Users/daniel/git/agent-design-language/.git/csdlc-v2/requests/279-review-guard.json"
    ],
    "parallel_group": "279-serial-06-review",
    "defer_reason": "Deferred until implementation is immutable and validation has passed."
  },
  {
    "lane": "hosted-required-checks",
    "proof_role": "Observe required PR checks on the exact reviewed published head before typed finish through the typed PR-state owner.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-pr-state",
      "--request",
      "/Users/daniel/git/agent-design-language/.git/csdlc-v2/requests/279-pr-state.json"
    ],
    "parallel_group": "279-serial-07-ci",
    "defer_reason": "Deferred until reviewed publication creates the exact PR observation target."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/279/validate_preparation_bundle.py`
- `node demos/html-observatory/tests/accessibility_responsive.test.mjs`
- `node demos/html-observatory/tests/conversation_sessions.test.mjs`
- `node demos/html-observatory/tests/operator_attention_inbox.test.mjs`
- `git diff --check`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-review guard --request /Users/daniel/git/agent-design-language/.git/csdlc-v2/requests/279-review-guard.json`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-pr-state --request /Users/daniel/git/agent-design-language/.git/csdlc-v2/requests/279-pr-state.json`

## Failure Semantics

Fail closed on missing dependency terminal truth, accessibility/responsive proof failure, overbroad Runtime or sibling scope, stale exact-head review, PR linkage failure, CI failure, or terminal-cache mismatch.

## Handoff

Retain typed evidence before convergence.
