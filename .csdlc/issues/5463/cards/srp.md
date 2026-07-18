# Structured Review Prompt

Template: 1.0.0

Issue: 5463

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.github/workflows/aws-codefriend-build.yaml
.github/workflows/aws-spot-remote-validation.yaml
.github/workflows/ci.yaml
.github/workflows/nightly-coverage-ratchet.yaml
.github/workflows/v0871_milestone_closeout_gate.yaml
adl/tools/test_ci_runtime_contracts.sh
adl/tools/test_ci_path_policy.sh
docs/tooling/GITHUB_ACTIONS_RUNTIME_PIN_INVENTORY.md

## Prompts

- Are all annotated occurrences replaced?
- Do major upgrades preserve used inputs and outputs?
- Does the static contract reject deprecated or floating pins?
- Are hosted annotations genuinely absent?

## Findings

[
  {
    "id": "F-5463-1",
    "severity": "p2",
    "summary": "Valid quoted YAML uses scalars bypass canonical and deprecated action pin enforcement.",
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

Revision: Some("git-blake3:9c1b1d30100bc92a05b7e0175314a5c132d3c051:840099b7166db6f3b804101949eccd6e40e0ea62e17b9a3fc25fccf76e27ea96")

Reviewer: Some("bounded-subagent-review-5463")

Result: changes_required
