# Structured Review Prompt

Template: 1.0.0

Issue: 663

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Issue #663 GCP warm two-node Terraform, startup scripts, focused tests, documentation, and issue-local evidence only.

## Prompts

- Does the change reuse the existing GCP and AWS designs without creating unnecessary abstractions?
- Can ordinary workload teardown delete a source snapshot, or leave a restored Persistent Disk accruing idle cost?
- Can normal startup perform any build, package installation, Git access, or model download?
- Are private networking, SSH/OS Login authority, artifact identity, and timing denominators explicit?
- Do local tests avoid claiming unmeasured live GCP startup performance?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
