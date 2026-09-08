# Structured Intent Prompt

Template: 1.0.0

Issue: 724

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide one simple GitHub-like v3 issue-create command without weakening typed dispatch or authority gates.

## Required Outcome

A documented simple issue-create command that projects operator flags into the existing typed v3 dispatch and preserves all mutation proof.

## Scope

- csdlc-v3/src/main.rs
- csdlc-v3/src/commands/remote/**
- csdlc-v3/tests/**
- docs/csdlc-v3/**
- .csdlc/prepared/issues/724/**
- .csdlc/issues/724/**

## Authority

- Issue authority is agent-logic/agent-design-language#724
- Execution depends on reviewed and merged #721 issue-create parity
- The simple form must use the existing typed dispatch, durable intent, authenticated readback, marker, and receipt path
- The command must fail closed while v3 operational authority is inactive

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Do not invoke raw gh for lifecycle mutation
- Do not bypass typed receipts or credential-child injection
- Do not merge or alter cutover authority
