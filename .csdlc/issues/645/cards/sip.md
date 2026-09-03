# Structured Intent Prompt

Template: 1.0.0

Issue: 645

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prevent C-SDLC v2 publication from recording terminal closing authority for stacked pull requests when GitHub does not expose the target issue through closingIssuesReferences.

## Required Outcome

Closing-mode csdlc-publish fails closed unless live GitHub PR readback confirms the exact target issue through closingIssuesReferences, and stacked checkpoint publication remains explicitly non-closing.

## Scope

- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/src/github.rs
- csdlc-v2/tests/**
- .csdlc/prepared/issues/645/**
- .csdlc/issues/645/**

## Authority

- Issue #645 owns the C-SDLC v2 publication guard for stacked closing relation readback
- PR #644 is reproducer evidence only and must not be retargeted or merged by this issue
- GitHub closingIssuesReferences is the live authority for terminal closing linkage
- Typed C-SDLC v2 remains the lifecycle authority

## Assumptions

- none

## Operator Constraints

- Do not merge, retarget, or mutate PR #644
- Do not use raw gh as a lifecycle write workaround
- Do not weaken body keyword validation
- Do not write tracked issue work on main
- Do not use AWS, paid provider APIs, big runners, or live provider inference
