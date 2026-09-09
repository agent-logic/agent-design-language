# CSM Runtime Spot host

This root creates one disposable, **publicly addressed** Spot EC2 instance for
Runtime testing, separate from the ALB and public edge stacks. It requires an
independent SSH recovery path even if user data, TLS, Runtime, observability, or
optional SSM startup fails. Use the separate `infra/aws/runtime/private-node`
root for private-only deployments; this change does not require public SSH there.

The stack does not create EC2 keys or bake in Runtime secrets. Select exactly one
existing operator-approved key pair whose private key you can use. Keep private
keys and secret values outside Terraform inputs, plans, and tracked files.

## Fast path

1. Copy `terraform.tfvars.example` into a local ignored var file. Replace the
   VPC/subnet placeholders, `key_name`, and documentation-only SSH address with
   your current public IPv4 `/32`. Verify that the selected key exists in the
   selected region and that its matching private key is available to your SSH
   client. Reuse this one key; do not create a second recovery key.
2. Choose application callers separately: set `alb_security_group_id` to the
   intended ALB, or set `operator_ingress_cidrs` to a narrow direct-smoke `/32`.
   SSH CIDRs permit only TCP/22 and never implicitly grant Runtime-port access.
3. With `AWS_PROFILE=agent-logic-admin`, verify the business account, initialize,
   and inspect a saved Terraform plan. A null/blank key or empty SSH CIDRs fails
   the instance preconditions during planning. Invalid IPv4 CIDRs and `/0` SSH
   ingress fail variable validation. `terraform validate` alone is not proof
   that supplied recovery inputs pass these plan-time conditions.
4. Apply only after approval of the exact cloud plan and bounded spend. Connect
   over SSH as `ec2-user` from the authorized `/32` before relying on Runtime or
   SSM. Then bootstrap Runtime on `0.0.0.0:20997` with the appropriate TLS
   certificate and attach the instance to the ALB if used.
5. Destroy the disposable stack after the bounded proof. Retain the exact
   instance, root-volume and security-group IDs and verify their absence after
   disposal. Do not delete the reused key pair, subnet, VPC or unrelated resources.

IMDSv2 remains required, root EBS remains encrypted and deletes on termination,
and application ingress remains limited to its separately declared callers.

## Focused local proof

Run from the issue checkout:

```sh
bash infra/aws/csm-runtime-spot/tests/run_contract.sh
```

The wrapper initializes the pinned provider, checks formatting and configuration,
and runs mocked plan tests. Provider installation may use the network; the tests
make no AWS calls and create no cloud resources. Terraform **1.7 or newer** is
required for [provider mocking](https://developer.hashicorp.com/terraform/language/tests/mocking).
Normal deployment retains the root's existing Terraform version requirement.

PVF lane: `public-ssh-contract`; deterministic mocked plan contracts, small local
CPU, not a release gate. Cases cover null/blank key, null/empty SSH list, invalid
and world-open CIDRs, a valid one-key `/32` public node, root input forwarding,
application caller isolation, IMDSv2 and encrypted disposable storage.

## Bounded live proof

Local mock tests do not prove network reachability. Before a live run, record the
approved account/profile, region, exact saved-plan digest, one existing key name,
current operator `/32`, selected subnet, instance type, maximum duration and cost
budget. Keep the private-key file path local. Coordinate with other paid AWS work.

For a minimal recovery/isolation proof, set both `operator_ingress_cidrs = []`
and `alb_security_group_id = null`. After the approved apply:

- Retain the instance/public address, root volume and security-group IDs.
- Verify an authenticated SSH command succeeds from the authorized `/32`.
- Through SSH, start a temporary health responder on the configured Runtime port
  and verify localhost success. Verify the same port is unreachable externally
  from the SSH-authorized address. This proves ingress separation rather than
  merely observing a port with no listener; it does not claim Runtime/TLS proof.
- Read back TCP/22 `/32` ingress, absence of application ingress, required IMDSv2,
  and root-volume encryption from AWS. Preserve sanitized observations.
- Stop the responder, destroy only this disposable stack, and retain terminated
  instance / absent root-volume / absent security-group readbacks. A failed probe
  must still lead to disposal; never treat failed SSH as permission to broaden CIDRs.

Live reachability and disposal evidence must be retained before claiming #770
complete. The initial implementation contains local proof only; no paid live run
is implied by the example or mocked tests.
