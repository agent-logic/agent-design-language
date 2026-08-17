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
  WebSocket: function WebSocket() {}
};
context.globalThis = context;
vm.createContext(context);
vm.runInContext(appSource, context, { filename: "app.js" });

const api = context.AdlHtmlObservatory;
assert.equal(typeof api.normalizeRuntimeConversationHistorySnapshot, "function");
assert.equal(typeof api.restoreConversationTranscriptFromRuntimeHistory, "function");
assert.equal(typeof api.safeConversationHistoryText, "function");

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

const nonMonotonic = api.normalizeRuntimeConversationHistorySnapshot({
  ...history,
  records: [
    { message_id: "m1", speaker_id: "operator", body: "a", journal_sequence: 2 },
    { message_id: "m2", speaker_id: "agent", body: "b", journal_sequence: 2 }
  ]
}, { runtime_incarnation_id: "runtime-1" });
assert.equal(nonMonotonic.accepted, false);
assert.equal(nonMonotonic.reason, "non_monotonic_runtime_history");

for (const forbidden of ["correlation_id", "result_hash", "private_key", "signature"]) {
  assert.equal(api.safeConversationHistoryText(`contains ${forbidden}`), "[redacted]");
}

console.log("PASS #278 Observatory transcript history validator");
