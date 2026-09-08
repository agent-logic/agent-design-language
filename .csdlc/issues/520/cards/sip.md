# Structured Intent Prompt

Template: 1.0.0

Issue: 520

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one findings-first internal review register for the exact final v0.92.1 milestone candidate.

## Required Outcome

A complete review denominator and canonical finding register expose every gap, partial implementation, inert path, unsupported claim, and release blocker at one exact candidate revision.

## Scope

- docs/milestones/v0.92.1/evidence/release/tail-04/**
- .csdlc/issues/520/**
- .csdlc/prepared/issues/520/**

## Authority

- Issue #520 owns review artifacts only
- TAIL-03/#519 supplies the immutable candidate
- Product remediation belongs to TAIL-06/#522
- External review belongs to TAIL-05/#521

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Do not execute before #519 has a reviewed green merge
- Review complete denominators rather than samples
- Do not merge, deploy, restart Runtime, or spend provider/cloud funds
