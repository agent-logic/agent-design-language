import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const testUrl = new URL(import.meta.url);

const [html, app] = await Promise.all([
  readFile(new URL("../index.html", testUrl), "utf8"),
  readFile(new URL("../app.js", testUrl), "utf8")
]);

await import("../app.js");

const {
  normalizeOperatorAttentionRequest,
  operatorAttentionRows,
  operatorAttentionViewModel,
  operatorAttentionActionPayload,
  renderOperatorAttentionInbox
} = globalThis.AdlHtmlObservatory;

for (const id of [
  "operator-attention-inbox",
  "operator-attention-title",
  "operator-attention-count",
  "operator-attention-unread",
  "operator-attention-filter",
  "operator-attention-priority-filter",
  "operator-attention-notifications",
  "operator-attention-list"
]) {
  assert.match(html, new RegExp(`id=["']${id}["']`));
}
for (const id of [
  "operator-attention-inbox",
  "operator-attention-count",
  "operator-attention-unread",
  "operator-attention-filter",
  "operator-attention-priority-filter",
  "operator-attention-notifications",
  "operator-attention-list"
]) {
  const matches = html.match(new RegExp(`id=["']${id}["']`, "g")) || [];
  assert.equal(matches.length, 1, `${id} must be unique`);
}

assert.match(app, /adl\.runtime_v3\.operator_attention\.request\.v1/);
assert.match(app, /adl\.runtime_v3\.operator_attention\.outcome\.v1/);
assert.match(app, /grants_authority: false/);
assert.match(app, /requires_runtime_authorization: true/);
assert.doesNotMatch(app, /operatorAttentionActionPayload[\s\S]*approved: true/);

const attention = {
  schema: "adl.runtime_v3.operator_attention.request.v1",
  request_id: "attn-001",
  source_agent_id: "agent-alpha",
  display_name: "Alpha",
  priority: "urgent",
  status: "open",
  reason: "policy_intervention",
  message: "Need operator attention before proceeding.",
  correlation_id: "corr-001",
  created_at_millis: 200,
  updated_at_millis: 201
};

assert.equal(normalizeOperatorAttentionRequest(attention).priority, "urgent");
assert.equal(normalizeOperatorAttentionRequest({ ...attention, request_id: "" }), null);
assert.equal(normalizeOperatorAttentionRequest({ ...attention, signature: "private" }), null);

const rows = operatorAttentionRows({
  operator_attention_requests: [
    { ...attention, request_id: "attn-low", priority: "low", created_at_millis: 1 },
    { ...attention, request_id: "attn-urgent", priority: "urgent", created_at_millis: 10 },
    { ...attention, request_id: "attn-urgent", priority: "high", updated_at_millis: 9 }
  ]
});
assert.deepEqual(rows.map((row) => row.request_id), ["attn-urgent", "attn-low"]);
assert.equal(rows[0].priority, "urgent", "newer duplicate must not replace fresher urgent state");

const view = operatorAttentionViewModel({
  operator_attention_requests: [
    attention,
    { ...attention, request_id: "attn-002", priority: "normal", status: "acknowledged", created_at_millis: 300 },
    { ...attention, request_id: "attn-003", priority: "high", status: "resolved", created_at_millis: 100 }
  ]
}, {
  statusFilter: "active",
  priorityFilter: "urgent",
  readRequestIds: ["attn-002"],
  locationHash: "#operator-attention-attn-001",
  notificationPreference: "enabled"
});
assert.deepEqual(view.rows.map((row) => row.request_id), ["attn-001"]);
assert.equal(view.rows[0].deep_link, "#operator-attention-attn-001");
assert.equal(view.rows[0].selected, true);
assert.equal(view.rows[0].unread, true);
assert.equal(view.unread_count, 1);
assert.equal(view.notification_enabled, true);
assert.equal(operatorAttentionViewModel({ operator_attention_requests: [attention] }, {
  readRequestIds: ["attn-001"],
  notificationPreference: "muted"
}).notification_enabled, false);

const reply = operatorAttentionActionPayload(attention, {
  action: "reply",
  message: "Acknowledged; Runtime policy still gates authority."
});
assert.deepEqual(reply, {
  schema: "adl.runtime_v3.operator_attention.outcome.v1",
  request_id: "attn-001",
  source_agent_id: "agent-alpha",
  correlation_id: "corr-001",
  outcome: "reply",
  grants_authority: false,
  authority_approved: false,
  operator_intervention_only: true,
  requires_runtime_authorization: true,
  message: "Acknowledged; Runtime policy still gates authority."
});
assert.throws(() => operatorAttentionActionPayload(attention, { action: "approve" }));
assert.equal(renderOperatorAttentionInbox({ operator_attention_requests: [attention] })[0].request_id, "attn-001");

console.log("WP-18C.06 Observatory operator attention inbox: PASS");
