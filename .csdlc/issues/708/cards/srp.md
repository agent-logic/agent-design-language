# Structured Review Prompt

Template: 1.0.0

Issue: 708

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/ingress.rs

## Prompts

- Can any admitted agent reach its first model turn without the active orientation snapshot?
- Does the recorded digest cover the exact delivered bytes rather than a mutable source or global resource?
- Can reload misreport the package delivered to an existing agent?
- Can invalid content replace the last valid active package?
- Does any wording or control path let orientation enlarge authority?
- Is the implementation smaller than a general prompt framework?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was focused on local Runtime assembly/control/ingress multipart behavior and did not perform live provider inference, paid Runtime execution, or external communication tests.
- Current branch head differs from reviewed source revision by generated C-SDLC review-assignment metadata only; v2 publication readiness must verify that metadata-only tail before publication.

## Review Result

Revision: Some("git-blake3:557f3779142bcf26cb3bc8e24bf5483619968112:0c3ec1f95d5d3f2c7a121aaf41e71b1f6410a80b1ff4ddb2f1c6b715218f4615")

Reviewer: Some("codex:issue-708-multipart-exact-head-review-r2")

Result: pass
