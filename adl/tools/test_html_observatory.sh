#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP_JS="${ROOT_DIR}/demos/html-observatory/app.js"
CONFIG_JSON="${ROOT_DIR}/demos/html-observatory/runtime-v3.config.json"

node - <<'NODE' "${APP_JS}" "${CONFIG_JSON}"
const fs = require("fs");
const vm = require("vm");
const assert = require("assert");

const appPath = process.argv[2];
const configPath = process.argv[3];
const source = fs.readFileSync(appPath, "utf8");
const config = JSON.parse(fs.readFileSync(configPath, "utf8"));

(async () => {
const observatoryFeed = {
  schema: "adl.runtime_v3.observatory_feed.v2",
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
    schema: "adl.runtime_v3.agent_roster_page.v1",
    revision: 7,
    scope: "local_runtime",
    total_count: 1,
    rendered_sample_count: 1,
    has_more: false,
    next_page_token: null,
    population_complete: false,
    sample: [{
      id: "shepherd",
      label: "Shepherd",
      role: "resident shepherd",
      state: "ready",
      health: "healthy",
      availability: "available",
      activity: "governing local Runtime",
      capabilities: ["runtime_governance", "agent_communication"],
      location: "local",
      communication_eligible: true,
      observed_at_unix_millis: 1785778500000,
      freshness_deadline_unix_millis: 1785778530000,
      source_revision: "0123456789abcdef0123456789abcdef01234567",
      provenance: "runtime_component_state"
    }]
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
const context = {
  console,
  URL,
  URLSearchParams,
  location: { search: "" },
  window: { location: { search: "" } },
  fetch: async (url, options = {}) => {
    calls.push({ url: String(url), options });
    if (String(url) === `${config.api_base}/v1/observatory`) {
      return { ok: true, status: 200, json: async () => observatoryFeed };
    }
    if (String(url) === `${config.api_base}/v1/ready`) {
      return { ok: true, status: 200, json: async () => readiness };
    }
    if (String(url).startsWith(`${config.api_base}/v1/agents?`)) {
      return { ok: true, status: 200, json: async () => observatoryFeed.agents };
    }
    if (String(url) === `${config.api_base}/v1/control`) {
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
assert.equal(api.getQueryApiBase(), config.api_base);
assert.equal(api.getRuntimeV3Config().signed_command_endpoint, "/v1/control");

const snapshot = api.runtimeV3SnapshotFromFeed(observatoryFeed, readiness);
const roster = api.buildRuntimeAgentRows({
  status: snapshot.status,
  health: snapshot.health,
  ready: snapshot.ready,
  metrics: snapshot.metrics,
  events: snapshot.events,
  packet: api.FALLBACK_PACKET
});
assert.equal(roster.length, 1);
assert.equal(roster[0].id, "shepherd");
assert.equal(roster[0].state, "ready");
assert.equal(roster[0].health, "healthy");
assert.equal(roster[0].communicationEligible, true);
assert.equal(roster[0].provenance, "runtime_component_state");
assert.equal(roster[0].sourceRevision, "0123456789abcdef0123456789abcdef01234567");
assert.deepEqual(
  api.buildRuntimeAgentRows({ status: { schema: observatoryFeed.schema, agent_population: { sample: [], total_count: 0 } } }),
  [],
  "an empty authoritative Runtime page must not invent fallback agents"
);
const rosterPage = await api.fetchRuntimeV3AgentRosterPage(config.api_base, "next-token");
assert.equal(rosterPage.sample[0].id, "shepherd");
assert(calls.some((call) => call.url.includes("/v1/agents?page_size=50&page_token=next-token")));
const cursorSnapshot = (runtimeId, revision) => ({
  status: { runtime_id: runtimeId, agent_population: { revision } }
});
assert.equal(api.acceptRuntimeRosterSnapshot(cursorSnapshot("runtime-a", 7)), true);
assert.equal(api.acceptRuntimeRosterSnapshot(cursorSnapshot("runtime-a", 7)), false, "duplicate revision rejected");
assert.equal(api.acceptRuntimeRosterSnapshot(cursorSnapshot("runtime-a", 6)), false, "out-of-order revision rejected");
assert.equal(api.acceptRuntimeRosterSnapshot(cursorSnapshot("runtime-a", 9)), true, "newer status revision accepted");
assert.equal(api.acceptRuntimeRosterSnapshot(cursorSnapshot("runtime-b", 1)), true, "Runtime restart resets cursor safely");

const eventCheck = await api.checkEventsEndpoint(api.getQueryApiBase());
assert.equal(eventCheck.schema, "adl.html_observatory.runtime_v3_event_check.v1");
assert.equal(eventCheck.events[0].event, "agent_ready");
assert.equal(api.normalizeEventEntries(eventCheck).length, 1);

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
assert(calls.some((call) => call.url === `${config.api_base}/v1/control` && call.options.method === "POST"));

assert.throws(
  () => api.normalizeTrustedRuntimeV3ApiBase("https://operator:token@localhost:20997"),
  new RegExp(`Runtime v3 selection requires HTTPS for ${config.api_base.replace(/^https:\/\//, "").replace(/:\d+$/, "")}\\.`)
);

await assert.rejects(
  () => api.submitRuntimeV3SignedControlCommand(config.api_base, { schema: "wrong" }),
  /adl.runtime.control_command.v1/
);
})().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

grep -q 'id="roster-search"' "${ROOT_DIR}/demos/html-observatory/index.html"
grep -q 'id="roster-presence-filter"' "${ROOT_DIR}/demos/html-observatory/index.html"
grep -q 'id="roster-sort"' "${ROOT_DIR}/demos/html-observatory/index.html"
grep -q 'id="roster-detail"' "${ROOT_DIR}/demos/html-observatory/index.html"
grep -q 'id="roster-load-more"' "${ROOT_DIR}/demos/html-observatory/index.html"
grep -q 'symbolic-ref", "HEAD' "${ROOT_DIR}/adl-runtime-kernel/build.rs"
grep -q 'track_git_path(&manifest_dir, &symbolic_ref)' "${ROOT_DIR}/adl-runtime-kernel/build.rs"

echo "PASS: HTML Observatory Runtime v3, signed command, and roster projection contract"
