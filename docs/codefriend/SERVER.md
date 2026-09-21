# CodeFriend Beta 1 server and model-access service

Issue #1056 implements the HTTP backend in ADL. The website remains in
`agent-logic/codefriend.ai`; repository extraction is separate v0.93 work.
This service supplies hosted update-cycle execution and restricted model requests for
installed local agents. Issue #1101 extends the original review protocol with
independently selected documentation, Mermaid-diagram and test-proposal activities.
It does not implement website GitHub login or agent pairing.
Those owners are #1057 and #1058. Full journey integration and qualification remain
#914 and #915.

## Deployment boundary

Install with `bash adl/tools/install_owner_binaries.sh --bin codefriend-server`.
Run `.adl/bin/codefriend-server --config <private-config.json> --listen 127.0.0.1:8080`.
Use an issue-local stable bin directory during validation; do not replace a shared
operational installation without coordinating its users.
The listener deliberately requires loopback. Hosted deployment needs an authenticated
website integration and TLS ingress on the same server, private persistent storage,
operator-provisioned credentials and an approved provider model. No deployment or
real-provider acceptance is claimed by the component tests.

The config is the serialized `server::Config`: absolute private `root` with an existing
parent, `credentials_file`, `provider` (the existing `ProviderInvocationRequestV1`
contract), exact compiled `candidate_revision`, `max_concurrent` (1–8),
`max_operations_per_subject` (1–1000) and `retention_seconds` (60–86400).
Provider config permits one attempt, timeout at most 60 seconds and 1–4096 output
tokens. Generated prompts are capped at 128 KiB per lane before any model dispatch.
Response bodies are capped at 4 MiB before HTTP decoding or Bedrock SDK Blob
aggregation, including error replies. Oversized responses fail without a successful
result; token limits alone are not a byte bound.
Do not preload input text. Only the service operator selects routes,
endpoint, model and provider credential references. Provision actual credentials
through approved server-side indirection; never send them to either client.
The service logs fixed event codes to stderr and reserves stdout for machine output.

The build embeds the Git revision and clean status of compilation-relevant source
inputs. The service rejects missing/dirty build provenance and any configured revision
that differs from its compiled identity. Build from a committed source candidate;
Git must be available at build time, but is not needed by a relocated installed binary.
Lifecycle cards and evidence do not affect source cleanliness. Adjacent provenance
JSON or runtime configuration cannot override the embedded identity.

## Identity contract for website and agent owners

The private credentials file is an array of `{token_hash, subject, mode, expires_at}`.
Use cryptographically random tokens with at least 256 bits of entropy. `token_hash`
is lowercase BLAKE3 hex; no bearer token belongs in this file, a URL or a log.
`subject` is the stable invited-user identifier, not client-supplied request content.
`mode` is `hosted` for website review credentials or `local_model` for paired-agent
credentials. `expires_at` is Unix seconds. The trusted website authentication owner
must map validated GitHub identities to invitations before provisioning credentials;
this backend does not accept GitHub identity headers as authority. Pairing must
provision/revoke its own local-model scoped credential. Never expose another user's
credential, shared administrative credentials or provider keys to a browser.

Update the file atomically. It is re-read on each authenticated request, so removal
or expiry immediately denies future requests and artifact access. A previously
admitted provider call may still finish; credential revocation does not undo it.
An empty JSON array is a valid deny-all registry: the service can start before the
first website credential is provisioned and after the last one is revoked. Missing,
malformed, duplicate or over-limit registries remain errors; no default credential
is created. Issue #1074's deterministic runtime regressions cover empty startup,
authentication before body polling, atomic provisioning/revocation and restart,
plus invalid-registry rejection. These bounded local CPU/files/HTTP tests are a
required regression gate, not deployed or paid-provider acceptance.
The TLS ingress must preserve `Authorization` and disable caching. Responses also
carry `Cache-Control: no-store`. There is no CORS wildcard or public signup.

## Versioned operation protocol

`POST /v1/operations`, with `Authorization: Bearer <opaque-token>`, accepts exactly:

- `operation_id`: new stable identifier, ASCII alphanumeric, hyphen or underscore;
- `packet`: existing bounded CodeFriend repository packet, admitted through existing
  privacy, provenance and retention validation;
- `mode`: `hosted` or `local_model`, matching the credential;
- `lane`: null for hosted, or one existing review lane for legacy local-model requests;
- optional `cycle`: `codefriend.update_cycle_plan.v1`, containing the exact repository,
  one or more activities in canonical `review`, `documentation`, `diagrams`, `tests`
  order, and a testing goal only when tests are selected.

Omitting `cycle` preserves the original review-only request and response. A hosted
cycle has a null lane. A paired local agent sends one local-model operation with a
null lane and the complete cycle plan; the durable operation reservation covers all
selected work, so reconnecting observes that operation rather than dispatching it
again. Invalid plans are rejected before an operation ID is reserved.

The service never accepts a client path, arbitrary prompt, provider route or identity.
The local-model gateway constructs the existing lane prompt from admitted evidence,
executes the configured provider, and validates returned findings with the existing
lane and finding validators. The agent retains orchestration/review execution; selected
evidence is sent to Agent Logic and the configured model provider. This is not an
all-local or zero-egress promise.

Authentication runs before any request-body consumption or JSON parsing. Missing
or invalid credentials return 401 even for malformed or oversized request bodies.

The response is HTTP 202 with the operation identity, request digest, packet/source
and candidate identities, expiry and status. Poll `GET /v1/operations/<id>`.
Fetch the bounded validated result using `GET /v1/operations/<id>/result` only after
`complete`. `POST /v1/operations/<id>/cancel` requests cancellation. Access is scoped
to the authenticated user and mode. Arbitrary provider logs/files are not served.
A local-model result includes `candidate_revision` and the actual invocation's
canonical `model_identity` (provider, model reference, provider model ID, runtime
surface, identity strength and optional resolved digest). A completed local-model
operation exposes the same identity; pending operations do not claim an observed
model identity. Clients must verify both identities against their expected candidate
and run contract, rather than using a constant gateway label.
A hosted review-only result is the existing four-perspective review result. A cycle
result is `codefriend.update_cycle_result.v1`, with one ordered result for each selected
activity. Documentation, diagram and test outputs are source-bound proposals; they do
not grant source mutation or publication authority. Diagram proposals use Mermaid
source. Test proposals retain the requested test goal or percentage target but leave
`measured_coverage_percent` null because this executor does not run a coverage tool.
Provider failure or malformed output is recorded against the affected activity and
cannot become a successful result. Publication approval and rendering remain separate.

## Failure, replay and retention

The service has one exclusive process lock for its persistent store. It durably reserves
an operation directory and identity before dispatch. Any reuse of the ID returns 409,
including different request bytes and interrupted reservations. After restart, active
operations become `interrupted` and never automatically dispatch again. An operator
must investigate effect ambiguity; a new ID is a new potentially paid operation, not
proof that an earlier call had no effect.

Cancellation retains the concurrency slot until the worker exits. Hosted cancellation
is checked between provider lanes; it cannot undo an active provider request. Late
success after cancellation is not published as a complete result. Failures have no
successful result endpoint. Per-subject quotas count every reserved operation across
restarts, including failures and expired operations; expiry does not replenish quota.
Changing quotas or rotating a store is an explicit operator action.

The binary purges expired packet bytes, provider logs and results every 30 seconds,
and handlers also check expiry. An active operation is cancelled first; its files are
purged after termination. Minimal operation identity tombstones persist to prevent
replay. Treat the store and its backups as private. External deployment backup policy
must honor the same retention contract.

Ctrl-C (SIGINT on Unix) stops HTTP acceptance gracefully and stops the periodic
retention task. It does not undo provider effects or authorize replay of an operation.

## Proof boundaries

`adl/tests/codefriend_server.rs` provides deterministic local component/protocol proof.
Injected-backend scenarios do not prove real hosted execution or model quality.
Before issue acceptance, retain a real bounded service/provider run with exact installed
candidate, provider/profile, input/output identities and the source issue failure matrix.
Website and installed-agent acceptance are separately required before Sprint 10 passes.
