# Structured Task Prompt

Template: 1.0.0

Issue: 194

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #194 only; final prerequisite for #142 integration/publication, without implementing #142 itself or public production ingress.

## Deliverables

- CloudFormation private network template with no IGW/NAT/public subnet route and endpoint-only maintenance/artifact paths
- Fail-closed runner with create/preflight/launch/smoke/delete/assert-zero actions and signal-safe cleanup contract
- Local tests for template invariants and preflight denial/fault paths
- Redacted live summary binding two-voter private mesh and single-GPU private model health receipts
- Truthful SOR/SRP/review/PR state before publication

## Acceptance

1. AC-1: No cloud spend starts without exact merged prerequisites, clean Wuji receipt or explicit non-claim, and approved Agent Logic account/role identity.
2. AC-2: Two AWS voters launch in distinct AZs with no public Runtime/model/Observatory exposure and unique non-exported identities.
3. AC-3: Every create is idempotent/discoverable after ambiguous failure, and cleanup proves zero instances, volumes, ENIs, security groups, sessions, credentials, processes, locks, and model caches where applicable.
4. AC-4: The serial hybrid run proves private model health/restart, snapshot recovery, Wuji partition, AWS continuity, heal/demotion, and true one-of-three halt.
5. AC-5: Receipts bind live account/AZ/topology/route/model/process/authority evidence without secrets or machine-local paths.
6. AC-6: Harness tests, dry-run fault injection, live proof, independent exact-head review, and publication pass.

## Dependencies

- WP-04.16a-c merged before publication credit
- Agent Logic AWS profile agent-logic-admin available outside repository
- Wuji voter receipt required before full serial hybrid recovery proof
- AWS GPU quota currently prevents simultaneous two-g6.xlarge voter model-health proof

## Inputs

- GitHub issue agent-logic/agent-design-language#194
- .csdlc/prepared/issues/194/design.md
- .csdlc/prepared/issues/194/diagram.mmd
- .csdlc/evidence/194/private-wuji-aws-recovery-live-summary.redacted.json

## Non Goals

- Implementing consensus internals
- Implementing authority policy or kernel continuity internals beyond the harness boundary
- Permanent infrastructure
- Public production ingress stack
- Merging #142 or claiming #142 completion
- Observatory scope beyond avoiding public exposure and consuming required terminal evidence
