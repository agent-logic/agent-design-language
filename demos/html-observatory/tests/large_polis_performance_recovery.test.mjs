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
  estimateLargePolisResourceMetrics,
  largePolisRecoverySequence,
  largePolisRecoveryViewModel,
  pruneLargePolisDomWindow,
  retainedLargePolisWindow
} = globalThis.AdlHtmlObservatory;

const implementationRevision = process.env.ADL_OBSERVATORY_IMPLEMENTATION_REVISION || "bound-by-csdlc-review-assignment";
const fixture = buildLargePolisPerformanceRecoveryFixture({
  agentCount: 2500,
  transcriptTurns: 5000,
  streamEvents: 1200,
  runtimeIncarnationChanged: true,
  candidateRevision: "557dd28d85746a8dc5109dcc674f5a606b8c9890",
  implementationRevision
});
const view = buildPanopticonViewModel(fixture.snapshot);
const metrics = evaluateLargePolisPerformanceRecovery(fixture);

assert.equal(view.agentTotal, 2500);
assert.equal(view.visibleAgentCount, LARGE_POLIS_LIMITS.maxVisibleAgents);
assert.equal(view.agents.length, LARGE_POLIS_LIMITS.maxVisibleAgents);
assert.equal(view.events.length, LARGE_POLIS_LIMITS.maxEventTail);
assert.equal(retainedLargePolisWindow(fixture.transcript).length, LARGE_POLIS_LIMITS.maxTranscriptTurns);

assert.equal(metrics.schema, "adl.html_observatory.large_polis_performance_recovery_metrics.v1");
assert.equal(metrics.candidate_revision, "557dd28d85746a8dc5109dcc674f5a606b8c9890");
assert.equal(metrics.implementation_revision, implementationRevision);
assert.equal(metrics.agent_total, 2500);
assert.equal(metrics.visible_agent_count, LARGE_POLIS_LIMITS.maxVisibleAgents);
assert.equal(metrics.transcript_total_turns, 5000);
assert.equal(metrics.retained_transcript_turns, LARGE_POLIS_LIMITS.maxTranscriptTurns);
assert.equal(metrics.stream_event_total, 1200);
assert.equal(metrics.retained_stream_events, LARGE_POLIS_LIMITS.maxEventTail);
assert.equal(metrics.bounded, true);
assert.equal(metrics.grants_authority, false);
assert.equal(metrics.resource_metrics.bounded_latency, true);
assert.equal(metrics.resource_metrics.bounded_dom_nodes, true);
assert.equal(metrics.resource_metrics.deterministic_projection_millis <= LARGE_POLIS_LIMITS.maxDeterministicProjectionMillis, true);
assert.equal(metrics.resource_metrics.projected_dom_nodes <= LARGE_POLIS_LIMITS.maxProjectedDomNodes, true);

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
assert.equal(metrics.recovery.actions.length, 5);
assert.equal(metrics.recovery.duplicate_action_prevented, false);
assert.equal(metrics.recovery_sequence.recovered, true);
assert.equal(metrics.recovery_sequence.stale_state_hidden, false);
assert.equal(metrics.recovery_sequence.observed_pending_action_count, 5);
assert.equal(metrics.recovery_sequence.resolved_pending_action_count, 5);
assert.equal(metrics.recovery_sequence.pending_action_count, 0);
assert.equal(metrics.recovery_sequence.dropped_pending_actions, 0);
assert.deepEqual(metrics.recovery_sequence.steps[0].pending_actions_after, metrics.recovery.actions);
assert.equal(metrics.recovery_sequence.steps[0].stale_state_visible, true);
assert.deepEqual(metrics.recovery_sequence.steps.at(-1).pending_actions_before, metrics.recovery.actions);
assert.deepEqual(metrics.recovery_sequence.steps.at(-1).resolved_pending_actions, metrics.recovery.actions);
assert.deepEqual(metrics.recovery_sequence.steps.at(-1).pending_actions_after, []);
assert.deepEqual(metrics.recovery_sequence.steps.at(-1).view.status, {
  reconnect: "ready",
  restart: "ready",
  backpressure: "ready",
  offline: "ready",
  versionMismatch: "ready"
});
assert.deepEqual(metrics.recovery_sequence.steps.at(-1).view.actions, []);

const repeatedReconnect = largePolisRecoveryViewModel({
  connected: false,
  runtimeIncarnationChanged: false,
  bufferedMessages: 0,
  offline: false,
  versionMismatch: false
});
assert.deepEqual(repeatedReconnect.actions, ["schedule_single_reconnect"]);
assert.equal(repeatedReconnect.grants_authority, false);

const duplicateSequence = largePolisRecoverySequence([
  { connected: false, bufferedMessages: 0 },
  { connected: true, bufferedMessages: 0 }
]);
assert.equal(duplicateSequence.recovered, true);
assert.equal(duplicateSequence.pending_action_count, 0);
assert.equal(duplicateSequence.dropped_pending_actions, 0);
assert.deepEqual(duplicateSequence.steps.at(-1).resolved_pending_actions, ["schedule_single_reconnect"]);

const unresolvedSequence = largePolisRecoverySequence([
  { connected: false, runtimeIncarnationChanged: true, bufferedMessages: 1200, offline: true, versionMismatch: true }
]);
assert.equal(unresolvedSequence.recovered, false);
assert.equal(unresolvedSequence.pending_action_count, 5);
assert.equal(unresolvedSequence.dropped_pending_actions, 5);

const resourceMetrics = estimateLargePolisResourceMetrics({
  visibleAgents: LARGE_POLIS_LIMITS.maxVisibleAgents,
  retainedTranscriptTurns: LARGE_POLIS_LIMITS.maxTranscriptTurns,
  retainedStreamEvents: LARGE_POLIS_LIMITS.maxEventTail,
  recoveryActions: 5
});
assert.equal(resourceMetrics.bounded_latency, true);
assert.equal(resourceMetrics.bounded_dom_nodes, true);

const removed = [];
const fakeContainer = {
  dataset: {},
  querySelectorAll() {
    return Array.from({ length: LARGE_POLIS_LIMITS.maxTranscriptTurns + 7 }, (_, index) => ({
      remove() {
        removed.push(index);
      }
    }));
  }
};
assert.equal(pruneLargePolisDomWindow(fakeContainer, ".conversation-turn"), 7);
assert.equal(fakeContainer.dataset.retainedTurnCount, String(LARGE_POLIS_LIMITS.maxTranscriptTurns));
assert.equal(fakeContainer.dataset.prunedTurnCount, "7");

const evidenceDir = new URL(".csdlc/evidence/280/", repoRoot);
await mkdir(evidenceDir, { recursive: true });
await writeFile(
  new URL("large_polis_performance_recovery_metrics.json", evidenceDir),
  `${JSON.stringify(metrics, null, 2)}\n`,
  "utf8"
);

console.log("WP-18C.07b large-Polis performance/recovery proof: PASS");
