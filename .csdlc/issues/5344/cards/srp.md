# Structured Review Prompt

Template: 1.0.0

Issue: 5344

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5344
.csdlc/prepared/issues/5344
adl/tools/ci_path_policy.sh
adl/tools/test_ci_path_policy.sh

## Prompts

- Does the tracked-path scan consume Git's NUL-delimited literal filenames without reintroducing line-oriented parsing?
- Do UTF-8 paths, spaces, and embedded newlines preserve complete path components during Windows portability validation?
- Do genuine Windows-illegal characters, trailing spaces or dots, backslashes, and reserved device names still fail closed?
- Does the focused regression mirror the PR failure by retaining a portable UTF-8 baseline path while proving ordinary and newline-hidden illegal components are rejected?
- Are the recovery, focused validation, and exact-head review records scoped truthfully to the two CI path-policy files?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:b493b89307aece6c1bb348a8741a8f0d5c3e6b90:493f594e6b2505ef40187fd704aea2033f7df248386a955196e26d10cc0ffe2a")

Reviewer: Some("subagent:019fac6c-4d03-74e3-90a2-3c3f07ed609d")

Result: pass
