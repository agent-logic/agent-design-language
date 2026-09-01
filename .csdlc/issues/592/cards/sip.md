# Structured Intent Prompt

Template: 1.0.0

Issue: 592

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Configure Polis on GCP to use Vertex AI through the governed Runtime/provider configuration path.

## Required Outcome

One reviewed, validated GCP/Vertex AI Polis configuration path that can be operated without leaking credentials or bypassing C-SDLC lifecycle authority.

## Scope

- docs/runtime/**
- docs/tooling/**
- infra/runtime-v3/**
- adl-runtime-kernel/**
- .csdlc/prepared/issues/592/**
- .csdlc/evidence/592/**
- .csdlc/issues/592/**

## Authority

- Issue authority is agent-logic/agent-design-language#592
- Dependency #528 must be terminal before execution begins
- C-SDLC v2 remains live authority until #505 cutover is explicitly approved
- No raw gh route is permitted for this canary

## Assumptions

- none

## Operator Constraints

- Do not bind or execute while #528 is non-terminal
- Do not print copy commit or retain credential material
- Do not perform live paid GCP calls without later explicit authorization
- Keep all canary evidence in the repository/worktree, not /private/tmp
