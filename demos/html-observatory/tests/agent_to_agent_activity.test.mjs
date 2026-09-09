import assert from "node:assert/strict";
import test from "node:test";
import { readFile } from "node:fs/promises";

await import(new URL("../app.js", import.meta.url));
const { describeConversationTurn, conversationTurnsInOrder } = globalThis.AdlHtmlObservatory;

// A2A is attributed by two runtime markers taken from the live wuji feed: the
// initiation leg carries public_output.agent_to_agent_initiation, and the reply
// leg is a work item prefixed a2a-work-. There is no sender_id on either leg.
const population = [
  { id: "shepherd", name: "beacon.axioma", label: "Beacon Axioma" },
  { id: "gemma-e4b", name: "ember.axioma", label: "Ember Axioma" }
];

test("initiation leg is A2A, named end-to-end, and shows the message actually sent", () => {
  const described = describeConversationTurn({
    workId: "conversation-0f7f054193179f79bfb56e7534ccbc9c",
    entry: {
      accepted_sequence: 3,
      public_output: {
        recipient_id: "shepherd",
        message: "Requested governed contact with gemma-e4b.",
        agent_to_agent_initiation: {
          schema: "adl.runtime.agent_to_agent_initiation_request.v2",
          recipient_name: "ember.axioma",
          message: "Welcome to Axioma Polis, Ember. I am Beacon Axioma, the Polis Shepherd."
        }
      }
    }
  }, population);
  assert.equal(described.kind, "a2a");
  assert.equal(described.title, "Beacon Axioma → Ember Axioma");
  // The wrapper text is bookkeeping; the initiation payload is the real turn.
  assert.match(described.detail, /Welcome to Axioma Polis/);
  assert.doesNotMatch(described.detail, /Requested governed contact/);
});

test("a2a-work reply leg is A2A without asserting an unnamed counterpart", () => {
  const described = describeConversationTurn({
    workId: "a2a-work-0bd209dc73be97cb",
    entry: {
      accepted_sequence: 7,
      public_output: {
        recipient_id: "gemma-e4b",
        message: "Hello Beacon Axioma. The pleasure is all mine."
      }
    }
  }, population);
  assert.equal(described.kind, "a2a");
  assert.equal(described.title, "Ember Axioma replied");
  assert.match(described.detail, /pleasure is all mine/);
});

test("an initiation payload with the wrong schema is not trusted as A2A", () => {
  const described = describeConversationTurn({
    workId: "conversation-x",
    entry: {
      accepted_sequence: 4,
      public_output: {
        recipient_id: "shepherd",
        message: "hello",
        agent_to_agent_initiation: { schema: "something.else.v1", recipient_id: "gemma-e4b" }
      }
    }
  }, population);
  assert.equal(described.kind, "conversation");
});

test("an ordinary operator turn is never labelled A2A", () => {
  const described = describeConversationTurn({
    workId: "conversation-39f437f5a0d49ee91ea40848c8ea60d2",
    entry: {
      accepted_sequence: 1,
      public_output: { recipient_id: "shepherd", message: "shepherd received your message." }
    }
  }, population);
  assert.equal(described.kind, "conversation");
  assert.equal(described.title, "Beacon Axioma replied");
});

test("an id absent from the roster falls back to the raw id, not a guess", () => {
  // The runtime uses both "shepherd" and "beacon" for the same agent; the
  // Observatory shows what it was given rather than inventing a mapping.
  const described = describeConversationTurn({
    workId: "conversation-y",
    entry: { accepted_sequence: 6, public_output: { recipient_id: "beacon", message: "ok" } }
  }, population);
  assert.equal(described.title, "beacon replied");
});

test("turns are ordered by accepted_sequence, not object key order", () => {
  const ordered = conversationTurnsInOrder({
    "conversation-z": { accepted_sequence: 9 },
    "conversation-a": { accepted_sequence: 2 },
    "conversation-m": { accepted_sequence: 5 }
  });
  assert.deepEqual(ordered.map((t) => t.workId), ["conversation-a", "conversation-m", "conversation-z"]);
});

test("long turns are truncated for the activity row", () => {
  const described = describeConversationTurn({
    workId: "a2a-work-e",
    entry: {
      accepted_sequence: 4,
      public_output: { recipient_id: "gemma-e4b", message: "x".repeat(400) }
    }
  }, population);
  assert.ok(described.detail.length <= 111, "detail must stay a single readable row");
  assert.ok(described.detail.endsWith("…"));
});

test("history replayed on first load is not stamped with a live clock time", async () => {
  const app = await readFile(new URL("../app.js", import.meta.url), "utf8");
  assert.match(app, /pushInspectorActivity\(\{ \.\.\.describeConversationTurn\(turn\), at: null \}\)/);
  assert.match(app, /item\.at === null\s*\n\s*\? "earlier"/);
});

test("A2A rows are visually distinguished in the stylesheet", async () => {
  const css = await readFile(new URL("../styles.css", import.meta.url), "utf8");
  assert.match(css, /\.insp-activity\[data-kind="a2a"\]/);
  assert.match(css, /\.insp-activity-badge/);
});
