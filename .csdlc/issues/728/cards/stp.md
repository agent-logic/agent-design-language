# Structured Task Prompt

Template: 1.0.0

Issue: 728

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Execute only the AWS-F-R disposable proof for #728; do not redesign the public edge, permanent networking, certificate issuance, or provider/cloud architecture.

## Deliverables

- preflight and authorization packet capturing account, region, roots, workspaces, state keys, module revisions, inputs, deadline, and cost ceiling
- saved Terraform plan digest evidence for ALB-origin and private-node roots
- bounded live proof evidence for target health and external request receipt
- reverse-destroy and zero-residue readback evidence
- updated AWS runtime platform runbook/evidence where needed
- fresh exact-head review and published PR with Closes #728

## Acceptance

1. AC-1: Preflight verifies agent-logic-admin, exact business account identity, us-west-2, current-main ancestry of PRs #577 and #583, exact module revisions, separate encrypted/locked backend keys and non-default workspaces, approved VPC/subnets/certificate/public-route inputs, and no direct public Runtime ingress.
2. AC-2: Operator authorization names the exact account, region, roots/workspaces/state keys, saved-plan digests, VPC/subnets/certificate/route selectors, instance type, AMI/artifact, runtime port/health path, ingress source, mutation deadline, cost ceiling, and mandatory reverse destroy; no apply occurs without it.
3. AC-3: One disposable private Runtime node is created from the exact module revision with no public IPv4 or direct public ingress and is attached to the exact AWS-F ALB target group; shared edge, network, build, and node state remain separately owned.
4. AC-4: Readback proves the exact target becomes healthy and a bounded external request through the operator-approved non-production route returns the expected status plus an instance/artifact identity marker tying the response to that target.
5. AC-5: Proof records exact source/module revisions, saved-plan digests, state/workspace identities, timing, cost, target-health, HTTP receipt, and production-traffic=false without credentials or sensitive retained values.
6. AC-6: Teardown detaches the target, destroys the private-node root, destroys the disposable ALB root when authorized, and proves the exact instance, target registration, security groups, load balancer/target group/listener, and other issue-owned disposable resources are absent; retained shared resources are enumerated and unchanged.
7. AC-7: A cleanup trap and deadline enforce destroy on failure; any residue is reported as a release blocker with exact selectors and owner rather than claimed clean.
8. AC-8: Fresh exact-head review reports no actionable findings before publication, and #516 is updated through its owner with the immutable proof reference.

## Dependencies

- Terminal #489 / merged PR #577
- Terminal corrective #579 / merged PR #583
- Public exposure authority #122
- Release-tail admission #516
- Approved Agent Logic business-account paid mutation authorization

## Inputs

- agent-logic/agent-design-language#728
- agent-logic/agent-design-language#489
- agent-logic/agent-design-language#579
- agent-logic/agent-design-language#122
- agent-logic/agent-design-language#516
- infra/aws/runtime/alb-origin
- infra/aws/runtime/private-node
- infra/aws/runtime/modules/private-runtime-node
- infra/aws/modules/csm-runtime-alb
- docs/operations/cloud/aws/runtime-platform/README.md

## Non Goals

- Rewrite terminal #489 or #579 truth
- Production traffic or cutover
- Route53 or ACM issuance
- Public-edge redesign
- CloudFormation retirement
- Cross-cloud conversion
- GPU qualification
- Long-running Runtime operation
