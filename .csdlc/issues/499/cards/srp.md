# Structured Review Prompt

Template: 1.0.0

Issue: 499

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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
    "fix_revision": "git-blake3:c3426c10ddf3c697bf06c7f7322529461e89aaa8:fd940e160cb5cb1196568623808285b5465bd6fa515b316dde7b4e4b7744d33e",
    "route": null
  },
  {
    "id": "review-p3-api-parity-method-coverage",
    "severity": "p3",
    "summary": "Initial review found the API parity validator omitted public inherent methods; fixed by checking 9 public inherent methods in addition to 83 top-level declarations and rerunning the validator successfully.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:c3426c10ddf3c697bf06c7f7322529461e89aaa8:fd940e160cb5cb1196568623808285b5465bd6fa515b316dde7b4e4b7744d33e",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Focused local validation passed; broader workspace integration remains deferred to PR CI.

## Review Result

Revision: Some("git-blake3:c3426c10ddf3c697bf06c7f7322529461e89aaa8:fd940e160cb5cb1196568623808285b5465bd6fa515b316dde7b4e4b7744d33e")

Reviewer: Some("gpt-5.5-subagent:01a03f3a-27bc-76a1-9478-a60c489aaa7b")

Result: pass
