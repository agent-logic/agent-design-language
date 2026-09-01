# Structured Task Prompt

Template: 1.0.0

Issue: 531

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Execute the Sprint 3 management closeout path after declared roster children have closed, retaining truthful evidence and stopping before any unsupported completion claim.

## Deliverables

- Sprint 3 closeout evidence artifact
- Roster and child disposition summary
- Review/validation record for the sprint-level result
- Typed publication and terminal closeout record when gates pass

## Acceptance

1. The closeout artifact records membership version 4 and all current roster children
2. Every child has live GitHub state and local lifecycle disposition recorded without overclaiming missing evidence
3. Child PR, merge, check, and ancestry evidence is retained or explicitly marked unavailable
4. The sprint result does not absorb child implementation or rerun paid/cloud work
5. Fresh sprint-end review finds no actionable findings before publication
6. Typed v2 publication and finish gates are used for the umbrella if review and validation pass

## Dependencies

- #495 XCL-01 Cross-cloud Runtime Terraform conversion
- #489 AWS-F AWS Runtime platform modules
- #496 AWS-G AWS CloudFormation retirement decision
- #494 GCP-E GCP GPU readiness smoke test

## Inputs

- .csdlc/issues/489
- .csdlc/issues/494
- .csdlc/issues/495
- .csdlc/issues/496
- docs/milestones/v0.92.1/evidence/cloud/**
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Child implementation
- Paid cloud launch or live resource mutation
- Production cutover
- Retiring C-SDLC v2 authority
- Closing or rewriting child issues
