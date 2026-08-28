# Structured Intent Prompt

Template: 1.0.0

Issue: 489

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one reviewed AWS Runtime platform-module set.

## Required Outcome

One reviewed Terraform module set for private Runtime network, edge, build, and independently scalable node groups.

## Scope

- AWS Runtime private network attachment and host or node group module set
- Private ingress posture with no direct public ingress to Runtime hosts
- Consumption of #122 public edge, ACM, Route53, WSS and allowed-origin authority without re-owning it
- Consumption of #488 adoption-register truth without reclassifying durable resources
- Exact Runtime artifact wiring, observability, cost/deadline, rollback, disposable proof, and zero-residue cleanup

## Authority

- Use the Agent Logic AWS business account through the approved AWS profile only for live readbacks or disposable proofs
- Do not print, copy, commit, or expose cloud credentials or sensitive account data
- Do not create production traffic or perform production cutover
- Do not re-own #122 public exposure, Route53, ACM, CloudFront, API Gateway, WAF, WSS, or allowed-origin work
- Do not re-own #488 adoption-register classifications
- Do not implement #496 CloudFormation retirement or #495 cross-cloud conversion

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Use standard runners only for hosted CI
- Preserve primary main cleanliness
- Avoid paid/cloud mutation unless the issue-specific proof explicitly authorizes it
- Keep #489 scoped to AWS-F Runtime platform modules
