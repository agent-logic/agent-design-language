#!/usr/bin/env node
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const appSource = await readFile(new URL("../../demos/html-observatory/app.js", import.meta.url), "utf8");
const context = {
  console,
  URL,
  URLSearchParams,
  setTimeout,
  clearTimeout,
  globalThis: {},
  document: {
    addEventListener() {},
    getElementById() { return null; },
    querySelector() { return null; },
    querySelectorAll() { return []; }
  },
  window: {
    location: { search: "", protocol: "https:", host: "localhost", pathname: "/" },
    addEventListener() {}
  },
  navigator: {},
  sessionStorage: { getItem() { return null; }, setItem() {} },
  WebSocket: Object.assign(function WebSocket() {}, { OPEN: 1 })
};
context.globalThis = context;
vm.createContext(context);
vm.runInContext(appSource, context, { filename: "app.js" });

const api = context.AdlHtmlObservatory;
assert.equal(typeof api.normalizeRuntimeConversationHistorySnapshot, "function");
assert.equal(typeof api.restoreConversationTranscriptFromRuntimeHistory, "function");
assert.equal(typeof api.safeConversationHistoryText, "function");
assert.equal(typeof api.requestRuntimeConversationHistory, "function");

const historyRequests = [];
api.requestRuntimeConversationHistory({ readyState: 1, send(value) { historyRequests.push(JSON.parse(value)); } }, "conversation-agent-a");
assert.deepEqual(historyRequests, [{
  schema: "adl.runtime_v3.observatory_conversation_history_request.v1",
  conversation_id: "conversation-agent-a",
  page_size: 2048
}]);
assert.equal(typeof api.requestRuntimeConversationHistory, "function");
assert.equal(typeof api.conversationTranscriptRenderKey, "function");

const history = {
  schema: "adl.runtime.conversation_history.v1",
  conversation_id: "conversation-agent-a",
  runtime_incarnation_id: "runtime-1",
  records: [
    {
      message_id: "m1",
      speaker_id: "operator",
      body: "hello",
      journal_sequence: 1,
      redacted: false
    },
    {
      message_id: "m2",
      speaker_id: "agent-a",
      body: "secret",
      journal_sequence: 2,
      redacted: true,
      redaction_reason: "public-safe"
    }
  ]
};

const accepted = api.normalizeRuntimeConversationHistorySnapshot(history, {
  runtime_incarnation_id: "runtime-1"
});
assert.equal(accepted.accepted, true);
assert.equal(accepted.records.length, 2);
assert.equal(accepted.records[1].body, "[redacted]");
assert.equal(accepted.records[1].status, "redacted");

const stale = api.normalizeRuntimeConversationHistorySnapshot(history, {
  runtime_incarnation_id: "runtime-2"
});
assert.equal(stale.accepted, false);
assert.equal(stale.reason, "stale_runtime_history");

const rendered = [];
const restored = api.restoreConversationTranscriptFromRuntimeHistory(
  {
    ...history,
    records: [
      {
        message_id: "m1",
        speaker_id: "operator",
        body: "bearer_token should not render",
        journal_sequence: 1,
        redacted: false
      }
    ]
  },
  { runtime_incarnation_id: "runtime-1" },
  (speaker, body, turnId, status) => rendered.push({ speaker, body, turnId, status })
);
assert.equal(restored.accepted, true);
assert.equal(rendered[0].body, "[redacted]");
assert.equal(rendered[0].status, "restored");

assert.equal(
  api.conversationTranscriptRenderKey("operator", "turn-restore:outbound"),
  api.conversationTranscriptRenderKey("operator", "turn-restore"),
  "restored outbound history and live accepted operator frame must share one render identity"
);
assert.equal(
  api.conversationTranscriptRenderKey("agent:shepherd", "turn-restore:reply"),
  api.conversationTranscriptRenderKey("agent", "turn-restore"),
  "restored reply history and live delivered agent frame must share one render identity"
);

const sent = [];
api.requestRuntimeConversationHistory({
  readyState: 1,
  send(payload) {
    sent.push(JSON.parse(payload));
  }
}, "conversation-shepherd", 2);
assert.deepEqual(sent, [{
  schema: "adl.runtime_v3.observatory_conversation_history_request.v1",
  conversation_id: "conversation-shepherd",
  page_size: 2
}]);
assert.throws(() => api.requestRuntimeConversationHistory({ readyState: 1, send() {} }, "../bad"));
assert.throws(() => api.requestRuntimeConversationHistory({ readyState: 1, send() {} }, "conversation-shepherd", 2049));

const nonMonotonic = api.normalizeRuntimeConversationHistorySnapshot({
  ...history,
  records: [
    { message_id: "m1", speaker_id: "operator", body: "a", journal_sequence: 2 },
    { message_id: "m2", speaker_id: "agent", body: "b", journal_sequence: 2 }
  ]
}, { runtime_incarnation_id: "runtime-1" });
assert.equal(nonMonotonic.accepted, false);
assert.equal(nonMonotonic.reason, "non_monotonic_runtime_history");

const productionHistory = {
  schema: "adl.runtime.conversation_history.v1",
  conversation_id: "conversation-ember",
  runtime_incarnation_id: "runtime-1",
  records: [
    { message_id: "turn-proof:outbound", speaker_id: "operator", body: "hello Ember", journal_sequence: 1, status: "delivered", redacted: false },
    { message_id: "turn-proof:reply", speaker_id: "agent:ember", body: "hello operator", journal_sequence: 2, status: "delivered", redacted: false }
  ],
  next_cursor: null
};
const freshTranscript = new Map();
const appendExactlyOnce = (speaker, body, messageId, status) => {
  const key = `${speaker}:${messageId}`;
  if (!freshTranscript.has(key)) freshTranscript.set(key, { speaker, body, messageId, status });
};
api.restoreConversationTranscriptFromRuntimeHistory(productionHistory, { runtime_incarnation_id: "runtime-1" }, appendExactlyOnce);
api.restoreConversationTranscriptFromRuntimeHistory(productionHistory, { runtime_incarnation_id: "runtime-1" }, appendExactlyOnce);
assert.deepEqual(Array.from(freshTranscript.values()), [
  { speaker: "operator", body: "hello Ember", messageId: "turn-proof:outbound", status: "delivered" },
  { speaker: "agent:ember", body: "hello operator", messageId: "turn-proof:reply", status: "delivered" }
]);

assert.equal(api.normalizeRuntimeConversationHistorySnapshot({
  ...productionHistory,
  records: Array.from({ length: 120 }, (_, index) => ({
    message_id: `m-${index}`,
    speaker_id: index % 2 === 0 ? "operator" : "agent:ember",
    body: "bounded complete retained transcript",
    journal_sequence: index + 1
  }))
}).accepted, true);

assert.equal(api.normalizeRuntimeConversationHistorySnapshot({
  ...productionHistory,
  records: Array.from({ length: 2049 }, (_, index) => ({
    message_id: `m-${index}`,
    speaker_id: "operator",
    body: "bounded",
    journal_sequence: index + 1
  }))
}).accepted, false);

for (const forbidden of ["correlation_id", "result_hash", "private_key", "signature"]) {
  assert.equal(api.safeConversationHistoryText(`contains ${forbidden}`), "[redacted]");
}

console.log("PASS #694 production transcript reload and replay bounds validator");
