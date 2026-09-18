# Invited beta hosting and idle shutdown

Tracked implementation: [#1077](https://github.com/agent-logic/agent-design-language/issues/1077).

The operator selected `beta.codefriend.ai` in the Agent Logic AWS account, with
manual AWS start and automatic stop after 30 minutes without meaningful activity
and without a running review. The gateway, website and host must agree that it is
safe to stop. This implementation is in progress: no installed idle controller or
live start/stop acceptance is established yet.

## Infrastructure preparation

The Terraform module is `tools/codefriend_host/terraform`. It creates an Ubuntu
24.04 amd64 `t3.small`, a 32 GiB encrypted persistent root volume, a stable IPv4
address, an HTTP/HTTPS security group and an SSM instance role/profile. Raw app
ports and SSH are not exposed. IMDSv2 is required. CPU credits use standard mode.
Instance-initiated shutdown stops compute; instance destruction is guarded and
the root volume is retained on termination. Storage and address costs continue
while compute is stopped.

Use the `agent-logic-admin` profile and verify its STS identity before preparing a
plan. Supply these reviewed values in a private file outside tracked paths:

- `company_account_id`: the approved business account returned by STS.
- `vpc_id` and `subnet_id`: a company public subnet with Internet gateway routing
  in `us-west-2`; the module verifies that the subnet belongs to the VPC.
- `ubuntu_ami_id`: an exact reviewed, available Canonical Ubuntu 24.04 amd64 image.
  The module checks publisher, image family and architecture; it does not select
  a changing latest image during apply.

Run from the bound issue worktree, with the private variable file under the
issue evidence directory:

```sh
terraform -chdir=tools/codefriend_host/terraform init -backend=false -input=false
terraform -chdir=tools/codefriend_host/terraform validate
terraform -chdir=tools/codefriend_host/terraform plan -input=false \
  -var-file=../../../.csdlc/evidence/1077/host.private.tfvars.json \
  -out=../../../.csdlc/evidence/1077/host.tfplan
terraform -chdir=tools/codefriend_host/terraform show \
  ../../../.csdlc/evidence/1077/host.tfplan
```

Retain plans, state and variable files privately. This module contains no DNS,
provider invocation, secret, certificate or application activation. Apply is a
separate explicitly authorized operation after reviewing the fresh plan and
application/shutdown readiness. No apply has been performed by this issue.

## Shutdown protocol being implemented

1. Measure meaningful authenticated activity across both website execution modes.
   Idle agent polling and unauthenticated traffic must not extend the idle timer.
2. Freeze website admissions, then gateway reservations, while allowing existing
   work and bounded observation to finish. Bind the stop attempt to the current
   service instances so that stale observations cannot authorize stopping a newly
   restarted service.
3. Check active workers, durable operations, pending HTTP work and cleanup.
   Uncertain or unreadable state denies shutdown. Preserve existing retention
   promises; a CPU-idle process may still be awaiting a provider or holding an
   unexpired user result.
4. Stop services gracefully under their supervisor while admissions remain
   frozen. Verify the expected services are stopped and retained source payloads
   have been cleaned before invoking the narrow host stop action.
5. Preserve consumed-operation identities across stop/start. Known operations
   may resume observation; a lost or uncertain POST must never be replayed.

The gateway currently exposes in-process `begin_drain`, `resume_admissions` and
`drained_without_payloads` primitives. The drain gate shares the reservation lock,
so a POST that reaches reservation after the gate closes is rejected without
consuming its operation ID. The readiness check runs existing expiry cleanup,
checks worker permits, rejects missing/corrupt operation state and refuses while
work/result payloads remain. These primitives alone cannot authorize host stop:
website admission/activity control, authenticated operator transport, supervisor
coordination and installed Linux proof are still required. They are not exposed
as public HTTP endpoints.

The unresolved retention choice is whether shutdown waits for current deadlines
or a separately approved shorter retention policy applies. Do not silently
change existing expiry promises or delete unexpired results to meet the timer.

## Acceptance still required

Focused gateway tests cover admission freeze/resume, active workers, retained
payloads and uncertain-state rejection. The coupled PVF inventory identifies
these as component proof, not installed shutdown evidence. Remaining proof must
cover cross-service races, stale control messages, service restart, cleanup
failures, idle polling, Linux service shutdown and actual authorized company EC2
stop/manual-start. Real OAuth, provider-backed reviews and both website journeys
remain Sprint 10 integration requirements.
