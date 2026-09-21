# CodeFriend Beta 1 server and model-access service

Issue #1056 implements the HTTP backend in ADL. The website remains in
`agent-logic/codefriend.ai`; repository extraction is separate v0.93 work.
This service supplies hosted review execution and restricted model requests for
installed local agents. It does not implement website GitHub login or agent pairing.
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
Provider config permits one attempt and 1–4096 output tokens. CodeFriend uses the
adapter's active-work mode: the legacy request `timeout_ms` field does not impose
a customer-cycle or provider-response deadline. A successful response remains valid
when it arrives after that configured duration. Connection establishment has a
separate transport guard; a lost response does not authorize repeating the request.
Generated prompts are capped at 128 KiB per lane before any model dispatch.
Response bodies are capped at 4 MiB before HTTP decoding or Bedrock SDK Blob
aggregation, including error replies. Oversized responses fail without a successful
result; token limits alone are not a byte bound.
Do not preload input text. Only the service operator selects routes,
endpoint, model and provider credential references. Provision actual credentials
through approved server-side indirection; never send them to either client.
The service logs fixed event codes to stderr and reserves stdout for machine output.

The host's 30-minute setting measures idle time only. Active or uncertain work
prevents automatic shutdown, even when a review takes longer than 30 minutes.
Elapsed cycle duration alone is not evidence of a hang. Inspect operation state,
worker liveness and observed progress before deciding that work is stalled.
Explicit cancellation and the original source authorization/retention rules remain
in force. The installed agent can resume observation of an acknowledged operation
past its legacy observation timer without repeating its submission.

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
- `lane`: null for hosted, or one existing review lane for local-model requests.

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
A hosted result is the existing four-perspective review result; publication approval,
rendering and full journey wiring remain their existing owners and #914.

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
