# Structured Review Prompt

Template: 1.0.0

Issue: 663

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/663
.csdlc/evidence/663
infra/gcp/workloads/modules/two-node-ollama-runtime
infra/gcp/workloads/warm-polis
docs/operations/cloud/gcp/WARM_POLIS_SNAPSHOT_RUNBOOK.md

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

- The paid live GCP timing and cleanup lane was not run because no exact company project and GCP spend authorization were provided; AC-8 explicitly permits this deferred disposition.

## Review Result

Revision: Some("git-blake3:ce1616a16ac07a08e3dac8adc5c6cc6ee4315836:27c12b9ed17bb70178a2948d820891efc2726aa0a7cd059c817c9616810df40a")

Reviewer: Some("codex-subagent:/root/issue_663_final_review")

Result: pass
