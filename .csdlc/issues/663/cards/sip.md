# Structured Intent Prompt

Template: 1.0.0

Issue: 663

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Add a minimal GCP warm-start implementation for the existing two-node Runtime/Guardian and G2/L4 Ollama topology by adapting the proven AWS #607 snapshot-backed storage design.

## Required Outcome

Terraform supports versioned Runtime and Ollama/model snapshots, disposable Persistent Disks restored per run, prebuilt immutable launch content, and truthful snapshot-to-ready timing without builds, installs, Git access, or model downloads during normal startup.

## Scope

- GCP two-node Runtime and Ollama Terraform module snapshot-restored disk support
- GCP snapshot preparation and warm launch roots
- deterministic mount, artifact identity, readiness, and timing contracts
- focused Terraform and shell contract tests
- one separately authorized live GCP stop/start timing proof

## Authority

- typed C-SDLC v2 remains lifecycle authority
- existing #495 and #509 GCP qualification truth remains unchanged
- existing #607 AWS implementation remains unchanged
- live GCP mutation requires explicit project and spend authorization
- ordinary GCP stop/start cannot preserve GPU VRAM

## Assumptions

- none

## Operator Constraints

- reuse the AWS #607 design and existing GCP modules while making GCP snapshots the inexpensive idle-state authority
- keep the implementation simple and focused
- normal startup must never compile, install packages, access Git, or download models
- use one existing SSH and OS Login authority
- do not run paid GCP resources without explicit budget authorization
- work only in the bound FastWork worktree
