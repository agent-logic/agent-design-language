# Structured Review Prompt

Template: 1.0.0

Issue: 608

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/608/recover-review-after-metadata-commit.json
.csdlc/issues/608
.csdlc/prepared/issues/608

## Prompts

- Does global location use the first-party global Vertex endpoint without requiring a custom override?
- Are regional endpoint behavior and trust policy preserved?
- Are thinking controls config-backed, mutually constrained, and tested?
- Does live proof avoid credential exposure and exclude Polis integration?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final integration gate before merge.
- The metadata-only review did not rerun provider or live cloud proof; it verified no product changes since the reviewed implementation head.

## Review Result

Revision: Some("git-blake3:d46604ada170f7cef9bbf8d971e56005c13a52f6:241640489af8523d82c931cdae6a2d5df87b29f45a9c591f1ddfb1b773d543b8")

Reviewer: Some("fresh-session:909271de-340e-45cf-8218-08759e98eab9")

Result: pass
