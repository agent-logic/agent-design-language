# Structured Intent Prompt

Template: 1.0.0

Issue: 493

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one private GCP platform foundation for disposable workloads.

## Required Outcome

One private GCP platform foundation providing approved operator access, storage classes, telemetry, and zero-residue disposal.

## Scope

- Private GCP VPC and subnet foundation for disposable non-GPU workloads
- IAP and OS Login operator access posture without static SSH-key acceptance
- Separate human and workload identities
- Separate state, artifact, model, continuity evidence, and log storage ownership
- Logging, metric, label, deadline, watchdog, and zero-residue cleanup proof surfaces

## Authority

- Consume #492 GCP-C organization/billing baseline as terminal dependency
- Do not implement GCP-E GPU qualification
- Do not carry production traffic
- Do not perform Shared VPC expansion
- Do not read, print, copy, retain, or commit credential material
- Do not create static service-account keys as acceptance evidence

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Use standard runners only for hosted CI
- Preserve primary main cleanliness
- Avoid paid/cloud mutation unless the issue-specific proof explicitly authorizes it
- Keep #493 scoped to GCP-D private platform foundation truth
