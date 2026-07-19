# Structured Review Prompt

Template: 1.0.0

Issue: 5489

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
docs/milestones/v0.91.8/SPRINT_PLAN_v0.91.8.md
docs/milestones/v0.91.8/WBS_v0.91.8.md
docs/milestones/v0.91.8/WP_EXECUTION_READINESS_v0.91.8.md
docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml

## Prompts

- Does the issue stay within its WP scope?
- Are claims supported by retained or fresh evidence?
- Are skipped and unproven surfaces explicit?
- Are sibling WP and release/activation non-claims preserved?

## Findings

[
  {
    "id": "F-5489-1",
    "severity": "p1",
    "summary": "Cutover/deletion sidecar issues #5343 and #5347 were executable in the plan but omitted from the card-factory missing-projection set.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6062400223e46149b07f623c88a2adc7b041d147:e9dad6931a21e227af49ecf439d4cd5d6a20580e8bee4ffdbf3ce72edf0fe583",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Fable 5 external shadow review was attempted but unavailable after fail-closed runs and is not recorded as approval.
- Live GitHub issue state for v0.91.8 issues was not re-verified during the final exact-revision review.

## Review Result

Revision: Some("git-blake3:6062400223e46149b07f623c88a2adc7b041d147:e9dad6931a21e227af49ecf439d4cd5d6a20580e8bee4ffdbf3ce72edf0fe583")

Reviewer: Some("subagent:019f77fb-5581-7920-9466-dd36bc76999d")

Result: pass
