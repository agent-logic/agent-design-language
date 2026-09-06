# Structured Task Prompt

Template: 1.0.0

Issue: 624

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Produce the #624 public hardening register, machine-readable sidecar receipt, focused validator, validation evidence, and review handoff.

## Deliverables

- Redacted corporate operational hardening register
- Machine-readable #624 hardening receipt
- Focused sidecar validator
- Truthful SOR/SRP lifecycle records

## Acceptance

1. AC-1: The sidecar record defines the full GitHub/CI, DNS/certificate, AWS guardrail, private custody, and deployment rollback denominator separately from #497 acceptance
2. AC-2: Each denominator row is proven with retained readback evidence or decomposed into a narrower follow-on with owner role, action, authority gate, and closeout condition
3. AC-3: The final evidence distinguishes corporate IP ownership acceptance from operational control-plane hardening
4. AC-4: No credentials, account IDs, secrets, private custody artifacts, recovery details, or sensitive identifiers are printed or committed
5. AC-5: No live cloud, DNS, certificate, GitHub administration, billing, custody, workflow, or production mutation is performed without explicit scoped authorization
6. AC-6: Focused validation proves denominator completeness, row disposition, evidence references, secret hygiene, and diff hygiene
7. AC-7: Bounded independent review has no unresolved actionable finding before publication

## Dependencies

- #497 closed by PR #613
- #634 merged sidecar scope correction
- #498 / PR #637 corporate diligence remains separate and does not complete #624

## Inputs

- agent-logic/agent-design-language#624
- agent-logic/agent-design-language#497
- agent-logic/agent-design-language#613
- agent-logic/agent-design-language#634
- docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.md
- docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.v1.json
- docs/milestones/v0.92.1/evidence/corporate/corp-c/control-plane-denominator.v1.json
- docs/milestones/v0.92.1/evidence/corporate/corp-c/github-ci-authority-readback.v1.json
- docs/milestones/v0.92.1/evidence/corporate/corp-c/dns-cert-deployment-readback.v1.json
- docs/milestones/v0.92.1/evidence/corporate/corp-c/aws-account-control-readback.v1.json

## Non Goals

- Reopen #497
- Replace counsel or infer private legal conclusions
- Perform live account or infrastructure mutations
- Commit private custody material
- Claim operational hardening rows are fixed when only follow-on routing exists
