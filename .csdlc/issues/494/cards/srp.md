# Structured Review Prompt

Template: 1.0.0

Issue: 494

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/494
.csdlc/issues/494
.csdlc/prepared/issues/494/design.md
.csdlc/prepared/issues/494/diagram.mmd
.csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh
docs/milestones/v0.92.1/evidence/cloud/gcp-e
infra/gcp/workloads/gpu-smoke-instance
infra/gcp/workloads/gpu-smoke-support
infra/gcp/workloads/modules/gpu-smoke-instance
infra/gcp/workloads/modules/gpu-smoke-support

## Prompts

- Does the design keep #494 to one bounded GCP-E On-Demand L4 readiness decision without absorbing DRT-D, XCL-01, AWS-G, Observatory, Unity, or production traffic?
- Are paid authorization, USD 20 ceiling, exact inputs, GPU inference/headroom, telemetry/cost/deadline, and zero-resource cleanup represented as machine-checkable proof?
- Does the plan avoid credential disclosure while still retaining enough redacted evidence for review?
- Are quota/capacity, cleanup, and cost failures fail-closed rather than silently accepted?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer did not run live GCP, credentials, paid commands, terraform apply, or cloud readbacks; accepted live evidence remains r8/r9.

## Review Result

Revision: Some("git-blake3:d86bb74e1b58ac869583d6f2233acbfa40fea36e:a0d85e1f320346025e2ee4d8abc0a15eda3678709493094670d46f6febf5f81e")

Reviewer: Some("fresh-session:ced1bc1e-98e3-4cdf-9967-8856ed5b90b1")

Result: pass
