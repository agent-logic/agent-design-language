# Issue #489 design: AWS-F Runtime platform modules

## Outcome

Issue #489 produces one reviewed AWS Runtime platform-module set for private Runtime deployment infrastructure. It consumes #122 public edge and #488 adoption-register truth, but it does not re-own public Route53/ACM exposure, CloudFormation retirement, production cutover, or Runtime behavior.

The result is a reusable Terraform module/root set for Runtime network attachment, private host/node groups, build/runtime artifact wiring, observability, cost/deadline controls, rollback, and zero-residue disposable deployment proof.

## Dependencies

- #122 public edge is terminal and owns public Route53/ACM/CloudFront/API-Gateway exposure.
- #488 AWS-E adoption register is terminal and proves the durable AWS resource denominator and ownership disposition.

## Owned paths

- `infra/aws/runtime/**`
- `docs/operations/cloud/aws/runtime-platform/**`
- `docs/milestones/v0.92.1/evidence/cloud/aws-f/**`
- `.csdlc/prepared/issues/489/**`
- `.csdlc/evidence/489/**`

## Non-goals

- Production cutover.
- Domain transfer.
- Runtime behavior fork.
- Public edge ownership already covered by #122.
- AWS resource adoption register ownership already covered by #488.
- CloudFormation retirement already covered by #496.
- Cross-cloud abstraction already covered by #495.

## Platform contract

The implemented module set must record:

- Runtime hosts have no direct public ingress;
- public entry and certificate ownership are consumed from #122 rather than recreated;
- durable resource adoption/ownership constraints are consumed from #488;
- shared edge, private network, build/runtime artifact, and independently scalable node-group states remain separated;
- disposable proof deployment binds the exact modules, proves an external request reaches the instance, and then destroys all owned resources with zero residue;
- cost/deadline controls are explicit and use standard runners for hosted CI.

## Validation lanes

Pre-bind lanes are local and non-mutating:

- `prebind-aws-runtime-platform-packet` checks design, dependency inputs, owned paths, and non-goal boundaries.
- `prebind-aws-runtime-platform-static` checks the readback/proof entrypoint without credentials or cloud mutation.

Post-bind lanes are deferred until implementation exists:

- `aws-f-runtime-platform-static` validates Terraform/docs/evidence contract, private-ingress posture, public-edge consumption, module separation, cost/deadline, and zero-residue selectors.
- `aws-f-saved-plan` records a reviewed saved plan without production traffic.
- `aws-f-disposable-path-proof` performs the approved disposable deployment/path/cleanup proof when cloud execution is authorized.
- `exact-head-review` records a fresh implementation review before publication.

## Stop conditions

Stop before bind, implementation, or cloud execution if:

- #122 or #488 terminal/ancestry truth is missing;
- public ingress appears on Runtime hosts unexpectedly;
- production traffic would be required;
- a plan would mutate adopted durable resources outside #488 disposition;
- CloudFormation retirement or cross-cloud abstraction is required to proceed;
- credentials or sensitive identifiers would be printed, copied, retained, or committed.
