# Installed local agent

Issue #1058 owns the installed agent. The website pairing/control owner is #1057;
#914 integrates both executors and #915 independently qualifies the final product.
This document describes the implemented local component contract. It is not a
claim of completed installation, website integration or real-provider acceptance.

## Authority and privacy

The agent uses outbound HTTPS only. It has no local HTTP listener and accepts no
remote filesystem paths, shell commands or arbitrary model prompts. The user authors
a local `Consent` JSON file with `schema: codefriend.local_agent.v1`, one absolute
`repository_path`, credential-free HTTPS `repository` identity, exact immutable
`revision`, existing bounded `scope`, `expires_at`, `retention_seconds` (60–86400),
`allow_model_egress: true` and `allow_result_upload: true`.

The consent digest covers every field, including the local path. Only its digest
is advertised to the website. Source acquisition uses the existing privacy filter;
selected evidence goes to Agent Logic's model service and configured model provider.
This is local update-cycle orchestration, not a zero-egress product. A validated review
or update-cycle record including selected admitted source evidence is uploaded to the website, still subject to exact-artifact approval before
external publication. Repository content is never executable authority.

## Pairing protocol

The invited GitHub website session issues a one-time pairing code with at least
256 bits of entropy, encoded as URL-safe characters. The installed agent exchanges
it using `POST /v1/agent/pair` and `{schema, code}`. The website must atomically consume
the code, bind the agent to the authenticated invited subject, and return:

- `schema: codefriend.local_agent.v1`
- `origin`: the exact HTTPS website origin with trailing slash
- `agent_id`, `subject`: bounded safe identifiers
- `agent_token`: a private opaque token for agent control
- `model_token`: a distinct private opaque token restricted to #1056 `local_model`
- `expires_at`: Unix seconds

Neither token is a provider key. Both are secrets: store only in owner-readable
files, exclude them from diagnostics, and revoke both on website unpair/logout
policy. Agent pairing tokens cannot authorize another subject or agent. The website
must check invitation, scope, expiry and revocation on every control request; the
model gateway rechecks its credential registry on every model request. Revocation
cannot undo a provider call already in flight.

Redirects and ambient proxy configuration are disabled. The pairing reply must
match the exact configured origin. A failed pairing exchange is not automatically
retried; inspect/revoke the previous pairing and issue a new code after ambiguity.
The explicit loopback fixture transport is solely for deterministic local tests.

## Run command

A legacy review command has exactly `schema`, `agent_id`, `subject`, `run_id`,
`consent_digest`, and `expires_at`. An update-cycle command adds `cycle`, a validated
`codefriend.update_cycle_plan.v1` whose repository must match local consent. It must match the pairing and current local
consent, and expire no later than either. Unknown fields are rejected. Each run ID
is durably reserved before acquisition or model dispatch. Reservation directories
survive disconnects and process restarts; their presence denies another dispatch,
including a crash before complete metadata was written. A new run ID is a new
potentially paid review, not a retry guarantee.

## Evidence boundary

The initial component tests cover identity/scope/expiry denial, explicit egress
consent, fixed transport origin, private pairing storage, exclusive process ownership
and crash/restart duplicate rejection. The local protocol fixtures additionally exercise four-lane orchestration, disconnect and cancellation, live consent removal, retention expiry, corrupted-report rejection and interrupted restart without dispatch. They do not prove a real website pairing service, installed multi-platform journey or real model-backed review; those acceptance steps remain outstanding.

## Installed commands and renewal

Build/install from the selected candidate with:

```
bash adl/tools/install_owner_binaries.sh --bin codefriend-agent
```

Use the installer's printed stable binary path. `pair --store <absolute-private-dir>
--origin https://<website-host>/ --code-file <private-file>` exchanges the code;
the code never appears in argv. `run --store <dir> --consent <private-json>` polls
continuously; `once` performs one poll. The code and consent files must be regular
owner-only files. Local consent is reloaded and must still match its original digest
before every model dispatch and result upload. Removing/changing it stops subsequent
work. The agent also checks website cancellation while observing a model operation.

`unpair --store <dir>` confirms website revocation, immediately scrubs retained
source/report/gateway payloads, and then removes local credentials. Run identity
tombstones remain to prevent replay, even if the agent is never invoked again.
For an expired or revoked pairing, `forget --store <dir>` removes local credentials
and retained review payloads; it explicitly does **not** claim remote revocation.
Revoke the old agent in the website and generate a fresh one-time code to pair again.

## Control and forwarding endpoints

All following requests use the agent-control bearer, distinct from the model bearer:

- `POST /v1/agent/poll`: `{schema, agent_id, consent_digest}`; response
  `{schema, command}` where command may be null.
- `GET /v1/agent/runs/<run_id>/control`: response
  `{schema, agent_id, subject, run_id, cancelled}`.
- `PUT /v1/agent/runs/<run_id>/result`: a `RunReport`; acknowledge
  `{schema, run_id, digest}` only after validating and storing the exact report.
- `POST /v1/agent/revoke`: `{schema, agent_id}`; return `{agent_id, revoked:true}`
  only after revoking both agent and model credentials.

The website must bind all commands, control reads and result uploads to the invited
subject and paired agent, reject stale/expired/revoked authority, and make a repeated
identical result PUT idempotent. A different digest for an existing run is a conflict.
A command redelivery never causes local acquisition/model dispatch again. The agent
may retransmit the exact retained result after checking current consent and cancellation.
A reservation with no acknowledged model operation is forwarded as interrupted;
a lost POST response never permits another POST. A durably acknowledged operation
instead resumes bounded GET/control observation after disconnect or restart.
Validated completed lane results and a completed aggregate cycle operation are reused while consent/retention remain valid;
only lanes that were never dispatched may start. The original 120-second observation
deadline persists across restart for each pending operation, while already completed
lane outputs remain reusable for the run retention period. Directory ancestry and
reservation files are synced before dispatch. A crash before reservation metadata
completed requires explicit investigation.

The final report, including all four lanes and admitted evidence, must fit the
4 MiB website/verifier limit. Oversized aggregates produce a bounded failure report
and payload cleanup, never an unforwardable successful completion.

`RunReport.result` is the legacy validated four-perspective review record **including its
selected admitted source evidence**, which is necessary for website artifact inspection.
For an update-cycle command, `result` is null and `RunReport.cycle_result` contains the
validated ordered activity results. `gateway_lanes` contains one `cycle` execution
identity because the complete plan is protected by one durable gateway reservation.
`allow_result_upload` explicitly authorizes that selected evidence in addition to
findings. It does not grant external publication. The website must display this
scope before consent/pairing and retain the record for no longer than `expires_at`.
Provider credentials and local filesystem paths are absent from the result record.

Retention is enforced before further model dispatch and at terminal exits, including
`once`. Expired local work/result payloads are purged while identity tombstones remain.
An in-flight remote call cannot be undone by local cancellation or consent revocation;
its capacity and retention are independently governed by #1056.

## Website report verification

The website must invoke the installed `codefriend-agent verify-report --report-file
PRIVATE_FILE` command before accepting a local-agent report. The file must be a
regular owner-only file of at most 4 MiB. Successful verification writes the
validated report as JSON to stdout; failures return a nonzero status with a
generic diagnostic. This command does not contact providers or the website.

Verification checks the native report digest, identity shape, expiry, complete
review-record contract, all four lane manifests and versions, or the complete
update-cycle plan, admission, selected activity results, embedded review, artifact
digests and nonempty admitted source references. It also checks exact agreement
between report and admitted-input retention deadlines. The website separately
binds the verified report to the authenticated agent, pending run and consent.
Contract integrity does not independently prove that a provider executed.

Local `RunReport.gateway_lanes` retains each lane's gateway candidate revision and
actual canonical provider/model identity. Completion requires all four lanes with
consistent candidate, provider, model, runtime surface, identity strength and pinned
digest (when available). Observation timestamps may differ between lanes. The
review route is a bounded digest of that retained identity tuple, not a constant
gateway label. The website must retain these identities and bind them into the
exact artifact approval digest. A claimed model identity is not stronger than its
recorded provider-asserted or pinned strength.
