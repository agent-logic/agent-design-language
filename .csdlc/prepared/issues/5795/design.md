# Issue 5795 Design: Governed Gemma GPU Shepherd MVP

## Outcome And Boundary

Issue 5795 lets an operator send one bounded Shepherd message from the separate
HTML Observatory through Runtime v3 governed ingress to an explicitly
configured Gemma GPU adapter and receive the real response plus execution
evidence. The MVP is optional and supports an Apple MLX/Metal runner on macOS
and a portable Ollama/CUDA model bundle on Linux/AWS through one subprocess
contract. It must distinguish unavailable, deterministic test-double, and
real-model states and may not present fake, cached, or retained responses as
live inference. The MLX artifact is a macOS proof input only and is never used
as the AWS or distributed-Polis deployment artifact.

The issue does not implement the v0.95 Shepherd training/evaluator program,
change the global default model, add a hosted-inference fallback, or redefine
Runtime, Observatory, or WP-14 protocol contracts. AWS is a bounded portability
proof target and a future self-hosted deployment environment, not an external
model-provider dependency.

## Source Baseline

- `adl-runtime-kernel/src/operations.rs` already declares Shepherd admission
  and its governed dependency chain.
- `adl-runtime-kernel/src/ingress.rs`, `control.rs`, `governed_operations.rs`,
  and `protocol_adapters.rs` own signed command admission, authorization, and
  adapter execution boundaries.
- `adl-runtime/src/runtime_api.rs`, `runtime_api_auth.rs`, and
  `tests/runtime_api_wss.rs` own authenticated HTTP/WSS transport.
- `demos/html-observatory/app.js` and `runtime-v3.config.json` own the separate
  client projection and operator channel.
- Existing provider profiles under `adl/src/provider/` are evidence inputs,
  not permission to route this MVP around Runtime v3.

## Design

Add a Runtime-owned local Shepherd adapter contract selected only by explicit
configuration. Admission validates the signed command, principal, capability,
runtime identity, message bounds, and operation policy before provider work.
The adapter launches the configured MLX/Metal or CUDA runner in a dedicated
process group with CPU, descriptor, and process-count rlimits, an address-space
rlimit where the host supports it, and an independent process-tree RSS
watchdog. The watchdog retains every observed descendant identity even when a
child creates a new session or process group. The request, timeout, output,
cancellation, and pipe-drain paths are all bounded and metadata is redacted.
Timeout, cancellation, normal child exit, and executor drop terminate the
original process group and every separately observed descendant. The
operator-attested runner must not deliberately daemonize and sever ancestry
before the watchdog can observe it. That behavior is outside this trusted
runner contract and requires an OS service sandbox or Linux cgroup deployment
boundary; process polling is not represented as kernel containment. Resource
accounting runs off the asynchronous Runtime worker threads. No model
availability is inferred from configuration alone.

Real-model classification requires a versioned runner handshake. Runtime sends
a fresh nonce plus the expected backend, exact model identity, model artifact
digest, runtime identity, and correlation identity. The runner must return all
of those bindings with a response before Runtime emits `real_local_model`.
The operator must pin the expected SHA-256 digest of the trusted runner bytes;
configuration fails closed if the executable does not match. Runtime reads the
runner once, verifies those exact bytes, and executes a private captured copy
rather than reopening the mutable configured path. Linux launches the captured
descriptor directly; macOS launches the fresh read-only private copy because
it lacks a portable descriptor-execution path. Executable bytes and the
canonical launch configuration are hashed into the redacted result. A
syntactically valid caller-supplied model name, post-configuration executable
replacement, or arbitrary nonempty subprocess output is insufficient.

The portable CUDA model, Ollama runtime, and restoration manifest are prepared
once and stored in a private, versioned Agent Logic S3 bucket. The manifest
pins every object key, S3 version ID, byte length, and SHA-256 digest. AWS proof
and future distributed-Polis nodes download those exact versions, verify every
digest before installation, and run the model locally on node-owned GPU
hardware. Mutable latest objects, per-node registry pulls, Bedrock, and hosted
inference are not valid restoration paths. S3 distributes immutable bytes; it
does not execute inference or hold runtime authority.

The Linux runtime contract includes the Ollama CUDA 12 userspace libraries in
the pinned Ollama archive. The proof host supplies the NVIDIA kernel driver
through an AWS NVIDIA-driver DLAMI. Startup verifies the expected GPU class,
driver presence, CUDA userspace library family and digest, and Ollama GPU
residency before inference. The CUDA compiler toolkit is not an inference
dependency and is not installed merely for proof theater.

The response envelope includes correlation identity, live-versus-replay
provenance, retention truth, and a truthful execution classification:
`unavailable`, `deterministic_test_double`, or `real_local_model`. Completed
idempotency replay rewrites successful Shepherd evidence to
`idempotency_replay` and `retained=true`; cached bytes may never masquerade as
fresh inference. A separate versioned failure envelope carries correlation,
runtime, unavailable classification, retention truth, and a bounded reason
code. Observatory renders those classifications but does not hold signing
keys, launch providers, or gain direct filesystem/model authority. Provider
failure leaves Runtime and the public read stream usable.

## Owned Paths

- `adl-runtime-kernel/src/shepherd.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/src/operations.rs`
- `adl-runtime-kernel/tests/shepherd.rs`
- `adl-runtime/tests/shepherd_local_model.rs`
- `adl/tools/run_wp5795_aws_gpu_proof.sh`
- `demos/html-observatory/shepherd.js`
- `demos/html-observatory/index.html`
- `adl/tools/validate_v092_shepherd_browser_roundtrip.mjs`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Invariants And Failure Semantics

- Unsigned, unauthorized, malformed, oversized, or wrong-runtime messages are
  rejected before local provider invocation.
- No cloud fallback, silent model substitution, or global-default change.
- AWS and distributed-Polis nodes use the portable CUDA bundle, never the
  Apple-only MLX bundle.
- Model restoration fails closed unless the artifact manifest and every
  versioned S3 object match their pinned SHA-256 digests and byte lengths.
- CUDA execution fails closed unless the NVIDIA driver and pinned Ollama CUDA
  12 userspace libraries are present and the model reports nonzero VRAM
  residency.
- Model path, prompt content, tokens, and private response data are not logged
  beyond the declared redacted evidence policy.
- Timeouts and cancellation release permits and preserve Runtime usability.
- The original subprocess process group is terminated on timeout,
  cancellation, successful parent exit, and executor future drop; reader
  draining is independently bounded.
- Process supervision also terminates descendants observed after `setsid()`.
  Deliberate ancestry-racing daemonization is prohibited by the attested-runner
  contract and is not misrepresented as same-user hostile-code containment.
- Completed idempotency replay is explicitly retained and cannot claim live
  execution.
- `real_local_model` requires an operator-pinned runner-program digest and a
  nonce-bound response matching the exact configured backend, model identity,
  and model artifact digest.
- Deterministic fakes prove adapter logic only; they cannot satisfy the real
  local-model acceptance criterion.
- Observatory status never upgrades retained/mock evidence to live proof.

## Dependencies And Coordination

WP-03 issue 5820 and TLS issue 5800 establish the launch path. WP-14 issue 5832
must freeze the command/WSS contract before final Observatory integration.
Preparation may proceed now, but implementation cannot cross those serial
gates or claim their surfaces.

## Validation Boundary

Deterministic tests cover admission, fake adapter behavior, timeout,
cancellation, malformed commands, status classification, redaction, and
unauthorized mutation. A local macOS Apple Metal/MLX lane must invoke the
explicitly configured model and retain a real response with correlation proof.
An AWS Linux lane must restore the pinned portable bundle from S3, run the same
exact-head test against a CUDA-backed Gemma model on a `g6.xlarge` (or an
explicitly recorded compatible GPU instance), prove GPU residency, and retain
the artifact-manifest/backend/model/runner/correlation digests.
The AWS runner must use the approved Agent Logic account/profile, enforce a
bounded run deadline and cost ceiling, and leave no test instance or temporary
volume running or retained after success or failure. Every paid run uses an
unguessable owner token, an exact-version S3 lock, owner-scoped instance and
volume tags, and three cleanup layers: local trap cleanup, a guest timer, and a
pre-existing tag-scoped EventBridge/Lambda reaper that is verified before
launch and terminates only overdue managed issue instances. Launch writes the
reaper deadline tags atomically with instance creation, avoiding a post-launch
deadline-registration gap. Manual and local-trap cleanup require the exact run
ID, owner token, and S3 lock version; the guest timer can delete only the exact
lock version captured for its launched owner. Lock release verifies retained
ownership before deleting that version. Missing hardware/model is a truthful
deferred or blocked lane, never a pass. Until the regional On-Demand G/VT quota
is at least four vCPUs, only artifact publication and the non-billing preflight
may run; no EC2 instance is launched.
Browser proof verifies the complete Observatory-to-Runtime round trip.
The implementation must add
`adl/tools/validate_v092_shepherd_browser_roundtrip.mjs`. That live validator
opens the real Observatory in Chrome, submits one uniquely correlated governed
message, proves Runtime admission invoked the configured MLX/Gemma adapter,
waits for the non-retained `real_local_model` response, verifies the browser
renders the same correlation and classification, and retains redacted Runtime,
adapter, WSS, and browser evidence. Legacy Observatory scripts, deterministic
adapters, and direct model invocation cannot satisfy this lane.

## Rollback

Rollback disables the optional adapter in configuration, removes the new
operation route and projection fields only if compatibility permits, reconnects
the Observatory in read-only mode, and verifies that Runtime health and WSS
remain usable. It does not switch to cloud or label a fake as production.

## Non-Goals

- Full v0.95 Shepherd/Gemma training, Aptitude Atlas, or evaluator buildout.
- Hosted inference, automatic cloud fallback, or provider billing integration.
- Global default model selection or broad intelligence/safety claims.
- Runtime/API/protocol redesign owned by 5820 or 5832.
- Observatory visual redesign or Unity consumer work.
