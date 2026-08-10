#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP_JS="${ROOT_DIR}/demos/html-observatory/app.js"
CONFIG_JSON="${ROOT_DIR}/demos/html-observatory/runtime-v3.config.json"
SERVER_JS="${ROOT_DIR}/adl/tools/serve_v092_html_observatory.mjs"
LIVE_VALIDATOR_JS="${ROOT_DIR}/adl/tools/validate_v092_html_observatory_live.mjs"
RUNTIME_BUILD_RS="${ROOT_DIR}/adl-runtime-kernel/build.rs"
INDEX_HTML="${ROOT_DIR}/demos/html-observatory/index.html"
IDENTITY_GOLDEN_JSON="${ROOT_DIR}/docs/api/runtime-v3/v1/acip-identity-message-golden.json"

node --check "${SERVER_JS}"
if rg -n 'wuji|localhost|127\.0\.0\.1' "${SERVER_JS}" | rg -v 'LISTEN_ADDRESS.*127\.0\.0\.1' >/dev/null; then
  echo "tracked Observatory server must not hard-code a deployment or TLS identity" >&2
  exit 1
fi
rg -F 'sectionValue("api", "public_base_url")' "${SERVER_JS}" >/dev/null
rg -F 'https://${runtimeHostname}:* wss://${runtimeHostname}:*' "${SERVER_JS}" >/dev/null
rg -F 'x-adl-source-revision' "${SERVER_JS}" >/dev/null
rg -F 'ADL_SOURCE_REVISION does not match Observatory repository HEAD' "${SERVER_JS}" >/dev/null
node --check "${LIVE_VALIDATOR_JS}"
if rg -n 'process\.kill|ADL_EXPECTED_RUNTIME_PID' "${LIVE_VALIDATOR_JS}" >/dev/null; then
  echo "live Observatory proof must use Guardian-owned signed restart, never raw PID signaling" >&2
  exit 1
fi
rg -F 'fs.open(pathname, "wx", 0o600)' "${LIVE_VALIDATOR_JS}" >/dev/null
rg -F 'assert.equal(sourceRevision, repositoryRevision' "${LIVE_VALIDATOR_JS}" >/dev/null
rg -F 'assert.equal(restartTarget.source_revision, sourceRevision' "${LIVE_VALIDATOR_JS}" >/dev/null
rg -F 'bytes differ from exact source revision' "${LIVE_VALIDATOR_JS}" >/dev/null
rg -F 'data-last-reconnect-delay-millis' "${LIVE_VALIDATOR_JS}" >/dev/null
rg -F 'root.dataset.lastReconnectDelayMillis = String(delay)' "${APP_JS}" >/dev/null
rg -F 'root.dataset.reconnectDecisionCount = String(reconnectDecisionCount + 1)' "${APP_JS}" >/dev/null
rg -F 'await renderLiveError(error, requestGeneration);' "${APP_JS}" >/dev/null
rg -F 'scheduleReconnect(requestGeneration);' "${APP_JS}" >/dev/null
rg -F 'liveStoppedByOperator || !isCurrentLiveGeneration(requestGeneration)' "${APP_JS}" >/dev/null
rg -F 'A new live connection starts without write authority until Runtime authenticates it.' "${APP_JS}" >/dev/null
rg -F 'await page.locator("#operator-logout").click();' "${LIVE_VALIDATOR_JS}" >/dev/null
rg -F 'restart did not produce a fresh reconnect decision' "${LIVE_VALIDATOR_JS}" >/dev/null
if rg -n 'std::env::var\("ADL_SOURCE_REVISION"\)|rerun-if-env-changed=ADL_SOURCE_REVISION' "${RUNTIME_BUILD_RS}" >/dev/null; then
  echo "Runtime source provenance must not accept a caller-supplied revision" >&2
  exit 1
fi
rg -F 'status", "--porcelain", "--untracked-files=no"' "${RUNTIME_BUILD_RS}" >/dev/null
rg -F 'symbolic-ref", "-q", "HEAD"' "${RUNTIME_BUILD_RS}" >/dev/null
rg -F 'unavailable-or-dirty' "${RUNTIME_BUILD_RS}" >/dev/null
if rg -n 'agent-chat-key-file|Operator signing key' "${INDEX_HTML}" "${APP_JS}" >/dev/null; then
  echo "normal Observatory chat must not expose or retain browser signing keys" >&2
  exit 1
fi

node - <<'NODE' "${APP_JS}" "${CONFIG_JSON}" "${IDENTITY_GOLDEN_JSON}"
const fs = require("fs");
const vm = require("vm");
const assert = require("assert");
const { webcrypto } = require("crypto");

const appPath = process.argv[2];
const configPath = process.argv[3];
const identityGoldenPath = process.argv[4];
const source = fs.readFileSync(appPath, "utf8");
const config = JSON.parse(fs.readFileSync(configPath, "utf8"));
const identityGolden = JSON.parse(fs.readFileSync(identityGoldenPath, "utf8"));

(async () => {
const observatoryFeed = {
  schema: "adl.runtime_v3.observatory_feed.v2",
  source_revision: "0".repeat(40),
  polis_name: "Konishi",
  runtime_instance_id: "runtime-v3-test",
  runtime_process_id: 12345,
  runtime_selection: "runtime_v3_explicit_opt_in",
  control: {
    port: 20997,
    read_endpoint: "/v1/observatory",
    websocket_endpoint: "/v1/observatory/ws",
    signed_command_endpoint: "/v1/control",
    signed_commands_required_for_mutation: true,
    bearer_token_required_for_read: false,
    login_required_for_mutation: true,
    browser_mutation_authority: true
  },
  health: {
    observability_ready: true,
    snapshot: {
      schema: "adl.runtime.control_snapshot.v1",
      revision: 1,
      topology_generation: 1,
      components: { runtime_api: "running" },
      restart_counts: {},
      queues: {},
      clock: { status: "authoritative" },
      continuity_head: { generation: 1, accepted_through: 1, topology_hash: "t", config_hash: "c", integrity: "verified" },
      lifecycle: "running",
      event_count: 1,
      observability_ready: true
    }
  },
  weather: {
    schema: "adl.runtime.weather_health.v1",
    resource_state: "healthy",
    shutdown_decision: "continue",
    gpu_proof_state: "unavailable_not_pass"
  },
  weather_freshness: {
    observed_at_unix_millis: 1785778500000,
    age_millis: 1,
    stale_after_millis: 30000,
    stale: false
  },
  agents: {
    total_count: 1,
    rendered_sample_count: 1,
    sample: [{ id: "agent-0001", label: "Shepherd", role: "runtime shepherd", state: "running", detail: "operator-addressable" }]
  },
  continuity: {},
  proof: {
    default_runtime_switch_authorized: false,
    runtime_v2_decommission_authorized: false,
    sidecar_required: false
  },
  events: [{ sequence: 1, component: "operator", event: "agent_ready", correlation_id: "test-1" }]
};

const readiness = {
  schema: "adl.runtime_v3.readiness.v1",
  ready: true,
  degraded_reasons: [],
  observability_ready: true,
  runtime_instance_id: "runtime-v3-test"
};

const calls = [];
let pendingTimer = null;
class MockWebSocket {
  constructor(url) {
    this.url = url;
    this.listeners = new Map();
    this.closeFrame = null;
  }
  addEventListener(name, listener) {
    this.listeners.set(name, listener);
  }
  emit(name, event = {}) {
    this.listeners.get(name)?.(event);
  }
  close(code, reason) {
    this.closeFrame = { code, reason };
  }
  send() {}
}
const context = {
  console,
  TextEncoder,
  crypto: webcrypto,
  WebSocket: MockWebSocket,
  setTimeout: (callback) => {
    pendingTimer = callback;
    return 1;
  },
  clearTimeout: () => {
    pendingTimer = null;
  },
  URL,
  URLSearchParams,
  location: { search: "" },
  window: { location: { search: "" } },
  fetch: async (url, options = {}) => {
    calls.push({ url: String(url), options });
    if (String(url) === "https://wuji.agent-logic.ai:20997/v1/observatory") {
      return { ok: true, status: 200, json: async () => observatoryFeed };
    }
    if (String(url) === "https://wuji.agent-logic.ai:20997/v1/ready") {
      return { ok: true, status: 200, json: async () => readiness };
    }
    if (String(url) === "https://wuji.agent-logic.ai:20997/v1/control") {
      const body = JSON.parse(String(options.body || "{}"));
      assert.equal(options.method, "POST");
      assert.equal(options.headers["Content-Type"], "application/json");
      assert.equal(body.schema, "adl.runtime.control_command.v1");
      return {
        ok: true,
        status: 200,
        json: async () => ({
          schema: "adl.runtime.control_response.v1",
          command_id: body.command_id,
          correlation_id: body.correlation_id,
          outcome: { snapshot: { lifecycle: "running" } }
        })
      };
    }
    return { ok: false, status: 404, json: async () => ({ code: "not_found" }) };
  },
  globalThis: {}
};
context.globalThis = context;
vm.runInNewContext(source, context);
const api = context.AdlHtmlObservatory;
api.applyRuntimeV3Config(config);

assert.equal(api.requestedRuntimeSelection(), "v3");
assert.equal(api.getQueryApiBase(), "https://wuji.agent-logic.ai:20997");
assert.throws(
  () => api.normalizeTrustedRuntimeV3ApiBase("https://localhost:21983"),
  /configured Runtime instance HTTPS hostname/
);
assert.throws(
  () => api.normalizeTrustedRuntimeV3ApiBase("https://127.0.0.1:21983"),
  /configured Runtime instance HTTPS hostname/
);
assert.equal(api.getRuntimeV3Config().signed_command_endpoint, "/v1/control");
assert.equal(api.classifyRuntimeV3Failure(new Error("backpressure")).state, "backpressure");
assert.equal(api.classifyRuntimeV3Failure(new Error("unsupported Runtime v3 observatory schema")).state, "incompatible");
assert.equal(api.classifyRuntimeV3Failure(new Error("403 origin denied")).state, "denied");
assert.equal(api.classifyRuntimeV3Failure(new Error("temporarily_unavailable")).state, "unavailable");
assert.equal(api.classifyRuntimeV3Failure(new Error("certificate failure")).state, "tls-or-origin");

let websocketFailure = null;
const incompatibleSocket = api.connectRuntimeV3ObservatoryWebSocket(
  "https://wuji.agent-logic.ai:20997",
  () => assert.fail("incompatible frame reached snapshot projection"),
  (error) => { websocketFailure = error; }
);
incompatibleSocket.emit("message", { data: JSON.stringify({ schema: "adl.runtime.future.v99" }) });
assert.match(websocketFailure.message, /Unsupported Runtime v3 Observatory schema/);
assert.deepEqual(incompatibleSocket.closeFrame, { code: 1008, reason: "invalid_observatory_frame" });

websocketFailure = null;
const staleSocket = api.connectRuntimeV3ObservatoryWebSocket(
  "https://wuji.agent-logic.ai:20997",
  () => {},
  (error) => { websocketFailure = error; }
);
staleSocket.emit("open");
assert.equal(typeof pendingTimer, "function");
pendingTimer();
assert.match(websocketFailure.message, /stream stale/);
assert.deepEqual(staleSocket.closeFrame, { code: 1008, reason: "stale_observatory_stream" });

const eventCheck = await api.checkEventsEndpoint(api.getQueryApiBase());
assert.equal(eventCheck.schema, "adl.html_observatory.runtime_v3_event_check.v1");
assert.equal(eventCheck.events[0].event, "agent_ready");
assert.equal(api.normalizeEventEntries(eventCheck).length, 1);

const cursor = api.createRuntimeV3StreamCursor();
let cursorSnapshot = cursor.accept({
  status: { runtime_id: "runtime-v3-test" },
  events: { events: [{ sequence: 9, event: "older" }, { sequence: 10, event: "current" }] }
});
assert.deepEqual(cursorSnapshot.events.events.map((event) => event.sequence), [9, 10]);
assert.equal(cursorSnapshot.stream_cursor.applied_event_count, 2);
cursorSnapshot = cursor.accept({
  status: { runtime_id: "runtime-v3-test" },
  events: { events: [{ sequence: 8, event: "late-old" }, { sequence: 11, event: "next" }] }
});
assert.deepEqual(cursorSnapshot.events.events.map((event) => event.sequence), [9, 10, 11]);
cursorSnapshot = cursor.accept({
  status: { runtime_id: "runtime-v3-test" },
  events: { events: [{ sequence: 13, event: "later" }, { sequence: 12, event: "next" }] }
});
assert.deepEqual(cursorSnapshot.events.events.map((event) => event.sequence), [9, 10, 11, 12, 13]);
assert.equal(cursorSnapshot.stream_cursor.applied_event_count, 5);
assert.throws(
  () => cursor.accept({
    status: { runtime_id: "runtime-v3-test" },
    events: { events: [{ sequence: 14, event: "next" }, { sequence: 16, event: "internal-gap" }] }
  }),
  /stream cursor gap after 14/
);
assert.throws(
  () => cursor.accept({
    status: { runtime_id: "runtime-v3-test" },
    events: { events: [{ sequence: 15, event: "gap" }] }
  }),
  /stream cursor gap after 13/
);

assert.equal(typeof api.buildSignedLayer8MessageCommand, "undefined");
const goldenUnsigned = { ...identityGolden.message, signature: "" };
const goldenDomain = new TextEncoder().encode("adl.acip.identity_message.v1\0");
const goldenPayload = new TextEncoder().encode(api.canonicalJson(goldenUnsigned));
const goldenSigningBytes = new Uint8Array(goldenDomain.length + goldenPayload.length);
goldenSigningBytes.set(goldenDomain);
goldenSigningBytes.set(goldenPayload, goldenDomain.length);
assert.equal(
  Buffer.from(goldenSigningBytes).toString("hex"),
  identityGolden.signing_bytes_hex
);
const goldenKey = await webcrypto.subtle.importKey(
  "raw",
  Uint8Array.from(identityGolden.public_key_hex.match(/../g), (byte) => Number.parseInt(byte, 16)),
  { name: "Ed25519" },
  false,
  ["verify"]
);
assert(await webcrypto.subtle.verify(
  { name: "Ed25519" },
  goldenKey,
  Uint8Array.from(identityGolden.message.signature.match(/../g), (byte) => Number.parseInt(byte, 16)),
  goldenSigningBytes
));
const replayKeyPair = await webcrypto.subtle.generateKey("Ed25519", true, ["sign", "verify"]);
const replayPublicKey = Buffer.from(
  await webcrypto.subtle.exportKey("raw", replayKeyPair.publicKey)
).toString("hex");
const replayAgent = {
  id: "agent-replay-test",
  signing_algorithm: "ed25519",
  signing_key_id: "agent-replay-key",
  signing_public_key: replayPublicKey
};
const signAck = async (sequence, correlationId, requestNonce) => {
  const now = Date.now();
  const message = {
    schema: "adl.acip.identity_message.v1",
    message_kind: "ack",
    sender_id: replayAgent.id,
    recipient_id: "layer8-operator",
    correlation_id: correlationId,
    causation_id: requestNonce,
    monotonic_sequence: sequence,
    issued_at_unix_millis: now,
    expires_at_unix_millis: now + 60_000,
    nonce: `ack-replay-test-${sequence}`,
    content: "verified response",
    signing_algorithm: "ed25519",
    signing_key_id: replayAgent.signing_key_id,
    signature: ""
  };
  const domain = new TextEncoder().encode("adl.acip.identity_message.v1\0");
  const payload = new TextEncoder().encode(api.canonicalJson(message));
  const bytes = new Uint8Array(domain.length + payload.length);
  bytes.set(domain);
  bytes.set(payload, domain.length);
  message.signature = Buffer.from(
    await webcrypto.subtle.sign("Ed25519", replayKeyPair.privateKey, bytes)
  ).toString("hex");
  return message;
};
const newestAck = await signAck(9, "replay-correlation-9", "replay-request-9");
await api.verifySignedIdentityMessage(
  newestAck,
  replayAgent,
  newestAck.correlation_id,
  newestAck.causation_id
);
const preRestartReplay = await signAck(8, "replay-correlation-8", "replay-request-8");
await assert.rejects(
  () => api.verifySignedIdentityMessage(
    preRestartReplay,
    replayAgent,
    preRestartReplay.correlation_id,
    preRestartReplay.causation_id
  ),
  /replayed or arrived behind/
);
await assert.rejects(
  () => api.verifySignedIdentityMessage(
    {
      schema: "adl.acip.identity_message.v1",
      message_kind: "ack",
      sender_id: "wrong-agent",
      recipient_id: "layer8-operator",
      correlation_id: "correlation-0001",
      causation_id: "causation-0001",
      monotonic_sequence: 1,
      issued_at_unix_millis: 1,
      expires_at_unix_millis: 2,
      nonce: "nonce-0001",
      content: "response",
      signing_algorithm: "ed25519",
      signing_key_id: "agent-key",
      signature: "00".repeat(64)
    },
    {
      id: "agent-0001",
      signing_algorithm: "ed25519",
      signing_key_id: "agent-key",
      signing_public_key: "00".repeat(32)
    },
    "correlation-0001"
  ),
  /untrusted or misrouted/
);
assert(!source.includes("decodeOperatorSigningSeed"));
assert(source.includes("adl.runtime_v3.observatory_layer8_intent.v1"));
assert(source.includes("verifySignedIdentityMessage"));

assert.deepEqual(api.classifyRuntimeV3Failure(new Error("unsupported runtime v3 observatory schema")), {
  state: "incompatible", label: "incompatible version"
});
assert.deepEqual(api.classifyRuntimeV3Failure(new Error("backpressure")), {
  state: "backpressure", label: "runtime backpressure"
});
assert.deepEqual(api.classifyRuntimeV3Failure(new Error("invalid_request")), {
  state: "malformed", label: "malformed runtime data"
});
assert.deepEqual(api.classifyRuntimeV3Failure(new Error("403 origin denied")), {
  state: "denied", label: "origin or authority denied"
});
assert.deepEqual(api.classifyRuntimeV3Failure(new Error("authentication_failed")), {
  state: "denied", label: "origin or authority denied"
});
assert.deepEqual(api.classifyRuntimeV3Failure(new Error("stale_runtime_instance")), {
  state: "stale", label: "runtime data stale"
});
assert.deepEqual(api.classifyRuntimeV3Failure(new Error("temporarily_unavailable")), {
  state: "unavailable", label: "runtime unavailable"
});
assert.deepEqual(api.classifyRuntimeV3Failure(new Error("certificate failure")), {
  state: "tls-or-origin", label: "TLS or origin failure"
});
assert.deepEqual(api.classifyRuntimeV3Failure(new Error("connection reset")), {
  state: "offline", label: "runtime offline"
});

const unavailableSnapshot = api.runtimeUnavailableSnapshot(new Error("connection refused"));
const unavailableView = api.buildPanopticonViewModel(unavailableSnapshot);
assert.equal(unavailableView.mode, "unavailable");
assert.equal(unavailableView.fetchedAt, "");
assert.equal(unavailableView.polisName, "Polis unavailable");
assert.equal(unavailableView.agentTotal, 0);
assert.equal(unavailableView.readyState, "unavailable");

const staleSnapshot = api.runtimeV3SnapshotFromFeed({
  ...observatoryFeed,
  weather_freshness: {
    observed_at_unix_millis: 1785778500000,
    age_millis: 45000,
    stale_after_millis: 30000,
    stale: true
  }
}, {
  ...readiness,
  weather_freshness: {
    observed_at_unix_millis: 1785778500000,
    age_millis: 45000,
    stale_after_millis: 30000,
    stale: true
  }
});
const staleView = api.buildPanopticonViewModel(staleSnapshot);
assert.equal(staleSnapshot.status.polis_name, "Konishi");
assert.equal(staleView.polisName, "Konishi");
assert.equal(staleView.readyState, "stale");
assert.equal(staleView.signals.find((signal) => signal.label === "readiness").value, "stale");
assert.match(
  staleView.signals.find((signal) => signal.label === "readiness").detail,
  /weather data stale/
);

const { weather_freshness: _omittedWeatherFreshness, ...feedWithoutWeatherFreshness } = observatoryFeed;
const missingWeatherSnapshot = api.runtimeV3SnapshotFromFeed(
  feedWithoutWeatherFreshness,
  readiness
);
assert.equal(missingWeatherSnapshot.ready.ready, false);
assert.equal(missingWeatherSnapshot.ready.state, "stale");
assert.equal(missingWeatherSnapshot.ready.weather_freshness.stale, true);
assert(missingWeatherSnapshot.ready.blocking_reasons.includes("weather_freshness_missing"));

const command = {
  schema: "adl.runtime.control_command.v1",
  runtime_instance_id: "runtime-v3-test",
  command_id: "operator-message-1",
  correlation_id: "operator-message-1",
  principal: "operator",
  action: { action: "snapshot" },
  signing_algorithm: "ed25519",
  signing_key_id: "operator-key",
  signature: "signed-fixture"
};
const response = await api.submitRuntimeV3SignedControlCommand(api.getQueryApiBase(), command);
assert.equal(response.schema, "adl.runtime.control_response.v1");
assert.equal(response.command_id, "operator-message-1");
assert(calls.some((call) => call.url === "https://wuji.agent-logic.ai:20997/v1/control" && call.options.method === "POST"));
assert.equal(
  api.normalizeTrustedRuntimeV3ApiBase("https://wuji.agent-logic.ai:22983"),
  "https://wuji.agent-logic.ai:22983"
);

assert.throws(
  () => api.normalizeTrustedRuntimeV3ApiBase("https://operator:token@wuji.agent-logic.ai:20997"),
  /configured Runtime instance HTTPS hostname/
);
assert.throws(
  () => api.normalizeTrustedRuntimeV3ApiBase("http://localhost:21983"),
  /configured Runtime instance HTTPS hostname/
);

await assert.rejects(
  () => api.submitRuntimeV3SignedControlCommand("https://wuji.agent-logic.ai:20997", { schema: "wrong" }),
  /adl.runtime.control_command.v1/
);
})().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

echo "PASS: HTML Observatory Runtime v3 default, event check, and signed command POST contract"
