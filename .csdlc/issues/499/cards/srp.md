# Structured Review Prompt

Template: 1.0.0

Issue: 499

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/resilience.rs
adl/src/resilience/*.rs
.csdlc/prepared/issues/499/validate-*.rb
publication hygiene for extracted modules and validators

## Prompts

- Does the implementation stay inside the declared unit boundary?
- Does every acceptance criterion have proving evidence?
- Are operator-only actions and private material kept outside Git?

## Findings

[
  {
    "id": "review-p1-untracked-extracted-modules",
    "severity": "p1",
    "summary": "Initial review found extracted resilience modules and validators were untracked and absent from git diff; fixed by staging and committing the full intended diff at c3426c10ddf3c697bf06c7f7322529461e89aaa8.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:39af22446b3d3d7da2b2e6926c48937f4e954fb5:c75a29027e4007c4a7ff7860b226d2367eb3db7c25f4929c1e025de90b35f53b",
    "route": null
  },
  {
    "id": "review-p3-api-parity-method-coverage",
    "severity": "p3",
    "summary": "Initial review found the API parity validator omitted public inherent methods; fixed by checking 9 public inherent methods in addition to 83 top-level declarations and rerunning the validator successfully.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:39af22446b3d3d7da2b2e6926c48937f4e954fb5:c75a29027e4007c4a7ff7860b226d2367eb3db7c25f4929c1e025de90b35f53b",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Focused local validation passed; broader workspace integration remains deferred to PR CI.

## Review Result

Revision: Some("git-blake3:39af22446b3d3d7da2b2e6926c48937f4e954fb5:c75a29027e4007c4a7ff7860b226d2367eb3db7c25f4929c1e025de90b35f53b")

Reviewer: Some("gpt-5.5-subagent:01a03f3a-27bc-76a1-9478-a60c489aaa7b")

Result: pass
