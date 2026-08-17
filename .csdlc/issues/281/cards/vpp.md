# Validation Planning Prompt

Template: 1.0.0

Issue: 281

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/281/design.md

Diagram: .csdlc/prepared/issues/281/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Validate #281 issue identity, dependency ancestry, security/privacy/adversarial proof scope, and forbidden sibling/parent ownership before bind.",
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
      ".csdlc/prepared/issues/281/validate_preparation_bundle.py"
    ],
    "parallel_group": "281-serial-01-prep",
    "defer_reason": null
  },
  {
    "lane": "observatory-security-privacy-adversarial",
    "proof_role": "Run deterministic Node-based XSS, credential/token, origin metadata, replay/confused-deputy/stale/denial, and redaction assertions over the HTML Observatory fixtures.",
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
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "node",
      "demos/html-observatory/tests/security_privacy_adversarial.test.mjs"
    ],
    "parallel_group": "281-serial-02-proof",
    "defer_reason": null
  },
  {
    "lane": "observatory-conversation-regression",
    "proof_role": "Rerun the focused HTML Observatory conversation-session regression touched by the security/privacy slice.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "node",
      "demos/html-observatory/tests/conversation_sessions.test.mjs"
    ],
    "parallel_group": "281-serial-03-ui-regression",
    "defer_reason": null
  },
  {
    "lane": "observatory-operator-attention-regression",
    "proof_role": "Rerun the focused HTML Observatory operator-attention regression touched by the security/privacy slice.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "node",
      "demos/html-observatory/tests/operator_attention_inbox.test.mjs"
    ],
    "parallel_group": "281-serial-04-ui-regression",
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
    "parallel_group": "281-serial-05-diff",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/281/validate_preparation_bundle.py`
- `node demos/html-observatory/tests/security_privacy_adversarial.test.mjs`
- `node demos/html-observatory/tests/conversation_sessions.test.mjs`
- `node demos/html-observatory/tests/operator_attention_inbox.test.mjs`
- `git diff --check`

## Failure Semantics

Fail closed on missing dependency terminal truth, security/privacy/adversarial proof failure, overbroad Runtime or sibling scope, stale exact-head review, PR linkage failure, CI failure, or terminal-cache mismatch.

## Handoff

Retain typed evidence before convergence.
