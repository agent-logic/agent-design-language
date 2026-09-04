# Structured Task Prompt

Template: 1.0.0

Issue: 679

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare and validate static hosting infrastructure readiness for observatory.csm.agent-logic.ai while preserving #512 as the product bundle owner and keeping live AWS actions separately authorized.

## Deliverables

- static bundle deployment contract
- S3 CloudFront ACM Route53 deployment-plan or Terraform surface
- CSP response-header and exact-origin CORS/WSS policy
- redacted AWS business-account readback scripts
- rollback and invalidation plan
- local validators for deployability, redaction, and no-live-mutation default

## Acceptance

1. AC-1: static Observatory assets use relative asset and config paths suitable for S3/CloudFront hosting
2. AC-2: S3 CloudFront ACM and Route53 plan names observatory.csm.agent-logic.ai and separates static hosting from Runtime API/WSS infrastructure
3. AC-3: exact-origin Runtime HTTPS/WSS compatibility is specified without embedding credentials or bearer tokens
4. AC-4: CSP and response-header policy is explicit and compatible with the static bundle and Runtime connections
5. AC-5: S3 versioning object ownership public access block OAC cache invalidation and rollback behavior are included
6. AC-6: AWS readback scripts fail closed unless agent-logic-admin or an approved equivalent is selected and retain redacted evidence only
7. AC-7: local validation proves no credentials or secrets are committed in bundle config URLs local persistence examples or evidence
8. AC-8: live AWS mutation/readback is either separately authorized and retained or truthfully deferred
9. AC-9: exact-head review has no unresolved actionable findings before publication

## Dependencies

- #512 supplies product Observatory bundle and multi-polis behavior
- operator authorization before live AWS mutation or paid/cloud deployment
- Agent Logic business AWS profile agent-logic-admin for authorized AWS readbacks

## Inputs

- agent-logic/agent-design-language#679
- agent-logic/agent-design-language#512
- demos/html-observatory/**
- demos/html-observatory/runtime-v3.config.json
- docs/api/runtime-v3/v1/observatory.openapi.json
- agent-logic/agent-design-language#122
- infra/aws/**
- docs/operations/cloud/aws/**

## Non Goals

- implement #512 product UI or multi-polis behavior
- perform live AWS mutation without explicit operator authorization
- own Runtime API/WSS server implementation or provider workloads
- embed secrets credentials tokens account IDs or private keys in committed artifacts
- reopen or depend on closed #122 as a lifecycle blocker
