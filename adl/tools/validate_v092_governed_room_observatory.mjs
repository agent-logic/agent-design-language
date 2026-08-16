#!/usr/bin/env node

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const appPath = new URL("../../demos/html-observatory/app.js", import.meta.url);
const htmlPath = new URL("../../demos/html-observatory/index.html", import.meta.url);
await import(pathToFileURL(appPath.pathname));

const observatory = globalThis.AdlHtmlObservatory;
assert(observatory, "HTML Observatory exports must be available");

const participants = observatory.normalizeGovernedRoomParticipants({
  sample: [
    { id: "scribe", label: "Scribe", communication_eligible: true, state: "ready", polis_id: "polis-local" },
    { id: "silent", label: "Silent", communication_eligible: false, state: "ready" },
    { id: "shepherd", label: "Shepherd", communication_eligible: true, state: "busy", polis_id: "polis-local" }
  ]
});
assert.deepEqual(participants.map((participant) => participant.participant_id), ["scribe", "shepherd"]);
assert.equal(participants[0].state, "joined");
assert.equal(participants[1].state, "unavailable");

const intent = observatory.buildGovernedRoomTurnIntent({
  roomId: "room-scribe-shepherd",
  turnId: "room-turn-1",
  turnSequence: 7,
  senderId: "operator",
  correlationId: "corr-room-1",
  recipients: ["shepherd", "scribe"],
  message: "coordinate the demo"
});
assert.equal(intent.schema, "adl.runtime.governed_room_turn.v1");
assert.deepEqual(intent.addressed_recipients, ["scribe", "shepherd"], "room recipients must be explicit and stable");
assert.equal(intent.turn_sequence, 7);

for (const recipients of [[], ["all"], ["*"], ["scribe", "scribe"], ["bad recipient"]]) {
  assert.throws(
    () => observatory.buildGovernedRoomTurnIntent({
      roomId: "room-proof",
      turnId: "room-turn-proof",
      turnSequence: 1,
      senderId: "operator",
      correlationId: "corr-proof",
      recipients,
      message: "hello"
    }),
    /implicit_broadcast_denied|duplicate_room_recipient/,
    `recipient set ${JSON.stringify(recipients)} must fail closed`
  );
}
assert.throws(
  () => observatory.buildGovernedRoomTurnIntent({
    roomId: "room-proof",
    turnId: "room-turn-proof",
    turnSequence: 1,
    senderId: "operator",
    correlationId: "corr-proof",
    recipients: ["a", "b", "c", "d", "e", "f", "g", "h", "i"],
    message: "hello"
  }),
  /room_recipient_limit_exceeded/,
  "room recipient sets must remain bounded"
);

const route = observatory.normalizeGovernedRoomRoute({
  schema: "adl.runtime.governed_room_route.v1",
  status: "partial_delivery",
  room_id: "room-scribe-shepherd",
  turn_id: "room-turn-1",
  turn_sequence: 7,
  addressed_recipients: ["shepherd", "scribe"],
  mentions: [
    { schema: "adl.runtime.governed_room_mention.v1", room_id: "room-scribe-shepherd", turn_id: "room-turn-1", recipient_id: "scribe", display_name: "Scribe" },
    { schema: "adl.runtime.governed_room_mention.v1", room_id: "room-scribe-shepherd", turn_id: "room-turn-1", recipient_id: "shepherd", display_name: "Shepherd" }
  ],
  deliveries: [
    { recipient_id: "scribe", state: "delivered" },
    { recipient_id: "shepherd", state: "timed_out", error: "recipient_delivery_timed_out" }
  ]
});
assert.deepEqual(route.addressed_recipients, ["scribe", "shepherd"]);
const rows = observatory.buildGovernedRoomRows(route);
assert.deepEqual(rows.map((row) => row.displayName), ["Scribe", "Shepherd"]);
assert.deepEqual(rows.map((row) => row.state), ["delivered", "timed_out"]);
assert.equal(rows[1].detail, "recipient_delivery_timed_out");

const html = readFileSync(htmlPath, "utf8");
for (const requiredId of [
  "governed-room-status",
  "governed-room-recipients",
  "governed-room-participants",
  "governed-room-transcript",
  "governed-room-message",
  "send-governed-room-turn"
]) {
  assert(html.includes(`id="${requiredId}"`), `HTML must expose #${requiredId}`);
}
assert(html.includes("multiple"), "room recipient select must allow explicit multi-agent selection");
assert(html.includes("Select 1-8 explicit recipients."), "UI must disclose bounded room size");
assert(html.includes("Broadcast and browser-selected implicit participants are denied."), "UI must disclose no implicit broadcast");

console.log(JSON.stringify({
  schema: "adl.issue_115.governed_room_observatory_validation.v1",
  status: "passed",
  cases: [
    "participants_filtered_and_state_mapped",
    "explicit_recipient_intent_sorted",
    "implicit_broadcast_denied",
    "room_recipient_limit_enforced",
    "partial_delivery_rows_attributed",
    "static_room_dom_anchors_present"
  ]
}, null, 2));
