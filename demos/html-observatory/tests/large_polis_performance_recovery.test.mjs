import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";

const testUrl = new URL(import.meta.url);
const repoRoot = new URL("../../../", testUrl);

await import("../app.js");

const {
  LARGE_POLIS_LIMITS,
  buildLargePolisPerformanceRecoveryFixture,
  buildPanopticonViewModel,
  evaluateLargePolisPerformanceRecovery,
  largePolisRecoveryViewModel,
  retainedLargePolisWindow
} = globalThis.AdlHtmlObservatory;

const fixture = buildLargePolisPerformanceRecoveryFixture({
  agentCount: 2500,
  transcriptTurns: 5000,
  streamEvents: 1200,
  runtimeIncarnationChanged: true
});
const view = buildPanopticonViewModel(fixture.snapshot);
const metrics = evaluateLargePolisPerformanceRecovery(fixture);

assert.equal(view.agentTotal, 2500);
assert.equal(view.visibleAgentCount, LARGE_POLIS_LIMITS.maxVisibleAgents);
assert.equal(view.agents.length, LARGE_POLIS_LIMITS.maxVisibleAgents);
assert.equal(view.events.length, LARGE_POLIS_LIMITS.maxEventTail);
assert.equal(retainedLargePolisWindow(fixture.transcript).length, LARGE_POLIS_LIMITS.maxTranscriptTurns);

assert.equal(metrics.schema, "adl.html_observatory.large_polis_performance_recovery_metrics.v1");
assert.equal(metrics.agent_total, 2500);
assert.equal(metrics.visible_agent_count, LARGE_POLIS_LIMITS.maxVisibleAgents);
assert.equal(metrics.transcript_total_turns, 5000);
assert.equal(metrics.retained_transcript_turns, LARGE_POLIS_LIMITS.maxTranscriptTurns);
assert.equal(metrics.stream_event_total, 1200);
assert.equal(metrics.retained_stream_events, LARGE_POLIS_LIMITS.maxEventTail);
assert.equal(metrics.bounded, true);
assert.equal(metrics.grants_authority, false);

assert.deepEqual(metrics.recovery.status, {
  reconnect: "degraded",
  restart: "requires_resync",
  backpressure: "throttled",
  offline: "offline",
  versionMismatch: "blocked_until_refresh"
});
assert.equal(metrics.recovery.runtime_authority_required, true);
assert.equal(metrics.recovery.grants_authority, false);
assert.ok(metrics.recovery.transitions.includes("socket_disconnected"));
assert.ok(metrics.recovery.transitions.includes("runtime_incarnation_changed"));
assert.ok(metrics.recovery.transitions.includes("stream_backpressure"));
assert.ok(metrics.recovery.transitions.includes("browser_offline"));
assert.ok(metrics.recovery.transitions.includes("client_runtime_version_mismatch"));
assert.ok(metrics.recovery.actions.length <= LARGE_POLIS_LIMITS.maxPendingRecoveryActions);
assert.equal(metrics.recovery.duplicate_action_prevented, true);

const repeatedReconnect = largePolisRecoveryViewModel({
  connected: false,
  runtimeIncarnationChanged: false,
  bufferedMessages: 0,
  offline: false,
  versionMismatch: false
});
assert.deepEqual(repeatedReconnect.actions, ["schedule_single_reconnect"]);
assert.equal(repeatedReconnect.grants_authority, false);

const evidenceDir = new URL(".csdlc/evidence/280/", repoRoot);
await mkdir(evidenceDir, { recursive: true });
await writeFile(
  new URL("large_polis_performance_recovery_metrics.json", evidenceDir),
  `${JSON.stringify(metrics, null, 2)}\n`,
  "utf8"
);

console.log("WP-18C.07b large-Polis performance/recovery proof: PASS");
