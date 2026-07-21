# Structured Review Prompt

Template: 1.0.0

Issue: 5615

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/5615
.csdlc/issues/5615
.csdlc/prepared/issues/5615
.github/workflows/ci.yaml
adl/config/validation_lane_selector.v0.91.6.json
adl/tools/ci_path_policy.sh
adl/tools/run_cargo_validation.sh
adl/tools/test_ci_path_policy.sh
adl/tools/test_ci_runtime_contracts.sh
adl/tools/test_run_cargo_validation.sh
adl/tools/test_select_validation_lanes.sh

## Prompts

- Can a C-SDLC v2 source/test diff report green if its standalone job is absent or skipped?
- Can metadata-only or C-SDLC-only changes launch ADL workspace or Runtime coverage?
- Do mixed diffs retain every stronger pre-existing proof requirement?
- Does the wrapper fail closed without a declared or writable external root?
- Are stable aggregate names and semantics unchanged?

## Findings

[
  {
    "id": "F-5615-1",
    "severity": "p1",
    "summary": "Selector authority can select standalone C-SDLC v2 proof while the classifier leaves its required boolean false.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5615-2",
    "severity": "p1",
    "summary": "Mixed C-SDLC v2 and Runtime v3 changes suppress the Runtime focused proof because routing matches exact lane strings.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5615-3",
    "severity": "p1",
    "summary": "Pre-created Cargo child symlinks can escape the validated external build root.",
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

Revision: Some("git-blake3:d0a23536ec79b6c2bf0067d7a8ba1b0e184dc925:d7c03dc4da802c7ec404063ddd472a0a78706662540d0b40f9435cebb3a2a05b")

Reviewer: Some("subagent:019f86ae-7d7a-7161-b2e4-67264db9ef0f")

Result: changes_required
