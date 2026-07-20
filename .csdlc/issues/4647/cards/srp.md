# Structured Review Prompt

Template: 1.0.0

Issue: 4647

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/4647
.csdlc/prepared/issues/4647
.csdlc/publication/4647.intent.json
adl/src/csm_api_gateway_bridge.rs

## Prompts

- Does the issue stay within its WP scope?
- Are claims supported by retained or fresh evidence?
- Are skipped and unproven surfaces explicit?
- Are sibling WP and release/activation non-claims preserved?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Exact-head bounded review by subagent 019f7e25-c21b-7ed1-a38d-ef3a838a6114 confirmed HEAD 04bf3d8401a2b8c156546bc64bde065a0342cb34 is CLEAN/fixed-confirmed for the post-publication CSM API Gateway coverage-hosted repair.
- The prior P2 protected-path finding is fixed: `adl/src/csm_api_gateway_bridge.rs` and `.csdlc/publication/4647.intent.json` are both present in protected paths.
- The branch changed-path-vs-protected-path check over `origin/main...HEAD` returned no output, and `csdlc-doctor` passed at generation 34 implemented.
- No AWS operation was run.

## Review Result

Revision: Some("git-blake3:04bf3d8401a2b8c156546bc64bde065a0342cb34:e912b20abd7fc64b42bc767522e512b37ddf4f8cddc3fa2d7d9092e31da49d59")

Reviewer: Some("subagent:019f7e25-c21b-7ed1-a38d-ef3a838a6114")

Result: pass
