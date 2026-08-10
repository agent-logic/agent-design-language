#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP_JS="${ROOT_DIR}/demos/html-observatory/app.js"
CONFIG_JSON="${ROOT_DIR}/demos/html-observatory/runtime-v3.config.json"
SERVER_JS="${ROOT_DIR}/adl/tools/serve_v092_html_observatory.mjs"
LIVE_VALIDATOR_JS="${ROOT_DIR}/adl/tools/validate_v092_html_observatory_live.mjs"
RUNTIME_BUILD_RS="${ROOT_DIR}/adl-runtime-kernel/build.rs"

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
rg -F 'restart did not produce a fresh reconnect decision' "${LIVE_VALIDATOR_JS}" >/dev/null
if rg -n 'std::env::var\("ADL_SOURCE_REVISION"\)|rerun-if-env-changed=ADL_SOURCE_REVISION' "${RUNTIME_BUILD_RS}" >/dev/null; then
  echo "Runtime source provenance must not accept a caller-supplied revision" >&2
  exit 1
fi
rg -F 'status", "--porcelain", "--untracked-files=no"' "${RUNTIME_BUILD_RS}" >/dev/null
rg -F 'symbolic-ref", "-q", "HEAD"' "${RUNTIME_BUILD_RS}" >/dev/null
rg -F 'unavailable-or-dirty' "${RUNTIME_BUILD_RS}" >/dev/null

node - <<'NODE' "${APP_JS}" "${CONFIG_JSON}"
const fs = require("fs");
const vm = require("vm");
const assert = require("assert");
const { webcrypto } = require("crypto");

const appPath = process.argv[2];
const configPath = process.argv[3];
const source = fs.readFileSync(appPath, "utf8");
const config = JSON.parse(fs.readFileSync(configPath, "utf8"));

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

await assert.rejects(
  () => api.buildSignedLayer8MessageCommand({
    runtimeInstanceId: "runtime-v3-test",
    recipientId: "agent-0001",
    content: "😀".repeat(1001),
    signingKeyText: "00".repeat(32)
  }),
  /4000 UTF-8 bytes/
);

const shepherdCommand = await api.buildSignedLayer8MessageCommand({
  runtimeInstanceId: "runtime-v3-test",
  recipientId: "shepherd",
  content: "Hello Shepherd",
  signingKeyText: "00".repeat(32)
});
assert.equal(shepherdCommand.action.work.kind, "shepherd");
const genericAgentCommand = await api.buildSignedLayer8MessageCommand({
  runtimeInstanceId: "runtime-v3-test",
  recipientId: "agent-0001",
  content: "Hello agent",
  signingKeyText: "00".repeat(32)
});
assert.equal(genericAgentCommand.action.work.kind, "agent");

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
