# Structured Task Prompt

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare now; only after all serial gates may a separately authorized session implement bounded public Observatory and Runtime-gateway exposure.

## Deliverables

- infra/aws/csm-public-edge/versions.tf
- infra/aws/csm-public-edge/variables.tf
- infra/aws/csm-public-edge/locals.tf
- infra/aws/csm-public-edge/main.tf
- infra/aws/csm-public-edge/outputs.tf
- infra/aws/csm-public-edge/terraform.tfvars.example
- infra/aws/csm-public-edge/.terraform.lock.hcl
- infra/aws/csm-public-edge/README.md
- adl/tools/validate_csm_public_edge_static.sh
- adl/tools/validate_csm_public_edge_live.sh
- docs/milestones/post-v0.92/features/CSM_PUBLIC_EDGE_TERRAFORM.md
- .csdlc/prepared/issues/122/terraform-execution-plan.md
- .csdlc/evidence/122/additional-origins-validation.md
- .csdlc/evidence/122/gemini-remediation-review.result.json
- .csdlc/evidence/122/gemini-remediation-review.log

## Acceptance

1. AC-1: Distributed Runtime integration is terminal, merged, ancestral, and independently reviewed before execution
2. AC-2: A separate operator authorization exists and the approved business profile resolves to the Agent Logic business account
3. AC-3: Both canonical hostnames resolve to issue-owned resources with valid ACM chains and ordinary platform trust
4. AC-4: The Observatory and Runtime gateway expose one matching exact revision with no private state or secret material
5. AC-5: CORS, CSP, WSS origins, authentication, signed Layer 8 writes, rate limits, redaction, and health reporting fail closed
6. AC-6: Deployment and rollback are bounded, idempotent, ownership-verifiable, and leave no orphaned issue-owned resources
7. AC-7: No EC2, Spot, or CodeBuild resource is created or operated
8. AC-8: Exact-head security and operations review has no unresolved actionable findings
9. AC-9: #122 remains non-gating for #83 and #111-#117

## Dependencies

- Hard serial gate: distributed Runtime terminal through merged, ancestral, independently reviewed proof
- Separate operator authorization for bounded AWS work
- Approved Agent Logic business profile verified at execution time
- #83 local implementation and independent validation are inputs, not work gated by #122
- #110 supplies product direction; #111-#117 may supply contracts but are not gated by #122

## Inputs

- agent-logic/agent-design-language#83
- agent-logic/agent-design-language#110
- agent-logic/agent-design-language#111
- agent-logic/agent-design-language#112
- agent-logic/agent-design-language#113
- agent-logic/agent-design-language#114
- agent-logic/agent-design-language#115
- agent-logic/agent-design-language#116
- agent-logic/agent-design-language#117

## Non Goals

- Any execution during v0.92 or any gate on #83 or #111-#117
- Product implementation, AWS use, deployment, publication, push, PR, merge, or closeout during preparation
- EC2, Spot, CodeBuild, birthday-specific behavior, or production marketing launch
- Weakening Runtime authorization or exposing private state, secrets, raw provider payloads, or internal topology
