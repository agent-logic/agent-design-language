import assert from "node:assert/strict";

globalThis.window = undefined;
await import("../app.js");

const { buildRuntimeAgentRows } = globalThis.AdlHtmlObservatory;

const [agent] = buildRuntimeAgentRows({
  status: {
    schema: "adl.runtime_v3.observatory_feed.v3",
    agent_population: {
      total_count: 1,
      sample: [{
        id: "ember",
        name: "ember.axioma",
        label: "Ember Axioma",
        role: "resident agent",
        provider: "ollama",
        model: "gemma4:e4b-mlx",
        last_snapshot_at_unix_millis: 1_786_000_000_000,
        last_archive_at_unix_millis: 1_786_000_001_000,
        snapshot_sequence: 12,
        pending_archive_count: 1,
        snapshot_state: "current",
        archive_state: "pending",
        state: "ready",
        detail: "ready",
        health: "healthy",
        availability: "available",
        communication_eligible: true,
        observed_at_unix_millis: 1_786_000_001_000,
        freshness_deadline_unix_millis: 1_786_000_031_000,
        source_revision: "test",
        provenance: "runtime_component_state"
      }]
    }
  }
});

assert.equal(agent.provider, "ollama");
assert.equal(agent.model, "gemma4:e4b-mlx");
assert.equal(agent.lastSnapshotAtUnixMillis, 1_786_000_000_000);
assert.equal(agent.lastArchiveAtUnixMillis, 1_786_000_001_000);
assert.equal(agent.snapshotSequence, 12);
assert.equal(agent.pendingArchiveCount, 1);
assert.equal(agent.snapshotState, "current");
assert.equal(agent.archiveState, "pending");

const [never] = buildRuntimeAgentRows({
  status: {
    schema: "adl.runtime_v3.observatory_feed.v3",
    agent_population: { total_count: 1, sample: [{ id: "beacon", name: "beacon.axioma" }] }
  }
});
assert.equal(never.provider, null);
assert.equal(never.model, null);
assert.equal(never.lastSnapshotAtUnixMillis, 0);
assert.equal(never.snapshotState, "never_snapshotted");
assert.equal(never.archiveState, "disabled");

console.log("agent continuity observatory projection: PASS");

const incident = { incident_id: "incident-1", resident_id: "ember", state: "open", response_status: "shepherd_unavailable", alert_status: "delivery_failed_retry_pending", next_alert_at_unix_millis: 1000, alert_delivered: false };
const [withIncident] = buildRuntimeAgentRows({ status: { schema: "adl.runtime_v3.observatory_feed.v3", agent_population: {sample:[{id:"ember"}]}, resident_incidents:[incident] }});
assert.deepEqual(withIncident.residentIncident, incident);
assert.equal(never.residentIncident, null);

const history = Array.from({length:12}, (_, n) => ({...incident, incident_id:`incident-${n+1}`, sequence:n+1, state:n===11 ? "open" : "recovered"})).reverse();
const [latest] = buildRuntimeAgentRows({status:{schema:"adl.runtime_v3.observatory_feed.v3",agent_population:{sample:[{id:"ember"}]},resident_incidents:history}});
assert.equal(latest.residentIncident.incident_id, "incident-12");
