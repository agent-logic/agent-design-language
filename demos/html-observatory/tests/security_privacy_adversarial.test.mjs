import assert from "node:assert/strict";
import { readFile, mkdir, writeFile } from "node:fs/promises";

const testUrl = new URL(import.meta.url);
const repoRoot = new URL("../../../", testUrl);

const [html, app] = await Promise.all([
  readFile(new URL("../index.html", testUrl), "utf8"),
  readFile(new URL("../app.js", testUrl), "utf8")
]);

await import("../app.js");

const {
  authenticateRuntimeV3ObservatorySocket,
  conversationFrameTransition,
  conversationFrameProvesAcceptance,
  conversationReconnectIntent,
  normalizeRuntimeConversationHistorySnapshot,
  safeConversationHistoryText,
  normalizeLayer8DeliveryState,
  hasForbiddenLayer8Disclosure,
  normalizeOperatorAttentionRequest,
  operatorAttentionRows,
  operatorAttentionViewModel,
  operatorAttentionActionPayload,
  normalizeTrustedRuntimeV3ApiBase
} = globalThis.AdlHtmlObservatory;

const adversarial = `<img src=x onerror="globalThis.__adlXss=1"><script>globalThis.__adlXss=2</script>`;

for (const id of [
  "agent-conversation-transcript",
  "agent-conversation-message",
  "operator-attention-list",
  "operator-write-token"
]) {
  assert.match(html, new RegExp(`id=["']${id}["']`), `${id} must be present`);
}

assert.match(app, /function escapeHtml\(value\)/);
assert.match(app, /content\.textContent = message/);
assert.match(app, /state\.textContent = status/);
assert.match(app, /operatorToken\?\.value\.trim\(\)/);
assert.doesNotMatch(app, /localStorage\?\.setItem\("adl\.runtimeV3\.observatoryToken"/);
assert.match(app, /sessionStorage\?\.setItem\("adl\.runtimeV3\.observatoryToken"/);
assert.doesNotMatch(app, /agent-conversation-key|conversation.*private.*key/i);

const safeHistory = normalizeRuntimeConversationHistorySnapshot({
  schema: "adl.runtime.conversation_history.v1",
  conversation_id: "conversation-sec-001",
  runtime_incarnation_id: "incarnation-a",
  records: [
    {
      turn_sequence: 1,
      turn_id: "turn-001",
      speaker_id: "operator",
      body: adversarial,
      status: "delivered"
    },
    {
      turn_sequence: 2,
      turn_id: "turn-002",
      speaker_id: "runtime",
      body: "bearer_token=secret-value",
      status: "private_key leaked"
    },
    {
      turn_sequence: 3,
      turn_id: "turn-003",
      speaker_id: "runtime",
      redacted: true,
      body: "raw provider payload",
      redaction_reason: "provider_payload"
    }
  ]
}, {
  runtime_incarnation_id: "incarnation-a"
});

assert.equal(safeHistory.accepted, true);
assert.equal(safeHistory.records[0].body, adversarial, "text is preserved for textContent-only rendering");
assert.equal(safeHistory.records[1].body, "[redacted]");
assert.equal(safeHistory.records[1].status, "[redacted]");
assert.equal(safeHistory.records[2].body, "[redacted]");
assert.equal(safeHistory.records[2].redacted, true);
assert.equal(
  normalizeRuntimeConversationHistorySnapshot({ ...safeHistory, schema: "unexpected", records: [] }).accepted,
  false,
  "unexpected history schemas are rejected"
);
assert.deepEqual(
  normalizeRuntimeConversationHistorySnapshot({
    schema: "adl.runtime.conversation_history.v1",
    conversation_id: "conversation-sec-001",
    runtime_incarnation_id: "old-incarnation",
    records: []
  }, { runtime_incarnation_id: "incarnation-a" }),
  { accepted: false, reason: "stale_runtime_history" }
);
assert.equal(safeConversationHistoryText("signature:abcdef"), "[redacted]");

const pending = {
  conversationId: "conversation-sec-001",
  turnId: "turn-001",
  recipientId: "agent-alpha",
  correlationId: "corr-alpha",
  runtimeIncarnationId: "incarnation-a",
  cancelRequested: false,
  disconnected: false,
  reconnectReplayCount: 0,
  terminal: false,
  intent: {
    schema: "adl.runtime_v3.observatory_conversation_intent.v1",
    conversation_id: "conversation-sec-001",
    turn_id: "turn-001",
    recipient_id: "agent-alpha",
    correlation_id: "corr-alpha",
    message: "hello"
  }
};
const delivered = {
  schema: "adl.runtime_v3.observatory_conversation_result.v1",
  status: "delivered",
  conversation_id: "conversation-sec-001",
  turn_id: "turn-001",
  recipient_id: "agent-alpha",
  correlation_id: "corr-alpha",
  reply: adversarial,
  turn_sequence: 1
};
assert.deepEqual(conversationFrameTransition(delivered, pending), {
  status: "delivered",
  terminal: true,
  reply: adversarial
});
assert.equal(conversationFrameTransition({ ...delivered, correlation_id: "corr-other" }, pending), null);
assert.equal(conversationFrameTransition({ ...delivered, recipient_id: "agent-other" }, pending), null);
assert.equal(conversationFrameTransition(delivered, null), null, "replay without a pending turn is ignored");
assert.equal(conversationFrameProvesAcceptance({ ...delivered, status: "refused", reply: undefined }), false);
assert.equal(conversationFrameProvesAcceptance({ ...delivered, status: "failed", reply: undefined }), true);

pending.disconnected = true;
assert.strictEqual(conversationReconnectIntent(pending, "incarnation-a"), pending.intent);
pending.disconnected = true;
assert.equal(conversationReconnectIntent({ ...pending, runtimeIncarnationId: "incarnation-a" }, "incarnation-b"), null);

const forbiddenLayer8 = {
  schema: "adl.runtime_v3.layer8.recipient_acknowledgement_response.v1",
  status: "delivered",
  recipient_id: "agent-alpha",
  correlation_hash: "hash-alpha",
  private_key: "must-not-render"
};
assert.equal(hasForbiddenLayer8Disclosure(forbiddenLayer8), true);
assert.equal(normalizeLayer8DeliveryState(forbiddenLayer8).state, "failed");
assert.equal(normalizeLayer8DeliveryState(forbiddenLayer8).correlationHash, "not disclosed");
assert.deepEqual(normalizeLayer8DeliveryState({
  schema: "adl.runtime_v3.layer8.recipient_acknowledgement_response.v1",
  runtime_unavailable: true,
  recipient_id: "agent-alpha",
  correlation_hash: "hash-alpha"
}), {
  state: "recovery",
  terminal: false,
  actionEnabled: false,
  label: "Runtime unavailable",
  detail: "No terminal delivery claim is rendered until Runtime serves a valid acknowledgement response.",
  recipientId: "agent-alpha",
  correlationHash: "hash-alpha",
  generation: null
});

const attention = {
  schema: "adl.runtime_v3.operator_attention.request.v1",
  request_id: "attn-sec-001",
  source_agent_id: "agent-alpha",
  display_name: adversarial,
  priority: "urgent",
  status: "open",
  reason: "policy_intervention",
  message: adversarial,
  correlation_id: "corr-alpha",
  created_at_millis: 1,
  updated_at_millis: 2
};
assert.equal(normalizeOperatorAttentionRequest({ ...attention, private_key: "secret" }), null);
assert.equal(normalizeOperatorAttentionRequest({ ...attention, raw_provider_payload: { text: "secret" } }), null);
const attentionRow = normalizeOperatorAttentionRequest(attention);
assert.equal(attentionRow.message, adversarial, "message is escaped at HTML rendering boundary");
assert.equal(operatorAttentionRows({
  operator_attention_requests: [
    { ...attention, updated_at_millis: 2 },
    { ...attention, priority: "low", updated_at_millis: 1 }
  ]
})[0].priority, "urgent", "stale duplicate cannot downgrade priority");
assert.equal(operatorAttentionViewModel({
  operator_attention_requests: [attention]
}).notification_enabled, true);
assert.deepEqual(operatorAttentionActionPayload(attention, {
  action: "reply",
  message: "No authority granted."
}), {
  schema: "adl.runtime_v3.operator_attention.outcome.v1",
  request_id: "attn-sec-001",
  source_agent_id: "agent-alpha",
  correlation_id: "corr-alpha",
  outcome: "reply",
  grants_authority: false,
  authority_approved: false,
  operator_intervention_only: true,
  requires_runtime_authorization: true,
  message: "No authority granted."
});
assert.throws(() => operatorAttentionActionPayload(attention, { action: "approve" }));

assert.equal(normalizeTrustedRuntimeV3ApiBase("https://runtime.dev.agent-logic.ai:20997"), "https://runtime.dev.agent-logic.ai:20997");
for (const unsafeBase of [
  "http://runtime.dev.agent-logic.ai:20997",
  "https://evil.example:20997",
  "https://token:secret@runtime.dev.agent-logic.ai:20997",
  "https://runtime.dev.agent-logic.ai:20997/path",
  "https://runtime.dev.agent-logic.ai:20997?token=secret"
]) {
  assert.throws(() => normalizeTrustedRuntimeV3ApiBase(unsafeBase));
}

const sent = [];
authenticateRuntimeV3ObservatorySocket({
  readyState: WebSocket.OPEN,
  send(payload) {
    sent.push(JSON.parse(payload));
  }
}, "operator-token");
assert.deepEqual(sent, [{
  schema: "adl.runtime_v3.observatory_ws_auth.v1",
  bearer_token: "operator-token"
}]);
assert.throws(() => authenticateRuntimeV3ObservatorySocket({ readyState: WebSocket.OPEN, send() {} }, ""));

const evidenceDir = new URL(".csdlc/evidence/281/", repoRoot);
await mkdir(evidenceDir, { recursive: true });
await writeFile(new URL("security_privacy_adversarial.json", evidenceDir), JSON.stringify({
  schema: "adl.observatory.security_privacy_adversarial_proof.v1",
  issue: 281,
  source: "demos/html-observatory/tests/security_privacy_adversarial.test.mjs",
  proof: [
    "xss_fixture_text_only",
    "credential_token_redaction",
    "trusted_https_origin_only",
    "replay_confused_deputy_stale_denial_fail_closed",
    "operator_attention_no_authority_grant"
  ],
  public_safe: true,
  contains_secrets: false,
  contains_private_cognition: false,
  contains_raw_provider_payloads: false
}, null, 2));

console.log("WP-18C.07c Observatory security/privacy/adversarial proof: PASS");
