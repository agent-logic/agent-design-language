import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const [html, app] = await Promise.all([
  readFile(new URL("../index.html", import.meta.url), "utf8"),
  readFile(new URL("../app.js", import.meta.url), "utf8")
]);
await import("../app.js");
const { conversationReplyFromFrame } = globalThis.AdlHtmlObservatory;

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
assert.match(app, /conversationSend\.disabled = !conversationAuthorized/);
assert.match(app, /agent\.state === "running"/);
assert.match(app, /frame\.status !== "delivered"/);
assert.match(app, /item\.textContent = message/);
assert.doesNotMatch(app, /agent-conversation-key|conversation.*private.*key/i);
assert.doesNotMatch(app, /frame\.result_hash.*appendConversationTurn/);

const pending = {
  turnId: "turn-1",
  recipientId: "agent-0001",
  correlationId: "0123456789abcdef0123456789abcdef"
};
const delivered = {
  schema: "adl.runtime_v3.observatory_conversation_result.v1",
  status: "delivered",
  turn_id: pending.turnId,
  recipient_id: pending.recipientId,
  correlation_id: pending.correlationId,
  reply: "Agent response",
  result_hash: "must-not-render"
};
assert.equal(conversationReplyFromFrame(delivered, pending), "Agent response");
assert.equal(conversationReplyFromFrame(delivered, null), null, "a replay has no pending turn");
assert.equal(conversationReplyFromFrame({ ...delivered, status: "refused" }, pending), null);
assert.equal(conversationReplyFromFrame({ ...delivered, recipient_id: "agent-0002" }, pending), null);
assert.equal(conversationReplyFromFrame({ ...delivered, correlation_id: "f".repeat(32) }, pending), null);
assert.equal(conversationReplyFromFrame({ ...delivered, reply: "" }, pending), null);

console.log("WP-18C.01 Observatory conversation contract: PASS");
