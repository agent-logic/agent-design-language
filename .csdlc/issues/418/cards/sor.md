# Structured Output Record

Template: 1.0.0

Issue: 418

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a narrow, fail-closed raw-gh break-glass policy with executable append-only receipt and reconciliation safeguards.

## Artifacts

- AGENTS.md
- docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md
- docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
- csdlc-v2/tests/gate_github_route_policy.rs
- .csdlc/prepared/issues/418/validate_gh_breakglass_policy.sh

## Execution

- Preserved typed C-SDLC v2 as default and final lifecycle authority while allowing six exact raw-gh transport forms only after a confirmed regression and explicit operation-scoped authorization.
- Required exact repository, target, branch, HEAD, typed generation and digest, redacted intent/result/reconciliation receipts, and a lifecycle freeze until typed reconciliation succeeds.
- Added executable positive and negative policy fixtures covering argv, body-file confinement, create-new event files, symlink rejection, overwrite rejection, and the no-body PR-ready route.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/418/validate_gh_breakglass_policy.sh"
    ],
    "purpose": "Validate the fail-closed break-glass policy contract.",
    "outcome": "passed",
    "evidence_ref": "gh-breakglass-policy-contract.log"
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
