# Structured Intent Prompt

Template: 1.0.0

Issue: 494

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one bounded GCP GPU readiness decision under the operator-approved USD 20 ceiling.

## Required Outcome

One bounded On-Demand L4 smoke-test qualification packet with exact inputs, cost, telemetry, GPU inference/headroom proof, and independent zero-resource cleanup.

## Scope

- GCP-E GPU readiness smoke Terraform/workload surfaces under infra/gcp/workloads/gpu-smoke
- One bounded On-Demand L4 VM shape selected from the approved company GCP project/account context
- Exact image, CUDA/driver, Ollama/model, zone, machine, GPU, service-account, label, and deadline inputs retained in evidence
- GPU detection, one small inference smoke, basic headroom telemetry, cost/deadline capture, and independent cleanup proof
- Runbook and issue-owned validator surfaces that make rerun, destroy, and zero-residue verification simple and explicit

## Authority

- Consume GCP-D #493 terminal private-platform foundation truth before paid execution
- Do not perform six-resident distributed Runtime qualification; DRT-D owns that later lane
- Do not absorb XCL-01 #495 cross-cloud Terraform conversion or AWS-G #496 retirement work
- Do not create production deployment, production traffic, DNS/public edge, Observatory, Unity, or provider-profile behavior
- Do not read, print, copy, retain, or commit credential material
- Do not exceed the operator-authorized USD 20 ceiling; stop and destroy if the cap, quota, or cleanup invariant is threatened

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Use standard runners only for hosted CI
- Preserve primary main cleanliness
- Use the already-authorized paid budget only for #494 GCP-E and only up to USD 20
- Keep live GCP evidence redacted and never expose credential contents
