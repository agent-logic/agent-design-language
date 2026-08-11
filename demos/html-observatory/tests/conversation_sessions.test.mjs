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
  assert.equal(index.review?.revision, head, "independent review must cover the exact HEAD");
  assert.equal(index.review?.decision, "approved", "independent exact-head review must be approved");
  console.log("WP-18C.01 exact-head review receipt: PASS");
  process.exit(0);
}

const [html, app] = await Promise.all([
  readFile(new URL("../index.html", testUrl), "utf8"),
  readFile(new URL("../app.js", testUrl), "utf8")
]);
await import("../app.js");
const {
  conversationFrameTransition,
  conversationReconnectIntent,
  conversationReplyFromFrame
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
assert.match(app, /agent\.state === "running"/);
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
for (const status of ["refused", "failed", "timed_out", "cancelled"]) {
  assert.deepEqual(
    conversationFrameTransition({ ...delivered, status, reply: undefined }, pending),
    { status, terminal: true, reply: null }
  );
}
assert.equal(conversationReplyFromFrame(delivered, pending), "Agent response");
assert.equal(conversationFrameTransition(delivered, null), null, "a replay has no pending turn");
assert.equal(conversationFrameTransition({ ...delivered, conversation_id: "conversation-other" }, pending), null);
assert.equal(conversationFrameTransition({ ...delivered, turn_id: "turn-other" }, pending), null);
assert.equal(conversationFrameTransition({ ...delivered, recipient_id: "agent-0002" }, pending), null);
assert.equal(conversationFrameTransition({ ...delivered, correlation_id: "f".repeat(32) }, pending), null);
assert.equal(conversationFrameTransition({ ...delivered, reply: "" }, pending), null);

pending.cancelRequested = true;
assert.deepEqual(conversationFrameTransition({
  ...delivered,
  status: "accepted",
  reply: undefined
}, pending), { status: "cancelling", terminal: false, reply: null });

pending.disconnected = true;
const replay = conversationReconnectIntent(pending);
assert.strictEqual(replay, pending.intent, "reconnect must resend the exact retained intent");
assert.equal(pending.reconnectReplayCount, 1);
assert.equal(conversationReconnectIntent({ ...pending, disconnected: true }), null, "a pending turn may replay at most once");
assert.equal(conversationReconnectIntent({ ...pending, terminal: true, disconnected: true, reconnectReplayCount: 0 }), null);

console.log("WP-18C.01 Observatory conversation contract: PASS");
