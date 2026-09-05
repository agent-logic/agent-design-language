# Structured Intent Prompt

Template: 1.0.0

Issue: 679

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prepare the S3/CloudFront/ACM/Route53 static-hosting sidecar for the #512 Observatory at observatory.csm.agent-logic.ai.

## Required Outcome

A reviewed and locally validated deployment-ready AWS static-hosting contract exists for the Observatory, including CSP/CORS/WSS compatibility, redacted readbacks, rollback behavior, and truthful live-apply deferral unless the operator separately authorizes AWS mutation.

## Scope

- infra/aws/observatory/**
- docs/operations/cloud/aws/observatory/**
- docs/milestones/v0.92.1/evidence/observatory/s3-deployable-observatory/**
- .csdlc/prepared/issues/679/**
- .csdlc/issues/679/**
- .csdlc/evidence/679/**
- demos/html-observatory/**

## Authority

- #679 owns static AWS deployability, not #512 product UI implementation
- #512 supplies the static Observatory bundle and multi-polis behavior
- live AWS mutation or paid deployment requires explicit operator authorization
- AWS execution/readback must use the Agent Logic business profile agent-logic-admin
- closed #122 is historical context only, not a blocking lifecycle route
- no credentials, tokens, private keys, or recovery material may be committed or retained in evidence

## Assumptions

- none

## Operator Constraints

- do not write implementation on main
- do not merge without authority
- do not mutate live AWS unless explicitly authorized
- do not weaken TLS, CORS, CSP, or WSS origin controls
- do not absorb #512 product work from Claude
