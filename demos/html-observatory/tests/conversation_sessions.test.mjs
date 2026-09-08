import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFile } from "node:fs/promises";

const testUrl = new URL(import.meta.url);
const repoRoot = new URL("../../../", testUrl);

if (process.argv.includes("--review-only")) {
  const index = JSON.parse(await readFile(new URL(".csdlc/issues/111/index.json", repoRoot), "utf8"));
  const head = execFileSync("git", ["rev-parse", "HEAD"], {
    cwd: repoRoot,
    encoding: "utf8"
  }).trim();
  assert.equal(index.review?.completed, true, "independent exact-head review must be complete");
  assert.equal(index.review?.findings?.length, 0, "independent exact-head review must have no findings");
  assert.ok(index.review?.reviewer, "independent exact-head review must name its reviewer");
  const reviewed = /^git-blake3:([0-9a-f]{40}):[0-9a-f]{64}$/
    .exec(index.review?.reviewed_revision || "");
  assert.ok(reviewed, "independent review must retain a typed Git revision and digest");
  const changedAfterReview = execFileSync("git", ["diff", "--name-only", reviewed[1], head], {
    cwd: repoRoot,
    encoding: "utf8"
  }).trim().split("\n").filter(Boolean);
  assert.ok(
    changedAfterReview.every((path) =>
      path.startsWith(".csdlc/issues/111/") ||
      path.startsWith(".csdlc/prepared/issues/111/") ||
      path.startsWith(".csdlc/evidence/111/")),
    `product paths changed after independent review: ${changedAfterReview.join(", ")}`
  );
  console.log("WP-18C.01 exact-head review receipt: PASS");
  process.exit(0);
}

const [html, app] = await Promise.all([
  readFile(new URL("../index.html", testUrl), "utf8"),
  readFile(new URL("../app.js", testUrl), "utf8")
]);
await import("../app.js");
const {
  conversationFrameProvesAcceptance,
  conversationFrameTransition,
  conversationReconnectIntent,
  conversationReplyFromFrame,
  normalizeRuntimeConversationHistorySnapshot,
  restoreConversationTranscriptFromRuntimeHistory
} = globalThis.AdlHtmlObservatory;

for (const id of [
  "agent-conversation-recipient",
  "agent-conversation-transcript",
  "agent-conversation-message",
  "send-agent-conversation",
  "agent-conversation-status"
]) {
  assert.match(html, new RegExp(`id=["']${id}["']`));
}

assert.match(app, /adl\.runtime_v3\.observatory_conversation_intent\.v1/);
assert.match(app, /adl\.runtime_v3\.observatory_conversation_result\.v1/);
assert.match(app, /adl\.runtime_v3\.observatory_conversation_cancel\.v1/);
assert.match(app, /conversationSend\.disabled = !conversationAuthorized/);
assert.match(app, /agent\.communication_eligible === true/);
assert.match(app, /item\.dataset\.turnId = turnId/);
assert.match(app, /pending\.cancelButton\?\.remove\(\)/);
assert.match(app, /pendingConversationTurns\.delete\(frame\.turn_id\)/);
assert.match(app, /if \(!transition\) return;/, "mismatched frames must return before pending deletion");
assert.doesNotMatch(app, /agent-conversation-key|conversation.*private.*key/i);
assert.doesNotMatch(app, /frame\.result_hash.*appendConversationTurn/);

const sendHandler = app.slice(
  app.indexOf("conversationSend?.addEventListener"),
  app.indexOf("const queryApiBase", app.indexOf("conversationSend?.addEventListener"))
);
assert.doesNotMatch(sendHandler, /appendConversationTurn\("operator"/);
assert.match(sendHandler, /conversationStatus\.textContent = "awaiting runtime"/);

const pending = {
  conversationId: "conversation-agent-0001",
  turnId: "turn-1",
  recipientId: "agent-0001",
  correlationId: "0123456789abcdef0123456789abcdef",
  runtimeIncarnationId: "incarnation-a",
  cancelRequested: false,
  disconnected: false,
  reconnectReplayCount: 0,
  terminal: false,
  intent: {
    schema: "adl.runtime_v3.observatory_conversation_intent.v1",
    conversation_id: "conversation-agent-0001",
    turn_id: "turn-1",
    recipient_id: "agent-0001",
    correlation_id: "0123456789abcdef0123456789abcdef",
    message: "Hello"
  }
};
const delivered = {
  schema: "adl.runtime_v3.observatory_conversation_result.v1",
  status: "delivered",
  conversation_id: pending.conversationId,
  turn_id: pending.turnId,
  recipient_id: pending.recipientId,
  correlation_id: pending.correlationId,
  reply: "Agent response",
  result_hash: "must-not-render"
};

assert.deepEqual(
  conversationFrameTransition({ ...delivered, status: "accepted", reply: undefined }, pending),
  { status: "accepted", terminal: false, reply: null }
);
assert.deepEqual(conversationFrameTransition(delivered, pending), {
  status: "delivered",
  terminal: true,
  reply: "Agent response"
});
assert.deepEqual(conversationFrameTransition({
  ...delivered,
  sender_id: "beacon",
  initiated_recipient_id: "ember",
  initiated_correlation_id: "abcdef0123456789abcdef0123456789",
  initiated_work_id: "a2a-work-0123456789abcdef"
}, pending), {
  status: "delivered",
  terminal: true,
  reply: "Agent response",
  senderId: "beacon",
  initiatedRecipientId: "ember",
  initiatedCorrelationId: "abcdef0123456789abcdef0123456789",
  initiatedWorkId: "a2a-work-0123456789abcdef"
});
for (const status of ["refused", "failed", "timed_out", "cancelled"]) {
  assert.deepEqual(
    conversationFrameTransition({ ...delivered, status, reply: undefined }, pending),
    { status, terminal: true, reply: null }
  );
}
assert.equal(conversationReplyFromFrame(delivered, pending), "Agent response");
assert.equal(conversationFrameProvesAcceptance({ ...delivered, turn_sequence: 1 }), true);
assert.equal(conversationFrameProvesAcceptance({ ...delivered, status: "timed_out", reply: undefined, turn_sequence: 2 }), true);
assert.equal(conversationFrameProvesAcceptance({ ...delivered, status: "refused", reply: undefined, turn_sequence: undefined }), false);
assert.equal(conversationFrameTransition(delivered, null), null, "a replay has no pending turn");
assert.equal(conversationFrameTransition({ ...delivered, conversation_id: "conversation-other" }, pending), null);
assert.equal(conversationFrameTransition({ ...delivered, turn_id: "turn-other" }, pending), null);
assert.equal(conversationFrameTransition({ ...delivered, recipient_id: "agent-0002" }, pending), null);
assert.equal(conversationFrameTransition({ ...delivered, correlation_id: "f".repeat(32) }, pending), null);
assert.equal(conversationFrameTransition({ ...delivered, reply: "" }, pending), null);

const a2aHistory = normalizeRuntimeConversationHistorySnapshot({
  schema: "adl.runtime.conversation_history.v1",
  conversation_id: "conversation-agent-0001",
  runtime_incarnation_id: "incarnation-a",
  records: [
    {
      journal_sequence: 1,
      message_id: "turn-1:outbound",
      speaker_id: "operator",
      body: "Please ask Beacon.",
      status: "delivered"
    },
    {
      journal_sequence: 2,
      message_id: "turn-1:reply",
      speaker_id: "agent:beacon",
      body: "I can ask Ember.",
      status: "delivered"
    },
    {
      journal_sequence: 3,
      history_kind: "agent_to_agent_turn",
      message_id: "turn-1:a2a-outbound",
      turn_id: "turn-a2a-1",
      causal_id: "a2a-beacon-ember-1:turn-a2a-1:a2a-work-1",
      sender_id: "beacon",
      recipient_id: "ember",
      work_id: "a2a-work-1",
      parent_conversation_id: "conversation-agent-0001",
      parent_turn_id: "turn-1",
      a2a_role: "outbound",
      speaker_id: "agent:beacon",
      body: "Ember, please answer Beacon.",
      status: "a2a_delivered"
    },
    {
      journal_sequence: 4,
      history_kind: "agent_to_agent_turn",
      message_id: "turn-1:a2a-reply",
      turn_id: "turn-a2a-1",
      causal_id: "a2a-beacon-ember-1:turn-a2a-1:a2a-work-1",
      sender_id: "beacon",
      recipient_id: "ember",
      work_id: "a2a-work-1",
      parent_conversation_id: "conversation-agent-0001",
      parent_turn_id: "turn-1",
      a2a_role: "reply",
      speaker_id: "agent:ember",
      body: "Ember generated a governed response for Beacon.",
      status: "a2a_delivered"
    }
  ]
}, {
  runtime_incarnation_id: "incarnation-a"
});
assert.equal(a2aHistory.accepted, true);
assert.deepEqual(a2aHistory.records.map((record) => record.body), [
  "Please ask Beacon.",
  "I can ask Ember.",
  "Ember, please answer Beacon.",
  "Ember generated a governed response for Beacon."
]);
assert.equal(a2aHistory.records[2].history_kind, "agent_to_agent_turn");
assert.equal(a2aHistory.records[2].a2a_role, "outbound");
assert.equal(a2aHistory.records[2].causal_id, a2aHistory.records[3].causal_id);
const restoredTurns = [];
const restoredHistory = restoreConversationTranscriptFromRuntimeHistory(a2aHistory, {
  runtime_incarnation_id: "incarnation-a"
}, (...args) => restoredTurns.push(args));
assert.equal(restoredHistory.accepted, true);
assert.deepEqual(restoredTurns.map((turn) => turn.slice(0, 4)), [
  ["operator", "Please ask Beacon.", "turn-1:outbound", "delivered"],
  ["agent:beacon", "I can ask Ember.", "turn-1:reply", "delivered"],
  ["agent:beacon", "Ember, please answer Beacon.", "turn-1:a2a-outbound", "a2a_delivered"],
  ["agent:ember", "Ember generated a governed response for Beacon.", "turn-1:a2a-reply", "a2a_delivered"]
]);

pending.cancelRequested = true;
assert.deepEqual(conversationFrameTransition({
  ...delivered,
  status: "accepted",
  reply: undefined
}, pending), { status: "cancelling", terminal: false, reply: null });

pending.disconnected = true;
const replay = conversationReconnectIntent(pending, "incarnation-a");
assert.strictEqual(replay, pending.intent, "reconnect must resend the exact retained intent");
assert.equal(pending.reconnectReplayCount, 1);
assert.equal(conversationReconnectIntent(pending, "incarnation-a"), null, "one disconnect event may replay at most once");
pending.disconnected = true;
assert.strictEqual(
  conversationReconnectIntent(pending, "incarnation-a"),
  pending.intent,
  "a later disconnect may retrieve the same idempotent Runtime result"
);
assert.equal(pending.reconnectReplayCount, 2);
assert.equal(conversationReconnectIntent({ ...pending, terminal: true, disconnected: true, reconnectReplayCount: 0 }, "incarnation-a"), null);

const restartedPending = {
  ...pending,
  terminal: false,
  disconnected: true,
  reconnectReplayCount: 0,
  restartUnavailable: false
};
assert.equal(
  conversationReconnectIntent(restartedPending, "incarnation-b"),
  null,
  "a changed Runtime incarnation must never receive an old pending turn"
);
assert.equal(restartedPending.restartUnavailable, true);
assert.equal(restartedPending.terminal, true);
assert.equal(restartedPending.disconnected, false);
assert.equal(restartedPending.reconnectReplayCount, 0);
assert.equal(
  conversationReconnectIntent({ ...restartedPending, disconnected: true }, "incarnation-b"),
  null,
  "restart-unavailable turns remain terminal"
);

const incarnationUnknown = {
  ...pending,
  terminal: false,
  disconnected: true,
  reconnectReplayCount: 0
};
assert.equal(conversationReconnectIntent(incarnationUnknown, null), null);
assert.equal(incarnationUnknown.terminal, false, "replay waits until the current socket proves its incarnation");
assert.equal(incarnationUnknown.reconnectReplayCount, 0);

console.log("WP-18C.01 Observatory conversation contract: PASS");
