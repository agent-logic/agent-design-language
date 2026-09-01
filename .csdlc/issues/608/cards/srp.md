# Structured Review Prompt

Template: 1.0.0

Issue: 608

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/608
.csdlc/prepared/issues/608
.git/csdlc-v2/requests/issue608-publish-reviewed.json

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
- The post-publication metadata review did not rerun provider or live cloud proof; it verified no product changes since the reviewed implementation head.

## Review Result

Revision: Some("git-blake3:d781fbe6f4ab63dafd9b4d0174728090d69526e1:3b74b1242841b5c3846a39841afa1a88074f6ea38b952ad5f6ed308e6745e3c9")

Reviewer: Some("fresh-session:32e6d0e1-5d2e-42eb-8763-8daf3f799f35")

Result: pass
