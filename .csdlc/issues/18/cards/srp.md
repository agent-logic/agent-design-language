# Structured Review Prompt

Template: 1.0.0

Issue: 18

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.

## Prompts

- Does the helper normalize only BrokenPipe?
- Do both split binaries use the helper for every machine-readable output path?
- Do regression tests prove process-level behavior without brittle shell dependencies?
- Are exit codes and stdout/stderr separation preserved for real errors?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The shared helper is adopted only by the two split GitHub binaries bounded by issue #18; unrelated C-SDLC binaries retain legacy direct-print output paths.
- The process regression relies on the large schema payload exceeding normal pipe buffering, while the direct unit test independently proves BrokenPipe classification.

## Review Result

Revision: Some("git-blake3:b50356cd00ab1645dcd825a11b4ed7688d8687c5:f2e18ea4d00fbedb28d18b06683ec6b386274071e9f1188412b2d28fd31be3fc")

Reviewer: Some("subagent:review_issue_18")

Result: pass
