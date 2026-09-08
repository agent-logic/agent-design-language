# Issue 728 design

Issue: #728
Title: [v0.92.1][AWS-F-R] Prove disposable Runtime deployment and zero residue

## Intent

#728 owns one bounded AWS-F live proof run. It does not introduce permanent
public-edge authority. The proof must create a disposable private Runtime node,
attach it to the AWS-F ALB-origin path, prove an external request reaches that
exact target, then destroy all issue-owned disposable resources and retain
zero-residue readbacks.

## Authority boundary

- C-SDLC v2 remains lifecycle authority.
- AWS execution uses the Agent Logic business profile `agent-logic-admin`.
- Public exposure, Route53, ACM issuance, CloudFront, WAF, and WSS policy
  remain #122-owned and are consumed only as approved inputs.
- Terminal #489 and #579 module truth is consumed read-only.
- #516 receives only a proof-reference update through its own owner after #728
  has immutable evidence.
- No production traffic, cutover, GPU qualification, or long-running Runtime is
  owned by this issue.

## Disposable proof sequence

1. Preflight read-only state:
   - verify GitHub issue dependencies and current-main ancestry for the merged
     AWS-F module PRs;
   - verify AWS profile, account, and region without printing credentials;
   - identify the exact Terraform roots, module revisions, backend keys,
     workspaces, VPC/subnets, certificate ARN, route, AMI/artifact, runtime
     port, and health path;
   - prove planned ingress is not direct public Runtime ingress.
2. Save Terraform plans:
   - initialize the ALB-origin and private-node roots with separate encrypted
     locked backend keys and non-default workspaces;
   - write plan files under issue-owned evidence or run directories;
   - record SHA256 digests and reject apply if either plan digest changes.
3. Apply within the authorized envelope only:
   - create one private disposable Runtime node with no public IPv4;
   - create or update only the disposable ALB-origin resources authorized for
     the proof;
   - attach the exact instance to the target group.
4. Prove the route:
   - wait for target health;
   - call the operator-approved non-production external route;
   - require the response status and body/header marker to bind to the exact
     instance/artifact.
5. Destroy in reverse order:
   - detach target;
   - destroy the private-node root;
   - destroy the disposable ALB-origin root when authorized;
   - read back absence for instance, target registration, security groups,
     load balancer, target group, listener, and other issue-owned selectors.

## Failure policy

The runner fails closed if account, region, state key, workspace, route,
certificate, subnet, VPC, AMI, saved-plan digest, deadline, cost ceiling, target
identity, or destroy selectors are missing or inconsistent. Any residue becomes
a release blocker with exact selectors. Static validation or ALB-only health
never counts as the live proof.

## Validation

- `.csdlc/prepared/issues/728/validate_preparation_bundle.sh` proves the local
  lifecycle/design/runner packet has the issue-owned executable denominator and
  required fail-closed markers.
- `docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh` is the
  live runner entrypoint. Before operator authorization it must refuse to apply
  and print the exact missing authorization/config gate.
- The final evidence must include the exact commands, digests, route receipt,
  target health, reverse destroy, and zero-residue readbacks without secrets.
