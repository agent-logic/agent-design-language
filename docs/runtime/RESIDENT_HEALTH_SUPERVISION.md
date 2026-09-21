# Resident health supervision

Issue #1108 adds non-model supervision of every resident in the Runtime's full
population, including the local shepherd. It does not use the Observatory's
paginated sample as its census. No restart, model substitution, permissions
change or paid recovery inference is performed by this supervisor.

## Detection and response

An independent task checks observed health every five seconds. Provider failure,
stale resident observations, unknown inference and inference evidence older than
five minutes have distinct reason codes. Metadata success does not establish
inference success. This is a bound after Runtime observes a problem, not a promise
to detect an external outage before the provider reports it.

Each resident has at most one open incident. Incidents survive binding replacement;
old response completions cannot update the successor binding. Removal retires the
incident without losing its pending alert. Re-admission can create a new incident.

The host-local shepherd receives a governed assessment request when ready, with a
30-second response deadline and at most three attempts with increasing delays.
It is asked to acknowledge and assess; its prose is not executed as a repair plan.
Existing metadata retries and governed local shepherd recovery continue separately.
A failed or absent shepherd never prevents independent incident creation or SNS
escalation. All incidents escalate after 30 seconds even when response slots are
busy. Verification-only incidents (unknown or aged inference) do not dispatch model
responses; they escalate for operator attention without recurring synthetic
inference. Confirmed failures and explicit help requests can dispatch responses.

Recovery requires current-binding successful inference observed after the incident
opened, along with fresh healthy observations. An acknowledgment or model-list
response cannot resolve an incident. Unknown evidence is a verification request,
not a claim that the provider is down. No model is guaranteed never to fail.

The primary shepherd executes only through loopback Ollama. Before inference,
Runtime checks the selected `/api/tags` entry and rejects `remote_host` or
`remote_model`. This uses the trusted local Ollama server's metadata; an arbitrary
proxy falsely claiming to be Ollama is outside that trust boundary. See the
[upstream API schema](https://github.com/ollama/ollama/blob/main/docs/openapi.yaml).

## Asking for help

An authenticated operator can request help for a known resident:

```sh
curl --fail --cacert "$RUNTIME_CA" \
  --header "Authorization: Bearer $RUNTIME_WRITE_TOKEN" \
  --request POST "$RUNTIME_API/v1/agents/beacon/help"
```

`RUNTIME_API` is the configured HTTPS Runtime endpoint, `RUNTIME_CA` its approved
trust certificate and `RUNTIME_WRITE_TOKEN` the existing Runtime write credential.
Do not place credential values in issue records or shell tracing output.

A resident may return this structured action through its existing governed
conversation execution:

```json
{"schema":"adl.runtime.provider_agent_action.v1","message":"I need operator help.","action":{"request_help":true}}
```

The requester is the admitted conversation recipient. Model-supplied identity or
freeform diagnostic fields cannot choose another resident or enter an alert.
Repeated requests while an incident is open are deduplicated.

## Restricted SNS transport

Configure a dedicated AWS role profile in the Runtime service account's AWS
configuration. It must resolve to the restricted shepherd role, not the business
administrator. The business administrator may provision the role; it is not a
Runtime alert publisher credential. Example profile:

```ini
[profile agent-logic-shepherd]
role_arn = arn:aws:iam::<business-account>:role/<shepherd-role>
source_profile = <approved-assume-role-source>
region = us-west-2
```

At authorized deployment, add this block to the Runtime init file:

```toml
[resident_alerts]
profile = "agent-logic-shepherd"
region = "us-west-2"
topic_arn = "arn:aws:sns:us-west-2:<business-account>:<runtime-health-topic>"
role_arn = "arn:aws:iam::<business-account>:role/<shepherd-role>"
```

The AWS CLI must be available to the Runtime service. The publisher validates the
exact assumed-role identity/account and topic region before publishing. Ambient
access-key environment variables and custom endpoint overrides are not used.
Each AWS subprocess has a ten-second timeout and is killed when cancelled.

`infra/aws/csm-runtime-health/shepherd.tf` declares the separate role: scoped
CloudWatch log reads, regional metric/alarm reads and `sns:Publish` to the single
health topic. It grants no SQS, IAM, SSM or CloudWatch write permissions. SNS alone
may send to the encrypted 14-day dedicated SQS queue, restricted by source topic
and account. The existing email subscription remains an SNS subscription.
Existing manually provisioned resources must be imported into the intended
Terraform state before applying; this change does not deploy or overwrite them.

## Durable evidence and delivery semantics

The Runtime operation-state directory contains `resident-health.json`, atomically
written and synchronized before any response or delivery reservation. Startup
refuses a corrupt journal. Write failure produces a redacted error event and does
not dispatch an unrecorded action. The journal contains structured incident facts,
not prompts, responses, credentials or provider error bodies.

SNS failures retry with exponential delay capped at five minutes. Missing alert
configuration leaves delivery visibly pending/failed; it never reports success.
`alert_delivered` means SNS returned a MessageId, not that email was read or SQS
consumed. Delivery is at least once: a crash after SNS accepts a message but before
local receipt persistence can duplicate the same stable incident ID. Consumers
must deduplicate by incident ID and revision. Material worsening (for example,
unverified inference becoming a confirmed failure) updates the reason and queues
a new revision even if the earlier alert was accepted. Pending notifications survive retirement and recovery.

`/v1/observatory` exposes `resident_incidents` from the same journal used by the
workers. Agent cards show reason, response, incident state, SNS acceptance/failure
and next alert retry. The authenticated help endpoint uses existing Runtime write
authority. Machine-readable responses stay on stdout/HTTP; redacted operational
failures use the existing tracing pipeline. No live service restart or deployment
is part of the implementation proof.

## Validation classification

PVF lane: runtime. Proof role: deterministic production-path regression and local
contract validation. Resources: local CPU, disposable filesystem, loopback provider
fixtures and mock AWS executable; no paid provider or AWS calls. Release gate:
required Runtime tests/CI. Coverage includes census, independent escalation,
unknown/stale inference, replacement fencing, retry exhaustion, restart recovery,
retired outbox entries, authenticated help, structured action projection, exact
role/account and SNS receipt handling. Observatory uses the existing JavaScript
projection test. Installed Runtime qualification is separate and pending authorized
deployment; a prior operator-confirmed SNS email test is transport evidence only.
