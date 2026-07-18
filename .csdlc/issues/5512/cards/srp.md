# Structured Review Prompt

Template: 1.0.0

Issue: 5512

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/tools/run_pr_fast_coverage_lane.sh
adl/tools/test_run_pr_fast_coverage_lane.sh

## Prompts

- Can a foreign binary selector still reach the ADL workspace?
- Can the detector trigger for an unrelated expression?
- Does the runtime companion retain auth, supervision, and topology coverage?
- Are both summaries still emitted?

## Findings

[
  {
    "id": "F-5512-1",
    "severity": "p2",
    "summary": "Substring bridge detection could silently discard unrelated coverage selectors.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:b3e00aa1d966bdead5c7bd39c442017619086625:3bbd97871d2832a3ab91d55cb480cb7498ca97c87e64224bc6d88c77d6e076de",
    "route": null
  },
  {
    "id": "F-5512-2",
    "severity": "p3",
    "summary": "Zero-valued fake summaries did not prove both coverage inputs were composed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:b3e00aa1d966bdead5c7bd39c442017619086625:3bbd97871d2832a3ab91d55cb480cb7498ca97c87e64224bc6d88c77d6e076de",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final proof that the exact #5504 expression interoperates with the GitHub llvm-cov and nextest toolchain.

## Review Result

Revision: Some("git-blake3:b3e00aa1d966bdead5c7bd39c442017619086625:3bbd97871d2832a3ab91d55cb480cb7498ca97c87e64224bc6d88c77d6e076de")

Reviewer: Some("subagent:019f7532-dfd0-7b52-a750-7df6cce35b42")

Result: pass
