# Structured Intent Prompt

Template: 1.0.0

Issue: 506

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one deterministic distributed-qualification and ACIP replay contract for requirements 181 and 182.

## Required Outcome

Identity, authority, duplicate-denial, and replay-conformance tests pass with exact receipts for requirements 181 and 182.

## Scope

- deterministic distributed qualification contract
- ACIP authority and replay conformance contract
- duplicate-denial and negative-matrix proof
- DRT-A evidence packet

## Authority

- owns adl-runtime qualification contract surfaces only
- consumes #181 and #182 requirement designs as predecessor inputs
- does not own paid AWS execution
- does not own Observatory redesign

## Assumptions

- none

## Operator Constraints

- no paid AWS execution under #506
- no Observatory redesign under #506
- work in a bound FastWork issue worktree after readiness approval
