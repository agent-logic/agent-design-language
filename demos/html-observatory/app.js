const FALLBACK_PACKET = {
  schema: "adl.csm_visibility_packet.v1",
  packet_id: "html-observatory-fallback",
  generated_at: "unloaded",
  source: {
    mode: "fallback",
    evidence_level: "fallback_only",
    claim_boundary: "The retained runtime packet did not load; this fallback only preserves the static UI shell."
  },
  manifold: {
    manifold_id: "unknown",
    display_name: "Runtime packet unavailable",
    state: "fallback",
    current_tick: 0,
    health: {
      summary: "Packet load failed; no runtime proof is claimed from fallback content."
    }
  },
  kernel: {
    pulse: { status: "fallback", completed_through_event_sequence: 0 },
    service_states: []
  },
  citizens: [],
  freedom_gate: {
    recent_docket: [],
    allow_count: 0,
    defer_count: 0,
    refuse_count: 0
  },
  invariants: [],
  trace: { trace_tail: [] },
  operator_actions: {
    available_actions: [],
    disabled_actions: [
      {
        action: "runtime_mutation",
        reason: "Disabled unless a retained ADL runtime packet is loaded."
      }
    ]
  },
  review: {
    primary_artifacts: [],
    caveats: ["Fallback shell is not runtime evidence."]
  }
};

const formatLabel = (value) =>
  String(value ?? "unknown")
    .replaceAll("_", " ")
    .replaceAll("-", " ");

const asArray = (value) => (Array.isArray(value) ? value : []);

function formatTimestampLabel(value) {
  if (!value) {
    return "retained";
  }
  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) {
    return String(value);
  }
  return parsed.toLocaleString([], {
    month: "short",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit"
  });
}

function formatCurrentTimestampLabel() {
  return formatTimestampLabel(new Date());
}

let livePollTimer = null;
let retainedPollTimer = null;
let liveReconnectTimer = null;
let liveReconnectAttempt = 0;
const OBSERVATORY_VERSION = "Runtime v3";
const OBSERVATORY_MANIFOLD_LABEL = `${OBSERVATORY_VERSION} CSM runtime mirror`;
const OBSERVATORY_PACKET_LABEL = `${OBSERVATORY_VERSION} Observatory proof packet`;
const RUNTIME_V3_DEFAULT_TRUSTED_HOST = "wuji.dev.csm.agent-logic.ai";
const RUNTIME_V3_DEFAULT_CONFIG = Object.freeze({
  api_base: `https://${RUNTIME_V3_DEFAULT_TRUSTED_HOST}:20997`,
  trusted_hosts: [RUNTIME_V3_DEFAULT_TRUSTED_HOST],
  health_endpoint: "/v1/health",
  observatory_endpoint: "/v1/observatory?schema=v3",
  readiness_endpoint: "/v1/ready",
  observatory_websocket_endpoint: "/v1/observatory/ws?schema=v3",
  signed_command_endpoint: "/v1/control",
  observatory_docs_endpoint: "/v1/observatory/docs/"
});
const RUNTIME_V3_OBSERVATORY_SCHEMA = "adl.runtime_v3.observatory_feed.v3";
const RUNTIME_V3_OBSERVATORY_WS_AUTH_SCHEMA = "adl.runtime_v3.observatory_ws_auth.v1";
const LARGE_POLIS_LIMITS = Object.freeze({
  maxVisibleAgents: 120,
  maxTranscriptTurns: 300,
  maxEventTail: 240,
  maxPendingRecoveryActions: 5,
  maxProjectedDomNodes: 1300,
  maxDeterministicProjectionMillis: 120
});
let runtimeV3Config = { ...RUNTIME_V3_DEFAULT_CONFIG };
const rosterUiState = {
  filter: "",
  presence: "all",
  sort: "name",
  selectedId: null,
  runtimeInstanceId: null,
  runtimeIncarnationId: null,
  revision: 0,
  eventCursor: null,
  resyncCount: 0,
  lastResyncReason: null
};

function publishRosterCursorState() {
  if (typeof document === "undefined") return;
  const root = document.documentElement;
  root.dataset.agentRosterRevision = String(rosterUiState.revision);
  root.dataset.agentRosterCursorPresent = rosterUiState.eventCursor ? "true" : "false";
  root.dataset.agentRosterResyncCount = String(rosterUiState.resyncCount);
  root.dataset.agentRosterResyncReason = rosterUiState.lastResyncReason || "none";
}
let lastPanopticonSnapshot = null;
let lastPanopticonPacket = FALLBACK_PACKET;

function acceptRuntimeRosterSnapshot(snapshot) {
  const runtimeInstanceId = snapshot?.status?.runtime_id || null;
  const runtimeIncarnationId = snapshot?.status?.runtime_incarnation_id || null;
  const revision = Number(snapshot?.status?.agent_population?.revision || 0);
  const eventCursor = snapshot?.status?.agent_population?.event_cursor;
  if (!runtimeInstanceId || !runtimeIncarnationId || !Number.isSafeInteger(revision) || revision < 0) return false;
  if (revision > 0 && (typeof eventCursor !== "string" || eventCursor.length === 0)) return false;
  if (
    rosterUiState.runtimeInstanceId !== runtimeInstanceId
    || rosterUiState.runtimeIncarnationId !== runtimeIncarnationId
  ) {
    rosterUiState.runtimeInstanceId = runtimeInstanceId;
    rosterUiState.runtimeIncarnationId = runtimeIncarnationId;
    rosterUiState.revision = revision;
    rosterUiState.eventCursor = eventCursor || null;
    rosterUiState.selectedId = null;
    rosterUiState.lastResyncReason = "runtime_incarnation_changed";
    rosterUiState.resyncCount += 1;
    publishRosterCursorState();
    return true;
  }
  if (revision <= rosterUiState.revision) return false;
  if (eventCursor === rosterUiState.eventCursor) return false;
  if (revision !== rosterUiState.revision + 1) {
    rosterUiState.lastResyncReason = "revision_gap";
    rosterUiState.resyncCount += 1;
  } else {
    rosterUiState.lastResyncReason = null;
  }
  rosterUiState.revision = revision;
  rosterUiState.eventCursor = eventCursor;
  rosterUiState.selectedId = null;
  publishRosterCursorState();
  return true;
}

function runtimeRosterCursorState() {
  return { ...rosterUiState };
}

function normalizeRuntimeV3Endpoint(value, fallback) {
  const endpoint = String(value || "").trim();
  return endpoint.startsWith("/") && !endpoint.startsWith("//") ? endpoint : fallback;
}

function hostFromApiBase(value) {
  try {
    return new URL(normalizeApiBase(value)).hostname.toLowerCase();
  } catch (_error) {
    return "";
  }
}

function normalizeRuntimeV3TrustedHosts(value, fallback = RUNTIME_V3_DEFAULT_CONFIG.trusted_hosts) {
  const rawHosts = Array.isArray(value) ? value : [];
  const hosts = rawHosts
    .map((host) => String(host || "").trim().toLowerCase())
    .filter((host) => /^[a-z0-9][a-z0-9.-]*[a-z0-9]$/.test(host));
  const unique = [...new Set(hosts)];
  return unique.length > 0 ? unique : [...fallback];
}

function applyRuntimeV3Config(config = {}) {
  const configuredApiBase = config.api_base || config.default_api_base;
  const configuredHost = hostFromApiBase(configuredApiBase);
  const trustedHosts = normalizeRuntimeV3TrustedHosts(
    config.trusted_hosts || config.trusted_runtime_hosts,
    configuredHost ? [configuredHost] : RUNTIME_V3_DEFAULT_CONFIG.trusted_hosts
  );
  const apiBase = normalizeRuntimeV3ConfigApiBase(configuredApiBase, trustedHosts);
  runtimeV3Config = {
    api_base: apiBase || RUNTIME_V3_DEFAULT_CONFIG.api_base,
    trusted_hosts: trustedHosts,
    health_endpoint: normalizeRuntimeV3Endpoint(
      config.health_endpoint,
      RUNTIME_V3_DEFAULT_CONFIG.health_endpoint
    ),
    observatory_endpoint: normalizeRuntimeV3Endpoint(
      config.observatory_endpoint,
      RUNTIME_V3_DEFAULT_CONFIG.observatory_endpoint
    ),
    readiness_endpoint: normalizeRuntimeV3Endpoint(
      config.readiness_endpoint,
      RUNTIME_V3_DEFAULT_CONFIG.readiness_endpoint
    ),
    observatory_websocket_endpoint: normalizeRuntimeV3Endpoint(
      config.observatory_websocket_endpoint,
      RUNTIME_V3_DEFAULT_CONFIG.observatory_websocket_endpoint
    ),
    signed_command_endpoint: normalizeRuntimeV3Endpoint(
      config.signed_command_endpoint,
      RUNTIME_V3_DEFAULT_CONFIG.signed_command_endpoint
    ),
    observatory_docs_endpoint: normalizeRuntimeV3Endpoint(
      config.observatory_docs_endpoint,
      RUNTIME_V3_DEFAULT_CONFIG.observatory_docs_endpoint
    )
  };
  return runtimeV3Config;
}

function normalizeRuntimeV3ConfigApiBase(value, trustedHosts = getRuntimeV3Config().trusted_hosts) {
  try {
    return value ? normalizeTrustedRuntimeV3ApiBase(value, trustedHosts) : "";
  } catch (_error) {
    return "";
  }
}

function getRuntimeV3Config() {
  return { ...runtimeV3Config };
}

const AWS_LINKAGES = [
  {
    issue: 4684,
    label: "Heartbeat publisher",
    state: "closed",
    proof: "Live CloudWatch heartbeat proof retained for the Agent Logic AWS profile.",
    evidence: "wp08_heartbeat_4684/live_heartbeat_summary.json"
  },
  {
    issue: 4685,
    label: "ACIP to SNS",
    state: "closed",
    proof: "Issue is closed and routed as the ACIP-SNS linkage lane for this demo.",
    evidence: "GitHub issue #4685"
  },
  {
    issue: 4686,
    label: "AWS signal integration",
    state: "open",
    proof: "Full runtime AWS signal bridge remains visible as pending integration work.",
    evidence: "GitHub issue #4686"
  },
  {
    issue: 4687,
    label: "Local polis SSM operations",
    state: "closed",
    proof: "Local polis SSM operations issue is closed and available as the SSM linkage lane.",
    evidence: "GitHub issue #4687"
  },
  {
    issue: 4688,
    label: "S3 ObsMem archive",
    state: "open",
    proof: "Community-memory archive policy remains planned until archive proof is retained.",
    evidence: "GitHub issue #4688"
  }
];

function parseCloudWatchEventMessage(event) {
  if (!event || typeof event !== "object") {
    return {};
  }
  const message = event.message;
  if (typeof message !== "string") {
    return {};
  }
  try {
    const parsed = JSON.parse(message);
    return parsed && typeof parsed === "object" ? parsed : {};
  } catch (_error) {
    return {};
  }
}

function buildViewModel(packet, reportText = "") {
  const safePacket = packet && typeof packet === "object" ? packet : FALLBACK_PACKET;
  const citizens = asArray(safePacket.citizens);
  const services = asArray(safePacket.kernel?.service_states);
  const decisions = asArray(safePacket.freedom_gate?.recent_docket);
  const invariants = asArray(safePacket.invariants);
  const traceTail = asArray(safePacket.trace?.trace_tail);
  const availableActions = asArray(safePacket.operator_actions?.available_actions);
  const disabledActions = asArray(safePacket.operator_actions?.disabled_actions);
  const artifacts = asArray(safePacket.review?.primary_artifacts);
  const caveats = asArray(safePacket.review?.caveats);
  const latestEvent = traceTail.reduce(
    (max, event) => Math.max(max, Number(event.event_sequence || 0)),
    0
  );

  return {
    packet: safePacket,
    reportText,
    citizens,
    services,
    decisions,
    invariants,
    traceTail,
    availableActions,
    disabledActions,
    artifacts,
    caveats,
    latestEvent,
    decisionCounts: {
      allow: safePacket.freedom_gate?.allow_count ?? decisions.filter((item) => item.decision === "allow").length,
      defer: safePacket.freedom_gate?.defer_count ?? decisions.filter((item) => item.decision === "defer").length,
      refuse: safePacket.freedom_gate?.refuse_count ?? decisions.filter((item) => item.decision === "refuse").length
    }
  };
}

function buildIntegrationViewModel({
  serviceManifest = {},
  apiText = "",
  cloudwatchSummary = {},
  cloudwatchEvents = {},
  acipSnsSummary = {},
  snsResourceSummary = {}
} = {}) {
  const events = asArray(cloudwatchEvents.events);
  const parsedEvents = events.map(parseCloudWatchEventMessage).filter((event) => Object.keys(event).length > 0);
  const latestEvent = parsedEvents.at(-1) || {};
  const cloudwatch = cloudwatchSummary.cloudwatch || {};
  const heartbeat = cloudwatchSummary.heartbeat || {};
  const redaction = cloudwatchSummary.redaction || {};
  const acipProjection = acipSnsSummary.acip_projection || {};
  const acipSns = acipSnsSummary.sns || {};
  const snsResource = snsResourceSummary.sns || {};
  const acipRedaction = acipSnsSummary.redaction || {};
  const acipRetainsFullAccountSha = Boolean(acipSnsSummary.aws_account_sha256 || snsResourceSummary.aws_account_sha256);
  const acipRedactionSafe =
    acipRedaction.credentials_recorded === false &&
    acipRedaction.raw_message_content_recorded === false &&
    !acipRetainsFullAccountSha;

  return {
    serviceManifest,
    apiText,
    cloudwatchSummary,
    cloudwatchEvents,
    acipSnsSummary,
    snsResourceSummary,
    parsedEvents,
    latestEvent,
    serviceRows: [
      {
        label: "Runtime owner",
        value: serviceManifest.runtime_owner || "unknown",
        detail: serviceManifest.csm_bin ? "Standalone csm binary owns runtime service startup." : "Service manifest did not record csm binary ownership.",
        state: serviceManifest.runtime_owner === "csm" ? "closed" : "open"
      },
      {
        label: "Service manager",
        value: serviceManifest.manager || "unknown",
        detail: serviceManifest.label || "No service label recorded.",
        state: serviceManifest.manager ? "closed" : "open"
      },
      {
        label: "Local API",
        value: apiText.includes("csm api serve --spec <agent-spec.yaml>") ? "csm api serve" : "not detected",
        detail: apiText.includes("/status") ? "/status, /health, /ready, /metrics, and /events documented." : "Expected CSM API endpoints were not detected.",
        state: apiText.includes("csm api serve --spec <agent-spec.yaml>") ? "closed" : "open"
      }
    ],
    cloudwatchRows: [
      {
        label: "Heartbeat status",
        value: cloudwatchSummary.status || "unknown",
        detail: `${heartbeat.signal_kind || "signal"} / ${heartbeat.runtime_id || "runtime unknown"}`,
        state: cloudwatchSummary.status || "open"
      },
      {
        label: "CloudWatch target",
        value: cloudwatch.log_group || "unknown log group",
        detail: `${cloudwatch.log_stream || "unknown stream"} / ${cloudwatch.event_count ?? events.length} events`,
        state: cloudwatch.target_kind === "cloudwatch_logs" || heartbeat.target_kind === "cloudwatch_logs" ? "passed" : "open"
      },
      {
        label: "Redaction posture",
        value: redaction.credentials_recorded === false ? "operations safe" : "needs review",
        detail: redaction.raw_account_id_recorded === false ? "No raw account id or credentials recorded in retained summary." : "Retained summary needs redaction review.",
        state: redaction.credentials_recorded === false && redaction.raw_account_id_recorded === false ? "passed" : "blocked"
      }
    ],
    acipRows: [
      {
        label: "ACIP projection",
        value: acipSnsSummary.status || "unknown",
        detail: `${acipProjection.signal_kind || "signal unknown"} / ${acipProjection.route_class || "route unknown"}; retained proof passed redaction hygiene.`,
        state: acipSnsSummary.status === "passed" && !acipRetainsFullAccountSha ? "passed" : "blocked"
      },
      {
        label: "SNS topic",
        value: acipSns.topic_name || snsResource.topic_name || "unknown topic",
        detail: acipSns.message_id ? `Retained SNS message ${acipSns.message_id}.` : "No retained SNS message id loaded.",
        state: acipSns.message_id ? "passed" : "open"
      },
      {
        label: "Redaction",
        value: acipRedactionSafe ? "operations safe" : "needs review",
        detail: acipRedactionSafe ? "No raw credentials, account id, topic ARN, private ACIP content, or full account SHA retained." : "Retained proof needs redaction review before operations-safe claim.",
        state: acipRedactionSafe ? "passed" : "blocked"
      }
    ]
  };
}

function setText(id, value) {
  const target = document.getElementById(id);
  if (target) {
    target.textContent = value;
  }
}

function setState(id, value) {
  const target = document.getElementById(id);
  if (target) {
    target.dataset.state = stateTone(value);
  }
}

function setHref(id, value) {
  const target = document.getElementById(id);
  if (target) {
    target.href = value;
  }
}

function setDataset(id, key, value) {
  const target = document.getElementById(id);
  if (target) {
    target.dataset[key] = value;
  }
}

function renderRows(targetId, rows) {
  const target = document.getElementById(targetId);
  if (target) {
    target.innerHTML = rows.join("");
  }
}

function escapeHtml(value) {
  return String(value ?? "")
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

const GOVERNED_ROOM_TURN_SCHEMA = "adl.runtime.governed_room_turn.v1";
const GOVERNED_ROOM_ROUTE_SCHEMA = "adl.runtime.governed_room_route.v1";
const GOVERNED_ROOM_MENTION_SCHEMA = "adl.runtime.governed_room_mention.v1";
const MAX_GOVERNED_ROOM_RECIPIENTS = 8;

function isSafeGovernedRoomIdentifier(value) {
  return typeof value === "string" &&
    value.length > 0 &&
    value.length <= 128 &&
    /^[A-Za-z0-9_.-]+$/.test(value);
}

function normalizeGovernedRoomParticipants(population) {
  return asArray(population?.sample)
    .filter((agent) => agent && typeof agent.id === "string" && agent.communication_eligible === true)
    .map((agent) => ({
      participant_id: agent.id,
      display_name: agent.label || agent.id,
      polis_id: agent.polis_id || agent.polis || "runtime-local",
      state: agent.state === "ready" || agent.state === "available" ? "joined" : "unavailable",
      policy_eligible: true
    }))
    .sort((left, right) => left.participant_id.localeCompare(right.participant_id));
}

function normalizeExplicitGovernedRoomRecipients(recipients) {
  const unique = new Set();
  for (const recipient of asArray(recipients).map((value) => String(value || "").trim())) {
    if (!isSafeGovernedRoomIdentifier(recipient) || recipient === "*" || recipient.toLowerCase() === "all") {
      throw new Error("implicit_broadcast_denied");
    }
    if (unique.has(recipient)) {
      throw new Error("duplicate_room_recipient");
    }
    unique.add(recipient);
  }
  if (unique.size === 0) {
    throw new Error("implicit_broadcast_denied");
  }
  if (unique.size > MAX_GOVERNED_ROOM_RECIPIENTS) {
    throw new Error("room_recipient_limit_exceeded");
  }
  return [...unique].sort();
}

function governedRoomIdentityForRecipients(recipients) {
  const addressedRecipients = normalizeExplicitGovernedRoomRecipients(recipients);
  return {
    roomId: `room-${addressedRecipients.join("-")}`,
    addressedRecipients
  };
}

function nextGovernedRoomTurnSequence(sequenceByRoom, roomId) {
  if (!(sequenceByRoom instanceof Map) || !isSafeGovernedRoomIdentifier(roomId)) {
    throw new Error("invalid_room_turn");
  }
  const current = sequenceByRoom.get(roomId) || 1;
  if (!Number.isSafeInteger(current) || current < 1) {
    throw new Error("invalid_room_turn");
  }
  sequenceByRoom.set(roomId, current + 1);
  return current;
}

function buildGovernedRoomTurnIntent({
  roomId,
  turnId,
  turnSequence = 1,
  senderId = "operator",
  correlationId,
  recipients = [],
  message = ""
} = {}) {
  const addressedRecipients = normalizeExplicitGovernedRoomRecipients(recipients);
  const trimmedMessage = String(message || "").trim();
  if (!isSafeGovernedRoomIdentifier(roomId) ||
      !isSafeGovernedRoomIdentifier(turnId) ||
      !isSafeGovernedRoomIdentifier(senderId) ||
      typeof correlationId !== "string" ||
      correlationId.length === 0 ||
      correlationId.length > 128 ||
      !Number.isSafeInteger(turnSequence) ||
      turnSequence < 1 ||
      trimmedMessage.length === 0 ||
      trimmedMessage.length > 4096) {
    throw new Error("invalid_room_turn");
  }
  return {
    schema: GOVERNED_ROOM_TURN_SCHEMA,
    room_id: roomId,
    turn_id: turnId,
    turn_sequence: turnSequence,
    sender_id: senderId,
    correlation_id: correlationId,
    addressed_recipients: addressedRecipients,
    message: trimmedMessage
  };
}

function normalizeGovernedRoomRoute(route = {}) {
  const addressedRecipients = normalizeExplicitGovernedRoomRecipients(route.addressed_recipients || []);
  const mentions = asArray(route.mentions).map((mention) => ({
    schema: mention.schema || GOVERNED_ROOM_MENTION_SCHEMA,
    room_id: String(mention.room_id || route.room_id || ""),
    turn_id: String(mention.turn_id || route.turn_id || ""),
    recipient_id: String(mention.recipient_id || ""),
    display_name: String(mention.display_name || mention.recipient_id || "unknown")
  })).filter((mention) => addressedRecipients.includes(mention.recipient_id));
  const deliveries = asArray(route.deliveries).map((delivery) => ({
    recipient_id: String(delivery.recipient_id || ""),
    state: String(delivery.state || "timed_out"),
    error: delivery.error ? String(delivery.error) : null
  })).filter((delivery) => addressedRecipients.includes(delivery.recipient_id));
  return {
    schema: route.schema || GOVERNED_ROOM_ROUTE_SCHEMA,
    status: String(route.status || "accepted"),
    room_id: String(route.room_id || ""),
    turn_id: String(route.turn_id || ""),
    turn_sequence: Number.isSafeInteger(route.turn_sequence) ? route.turn_sequence : 0,
    addressed_recipients: addressedRecipients,
    mentions,
    deliveries,
    error: route.error ? String(route.error) : null
  };
}

function buildGovernedRoomRows(route = {}) {
  const normalized = normalizeGovernedRoomRoute(route);
  const deliveryByRecipient = new Map(normalized.deliveries.map((delivery) => [delivery.recipient_id, delivery]));
  return normalized.addressed_recipients.map((recipientId) => {
    const mention = normalized.mentions.find((candidate) => candidate.recipient_id === recipientId);
    const delivery = deliveryByRecipient.get(recipientId);
    return {
      recipientId,
      displayName: mention?.display_name || recipientId,
      state: delivery?.state || normalized.status,
      detail: delivery?.error || normalized.error || `room turn ${normalized.turn_sequence || "pending"}`
    };
  });
}
const LAYER8_RECIPIENT_ACK_ENDPOINT = "/v1/layer8/recipient-acknowledgement";
const LAYER8_RECIPIENT_ACK_RESPONSE_SCHEMA =
  "adl.runtime_v3.layer8.recipient_acknowledgement_response.v1";
const LAYER8_FORBIDDEN_DISCLOSURE_FIELDS = new Set([
  "acknowledgement",
  "correlation_id",
  "ed25519",
  "policy",
  "private_key",
  "proof_hash",
  "provider_payload",
  "raw_correlation_id",
  "signed_request",
  "signature"
]);

function hasForbiddenLayer8Disclosure(value) {
  if (value == null || typeof value !== "object") {
    return false;
  }
  return Object.entries(value).some(([key, nested]) =>
    LAYER8_FORBIDDEN_DISCLOSURE_FIELDS.has(String(key).toLowerCase()) ||
    hasForbiddenLayer8Disclosure(nested)
  );
}

function safeLayer8Value(value, fallback = "not disclosed") {
  const text = String(value ?? "").trim();
  return /^[a-zA-Z0-9._:-]{1,160}$/.test(text) ? text : fallback;
}

function normalizeLayer8DeliveryState(input = {}) {
  const response = input && typeof input === "object" ? input : {};
  const schemaOk = response.schema === LAYER8_RECIPIENT_ACK_RESPONSE_SCHEMA;
  const status = String(response.status || response.delivery || "").toLowerCase();
  const error = String(response.error || response.reason || "");
  const correlationHash = safeLayer8Value(response.correlation_hash, "hash unavailable");
  const recipientId = safeLayer8Value(response.recipient_id, "recipient hidden");
  const generation = Number.isSafeInteger(response.recipient_credential_generation)
    ? response.recipient_credential_generation
    : null;
  const disclosureBlocked = hasForbiddenLayer8Disclosure(response);

  if (response.runtime_unavailable === true || status === "unavailable" || error === "runtime_unavailable") {
    return {
      state: "recovery",
      terminal: false,
      actionEnabled: false,
      label: "Runtime unavailable",
      detail: "No terminal delivery claim is rendered until Runtime serves a valid acknowledgement response.",
      recipientId,
      correlationHash,
      generation: null
    };
  }
  if (!schemaOk || disclosureBlocked) {
    return {
      state: "failed",
      terminal: true,
      actionEnabled: false,
      label: "Malformed response",
      detail: disclosureBlocked
        ? "Runtime response contained private or raw authority material and was blocked."
        : "Runtime response did not match the recipient-acknowledgement schema.",
      recipientId,
      correlationHash: "not disclosed",
      generation: null
    };
  }
  if (error === "credential_revoked" || status === "revoked") {
    return {
      state: "revoked",
      terminal: true,
      actionEnabled: false,
      label: "Credential revoked",
      detail: "Runtime demoted the action after credential revocation.",
      recipientId,
      correlationHash,
      generation: null
    };
  }
  if (status === "delivered") {
    return {
      state: "delivered",
      terminal: true,
      actionEnabled: Boolean(response.action_released),
      label: response.action_released ? "Delivered / action released" : "Delivered",
      detail: response.action_released
        ? "Runtime verified delivery and released the operator action."
        : "Runtime verified delivery; no release flag was present.",
      recipientId,
      correlationHash,
      generation
    };
  }
  if (status === "refused") {
    return {
      state: "refused",
      terminal: true,
      actionEnabled: false,
      label: "Signed refusal",
      detail: `Runtime verified a signed refusal${error ? `: ${safeLayer8Value(error, "redacted")}` : "."}`,
      recipientId,
      correlationHash,
      generation: null
    };
  }
  if (status === "failed") {
    return {
      state: "failed",
      terminal: true,
      actionEnabled: false,
      label: "Verification failed",
      detail: `Runtime failed the acknowledgement${error ? `: ${safeLayer8Value(error, "redacted")}` : "."}`,
      recipientId,
      correlationHash,
      generation: null
    };
  }
  return {
    state: "recovery",
    terminal: false,
    actionEnabled: false,
    label: "Runtime unavailable",
    detail: "No terminal delivery claim is rendered until Runtime serves a valid acknowledgement response.",
    recipientId,
    correlationHash,
    generation: null
  };
}

function layer8DeliveryRows(responses = []) {
  return asArray(responses).map((response) => normalizeLayer8DeliveryState(response));
}

function renderLayer8DeliveryPanel(responses = []) {
  if (typeof document === "undefined") {
    return layer8DeliveryRows(responses);
  }
  const root = document.querySelector(".ops-command");
  if (!root) {
    return [];
  }
  let panel = document.getElementById("layer8-delivery-panel");
  if (!panel) {
    panel = document.createElement("section");
    panel.className = "layer8-delivery-panel";
    panel.id = "layer8-delivery-panel";
    panel.setAttribute("aria-labelledby", "layer8-delivery-title");
    panel.innerHTML = `
      <div class="panel-head">
        <div>
          <p class="eyebrow">Layer 8 / recipient acknowledgement</p>
          <h2 id="layer8-delivery-title">Delivery state</h2>
        </div>
        <span class="mini-badge" id="layer8-delivery-count">0 states</span>
      </div>
      <ol class="layer8-delivery-list" id="layer8-delivery-list" aria-live="polite"></ol>
    `;
    root.append(panel);
  }
  const rows = layer8DeliveryRows(responses);
  setText("layer8-delivery-count", `${rows.length} states`);
  renderRows("layer8-delivery-list", rows.map((row) => `
    <li class="layer8-delivery-row" data-state="${escapeHtml(row.state)}">
      <span class="mini-badge" data-tone="${escapeHtml(row.state === "delivered" ? "ok" : row.state === "recovery" ? "warn" : "blocked")}">${escapeHtml(row.label)}</span>
      <span><strong>${escapeHtml(row.recipientId)}</strong><br><span class="row-detail">${escapeHtml(row.detail)}</span></span>
      <span class="row-detail">correlation hash: ${escapeHtml(row.correlationHash)}</span>
    </li>
  `));
  return rows;
}

async function submitLayer8RecipientAcknowledgement(apiBase, signedPair) {
  const base = normalizeApiBase(apiBase);
  if (!base) {
    throw new Error("Runtime API base is required.");
  }
  const response = await fetch(`${base}${LAYER8_RECIPIENT_ACK_ENDPOINT}`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(signedPair)
  });
  return normalizeLayer8DeliveryState(await response.json());
}

const OPERATOR_ATTENTION_REQUEST_SCHEMA = "adl.runtime_v3.operator_attention.request.v1";
const OPERATOR_ATTENTION_OUTCOME_SCHEMA = "adl.runtime_v3.operator_attention.outcome.v1";
const OPERATOR_ATTENTION_STATUS = new Set(["open", "acknowledged", "replied", "deferred", "resolved", "refused", "expired"]);
const OPERATOR_ATTENTION_PRIORITY_WEIGHT = {
  urgent: 4,
  high: 3,
  normal: 2,
  low: 1
};
const OPERATOR_ATTENTION_FORBIDDEN_FIELDS = new Set([
  "authority",
  "capability",
  "ed25519",
  "private_key",
  "raw_provider_payload",
  "signed_request",
  "signature"
]);

function hasForbiddenOperatorAttentionDisclosure(value) {
  if (value == null || typeof value !== "object") {
    return false;
  }
  return Object.entries(value).some(([key, nested]) =>
    OPERATOR_ATTENTION_FORBIDDEN_FIELDS.has(String(key).toLowerCase()) ||
    hasForbiddenOperatorAttentionDisclosure(nested)
  );
}

function normalizeOperatorAttentionRequest(input = {}) {
  const request = input && typeof input === "object" ? input : {};
  if (request.schema !== OPERATOR_ATTENTION_REQUEST_SCHEMA) {
    return null;
  }
  const requestId = safeLayer8Value(request.request_id || request.id, "");
  const sourceAgentId = safeLayer8Value(request.source_agent_id || request.agent_id, "unknown-agent");
  const status = String(request.status || "open").toLowerCase();
  const priority = String(request.priority || "normal").toLowerCase();
  const message = safeConversationHistoryText(request.message || request.summary || "Operator attention requested.");
  if (!requestId || hasForbiddenOperatorAttentionDisclosure(request)) {
    return null;
  }
  return {
    schema: OPERATOR_ATTENTION_REQUEST_SCHEMA,
    request_id: requestId,
    source_agent_id: sourceAgentId,
    display_name: safeConversationHistoryText(request.display_name || request.source_display_name || sourceAgentId),
    status: OPERATOR_ATTENTION_STATUS.has(status) ? status : "open",
    priority: OPERATOR_ATTENTION_PRIORITY_WEIGHT[priority] ? priority : "normal",
    reason: safeLayer8Value(request.reason || "clarification", "clarification"),
    message,
    correlation_id: safeLayer8Value(request.correlation_id || request.correlation_hash || "redacted", "redacted"),
    related_conversation_id: request.related_conversation_id ? safeLayer8Value(request.related_conversation_id) : null,
    related_work_id: request.related_work_id ? safeLayer8Value(request.related_work_id) : null,
    created_at_millis: Number.isSafeInteger(request.created_at_millis) ? request.created_at_millis : 0,
    updated_at_millis: Number.isSafeInteger(request.updated_at_millis) ? request.updated_at_millis : 0
  };
}

function operatorAttentionRows(packet = {}) {
  const candidates = asArray(packet.operator_attention_requests || packet.operator_attention?.requests);
  const byId = new Map();
  for (const candidate of candidates) {
    const row = normalizeOperatorAttentionRequest(candidate);
    if (!row) continue;
    const existing = byId.get(row.request_id);
    if (!existing || row.updated_at_millis >= existing.updated_at_millis) {
      byId.set(row.request_id, row);
    }
  }
  return [...byId.values()].sort((left, right) =>
    (OPERATOR_ATTENTION_PRIORITY_WEIGHT[right.priority] - OPERATOR_ATTENTION_PRIORITY_WEIGHT[left.priority]) ||
    (left.created_at_millis - right.created_at_millis) ||
    left.request_id.localeCompare(right.request_id)
  );
}

function operatorAttentionViewModel(packet = {}, options = {}) {
  const statusFilter = String(options.statusFilter || "active").toLowerCase();
  const priorityFilter = String(options.priorityFilter || "all").toLowerCase();
  const readRequestIds = new Set(asArray(options.readRequestIds).map(String));
  const selectedHash = String(options.locationHash || "").replace(/^#/, "");
  const notificationPreference = String(options.notificationPreference || "enabled").toLowerCase();
  const rows = operatorAttentionRows(packet)
    .map((row) => {
      const read = readRequestIds.has(row.request_id) || row.status !== "open";
      const anchor = `operator-attention-${row.request_id}`;
      return {
        ...row,
        read,
        unread: !read,
        deep_link: `#${anchor}`,
        anchor,
        selected: selectedHash === anchor
      };
    })
    .filter((row) => {
      const statusOk = statusFilter === "all" ||
        (statusFilter === "active" && !["resolved", "refused", "expired"].includes(row.status)) ||
        row.status === statusFilter;
      const priorityOk = priorityFilter === "all" || row.priority === priorityFilter;
      return statusOk && priorityOk;
    });
  const unreadCount = rows.filter((row) => row.unread).length;
  return {
    rows,
    active_count: rows.filter((row) => !["resolved", "refused", "expired"].includes(row.status)).length,
    unread_count: unreadCount,
    notification_preference: notificationPreference,
    notification_enabled: notificationPreference !== "muted" && unreadCount > 0
  };
}

function operatorAttentionActionPayload(request, outcome = {}) {
  const row = normalizeOperatorAttentionRequest(request);
  if (!row) {
    throw new Error("valid operator attention request is required");
  }
  const action = String(outcome.action || outcome.status || "acknowledge").toLowerCase();
  if (!["acknowledge", "reply", "defer", "resolve", "refuse"].includes(action)) {
    throw new Error("unsupported operator attention outcome");
  }
  const payload = {
    schema: OPERATOR_ATTENTION_OUTCOME_SCHEMA,
    request_id: row.request_id,
    source_agent_id: row.source_agent_id,
    correlation_id: row.correlation_id,
    outcome: action,
    grants_authority: false,
    authority_approved: false,
    operator_intervention_only: true,
    requires_runtime_authorization: true
  };
  if (action === "reply" || action === "refuse") {
    payload.message = safeConversationHistoryText(outcome.message || outcome.reason || "");
    if (!payload.message) {
      throw new Error("operator attention outcome message required");
    }
  }
  if (action === "defer") {
    payload.until_millis = Number.isSafeInteger(outcome.until_millis) ? outcome.until_millis : null;
    if (!payload.until_millis) {
      throw new Error("operator attention defer time required");
    }
  }
  return payload;
}

function renderOperatorAttentionInbox(packet = {}) {
  if (typeof document === "undefined") {
    return operatorAttentionViewModel(packet).rows;
  }
  const statusFilter = document.getElementById("operator-attention-filter")?.value || "active";
  const priorityFilter = document.getElementById("operator-attention-priority-filter")?.value || "all";
  const notifications = document.getElementById("operator-attention-notifications");
  const readRequestIds = JSON.parse(globalThis.localStorage?.getItem("adl.operatorAttention.readRequestIds") || "[]");
  const view = operatorAttentionViewModel(packet, {
    statusFilter,
    priorityFilter,
    readRequestIds,
    locationHash: globalThis.location?.hash || "",
    notificationPreference: notifications?.checked === false ? "muted" : "enabled"
  });
  const rows = view.rows;
  const list = document.getElementById("operator-attention-list");
  const count = document.getElementById("operator-attention-count");
  const unread = document.getElementById("operator-attention-unread");
  if (!list) {
    return rows;
  }
  for (const control of [
    document.getElementById("operator-attention-filter"),
    document.getElementById("operator-attention-priority-filter"),
    notifications
  ]) {
    if (control && !control.dataset.operatorAttentionBound) {
      control.dataset.operatorAttentionBound = "true";
      control.addEventListener("change", () => renderOperatorAttentionInbox(packet));
    }
  }
  if (count) count.textContent = `${view.active_count} active`;
  if (unread) unread.textContent = `${view.unread_count} unread`;
  renderRows("operator-attention-list", rows.length ? rows.map((row) => `
    <li class="operator-attention-row" id="${escapeHtml(row.anchor)}" data-priority="${escapeHtml(row.priority)}" data-status="${escapeHtml(row.status)}" data-read="${escapeHtml(row.read ? "true" : "false")}" data-selected="${escapeHtml(row.selected ? "true" : "false")}">
      <span class="mini-badge" data-tone="${escapeHtml(row.priority === "urgent" ? "blocked" : row.priority === "high" ? "warn" : "ok")}">${escapeHtml(row.priority)}</span>
      <span><strong>${escapeHtml(row.display_name)}</strong><br><span class="row-detail">${escapeHtml(row.reason)} · ${escapeHtml(row.message)}</span></span>
      <span class="row-detail">${escapeHtml(row.status)} · <a href="${escapeHtml(row.deep_link)}">${escapeHtml(row.request_id)}</a></span>
    </li>
  `) : [`<li class="conversation-empty">No agent is currently requesting operator attention.</li>`]);
  return rows;
}

function buildOperatorEnvelope({ channel = "events", message = "", packetId = "", acipSnsSummary = {}, snsResourceSummary = {} } = {}) {
  const acipProjection = acipSnsSummary.acip_projection || {};
  const acipSns = acipSnsSummary.sns || {};
  const snsResource = snsResourceSummary.sns || {};
  const sanitizedMessage = String(message || "").slice(0, 800);
  return {
    schema: "adl.html_observatory.operator_message.v1",
    channel,
    intent: "operator_communication",
    delivery: "prepared_client_side",
    runtime_mutation_claimed: false,
    packet_id: packetId,
    message: sanitizedMessage,
    acip_message: {
      schema: "acip.message.v1",
      sender: "html-observatory-operator",
      recipient: channel === "events" ? "csm-runtime-api" : "csm-runtime-owner",
      mode: channel === "events" ? "read_request" : "projection_request",
      ordering: "client_draft",
      visibility: "operator_visible",
      traceability: "prepared_envelope_only",
      authority_granted: false,
      content_summary: sanitizedMessage || "No operator message supplied."
    },
    aws_projection: channel === "acip_sns" ? {
      schema: "adl.runtime.aws_signal.v1",
      signal_kind: "acip_projection",
      route_class: acipProjection.route_class || "cross_boundary_deferred",
      target_kind: "sns",
      topic_name: acipSns.topic_name || snsResource.topic_name || "unknown",
      retained_message_id: acipSns.message_id || null,
      retained_proof_status: acipSnsSummary.status || "unknown",
      retained_hygiene_issue: null,
      live_publish_claimed: false
    } : null,
    allowed_live_check: channel === "events" ? "/events" : null
  };
}

function renderEnvelope(envelope) {
  const target = document.getElementById("message-envelope");
  if (target) {
    target.textContent = JSON.stringify(envelope, null, 2);
  }
}

const DASHBOARD_FOCUS = {
  runtime: {
    kicker: "Runtime",
    title: "Runtime mirror",
    status: "active",
    target: "#runtime-proof",
    focusTarget: "#hero-ready-state",
    detail: "Runtime readiness, event tail, CloudWatch proof, and retained/live mode are visible in the fixed dashboard.",
    facts: ["Readiness and kernel state", "Event preview and gauges", "Retained/live mode status"]
  },
  agents: {
    kicker: "Agents",
    title: "CSM polis topology",
    status: "map ready",
    target: "#panopticon",
    focusTarget: "#hero-agent-map",
    detail: "Agent roster, scheduler, telemetry, event stream, and checkpoint lanes are mirrored in the panopticon map.",
    facts: ["Role-specific topology icons", "Agent roster summary", "Health and signal lanes"]
  },
  "csm-api": {
    kicker: "CSM API",
    title: "Local control plane",
    status: "public reads + governed writes",
    target: "#csm-api",
    focusTarget: "#hero-api-list",
    detail: "Runtime state is publicly readable; login and a trusted signature are required for control writes.",
    facts: ["/v1/health, /v1/metrics, /v1/observatory", "Signed /v1/control commands", "Authenticated full-duplex WSS"]
  },
  cloudwatch: {
    kicker: "AWS",
    title: "CloudWatch heartbeat",
    status: "source-linked",
    target: "#cloudwatch",
    focusTarget: "#hero-cloudwatch-state",
    detail: "CloudWatch rows and event-tail evidence are loaded from retained redacted AWS proof artifacts.",
    facts: ["WP-08 heartbeat proof", "Redacted event tail", "No browser AWS write authority"]
  },
  communication: {
    kicker: "Operator communication",
    title: "ACIP/SNS envelope",
    status: "prepared",
    target: "#communication",
    focusTarget: ".compact-composer",
    detail: "Comms prepare ACIP messages and mirror retained SNS proof; live AWS mutation remains runtime-owned.",
    facts: ["ACIP message draft", "SNS projection proof", "Redaction hygiene passed"]
  },
  governance: {
    kicker: "Governance",
    title: "Freedom gate",
    status: "bounded",
    target: "#governance",
    focusTarget: "#hero-governance-state",
    detail: "Decision, invariant, and proposal-only action surfaces preserve the packet claim boundary.",
    facts: ["Freedom gate decisions", "Runtime invariants", "Proposal-only actions"]
  },
  evidence: {
    kicker: "Evidence",
    title: "Proof packet",
    status: "linked",
    target: "#evidence",
    focusTarget: "#packet-link",
    detail: "Packet, operator report, CSM API proof, metrics mirror, and CloudWatch artifacts remain source-linked.",
    facts: ["Visibility packet", "Operator report", "CSM/AWS proof refs"]
  }
};

function updateDashboardFocus(key = "runtime", extraDetail = "") {
  const selected = DASHBOARD_FOCUS[key] || DASHBOARD_FOCUS.runtime;
  const root = document.querySelector(".observatory");
  if (root) root.dataset.dashboardSurface = key === "agents" ? "agents" : "runtime";
  setText("dashboard-focus-kicker", selected.kicker);
  setText("dashboard-focus-title", selected.title);
  setText("dashboard-focus-status", selected.status);
  setState("dashboard-focus-status", selected.status);
  setText("dashboard-focus-detail", extraDetail || selected.detail);
  setHref("dashboard-focus-link", selected.focusTarget);
  setText("dashboard-focus-link", `View ${selected.title}`);
  renderRows("dashboard-focus-list", asArray(selected.facts).map((fact) => `
    <span class="dashboard-focus-item">${escapeHtml(fact)}</span>
  `));
  document.querySelectorAll("[data-dashboard-link]").forEach((link) => {
    const isActive = link.dataset.dashboardLink === key;
    if (isActive) {
      link.setAttribute("aria-current", "page");
    } else {
      link.removeAttribute("aria-current");
    }
  });
}

function bindDashboardNavigation(packet = FALLBACK_PACKET) {
  document.querySelectorAll("[data-dashboard-link]").forEach((link) => {
    link.addEventListener("click", (event) => {
      event.preventDefault();
      const key = link.dataset.dashboardLink || "runtime";
      updateDashboardFocus(key);
      const selected = DASHBOARD_FOCUS[key] || DASHBOARD_FOCUS.runtime;
      globalThis.history?.replaceState(null, "", selected.target);
      const focusTarget = key === "agents"
        ? document.getElementById("panopticon")
        : document.getElementById("dashboard-focus-panel");
      focusTarget?.setAttribute("tabindex", "-1");
      focusTarget?.focus({ preventScroll: true });
      if (key === "communication") {
        document.getElementById("prepare-envelope")?.click();
      }
    });
  });

  document.getElementById("dashboard-focus-link")?.addEventListener("click", (event) => {
    event.preventDefault();
    const target = document.querySelector(document.getElementById("dashboard-focus-link")?.getAttribute("href") || "#hero-ready-state");
    if (target) {
      target.setAttribute("tabindex", "-1");
      target.focus();
    }
  });

  document.getElementById("export-proof")?.addEventListener("click", () => {
    const manifest = {
      schema: "adl.html_observatory.export_manifest.v1",
      version: OBSERVATORY_VERSION,
      packet_id: displayPacketId(packet.packet_id || ""),
      exported_at: new Date().toISOString(),
      runtime_mode: document.getElementById("statusbar-mode")?.textContent || "unknown",
      runtime_status: document.getElementById("dashboard-live-test-status")?.textContent || "unknown",
      csm_api_base: document.getElementById("dashboard-live-api-base")?.value || "",
      cloudwatch_status: document.getElementById("cloudwatch-status")?.textContent || "unknown",
      communication_status: document.getElementById("communication-status")?.textContent || "unknown",
      mutation_claimed: false
    };
    const blob = new Blob([JSON.stringify(manifest, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = "adl-html-observatory-proof-manifest.json";
    document.body.appendChild(link);
    link.click();
    link.remove();
    globalThis.setTimeout(() => URL.revokeObjectURL(url), 1000);
    updateDashboardFocus("evidence", "Export prepared a local proof manifest from the visible dashboard state.");
  });

  updateDashboardFocus("runtime");
}

function normalizeApiBase(value) {
  return String(value || "").trim().replace(/\/+$/, "");
}

function displayManifoldId(_value) {
  return OBSERVATORY_MANIFOLD_LABEL;
}

function displayPacketId(_value) {
  return OBSERVATORY_PACKET_LABEL;
}

function displayClaimBoundary(source = {}) {
  const evidenceLevel = formatLabel(source.evidence_level || "bounded local runtime capture");
  return `${OBSERVATORY_VERSION} Observatory consumes a ${evidenceLevel} from runtime-owned artifacts, supports public monitoring and runtime-governed signed writes, and does not claim browser-owned authority or v0.92 coherence.`;
}

function displayMilestoneText(value) {
  return String(value ?? "")
    .replaceAll("v0.91.6", OBSERVATORY_VERSION)
    .replaceAll("v0916", "runtime-v3")
    .replaceAll("v0.91.7", OBSERVATORY_VERSION)
    .replaceAll("v0917", "runtime-v3");
}

function isLoopbackApiBase(value) {
  const base = normalizeApiBase(value);
  try {
    const parsed = new URL(base);
    return (
      ["http:", "https:"].includes(parsed.protocol) &&
      parsed.hostname === "localhost"
    );
  } catch (_error) {
    return false;
  }
}

function getQueryApiBase() {
  const params = new URLSearchParams(window.location.search);
  const candidate = params.get("runtimeApiBase") || params.get("csmApiBase") || params.get("apiBase") || "";
  if (requestedRuntimeSelection() === "v3" && !candidate) {
    return getRuntimeV3Config().api_base;
  }
  const normalized = normalizeApiBase(candidate);
  if (requestedRuntimeSelection() === "v3") {
    return isRuntimeV3ApiBase(normalized) ? normalizeTrustedRuntimeV3ApiBase(normalized) : "";
  }
  return isLoopbackApiBase(normalized) ? normalized : "";
}

function requestedRuntimeSelection() {
  const params = new URLSearchParams(window.location.search);
  const explicit = params.get("runtime") || params.get("runtimeSelection");
  if (explicit) {
    return String(explicit).toLowerCase();
  }
  if (params.get("csmApiBase")) {
    return "v2";
  }
  return "v3";
}

function isRuntimeV3ApiBase(value) {
  try {
    normalizeTrustedRuntimeV3ApiBase(value);
    return true;
  } catch (_error) {
    return false;
  }
}

function normalizeTrustedRuntimeV3ApiBase(value, trustedHosts = getRuntimeV3Config().trusted_hosts) {
  const base = normalizeApiBase(value);
  const parsed = new URL(base);
  const observatoryHost = String(globalThis.location?.hostname || "").toLowerCase();
  const allowedHosts = normalizeRuntimeV3TrustedHosts(trustedHosts);
  const allowedHost = allowedHosts.includes(parsed.hostname.toLowerCase())
    || (observatoryHost && parsed.hostname === observatoryHost);
  if (
    parsed.protocol !== "https:" ||
    !allowedHost ||
    parsed.username ||
    parsed.password ||
    parsed.pathname !== "/" ||
    parsed.search ||
    parsed.hash
  ) {
    throw new Error("Runtime v3 selection requires HTTPS for a configured Runtime host or this Observatory host.");
  }
  return parsed.origin;
}

function shouldAutoConnectLive() {
  const params = new URLSearchParams(window.location.search);
  return ["1", "true", "live", "connect"].includes(String(params.get("live") || params.get("connect") || "").toLowerCase());
}

async function checkEventsEndpoint(apiBase) {
  const base = normalizeApiBase(apiBase);
  if (!base) {
    throw new Error("Enter a Runtime v3 or loopback CSM API base first.");
  }
  if (requestedRuntimeSelection() === "v3") {
    if (!isRuntimeV3ApiBase(base)) {
      throw new Error("Runtime v3 event checks require HTTPS for a configured Runtime host.");
    }
    const snapshot = await fetchRuntimeV3ObservatorySnapshot(base);
    return {
      schema: "adl.html_observatory.runtime_v3_event_check.v1",
      events: normalizeEventEntries(snapshot.events)
    };
  }
  if (!isLoopbackApiBase(base)) {
    throw new Error("Only loopback CSM API bases are allowed.");
  }
  const response = await fetch(`${base}/events`, { method: "GET" });
  if (!response.ok) {
    throw new Error(`/events returned ${response.status}`);
  }
  return response.json();
}

async function fetchRuntimeEndpoint(apiBase, endpoint) {
  const base = normalizeApiBase(apiBase);
  if (!base) {
    throw new Error("Enter a loopback CSM API base first.");
  }
  if (!isLoopbackApiBase(base)) {
    throw new Error("Only loopback CSM API bases are allowed.");
  }
  const response = await fetch(`${base}${endpoint}`, { method: "GET" });
  if (!response.ok) {
    throw new Error(`${endpoint} returned ${response.status}`);
  }
  return response.json();
}

async function fetchRuntimeSnapshot(apiBase) {
  if (requestedRuntimeSelection() === "v3") {
    if (!isRuntimeV3ApiBase(apiBase)) {
      throw new Error("Runtime v3 selection requires a configured HTTPS runtime API base.");
    }
    return fetchRuntimeV3ObservatorySnapshot(apiBase);
  }
  const endpoints = ["/status", "/health", "/ready", "/metrics", "/events"];
  const settled = await Promise.allSettled(
    endpoints.map((endpoint) => fetchRuntimeEndpoint(apiBase, endpoint))
  );
  const snapshot = {
    mode: "live",
    fetchedAt: new Date().toISOString(),
    errors: {}
  };
  endpoints.forEach((endpoint, index) => {
    const key = endpoint.slice(1);
    const result = settled[index];
    if (result.status === "fulfilled") {
      snapshot[key] = result.value;
    } else {
      snapshot.errors[key] = result.reason instanceof Error ? result.reason.message : "unknown error";
    }
  });
  return snapshot;
}

async function fetchRuntimeV3ObservatorySnapshot(apiBase) {
  const base = normalizeTrustedRuntimeV3ApiBase(apiBase);
  const config = getRuntimeV3Config();
  const [observatoryResponse, readiness, health] = await Promise.all([
    fetch(`${base}${config.observatory_endpoint}`, { method: "GET" }),
    fetchRuntimeV3Readiness(base),
    fetchRuntimeV3Health(base)
  ]);
  if (observatoryResponse.status !== 200) {
    throw new Error(`${config.observatory_endpoint} returned ${observatoryResponse.status}`);
  }
  const feed = await observatoryResponse.json();
  return runtimeV3SnapshotFromFeed(feed, readiness, health);
}

async function fetchRuntimeV3AgentRosterPage(apiBase, pageToken, eventCursor = null, pageSize = 50) {
  const base = normalizeTrustedRuntimeV3ApiBase(apiBase);
  const url = new URL(`${base}/v1/agents`);
  url.searchParams.set("page_size", String(pageSize));
  if (pageToken) url.searchParams.set("page_token", pageToken);
  if (eventCursor) url.searchParams.set("event_cursor", eventCursor);
  const response = await fetch(url, { method: "GET" });
  if (!response.ok) throw new Error(`/v1/agents returned ${response.status}`);
  const page = await response.json();
  if (page.schema !== "adl.runtime_v3.agent_roster_page.v1") {
    throw new Error("Runtime returned an unsupported roster page");
  }
  return page;
}

async function authenticateRuntimeRosterSuccessor(apiBase, snapshot) {
  const population = snapshot?.status?.agent_population;
  const revision = Number(population?.revision || 0);
  if (
    rosterUiState.runtimeInstanceId !== snapshot?.status?.runtime_id
    || rosterUiState.runtimeIncarnationId !== snapshot?.status?.runtime_incarnation_id
    || revision !== rosterUiState.revision + 1
    || !rosterUiState.eventCursor
  ) return;
  const authenticated = await fetchRuntimeV3AgentRosterPage(
    apiBase,
    null,
    rosterUiState.eventCursor,
    100
  );
  if (authenticated.revision !== revision || authenticated.event_cursor !== population.event_cursor) {
    throw new Error("Runtime roster cursor authentication mismatch");
  }
}

async function fetchRuntimeV3AgentDetail(apiBase, agentId) {
  const base = normalizeTrustedRuntimeV3ApiBase(apiBase);
  const response = await fetch(`${base}/v1/agents/${encodeURIComponent(agentId)}`, { method: "GET" });
  if (!response.ok) throw new Error(`/v1/agents/{agent_id} returned ${response.status}`);
  const detail = await response.json();
  if (detail.schema !== "adl.runtime_v3.agent_roster_entry.v1" || detail.id !== agentId) {
    throw new Error("Runtime returned an incompatible agent detail");
  }
  return detail;
}

async function fetchRuntimeV3Readiness(base) {
  const endpoint = getRuntimeV3Config().readiness_endpoint;
  const response = await fetch(`${base}${endpoint}`, { method: "GET" });
  if (response.status !== 200) {
    throw new Error(`${endpoint} returned ${response.status}`);
  }
  return response.json();
}

async function fetchRuntimeV3Health(base) {
  const endpoint = getRuntimeV3Config().health_endpoint;
  const response = await fetch(`${base}${endpoint}`, { method: "GET" });
  if (response.status !== 200) {
    throw new Error(`${endpoint} returned ${response.status}`);
  }
  return response.json();
}

async function submitRuntimeV3SignedControlCommand(apiBase, command) {
  const base = normalizeTrustedRuntimeV3ApiBase(apiBase);
  if (!command || command.schema !== "adl.runtime.control_command.v1") {
    throw new Error("Expected an adl.runtime.control_command.v1 signed envelope.");
  }
  const endpoint = getRuntimeV3Config().signed_command_endpoint;
  const response = await fetch(`${base}${endpoint}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(command)
  });
  const payload = await response.json().catch(() => ({
    schema: "adl.runtime.control_error.v1",
    code: "invalid_response"
  }));
  if (!response.ok) {
    const code = payload?.code || `HTTP ${response.status}`;
    throw new Error(`/v1/control rejected the signed command: ${code}`);
  }
  return payload;
}

function runtimeV3SnapshotFromFeed(feed, readiness = null, healthReport = null) {
  if (feed.schema !== RUNTIME_V3_OBSERVATORY_SCHEMA) {
    throw new Error(`Unsupported Runtime v3 Observatory schema: ${feed.schema || "missing"}`);
  }
  const polisIdentity = projectPolisIdentity(feed.polis_identity);
  const snapshot = feed.health?.snapshot || {};
  const weather = feed.weather || {};
  const weatherFreshness = feed.weather_freshness || {};
  const hasReadiness = typeof readiness?.ready === "boolean";
  const degradedReasons = asArray(readiness?.degraded_reasons);
  const events = asArray(feed.events);
  return {
    mode: "live",
    runtimeSelection: feed.runtime_selection || "runtime_v3_explicit_opt_in",
    polisIdentity,
    fetchedAt: new Date().toISOString(),
    status: {
      schema: feed.schema,
      runtime_owner: "runtime-v3",
      runtime_id: feed.runtime_instance_id,
      runtime_incarnation_id: feed.runtime_incarnation_id,
      agent_instance_id: feed.runtime_instance_id,
      agent_population: feed.agents,
      status: snapshot.lifecycle || "unknown",
      observability: snapshot.observability,
      topology_generation: snapshot.topology_generation,
      control: feed.control,
      proof: feed.proof
    },
    health: {
      status: feed.health?.observability_ready ? "healthy" : "pending",
      summary: "Runtime v3 observatory feed",
      components: snapshot.components || {},
      queues: snapshot.queues || {},
      runtime_api: healthReport || null
    },
    ready: {
      schema: readiness?.schema,
      status: hasReadiness ? (readiness.ready ? "ready" : "degraded") : (snapshot.observability_ready ? "ready" : "pending"),
      ready: hasReadiness ? readiness.ready === true : snapshot.observability_ready === true,
      state: hasReadiness ? (readiness.ready ? "ready" : "degraded") : (snapshot.observability_ready ? "ready" : "pending"),
      blocking_reasons: hasReadiness ? degradedReasons : (snapshot.observability_ready ? [] : ["observability_not_ready"]),
      weather_freshness: readiness?.weather_freshness || weatherFreshness
    },
    metrics: {
      gauges: {
        event_count: snapshot.event_count || events.length,
        component_count: Object.keys(snapshot.components || {}).length,
        agent_count: feed.agents?.total_count ?? null,
        agent_sample_count: feed.agents?.rendered_sample_count ?? null,
        queue_count: Object.keys(snapshot.queues || {}).length,
        weather_cpu_basis_points: weather.sample?.cpu_basis_points?.value ?? null,
        network_received_bytes: weather.sample?.network_received_bytes?.value ?? null,
        network_transmitted_bytes: weather.sample?.network_transmitted_bytes?.value ?? null,
        weather_age_millis: weatherFreshness.age_millis ?? null,
        weather_stale_after_millis: weatherFreshness.stale_after_millis ?? null
      },
      states: {
        lifecycle: snapshot.lifecycle || "unknown",
        resource_state: weather.resource_state || "unknown",
        shutdown_decision: weather.shutdown_decision || "unknown",
        gpu_proof_state: weather.gpu_proof_state || "unknown",
        weather_stale: weatherFreshness.stale ?? null
      }
    },
    events: { events },
    continuity: feed.continuity,
    proof: feed.proof,
    errors: {}
  };
}

function projectPolisIdentity(identity) {
  const safeIdentifier = /^[A-Za-z0-9._:-]{1,128}$/;
  const safeDomain = /^(?=.{1,253}$)[a-z0-9](?:[a-z0-9.-]*[a-z0-9])?$/;
  const text = String(identity?.display_name || "");
  let runtimeApi;
  let observatoryOrigin;
  try {
    runtimeApi = new URL(String(identity?.runtime_api_base || ""));
    observatoryOrigin = new URL(String(identity?.observatory_public_origin || ""));
  } catch (_error) {
    throw new Error("Runtime Observatory feed has invalid Polis identity URLs");
  }
  if (
    !safeIdentifier.test(String(identity?.polis_id || ""))
    || !safeDomain.test(String(identity?.public_domain || ""))
    || !text.trim()
    || text !== text.trim()
    || text.length > 128
    || runtimeApi.protocol !== "https:"
    || runtimeApi.hostname !== identity.public_domain
    || observatoryOrigin.protocol !== "https:"
    || observatoryOrigin.origin !== String(identity.observatory_public_origin)
  ) {
    throw new Error("Runtime Observatory feed has invalid Polis identity");
  }
  return Object.freeze({
    polisId: identity.polis_id,
    displayName: text,
    publicDomain: identity.public_domain,
    runtimeApiBase: runtimeApi.toString().replace(/\/$/, ""),
    observatoryPublicOrigin: observatoryOrigin.origin
  });
}

function connectRuntimeV3ObservatoryWebSocket(
  apiBase,
  onSnapshot,
  onError,
  onClose = onError,
  onControlFrame = () => {}
) {
  const base = normalizeTrustedRuntimeV3ApiBase(apiBase);
  const endpoint = new URL(`${base}${getRuntimeV3Config().observatory_websocket_endpoint}`);
  endpoint.protocol = "wss:";
  const socket = new WebSocket(endpoint.toString());
  socket.addEventListener("open", () => {
    const writeToken = globalThis.sessionStorage?.getItem("adl.runtimeV3.observatoryToken") || "";
    if (writeToken) {
      authenticateRuntimeV3ObservatorySocket(socket, writeToken);
    }
  });
  socket.addEventListener("message", (event) => {
    try {
      const frame = JSON.parse(String(event.data));
      if (frame.schema === RUNTIME_V3_OBSERVATORY_SCHEMA) {
        onSnapshot(runtimeV3SnapshotFromFeed(frame));
      } else if (frame.schema === "adl.runtime_v3.observatory_ws_control_result.v1" ||
                 frame.schema === "adl.runtime_v3.observatory_conversation_result.v1" ||
                 frame.schema === GOVERNED_ROOM_ROUTE_SCHEMA ||
                 frame.schema === "adl.runtime_v3.observatory_governed_room_result.v1" ||
                 frame.schema === "adl.csm.acip_carrier.websocket_frame.v1") {
        onControlFrame(frame);
      }
    } catch (error) {
      onError(error instanceof Error ? error : new Error("Runtime v3 Observatory frame is invalid."));
      socket.close(1008, "invalid_observatory_frame");
    }
  });
  socket.addEventListener("error", () => {
    onError(new Error("Runtime v3 Observatory WebSocket failed."));
  });
  socket.addEventListener("close", (event) => {
    onClose(new Error(`Runtime v3 Observatory WebSocket closed (${event.code}).`));
  });
  return socket;
}

function authenticateRuntimeV3ObservatorySocket(socket, token) {
  if (!socket || socket.readyState !== WebSocket.OPEN) {
    throw new Error("Runtime v3 Observatory WebSocket is not open.");
  }
  const writeToken = String(token || "").trim();
  if (!writeToken) {
    throw new Error("A runtime operator token is required for writes.");
  }
  socket.send(JSON.stringify({
    schema: RUNTIME_V3_OBSERVATORY_WS_AUTH_SCHEMA,
    bearer_token: writeToken
  }));
}

const CONVERSATION_RESULT_STATUSES = new Set([
  "accepted",
  "delivered",
  "refused",
  "failed",
  "timed_out",
  "cancelled"
]);

function conversationFrameTransition(frame, pending) {
  if (!frame || !pending ||
      frame.schema !== "adl.runtime_v3.observatory_conversation_result.v1" ||
      frame.conversation_id !== pending.conversationId ||
      frame.turn_id !== pending.turnId ||
      !CONVERSATION_RESULT_STATUSES.has(frame.status)) {
    return null;
  }
  if (pending.cancelRequested &&
      frame.status === "accepted" &&
      frame.recipient_id === pending.recipientId &&
      frame.correlation_id === pending.correlationId) {
    return { status: "cancelling", terminal: false, reply: null };
  }
  if (frame.recipient_id !== pending.recipientId ||
      frame.correlation_id !== pending.correlationId) {
    return null;
  }
  const reply = frame.status === "delivered" &&
    typeof frame.reply === "string" &&
    frame.reply.trim() &&
    frame.reply.length <= 4096
    ? frame.reply
    : null;
  if (frame.status === "delivered" && !reply) {
    return null;
  }
  return {
    status: frame.status,
    terminal: frame.status !== "accepted",
    reply,
    senderId: typeof frame.sender_id === "string" && frame.sender_id.length <= 128
      ? frame.sender_id
      : null,
    initiatedWorkId: typeof frame.initiated_work_id === "string" && frame.initiated_work_id.length <= 128
      ? frame.initiated_work_id
      : null
  };
}

function conversationReplyFromFrame(frame, pending) {
  return conversationFrameTransition(frame, pending)?.reply || null;
}

function conversationFrameProvesAcceptance(frame) {
  return frame?.status === "accepted" ||
    (CONVERSATION_RESULT_STATUSES.has(frame?.status) &&
      frame.status !== "refused" &&
      Number.isSafeInteger(frame.turn_sequence) &&
      frame.turn_sequence > 0);
}

function conversationReconnectIntent(pending, runtimeIncarnationId) {
  if (!pending || !pending.disconnected || pending.terminal) {
    return null;
  }
  if (typeof runtimeIncarnationId !== "string" || runtimeIncarnationId.length === 0) {
    return null;
  }
  if (pending.runtimeIncarnationId !== runtimeIncarnationId) {
    pending.restartUnavailable = true;
    pending.terminal = true;
    pending.disconnected = false;
    return null;
  }
  pending.reconnectReplayCount += 1;
  pending.disconnected = false;
  return pending.intent;
}

const OBSERVATORY_CONVERSATION_HISTORY_SCHEMA = "adl.runtime.conversation_history.v1";
const FORBIDDEN_CONVERSATION_HISTORY_FIELDS = [
  "bearer_token",
  "operator_token",
  "private_key",
  "signature",
  "correlation_id",
  "result_hash"
];

function safeConversationHistoryText(value, fallback = "[redacted]") {
  const text = typeof value === "string" ? value : "";
  if (!text.trim()) return fallback;
  const lower = text.toLowerCase();
  if (FORBIDDEN_CONVERSATION_HISTORY_FIELDS.some((field) => lower.includes(field))) {
    return fallback;
  }
  return text.slice(0, 4096);
}

function normalizeRuntimeConversationHistorySnapshot(history, feed = {}) {
  if (!history ||
      history.schema !== OBSERVATORY_CONVERSATION_HISTORY_SCHEMA ||
      typeof history.conversation_id !== "string" ||
      history.conversation_id.length === 0 ||
      !Array.isArray(history.records)) {
    return { accepted: false, reason: "invalid_runtime_history" };
  }
  const expectedIncarnation = feed.runtime_incarnation_id || feed.runtimeIncarnationId || "";
  if (expectedIncarnation &&
      history.runtime_incarnation_id &&
      history.runtime_incarnation_id !== expectedIncarnation) {
    return { accepted: false, reason: "stale_runtime_history" };
  }
  let lastSequence = 0;
  const records = [];
  for (const record of history.records) {
    const sequence = Number(record.turn_sequence ?? record.journal_sequence ?? 0);
    if (!Number.isSafeInteger(sequence) || sequence <= lastSequence) {
      return { accepted: false, reason: "non_monotonic_runtime_history" };
    }
    lastSequence = sequence;
    records.push({
      conversation_id: history.conversation_id,
      message_id: String(record.message_id || record.turn_id || `history-${sequence}`),
      speaker_id: safeConversationHistoryText(record.speaker_id || "runtime"),
      body: record.redacted ? "[redacted]" : safeConversationHistoryText(record.body),
      status: record.redacted ? "redacted" : safeConversationHistoryText(record.status || "restored"),
      turn_sequence: sequence,
      redacted: record.redacted === true,
      redaction_reason: record.redaction_reason ? safeConversationHistoryText(record.redaction_reason) : null
    });
  }
  return {
    accepted: true,
    schema: history.schema,
    conversation_id: history.conversation_id,
    runtime_incarnation_id: history.runtime_incarnation_id || expectedIncarnation || null,
    records
  };
}

function restoreConversationTranscriptFromRuntimeHistory(history, feed = {}, appendTurn = null) {
  const normalized = normalizeRuntimeConversationHistorySnapshot(history, feed);
  if (!normalized.accepted) {
    return normalized;
  }
  if (typeof appendTurn === "function") {
    for (const record of normalized.records) {
      appendTurn(record.speaker_id, record.body, record.message_id, record.status);
    }
  }
  return normalized;
}

async function fetchRetainedRuntimeSnapshot(refs = {}) {
  const [status, health, ready, metrics, events] = await Promise.all([
    loadJson(refs.statusRef).catch((error) => ({ __load_error: error instanceof Error ? error.message : "status load failed" })),
    loadJson(refs.healthRef).catch((error) => ({ __load_error: error instanceof Error ? error.message : "health load failed" })),
    loadJson(refs.readyRef).catch((error) => ({ __load_error: error instanceof Error ? error.message : "ready load failed" })),
    loadJson(refs.metricsRef).catch((error) => ({ __load_error: error instanceof Error ? error.message : "metrics load failed" })),
    loadJson(refs.eventsRef).catch((error) => ({ __load_error: error instanceof Error ? error.message : "events load failed" }))
  ]);
  return {
    mode: "published",
    fetchedAt: new Date().toISOString(),
    status,
    health,
    ready,
    metrics,
    events,
    errors: Object.fromEntries(
      Object.entries({ status, health, ready, metrics, events })
        .filter(([, value]) => value?.__load_error)
        .map(([key, value]) => [key, value.__load_error])
    )
  };
}

function flattenStatusRows(value, prefix = "", rows = []) {
  if (rows.length >= 14 || value == null) {
    return rows;
  }
  if (typeof value !== "object") {
    rows.push({ label: prefix || "value", value });
    return rows;
  }
  if (Array.isArray(value)) {
    rows.push({ label: prefix || "items", value: value.length });
    return rows;
  }
  Object.entries(value).forEach(([key, nested]) => {
    if (rows.length >= 14) {
      return;
    }
    const label = prefix ? `${prefix}.${key}` : key;
    if (nested == null || typeof nested !== "object") {
      rows.push({ label, value: nested });
    } else if (Array.isArray(nested)) {
      rows.push({ label, value: `${nested.length} items` });
    } else {
      flattenStatusRows(nested, label, rows);
    }
  });
  return rows;
}

function stateTone(value) {
  const normalized = String(value || "").toLowerCase();
  if (["running", "active", "ready", "healthy", "ok", "completed", "closed", "clear", "awake"].some((token) => normalized.includes(token))) {
    return "active";
  }
  if (["failed", "error", "refuse", "blocked", "timeout"].some((token) => normalized.includes(token))) {
    return "failed";
  }
  if (["degraded", "not_ready", "pending", "paused", "stale", "saturated"].some((token) => normalized.includes(token))) {
    return "degraded";
  }
  return normalized || "unknown";
}

function iconForAgent(agent = {}) {
  const text = `${agent.role || ""} ${agent.label || ""} ${agent.id || ""}`.toLowerCase();
  if (text.includes("owner")) {
    return "icon-agent";
  }
  if (text.includes("scheduler") || text.includes("cadence")) {
    return "icon-clock";
  }
  if (text.includes("telemetry") || text.includes("observability")) {
    return "icon-pulse";
  }
  if (text.includes("event") || text.includes("stream")) {
    return "icon-bolt";
  }
  if (text.includes("checkpoint") || text.includes("continuity") || text.includes("custody")) {
    return "icon-database";
  }
  if (text.includes("policy") || text.includes("gate") || text.includes("readiness")) {
    return "icon-shield";
  }
  return "icon-bot";
}

function eventMessageToObject(event) {
  if (!event || typeof event !== "object") {
    return {};
  }
  if (typeof event.message === "string") {
    try {
      const parsed = JSON.parse(event.message);
      return parsed && typeof parsed === "object" ? parsed : { message: event.message };
    } catch (_error) {
      return { message: event.message };
    }
  }
  if (event.details && typeof event.details === "object") {
    return {
      ...event,
      event_type: event.event || event.details.event || event.details.event_name,
      status: event.result || event.details.result || event.status,
      runtime_id: event.runtime_id || event.agent_instance_id || event.details.runtime_id,
      timestamp: event.at || event.timestamp || event.details.timestamp
    };
  }
  return {
    ...event,
    event_type: event.event || event.event_type,
    runtime_id: event.runtime_id || event.agent_instance_id,
    timestamp: event.at || event.timestamp
  };
}

function normalizeEventEntries(eventEnvelope = {}) {
  if (Array.isArray(eventEnvelope)) {
    return eventEnvelope;
  }
  if (Array.isArray(eventEnvelope.events)) {
    return eventEnvelope.events;
  }
  if (Array.isArray(eventEnvelope.entries)) {
    return eventEnvelope.entries;
  }
  if (Array.isArray(eventEnvelope.events?.entries)) {
    return eventEnvelope.events.entries;
  }
  if (Array.isArray(eventEnvelope.tail)) {
    return eventEnvelope.tail;
  }
  return [];
}

function retainedLargePolisWindow(rows = [], limit = LARGE_POLIS_LIMITS.maxTranscriptTurns) {
  const safeLimit = Math.max(0, Number(limit) || 0);
  return asArray(rows).slice(Math.max(0, asArray(rows).length - safeLimit));
}

function pruneLargePolisDomWindow(container, selector, limit = LARGE_POLIS_LIMITS.maxTranscriptTurns) {
  if (!container || typeof container.querySelectorAll !== "function") return 0;
  const rows = Array.from(container.querySelectorAll(selector));
  const removeCount = Math.max(0, rows.length - Math.max(0, Number(limit) || 0));
  rows.slice(0, removeCount).forEach((row) => row.remove());
  container.dataset.retainedTurnCount = String(rows.length - removeCount);
  container.dataset.prunedTurnCount = String(removeCount);
  return removeCount;
}

function largePolisRecoveryViewModel(state = {}) {
  const transitions = [];
  const actions = [];
  const status = {
    reconnect: state.connected === false ? "degraded" : "ready",
    restart: state.runtimeIncarnationChanged === true ? "requires_resync" : "ready",
    backpressure: Number(state.bufferedMessages || 0) > Number(state.backpressureThreshold || 1000) ? "throttled" : "ready",
    offline: state.offline === true ? "offline" : "ready",
    versionMismatch: state.versionMismatch === true ? "blocked_until_refresh" : "ready"
  };

  if (status.reconnect === "degraded") {
    transitions.push("socket_disconnected");
    actions.push("schedule_single_reconnect");
  }
  if (status.restart === "requires_resync") {
    transitions.push("runtime_incarnation_changed");
    actions.push("discard_stale_pending_turns");
  }
  if (status.backpressure === "throttled") {
    transitions.push("stream_backpressure");
    actions.push("pause_nonessential_rendering");
  }
  if (status.offline === "offline") {
    transitions.push("browser_offline");
    actions.push("show_offline_state");
  }
  if (status.versionMismatch === "blocked_until_refresh") {
    transitions.push("client_runtime_version_mismatch");
    actions.push("block_mutating_controls");
  }

  return {
    schema: "adl.html_observatory.large_polis_recovery_view.v1",
    status,
    transitions,
    actions: [...new Set(actions)].slice(0, LARGE_POLIS_LIMITS.maxPendingRecoveryActions),
    duplicate_action_prevented: actions.length > new Set(actions).size,
    grants_authority: false,
    runtime_authority_required: true
  };
}

function largePolisRecoverySequence(states = []) {
  const pendingActions = new Set();
  let observedPendingActionCount = 0;
  let resolvedPendingActionCount = 0;
  let hiddenStaleState = false;
  const steps = asArray(states).map((state, index) => {
    const pendingBefore = [...pendingActions];
    const rawView = largePolisRecoveryViewModel(state);
    const repeatedPendingActions = rawView.actions.filter((action) => pendingActions.has(action));
    const view = {
      ...rawView,
      actions: rawView.actions.filter((action) => !pendingActions.has(action)),
      duplicate_action_prevented: rawView.duplicate_action_prevented || repeatedPendingActions.length > 0
    };
    for (const action of view.actions) {
      pendingActions.add(action);
    }
    observedPendingActionCount = Math.max(observedPendingActionCount, pendingActions.size);
    const isHealthy = Object.values(view.status).every((status) => status === "ready") && view.actions.length === 0;
    const resolvedActions = isHealthy ? [...pendingActions] : [];
    if (!isHealthy && pendingBefore.length > 0 && view.transitions.length === 0 && view.actions.length === 0) {
      hiddenStaleState = true;
    }
    if (isHealthy) {
      resolvedPendingActionCount += resolvedActions.length;
      pendingActions.clear();
    }
    return {
      sequence: index + 1,
      view,
      pending_actions_before: pendingBefore,
      pending_actions_after: [...pendingActions],
      resolved_pending_actions: resolvedActions,
      stale_state_visible: view.transitions.length > 0
        || pendingBefore.length > 0
        || view.actions.length > 0
        || resolvedActions.length > 0
    };
  });
  const terminal = steps.at(-1)?.view;
  const terminalPendingActions = steps.at(-1)?.pending_actions_after || [];
  return {
    schema: "adl.html_observatory.large_polis_recovery_sequence.v1",
    steps,
    recovered: terminal
      ? Object.values(terminal.status).every((state) => state === "ready") && terminal.actions.length === 0
        && terminalPendingActions.length === 0
        && resolvedPendingActionCount >= observedPendingActionCount
      : false,
    stale_state_hidden: hiddenStaleState || terminalPendingActions.length > 0,
    observed_pending_action_count: observedPendingActionCount,
    resolved_pending_action_count: resolvedPendingActionCount,
    pending_action_count: terminalPendingActions.length,
    dropped_pending_actions: Math.max(0, observedPendingActionCount - resolvedPendingActionCount),
    duplicate_actions: steps.reduce((count, step) => count + (step.view.duplicate_action_prevented ? 1 : 0), 0)
  };
}

function estimateLargePolisResourceMetrics({
  visibleAgents = 0,
  retainedTranscriptTurns = 0,
  retainedStreamEvents = 0,
  recoveryActions = 0
} = {}) {
  const projectedDomNodes =
    Number(visibleAgents) * 3
    + Number(retainedTranscriptTurns) * 2
    + Number(retainedStreamEvents)
    + Number(recoveryActions);
  const deterministicProjectionMillis = Math.ceil(projectedDomNodes / 25);
  return {
    schema: "adl.html_observatory.large_polis_resource_metrics.v1",
    projected_dom_nodes: projectedDomNodes,
    max_projected_dom_nodes: LARGE_POLIS_LIMITS.maxProjectedDomNodes,
    deterministic_projection_millis: deterministicProjectionMillis,
    max_deterministic_projection_millis: LARGE_POLIS_LIMITS.maxDeterministicProjectionMillis,
    bounded_dom_nodes: projectedDomNodes <= LARGE_POLIS_LIMITS.maxProjectedDomNodes,
    bounded_latency: deterministicProjectionMillis <= LARGE_POLIS_LIMITS.maxDeterministicProjectionMillis
  };
}

function buildLargePolisPerformanceRecoveryFixture({
  agentCount = 2500,
  transcriptTurns = 5000,
  streamEvents = 1200,
  runtimeIncarnationChanged = true,
  candidateRevision = "557dd28d85746a8dc5109dcc674f5a606b8c9890",
  implementationRevision = "unassigned"
} = {}) {
  const agents = Array.from({ length: agentCount }, (_, index) => ({
    id: `agent-${String(index + 1).padStart(5, "0")}`,
    label: `Polis Agent ${index + 1}`,
    role: index % 7 === 0 ? "moderator" : "citizen",
    state: index % 11 === 0 ? "busy" : "ready",
    detail: "deterministic large-Polis fixture",
    communication_eligible: index % 3 === 0,
    source_revision: "fixture"
  }));
  const transcript = Array.from({ length: transcriptTurns }, (_, index) => ({
    turn_id: `turn-${String(index + 1).padStart(6, "0")}`,
    sequence: index + 1,
    speaker: index % 2 === 0 ? "operator" : `agent-${String((index % Math.max(agentCount, 1)) + 1).padStart(5, "0")}`,
    text: `Deterministic long transcript turn ${index + 1}`
  }));
  const events = Array.from({ length: streamEvents }, (_, index) => ({
    event_type: index % 10 === 0 ? "stream_pressure" : "agent_tick",
    sequence: index + 1,
    status: "observed",
    runtime_id: "runtime-large-polis",
    timestamp: `2026-08-17T00:${String(Math.floor(index / 60)).padStart(2, "0")}:${String(index % 60).padStart(2, "0")}Z`
  }));

  return {
    snapshot: {
      mode: "retained",
      fetchedAt: "2026-08-17T00:00:00Z",
      status: {
        schema: RUNTIME_V3_OBSERVATORY_SCHEMA,
        runtime_id: "runtime-large-polis",
        runtime_incarnation_id: runtimeIncarnationChanged ? "incarnation-b" : "incarnation-a",
        source_revision: candidateRevision,
        implementation_revision: implementationRevision,
        agent_population: {
          total_count: agentCount,
          revision: 42,
          event_cursor: "cursor-large-polis-42",
          sample: agents,
          has_more: agentCount > LARGE_POLIS_LIMITS.maxVisibleAgents,
          next_page_token: "page-2"
        }
      },
      health: { status: "ready" },
      ready: { status: "ready" },
      metrics: {
        gauges: {
          agent_count: agentCount,
          transcript_turn_count: transcriptTurns,
          stream_event_count: streamEvents
        }
      },
      events: { events }
    },
    transcript,
    recovery: {
      connected: false,
      runtimeIncarnationChanged,
      bufferedMessages: streamEvents,
      backpressureThreshold: 1000,
      offline: true,
      versionMismatch: true
    }
  };
}

function evaluateLargePolisPerformanceRecovery(fixture = buildLargePolisPerformanceRecoveryFixture()) {
  const vm = buildPanopticonViewModel(fixture.snapshot, FALLBACK_PACKET);
  const transcriptWindow = retainedLargePolisWindow(fixture.transcript);
  const recovery = largePolisRecoveryViewModel(fixture.recovery);
  const recoverySequence = largePolisRecoverySequence([
    fixture.recovery,
    {
      connected: true,
      runtimeIncarnationChanged: false,
      bufferedMessages: 0,
      backpressureThreshold: fixture.recovery?.backpressureThreshold || 1000,
      offline: false,
      versionMismatch: false
    }
  ]);
  const resourceMetrics = estimateLargePolisResourceMetrics({
    visibleAgents: vm.visibleAgentCount,
    retainedTranscriptTurns: transcriptWindow.length,
    retainedStreamEvents: vm.events.length,
    recoveryActions: recovery.actions.length
  });
  return {
    schema: "adl.html_observatory.large_polis_performance_recovery_metrics.v1",
    candidate_revision: runTimeCandidateRevision(fixture.snapshot),
    implementation_revision: fixture.snapshot?.status?.implementation_revision || "unassigned",
    agent_total: vm.agentTotal,
    visible_agent_count: vm.visibleAgentCount,
    max_visible_agents: LARGE_POLIS_LIMITS.maxVisibleAgents,
    transcript_total_turns: asArray(fixture.transcript).length,
    retained_transcript_turns: transcriptWindow.length,
    max_transcript_turns: LARGE_POLIS_LIMITS.maxTranscriptTurns,
    stream_event_total: normalizeEventEntries(fixture.snapshot?.events).length,
    retained_stream_events: vm.events.length,
    max_event_tail: LARGE_POLIS_LIMITS.maxEventTail,
    resource_metrics: resourceMetrics,
    recovery,
    recovery_sequence: recoverySequence,
    bounded: vm.visibleAgentCount <= LARGE_POLIS_LIMITS.maxVisibleAgents
      && transcriptWindow.length <= LARGE_POLIS_LIMITS.maxTranscriptTurns
      && vm.events.length <= LARGE_POLIS_LIMITS.maxEventTail
      && recovery.actions.length <= LARGE_POLIS_LIMITS.maxPendingRecoveryActions
      && recoverySequence.recovered
      && resourceMetrics.bounded_dom_nodes
      && resourceMetrics.bounded_latency,
    grants_authority: false
  };
}

function runTimeCandidateRevision(snapshot = {}) {
  return snapshot.status?.source_revision
    || snapshot.status?.agent_population?.source_revision
    || snapshot.status?.runtime_incarnation_id
    || "unknown";
}

function normalizeMetricRows(metrics = {}) {
  const rows = flattenStatusRows(metrics)
    .filter((row) => ["number", "string", "boolean"].includes(typeof row.value))
    .slice(0, 8);
  return rows.length ? rows : [{ label: "metrics", value: "not exposed" }];
}

function buildRuntimeAgentRows({ status = {}, health = {}, ready = {}, metrics = {}, events = [], packet = FALLBACK_PACKET } = {}) {
  const hasApiStatus = Object.keys(status || {}).length > 0 && !status.__load_error;
  const retainedCitizens = asArray(packet.citizens);
  const primaryAgentId =
    status.agent_instance_id ||
    status.agent_id ||
    status.runtime_id ||
    status.instance_id ||
    packet.manifold?.manifold_id ||
    "csm-runtime";
  const primaryState =
    status.status ||
    status.state ||
    status.current_agent_status?.state ||
    status.agent_status?.state ||
    ready.status ||
    ready.ready ||
    health.status ||
    packet.manifold?.state ||
    "unknown";
  const agentPopulation = status.agent_population || {};
  const agentSample = asArray(agentPopulation.sample);

  if (!hasApiStatus) {
    return retainedCitizens.map((citizen) => ({
      id: citizen.citizen_id || citizen.display_name,
      label: citizen.display_name,
      role: citizen.role || "agent",
      state: citizen.lifecycle_state || citizen.continuity_status,
      detail: citizen.continuity_status || "retained citizen lane"
    })).slice(0, 6);
  }

  if (agentSample.length || status.schema === RUNTIME_V3_OBSERVATORY_SCHEMA) {
    return agentSample.slice(0, LARGE_POLIS_LIMITS.maxVisibleAgents).map((agent) => ({
      id: agent.id,
      label: agent.label || agent.id,
      role: agent.role || "runtime agent",
      state: agent.state || primaryState,
      detail: agent.detail || `${agentPopulation.total_count || agentSample.length} configured agents`,
      health: agent.health || "unknown",
      availability: agent.availability || "unknown",
      activity: agent.activity || null,
      capabilities: asArray(agent.capabilities),
      location: agent.location || null,
      communicationEligible: agent.communication_eligible === true,
      observedAtUnixMillis: Number(agent.observed_at_unix_millis || 0),
      freshnessDeadlineUnixMillis: Number(agent.freshness_deadline_unix_millis || 0),
      sourceRevision: agent.source_revision || "unknown",
      provenance: agent.provenance || "unknown"
    }));
  }

  return [
    {
      id: primaryAgentId,
      label: status.agent_name || status.display_name || status.agent_instance_id || "CSM runtime",
      role: "runtime owner",
      state: primaryState,
      detail: status.runtime_owner ? `owner: ${status.runtime_owner}` : "loopback API status"
    },
    {
      id: `${primaryAgentId}:readiness`,
      label: "Readiness gate",
      role: "control gate",
      state: ready.status || ready.ready || primaryState,
      detail: asArray(ready.blocking_reasons).length ? ready.blocking_reasons.join(", ") : "no blocking reasons"
    },
    {
      id: `${primaryAgentId}:scheduler`,
      label: "Scheduler watcher",
      role: "cadence",
      state: status.scheduler?.status || metrics.states?.agent_state || status.agent_status?.state || primaryState,
      detail: status.scheduler?.cadence_source || `cycles: ${metrics.gauges?.completed_cycle_count ?? status.agent_status?.completed_cycle_count ?? "unknown"}`
    },
    {
      id: `${primaryAgentId}:observability`,
      label: "Observability bridge",
      role: "telemetry",
      state: status.otel?.status?.status || status.otel?.log?.status || health.status || "unknown",
      detail: status.otel?.status?.schema || status.otel?.log?.ref || "OTel and event logs"
    },
    {
      id: `${primaryAgentId}:events`,
      label: "Event stream tail",
      role: "operator stream",
      state: events.length ? "active" : "quiet",
      detail: `${events.length} retained CSM events`
    },
    {
      id: `${primaryAgentId}:continuity`,
      label: "Continuity checkpoint",
      role: "state custody",
      state: status.checkpoint?.status || status.continuity?.checkpoint?.status || "unknown",
      detail: status.checkpoint?.checkpoint_ref || status.continuity?.checkpoint?.ref || "continuity state"
    }
  ].slice(0, 6);
}

function buildPanopticonViewModel(snapshot = {}, packet = FALLBACK_PACKET) {
  const status = snapshot.status || {};
  const health = snapshot.health || {};
  const ready = snapshot.ready || {};
  const metrics = snapshot.metrics || {};
  const eventEnvelope = snapshot.events || {};
  const events = normalizeEventEntries(eventEnvelope)
    .slice(-LARGE_POLIS_LIMITS.maxEventTail)
    .map(eventMessageToObject);
  const statusRows = flattenStatusRows(status);
  const liveAgents = buildRuntimeAgentRows({ status, health, ready, metrics, events, packet });
  const agentTotal = Number(status.agent_population?.total_count ?? metrics.gauges?.agent_count ?? liveAgents.length);
  const rosterNeedle = rosterUiState.filter.trim().toLocaleLowerCase();
  const visibleAgents = liveAgents
    .filter((agent) => rosterUiState.presence === "all" || agent.state === rosterUiState.presence)
    .filter((agent) => !rosterNeedle || [agent.id, agent.label, agent.role].some((value) => String(value || "").toLocaleLowerCase().includes(rosterNeedle)))
    .sort((left, right) => {
      const primary = rosterUiState.sort === "presence"
        ? String(left.state).localeCompare(String(right.state))
        : String(left.label || left.id).localeCompare(String(right.label || right.id));
      return primary || String(left.id).localeCompare(String(right.id));
    });

  const signalRows = [
    {
      label: "health",
      value: health.status || health.state || "unknown",
      detail: health.reason || health.summary || health.message || "CSM /health"
    },
    {
      label: "readiness",
      value: ready.status || ready.state || ready.ready || "unknown",
      detail: ready.reason || ready.summary || ready.message || "CSM /ready"
    },
    {
      label: "events",
      value: `${events.length} events`,
      detail: eventEnvelope.event_stream_ref || eventEnvelope.source || eventEnvelope.schema || "CSM /events"
    },
    {
      label: "errors",
      value: Object.keys(snapshot.errors || {}).length ? "partial" : "none",
      detail: Object.values(snapshot.errors || {}).join("; ") || "all requested endpoints responded"
    }
  ];

  return {
    mode: snapshot.mode || "retained",
    fetchedAt: snapshot.fetchedAt || "",
    agents: visibleAgents,
    allAgents: liveAgents,
    agentTotal,
    visibleAgentCount: visibleAgents.length,
    signals: signalRows,
    metrics: normalizeMetricRows(metrics),
    events,
    statusRows,
    readyState: ready.status || ready.state || ready.ready || "unknown"
  };
}

function renderPanopticon(snapshot = {}, packet = FALLBACK_PACKET) {
  lastPanopticonSnapshot = snapshot;
  lastPanopticonPacket = packet;
  const vm = buildPanopticonViewModel(snapshot, packet);
  setText("polis-display-name", snapshot.polisIdentity?.displayName || "Unavailable");
  setText("polis-public-domain", snapshot.polisIdentity?.publicDomain || "Unavailable");
  const sourceLabel = vm.mode === "live" ? "Live Runtime API" : vm.mode === "published" ? "Published Runtime Evidence" : "Retained Runtime Evidence";
  const hasAuthoritativeLiveRuntimeFeed =
    vm.mode === "live" &&
    snapshot.status?.schema === RUNTIME_V3_OBSERVATORY_SCHEMA &&
    snapshot.status?.agent_population &&
    Number(snapshot.status.agent_population.total_count || 0) >= 0;
  setText("live-status", vm.mode === "live" ? "live loopback" : vm.mode === "published" ? "published runtime mirror" : "retained fallback");
  setText("hero-live-mode", vm.mode === "live" ? "Online" : vm.mode === "published" ? "Published" : "Retained");
  setText("hero-map-mode", vm.mode === "live" ? "live graph" : vm.mode === "published" ? "published graph" : "retained graph");
  setText("hero-event-title", vm.mode === "live" ? "Event Stream (Live Loopback)" : "Event Stream");
  setText("statusbar-mode", vm.mode === "live" ? "Live Loopback" : vm.mode === "published" ? "Published Mirror" : "Retained Mirror");
  setText("runtime-source-label", sourceLabel);
  setText("statusbar-runtime-label", sourceLabel);
  if (hasAuthoritativeLiveRuntimeFeed) {
    setText("packet-status", "CSM Runtime");
    document.getElementById("packet-status")?.setAttribute("data-state", "ok");
    setText("claim-boundary", "Live Runtime v3 Observatory feed loaded from the configured loopback API.");
    setText("evidence-level", "Runtime v3 Observatory feed");
    document.getElementById("evidence-level")?.setAttribute("data-tone", "ok");
  }
  const modeSelect = document.getElementById("top-mode-select");
  if (modeSelect) {
    modeSelect.value = vm.mode === "live" ? "live" : vm.mode === "published" ? "published" : "retained";
  }
  setText("statusbar-updated", vm.mode === "live" ? formatTimestampLabel(vm.fetchedAt) : formatCurrentTimestampLabel());
  setDataset("statusbar-indicator", "state", vm.mode === "live" ? "live" : vm.mode === "published" ? "published" : "fallback");
  setText("agent-count", `${vm.visibleAgentCount.toLocaleString()} of ${vm.agentTotal.toLocaleString()} visible`);
  const loadMore = document.getElementById("roster-load-more");
  if (loadMore) {
    loadMore.hidden = snapshot.status?.agent_population?.has_more !== true;
    loadMore.disabled = !snapshot.status?.agent_population?.next_page_token;
  }
  setText("hero-agent-count", `${vm.agentTotal.toLocaleString()} Agents`);
  setText("live-readiness", formatLabel(vm.readyState));
  setText("hero-ready-state", formatLabel(vm.readyState));
  setDataset("hero-agent-map", "state", formatLabel(vm.readyState));
  setText("live-updated", vm.fetchedAt ? new Date(vm.fetchedAt).toLocaleTimeString() : "not connected");
  setText("live-event-count", `${vm.events.length} events`);
  setText("hero-event-count", `${vm.events.length} Events`);
  setText("hero-gauge-agents", vm.agentTotal.toLocaleString());
  setText("hero-gauge-events", String(vm.events.length));
  setText("hero-gauge-metrics", String(vm.metrics.length));
  setText("hero-gauge-ready", formatLabel(vm.readyState));
  setText("agent-heartbeat", vm.fetchedAt ? new Date(vm.fetchedAt).toLocaleTimeString() : "retained");
  setText("agent-state", formatLabel(vm.readyState));
  setText("hero-event-detail", vm.events.length ? `${vm.events.length} retained or live CSM events visible.` : "No CSM events visible yet.");
  setText("live-metric-count", `${vm.metrics.length} gauges`);
  setText("hero-ready-detail", vm.signals.find((signal) => signal.label === "readiness")?.detail || "CSM /ready");
  setText("hero-latest-event", vm.events.length ? `event ${vm.events.length}` : "event 0");
  setState("hero-ready-state", vm.readyState);

  renderRows("panopticon-map", vm.agents.map((agent) => `
    <article class="agent-node" data-state="${escapeHtml(stateTone(agent.state))}">
      <span class="row-kicker">${escapeHtml(formatLabel(agent.role))}</span>
      <strong>${escapeHtml(agent.label || agent.id)}</strong>
      <p class="row-detail">${escapeHtml(formatLabel(agent.state))} / ${escapeHtml(agent.detail || agent.id)}</p>
    </article>
  `));

  renderRows("hero-agent-map", vm.agents.length ? vm.agents.slice(0, 6).map((agent) => `
    <article class="hero-agent-node" data-state="${escapeHtml(stateTone(agent.state))}">
      <svg class="node-icon"><use href="#${escapeHtml(iconForAgent(agent))}"></use></svg>
      <span class="row-kicker">${escapeHtml(formatLabel(agent.role))}</span>
      <strong>${escapeHtml(agent.label || agent.id)}</strong>
      <p class="row-detail">${escapeHtml(formatLabel(agent.state))}</p>
    </article>
  `) : [`
    <article class="hero-agent-node" data-state="pending">
      <svg class="node-icon"><use href="#icon-bot"></use></svg>
      <span class="row-kicker">agents</span>
      <strong>Waiting</strong>
      <p class="row-detail">No retained or live CSM agents visible yet.</p>
    </article>
  `]);

  renderRows("live-agent-list", vm.agents.map((agent) => `
    <button type="button" class="agent-row roster-row" data-state="${escapeHtml(stateTone(agent.state))}" data-agent-id="${escapeHtml(agent.id)}" aria-pressed="${rosterUiState.selectedId === agent.id ? "true" : "false"}">
      <span class="row-kicker">${escapeHtml(agent.id)}</span>
      <strong>${escapeHtml(agent.label || agent.id)}</strong>
      <span class="row-detail">${escapeHtml(formatLabel(agent.state))} / ${escapeHtml(formatLabel(agent.role))}</span>
    </button>
  `));

  const selected = vm.allAgents.find((agent) => agent.id === rosterUiState.selectedId);
  const detail = document.getElementById("roster-detail");
  if (detail) {
    detail.innerHTML = selected ? `
      <span class="row-kicker">${escapeHtml(selected.id)} / ${escapeHtml(formatLabel(selected.provenance))}</span>
      <strong>${escapeHtml(selected.label || selected.id)}</strong>
      <dl class="roster-facts">
        <div><dt>Presence</dt><dd>${escapeHtml(formatLabel(selected.state))}</dd></div>
        <div><dt>Health</dt><dd>${escapeHtml(formatLabel(selected.health))}</dd></div>
        <div><dt>Availability</dt><dd>${escapeHtml(formatLabel(selected.availability))}</dd></div>
        <div><dt>Communication</dt><dd>${selected.communicationEligible ? "Eligible" : "Unavailable"}</dd></div>
        <div><dt>Location</dt><dd>${escapeHtml(selected.location || "Redacted")}</dd></div>
        <div><dt>Source revision</dt><dd>${escapeHtml(selected.sourceRevision)}</dd></div>
      </dl>
    ` : `
      <span class="row-kicker">Selection</span>
      <strong>No visible agent selected</strong>
      <p class="row-detail">Select a Runtime-authorized roster row to inspect current presence evidence.</p>
    `;
  }

  renderRows("live-signal-list", vm.signals.map((signal) => `
    <article class="signal-row" data-state="${escapeHtml(stateTone(signal.value))}">
      <span class="row-kicker">${escapeHtml(formatLabel(signal.label))}</span>
      <strong>${escapeHtml(formatLabel(signal.value))}</strong>
      <p class="row-detail">${escapeHtml(signal.detail)}</p>
    </article>
  `));

  renderRows("live-metric-list", vm.metrics.map((metric) => `
    <article class="metric-row">
      <strong>${escapeHtml(formatLabel(metric.label))}</strong>
      <span class="metric-value">${escapeHtml(metric.value)}</span>
    </article>
  `));

  renderRows("live-event-stream", vm.events.slice(-8).map((event, index) => `
    <li class="trace-row">
      <span class="trace-seq">${String(index + 1).padStart(2, "0")}</span>
      <span><strong>${escapeHtml(formatLabel(event.signal_kind || event.event_type || event.status || "event"))}</strong><br><span class="row-detail">${escapeHtml(event.runtime_id || event.agent_id || event.correlation_id || event.timestamp || event.message || "retained event")}</span></span>
    </li>
  `));

  const heroEventRows = vm.events.length ? vm.events.slice(-6).map((event, index) => {
    const eventName = formatLabel(event.signal_kind || event.event_type || event.status || "event");
    const source = event.agent_id || event.agent_instance_id || event.runtime_id || "csm";
    const state = formatLabel(event.status || event.result || event.details?.result || "ok");
    const tick = event.manifold_tick || event.tick || event.sequence || event.event_sequence || index + 1;
    const severity = stateTone(state) === "failed" ? "ERROR" : stateTone(state) === "degraded" ? "WARN" : "INFO";
    const eventTime = event.timestamp ? new Date(event.timestamp).toLocaleTimeString([], { hour12: false }) : `T-${String(vm.events.length - index).padStart(2, "0")}`;
    return `
    <li class="trace-row event-table-row">
      <span class="trace-seq">${escapeHtml(eventTime)}</span>
      <span class="event-severity" data-state="${escapeHtml(stateTone(state))}">${escapeHtml(severity)}</span>
      <span class="event-source">${escapeHtml(source)}</span>
      <span><strong>${escapeHtml(eventName)}</strong></span>
      <span class="event-state" data-state="${escapeHtml(stateTone(state))}">${escapeHtml(state)}</span>
      <span class="event-tick">${escapeHtml(tick)}</span>
    </li>
  `;
  }) : [`
    <li class="trace-row event-table-row">
      <span class="trace-seq">00</span>
      <span class="event-severity" data-state="degraded">WAIT</span>
      <span class="event-source">CSM API</span>
      <span><strong>Waiting</strong></span>
      <span class="event-state" data-state="degraded">pending</span>
      <span class="event-tick">0</span>
    </li>
  `];
  renderRows("hero-event-stream", [
    `<li class="event-table-header" aria-hidden="true">
      <span>Time</span>
      <span>Severity</span>
      <span>Source</span>
      <span>Event</span>
      <span>State</span>
      <span>Tick</span>
    </li>`,
    ...heroEventRows
  ]);
}

function renderObservatory(packet, reportText = "", state = "ok") {
  const vm = buildViewModel(packet, reportText);
  const source = vm.packet.source || {};
  const manifold = vm.packet.manifold || {};
  const pulse = vm.packet.kernel?.pulse || {};

  setText("packet-status", state === "ok" ? "CSM Runtime" : "Fallback shell");
  document.getElementById("packet-status")?.setAttribute("data-state", state);
  setText("claim-boundary", displayClaimBoundary(source));
  setText("evidence-level", formatLabel(source.evidence_level));
  document.getElementById("evidence-level")?.setAttribute("data-tone", state === "ok" ? "ok" : "warn");
  setText("packet-heading", "Owner Agent (owner-v2)");
  setText("manifold-id", displayManifoldId(manifold.manifold_id));
  setText("manifold-state", formatLabel(manifold.state));
  setText("manifold-tick", String(manifold.current_tick ?? 0));
  setText("packet-id", displayPacketId(vm.packet.packet_id));
  setText("hero-uptime", formatCurrentTimestampLabel());
  setText("rail-capture-time", formatCurrentTimestampLabel());
  setText("rail-manifold-id", displayManifoldId(manifold.manifold_id));
  setText("rail-state", formatLabel(manifold.state));
  setText("rail-tick", String(manifold.current_tick ?? 0));
  setText("statusbar-source", displayManifoldId(manifold.manifold_id || vm.packet.packet_id));
  setText("kernel-status", formatLabel(pulse.status));
  setText("latest-event", `event ${vm.latestEvent}`);
  setText("decision-counts", `${vm.decisionCounts.allow} / ${vm.decisionCounts.defer} / ${vm.decisionCounts.refuse}`);
  setText(
    "report-summary",
    reportText.includes("CSM Observatory Operator Report")
      ? "The operator report loaded from the same retained runtime artifact root as the packet."
      : "The operator report link is retained; report text did not load in this browser context."
  );

  renderRows("orbit-map", [
    `<div class="orbit-center"><strong>${formatLabel(manifold.state)}</strong><span class="row-kicker">${formatLabel(source.mode)}</span></div>`,
    ...vm.citizens.slice(0, 3).map((citizen) => `
      <article class="orbit-node">
        <span class="row-kicker">${formatLabel(citizen.lifecycle_state)} / ${formatLabel(citizen.role)}</span>
        <strong>${citizen.display_name}</strong>
        <p class="row-detail">${formatLabel(citizen.continuity_status)}</p>
      </article>
    `)
  ]);

  renderRows("service-list", vm.services.map((service) => `
    <article class="service-row">
      <span class="row-kicker">${formatLabel(service.service_id)}</span>
      <strong>${formatLabel(service.state || service.lifecycle_state)}</strong>
    </article>
  `));

  renderRows("decision-stack", vm.decisions.map((decision) => `
    <article class="decision-row" data-decision="${decision.decision}">
      <span class="row-kicker">${formatLabel(decision.decision)} / ${decision.actor}</span>
      <strong>${formatLabel(decision.action)}</strong>
      <p class="row-detail">${decision.rationale || decision.evidence_ref || "No rationale recorded."}</p>
    </article>
  `));

  renderRows("trace-list", vm.traceTail.map((event) => `
    <li class="trace-row">
      <span class="trace-seq">${String(event.event_sequence).padStart(2, "0")}</span>
      <span><strong>${formatLabel(event.event_type)}</strong><br><span class="row-detail">${event.summary}</span></span>
    </li>
  `));

  renderRows("invariant-list", vm.invariants.map((invariant) => `
    <article class="invariant-row">
      <span class="row-kicker">${formatLabel(invariant.severity)} / ${formatLabel(invariant.state)}</span>
      <strong>${invariant.name}</strong>
      <p class="row-detail">${invariant.evidence_ref}</p>
    </article>
  `));

  renderRows("action-list", [
    ...vm.availableActions.map((action) => `
      <article class="action-row">
      <span class="row-kicker">available / ${displayMilestoneText(formatLabel(action.mode))}</span>
      <strong>${displayMilestoneText(formatLabel(action.action))}</strong>
      <p class="row-detail">${displayMilestoneText(formatLabel(action.status))}</p>
      </article>
    `),
    ...vm.disabledActions.map((action) => `
      <article class="action-row">
        <span class="row-kicker">disabled</span>
      <strong>${displayMilestoneText(formatLabel(action.action))}</strong>
      <p class="row-detail">${displayMilestoneText(action.reason)}</p>
      </article>
    `)
  ]);
}

function renderIntegrations(integrationInputs = {}) {
  const vm = buildIntegrationViewModel(integrationInputs);

  const csmApiStatus = vm.serviceRows.every((row) => row.state === "closed") ? "wired" : "check evidence";
  const cloudwatchStatus = vm.cloudwatchSummary.status || "pending";
  setText("csm-api-status", csmApiStatus);
  setText("hero-csm-api-status", csmApiStatus);
  setText("cloudwatch-status", cloudwatchStatus === "passed" ? "live proof" : formatLabel(cloudwatchStatus));
  setText("hero-cloudwatch-state", cloudwatchStatus === "passed" ? "CloudWatch Proven" : formatLabel(cloudwatchStatus));
  setText(
    "hero-cloudwatch-detail",
    vm.cloudwatchRows.find((row) => row.label === "CloudWatch target")?.detail || "CloudWatch heartbeat proof pending load."
  );
  setState("hero-cloudwatch-state", vm.cloudwatchSummary.status || "pending");
  setText("cloudwatch-event-count", `${vm.parsedEvents.length} events`);

  renderRows("csm-api-list", vm.serviceRows.map((row) => `
    <article class="integration-row" data-state="${row.state}">
      <span class="row-kicker">${formatLabel(row.label)}</span>
      <strong>${formatLabel(row.value)}</strong>
      <p class="row-detail">${row.detail}</p>
    </article>
  `));

  renderRows("hero-api-list", vm.serviceRows.slice(0, 3).map((row, index) => `
    <span class="api-mini-row" data-state="${escapeHtml(stateTone(row.state))}">
      <span>GET ${index === 0 ? "/status" : index === 1 ? "/health" : "/ready"}</span>
      <strong>${row.state === "closed" ? "proved" : escapeHtml(formatLabel(row.state))}</strong>
      <em>retained</em>
    </span>
  `));

  renderRows("cloudwatch-list", vm.cloudwatchRows.map((row) => `
    <article class="integration-row" data-state="${row.state}">
      <span class="row-kicker">${formatLabel(row.label)}</span>
      <strong>${formatLabel(row.value)}</strong>
      <p class="row-detail">${row.detail}</p>
    </article>
  `));

  renderRows("cloudwatch-events-list", vm.parsedEvents.slice(-5).map((event, index) => `
    <li class="trace-row">
      <span class="trace-seq">${String(index + 1).padStart(2, "0")}</span>
      <span><strong>${formatLabel(event.signal_kind)} / ${formatLabel(event.payload?.state)}</strong><br><span class="row-detail">${event.runtime_id || "unknown runtime"} / ${event.timestamp || "unknown timestamp"}</span></span>
    </li>
  `));

  renderRows("aws-linkage-list", AWS_LINKAGES.map((linkage) => `
    <article class="linkage-row" data-state="${linkage.state}">
      <span class="row-kicker">#${linkage.issue} / ${formatLabel(linkage.state)}</span>
      <strong>${linkage.label}</strong>
      <p class="row-detail">${linkage.proof} Evidence: ${linkage.evidence}.</p>
    </article>
  `));

  renderRows("communication-proof-list", vm.acipRows.map((row) => `
    <article class="communication-proof-row" data-state="${row.state}">
      <span class="row-kicker">${formatLabel(row.label)}</span>
      <strong>${formatLabel(row.value)}</strong>
      <p class="row-detail">${row.detail}</p>
    </article>
  `));

  renderRows("compact-comms-proof", vm.acipRows.slice(0, 3).map((row) => `
    <span class="compact-proof-chip" data-state="${row.state}">
      <span>${formatLabel(row.label)}</span>
      <strong>${formatLabel(row.value)}</strong>
    </span>
  `));
}

function bindCommunication(packet = FALLBACK_PACKET, acipSnsSummary = {}, snsResourceSummary = {}) {
  const channel = document.getElementById("operator-channel");
  const message = document.getElementById("operator-message");
  const compactMessage = document.getElementById("compact-operator-message");
  const apiBase = document.getElementById("runtime-api-base");
  const prepare = document.getElementById("prepare-envelope");
  const checkEvents = document.getElementById("check-events");
  const compactClear = document.getElementById("compact-clear-envelope");
  const packetId = displayPacketId(packet.packet_id || "");
  const setCommunicationStatus = (status) => {
    setText("communication-status", status);
    setText("hero-communication-status", status);
  };

  const updateEnvelope = () => {
    const envelope = buildOperatorEnvelope({
      channel: channel?.value || "events",
      message: compactMessage?.value || message?.value || "",
      packetId,
      acipSnsSummary,
      snsResourceSummary
    });
    renderEnvelope(envelope);
    setCommunicationStatus("envelope ready");
  };

  prepare?.addEventListener("click", updateEnvelope);
  compactMessage?.addEventListener("input", () => {
    if (message) {
      message.value = compactMessage.value;
    }
  });
  compactClear?.addEventListener("click", () => {
    if (message) {
      message.value = "";
    }
    if (compactMessage) {
      compactMessage.value = "";
    }
    renderEnvelope({});
    setCommunicationStatus("draft cleared");
  });
  checkEvents?.addEventListener("click", async () => {
    setCommunicationStatus("checking /events");
    try {
      const events = await checkEventsEndpoint(apiBase?.value || "");
      const eventEntries = normalizeEventEntries(events);
      const envelope = buildOperatorEnvelope({
        channel: "events",
        message: `Read ${eventEntries.length} retained CSM events from live API.`,
        packetId,
        acipSnsSummary,
        snsResourceSummary
      });
      renderEnvelope({ ...envelope, live_event_count: eventEntries.length });
      setCommunicationStatus("events reachable");
    } catch (error) {
      renderEnvelope({
        ...buildOperatorEnvelope({
          channel: channel?.value || "events",
          message: compactMessage?.value || message?.value || "",
          packetId,
          acipSnsSummary,
          snsResourceSummary
        }),
        live_check_error: error instanceof Error ? error.message : "unknown error"
      });
      setCommunicationStatus("offline draft");
    }
  });

  updateEnvelope();
}

function bindLivePanopticon(packet = FALLBACK_PACKET) {
  const apiBase = document.getElementById("live-api-base");
  const dashboardBase = document.getElementById("dashboard-live-api-base");
  const communicationBase = document.getElementById("runtime-api-base");
  const connect = document.getElementById("connect-live");
  const refresh = document.getElementById("refresh-live");
  const stop = document.getElementById("stop-live");
  const dashboardConnect = document.getElementById("dashboard-connect-live");
  const dashboardRefresh = document.getElementById("dashboard-refresh-live");
  const dashboardStop = document.getElementById("dashboard-stop-live");
  const modeSelect = document.getElementById("top-mode-select");
  const operatorToken = document.getElementById("operator-write-token");
  const operatorLogin = document.getElementById("operator-login");
  const operatorLogout = document.getElementById("operator-logout");
  const operatorAuthStatus = document.getElementById("operator-auth-status");
  const signedControlCommand = document.getElementById("signed-control-command");
  const sendSignedCommand = document.getElementById("send-signed-command");
  const operatorControlResult = document.getElementById("operator-control-result");
  const conversationRecipient = document.getElementById("agent-conversation-recipient");
  const conversationMessage = document.getElementById("agent-conversation-message");
  const conversationSend = document.getElementById("send-agent-conversation");
  const conversationStatus = document.getElementById("agent-conversation-status");
  const conversationTranscript = document.getElementById("agent-conversation-transcript");
  const pendingConversationTurns = new Map();
  const roomRecipients = document.getElementById("governed-room-recipients");
  const roomParticipants = document.getElementById("governed-room-participants");
  const roomTranscript = document.getElementById("governed-room-transcript");
  const roomMessage = document.getElementById("governed-room-message");
  const roomSend = document.getElementById("send-governed-room-turn");
  const roomStatus = document.getElementById("governed-room-status");
  const governedRoomSequences = new Map();
  let conversationAuthorized = false;
  const rosterSearch = document.getElementById("roster-search");
  const rosterPresence = document.getElementById("roster-presence-filter");
  const rosterSort = document.getElementById("roster-sort");
  const rosterList = document.getElementById("live-agent-list");
  const rosterLoadMore = document.getElementById("roster-load-more");
  let lastLiveError = null;
  let runtimeBaseActive = false;
  let liveSocket = null;
  let liveStoppedByOperator = false;
  let liveRequestGeneration = 0;
  let runtimeV3Readiness = null;
  let runtimeV3ReadinessRefresh = null;
  let liveRuntimeIncarnationId = null;
  const nextLiveGeneration = () => {
    liveRequestGeneration += 1;
    return liveRequestGeneration;
  };
  const isCurrentLiveGeneration = (generation) => generation === liveRequestGeneration;
  const fetchCurrentRuntimeV3Readiness = async (base) => {
    const readiness = await fetchRuntimeV3Readiness(normalizeTrustedRuntimeV3ApiBase(base));
    return {
      schema: readiness?.schema,
      status: readiness?.ready ? "ready" : "degraded",
      ready: readiness?.ready === true,
      state: readiness?.ready ? "ready" : "degraded",
      blocking_reasons: asArray(readiness?.degraded_reasons),
      weather_freshness: readiness?.weather_freshness
    };
  };
  const refs = {
    statusRef: document.querySelector(".observatory")?.dataset.csmStatusRef || "",
    healthRef: document.querySelector(".observatory")?.dataset.csmHealthRef || "",
    readyRef: document.querySelector(".observatory")?.dataset.csmReadyRef || "",
    metricsRef: document.querySelector(".observatory")?.dataset.csmMetricsRef || "",
    eventsRef: document.querySelector(".observatory")?.dataset.csmEventsRef || ""
  };

  const mirrorApiBase = (base) => {
    [apiBase, dashboardBase, communicationBase].forEach((input) => {
      if (input && base && !input.value) {
        input.value = base;
      }
    });
  };

  const setRuntimeTestStatus = (status, detail = "") => {
    setText("dashboard-live-test-status", status);
    setState("dashboard-live-test-status", status);
    if (detail) {
      setText("dashboard-live-test-detail", detail);
    }
  };

  const setWriteAccess = (enabled, status, detail) => {
    conversationAuthorized = enabled;
    if (operatorAuthStatus) {
      operatorAuthStatus.textContent = status;
      operatorAuthStatus.dataset.state = enabled ? "passed" : "open";
    }
    if (sendSignedCommand) {
      const canPostSignedCommand = requestedRuntimeSelection() === "v3" && isRuntimeV3ApiBase(readApiBase() || getRuntimeV3Config().api_base);
      sendSignedCommand.disabled = !enabled && !canPostSignedCommand;
    }
    if (conversationSend) {
      conversationSend.disabled = !enabled || !conversationRecipient?.value;
    }
    updateRoomSendState();
    if (operatorControlResult && detail) {
      operatorControlResult.textContent = detail;
    }
  };

  const updateConversationRoster = (population) => {
    if (!conversationRecipient) return;
    const previous = conversationRecipient.value;
    const agents = asArray(population?.sample).filter((agent) =>
      agent && typeof agent.id === "string" && agent.communication_eligible === true
    );
    conversationRecipient.replaceChildren();
    if (agents.length === 0) {
      const option = document.createElement("option");
      option.value = "";
      option.textContent = "No live agents";
      conversationRecipient.append(option);
      conversationRecipient.disabled = true;
      if (conversationStatus) conversationStatus.textContent = "waiting for runtime";
    } else {
      agents.forEach((agent) => {
        const option = document.createElement("option");
        option.value = agent.id;
        option.textContent = agent.label || agent.id;
        conversationRecipient.append(option);
      });
      conversationRecipient.value = agents.some((agent) => agent.id === previous)
        ? previous
        : agents[0].id;
      conversationRecipient.disabled = false;
      if (conversationStatus) conversationStatus.textContent = conversationAuthorized ? "ready" : "login required";
    }
    if (conversationSend) {
      conversationSend.disabled = !conversationAuthorized || !conversationRecipient.value;
    }
  };

  const selectedRoomRecipients = () =>
    Array.from(roomRecipients?.selectedOptions || [])
      .map((option) => option.value)
      .filter(Boolean);

  const updateRoomSendState = () => {
    if (roomSend) {
      roomSend.disabled = !conversationAuthorized ||
        selectedRoomRecipients().length === 0 ||
        !(roomMessage?.value || "").trim();
    }
  };

  const renderGovernedRoomParticipants = (participants) => {
    if (!roomParticipants) return;
    if (participants.length === 0) {
      roomParticipants.innerHTML = '<span class="room-participant-empty">No Runtime-eligible participants.</span>';
      return;
    }
    roomParticipants.innerHTML = participants.map((participant) => `
      <span class="room-participant" data-state="${escapeHtml(participant.state)}">
        <strong>${escapeHtml(participant.display_name)}</strong>
        <span>${escapeHtml(participant.participant_id)}</span>
      </span>
    `).join("");
  };

  const updateGovernedRoomRoster = (population) => {
    if (!roomRecipients) return;
    const previous = new Set(selectedRoomRecipients());
    const participants = normalizeGovernedRoomParticipants(population);
    roomRecipients.replaceChildren();
    if (participants.length === 0) {
      const option = document.createElement("option");
      option.value = "";
      option.textContent = "No live agents";
      roomRecipients.append(option);
      roomRecipients.disabled = true;
      if (roomStatus) roomStatus.textContent = "waiting for runtime";
    } else {
      participants.forEach((participant) => {
        const option = document.createElement("option");
        option.value = participant.participant_id;
        option.textContent = participant.display_name;
        option.selected = previous.has(participant.participant_id);
        roomRecipients.append(option);
      });
      roomRecipients.disabled = false;
      if (roomStatus) roomStatus.textContent = conversationAuthorized ? "ready" : "login required";
    }
    renderGovernedRoomParticipants(participants);
    updateRoomSendState();
  };

  const appendRoomTurn = (speaker, message, turnId, status = "", rows = []) => {
    if (!roomTranscript) return null;
    roomTranscript.querySelector(".conversation-empty")?.remove();
    const item = document.createElement("li");
    item.className = "conversation-turn governed-room-turn";
    item.dataset.speaker = speaker;
    if (turnId) item.dataset.turnId = turnId;
    const content = document.createElement("span");
    content.className = "conversation-turn-content";
    content.textContent = message;
    item.append(content);
    if (rows.length > 0) {
      const list = document.createElement("ul");
      list.className = "governed-room-delivery-list";
      list.innerHTML = rows.map((row) => `
        <li data-state="${escapeHtml(row.state)}">
          <strong>${escapeHtml(row.displayName)}</strong>
          <span>${escapeHtml(row.state)} / ${escapeHtml(row.detail)}</span>
        </li>
      `).join("");
      item.append(list);
    }
    const state = document.createElement("span");
    state.className = "conversation-turn-status";
    state.textContent = status;
    item.append(state);
    roomTranscript.append(item);
    pruneLargePolisDomWindow(roomTranscript, ".conversation-turn", LARGE_POLIS_LIMITS.maxTranscriptTurns);
    roomTranscript.scrollTop = roomTranscript.scrollHeight;
    return item;
  };

  const appendConversationTurn = (speaker, message, turnId, status = "") => {
    if (!conversationTranscript) return;
    conversationTranscript.querySelector(".conversation-empty")?.remove();
    const item = document.createElement("li");
    item.className = "conversation-turn";
    item.dataset.speaker = speaker;
    if (turnId) item.dataset.turnId = turnId;
    const content = document.createElement("span");
    content.className = "conversation-turn-content";
    content.textContent = message;
    item.append(content);
    const state = document.createElement("span");
    state.className = "conversation-turn-status";
    state.textContent = status;
    item.append(state);
    conversationTranscript.append(item);
    pruneLargePolisDomWindow(conversationTranscript, ".conversation-turn", LARGE_POLIS_LIMITS.maxTranscriptTurns);
    conversationTranscript.scrollTop = conversationTranscript.scrollHeight;
    return item;
  };

  const conversationTurnElement = (turnId) =>
    Array.from(conversationTranscript?.querySelectorAll(".conversation-turn") || [])
      .find((item) => item.dataset.turnId === turnId && item.dataset.speaker === "operator") || null;

  const setConversationTurnStatus = (pending, status) => {
    const item = conversationTurnElement(pending.turnId);
    const state = item?.querySelector(".conversation-turn-status");
    if (state) state.textContent = status;
    if (conversationStatus) conversationStatus.textContent = status;
  };

  const sendConversationCancel = (pending) => {
    if (!conversationAuthorized || !liveSocket || liveSocket.readyState !== WebSocket.OPEN || pending.terminal) {
      setConversationTurnStatus(pending, "connection required");
      return;
    }
    pending.cancelRequested = true;
    liveSocket.send(JSON.stringify({
      schema: "adl.runtime_v3.observatory_conversation_cancel.v1",
      conversation_id: pending.conversationId,
      turn_id: pending.turnId,
      correlation_id: pending.correlationId
    }));
    pending.cancelButton?.setAttribute("disabled", "");
    setConversationTurnStatus(pending, "cancelling");
  };

  const renderAcceptedConversationTurn = (pending) => {
    if (pending.operatorRendered) return;
    const item = appendConversationTurn("operator", pending.message, pending.turnId, "accepted");
    const cancel = document.createElement("button");
    cancel.type = "button";
    cancel.className = "conversation-turn-cancel";
    cancel.title = "Cancel this turn";
    cancel.setAttribute("aria-label", "Cancel this turn");
    cancel.textContent = "Cancel";
    cancel.addEventListener("click", () => sendConversationCancel(pending));
    item?.append(cancel);
    pending.cancelButton = cancel;
    pending.operatorRendered = true;
  };

  const markPendingConversationsDisconnected = () => {
    for (const pending of pendingConversationTurns.values()) {
      if (pending.terminal) continue;
      pending.disconnected = true;
      setConversationTurnStatus(pending, "disconnected");
    }
  };

  const replayPendingConversationsAfterAuthentication = () => {
    if (!conversationAuthorized || !liveRuntimeIncarnationId ||
        !liveSocket || liveSocket.readyState !== WebSocket.OPEN) return;
    for (const [turnId, pending] of pendingConversationTurns.entries()) {
      const intent = conversationReconnectIntent(pending, liveRuntimeIncarnationId);
      if (!intent && pending.restartUnavailable) {
        pending.cancelButton?.remove();
        setConversationTurnStatus(pending, "restart_unavailable");
        pendingConversationTurns.delete(turnId);
        continue;
      }
      if (!intent) continue;
      liveSocket.send(JSON.stringify(intent));
      setConversationTurnStatus(pending, "reconnecting");
    }
  };

  const renderControlFrame = (frame) => {
    if (frame.status === "authenticated") {
      setWriteAccess(true, "write access enabled", JSON.stringify(frame, null, 2));
      replayPendingConversationsAfterAuthentication();
      return;
    }
    if (frame.schema === GOVERNED_ROOM_ROUTE_SCHEMA ||
        frame.schema === "adl.runtime_v3.observatory_governed_room_result.v1") {
      try {
        const normalized = normalizeGovernedRoomRoute(frame.route || frame);
        appendRoomTurn(
          "runtime",
          normalized.error ? `Room turn rejected: ${normalized.error}` : `Room turn ${normalized.status}`,
          normalized.turn_id,
          normalized.status,
          buildGovernedRoomRows(normalized)
        );
        if (roomStatus) roomStatus.textContent = normalized.status;
        updateRoomSendState();
      } catch (error) {
        if (roomStatus) roomStatus.textContent = error instanceof Error ? error.message : "room frame rejected";
      }
      return;
    }
    if (frame.schema === "adl.runtime_v3.observatory_conversation_result.v1") {
      const pending = pendingConversationTurns.get(frame.turn_id);
      const transition = conversationFrameTransition(frame, pending);
      if (!transition) return;
      if (frame.status === "accepted" && transition.status === "accepted") {
        renderAcceptedConversationTurn(pending);
      }
      if (transition.terminal && !pending.operatorRendered && conversationFrameProvesAcceptance(frame)) {
        renderAcceptedConversationTurn(pending);
      }
      if (transition.terminal && !pending.operatorRendered) {
        appendConversationTurn(
          "runtime",
          `Turn ${transition.status}${frame.error ? `: ${frame.error}` : ""}`,
          pending.turnId,
          transition.status
        );
      }
      setConversationTurnStatus(pending, transition.status);
      if (transition.reply) {
        const speaker = transition.senderId
          ? `agent:${transition.senderId}`
          : "agent";
        const status = transition.initiatedWorkId
          ? `delivered / A2A ${transition.initiatedWorkId}`
          : "delivered";
        appendConversationTurn(speaker, transition.reply, pending.turnId, status);
      }
      if (transition.terminal) {
        pending.terminal = true;
        pending.cancelButton?.remove();
        pendingConversationTurns.delete(frame.turn_id);
      }
      if (conversationSend) {
        conversationSend.disabled = !conversationAuthorized || !conversationRecipient?.value;
      }
      return;
    }
    if (frame.error === "credential_revoked" ||
        frame.error === "authentication_failed" ||
        frame.error === "write_authentication_required") {
      setWriteAccess(false, "public read", JSON.stringify(frame, null, 2));
      return;
    }
    if (operatorControlResult) {
      operatorControlResult.textContent = JSON.stringify(frame, null, 2);
    }
  };

  const readApiBase = () => normalizeApiBase(dashboardBase?.value || apiBase?.value || communicationBase?.value || "");
  const setLiveConnectionState = (state) => {
    document.querySelector(".observatory")?.setAttribute("data-live-connection", state);
  };
  mirrorApiBase(getQueryApiBase());

  if (rosterSearch && !rosterSearch.dataset.rosterBound) {
    rosterSearch.dataset.rosterBound = "true";
    rosterSearch.addEventListener("input", () => {
      rosterUiState.filter = rosterSearch.value;
      if (lastPanopticonSnapshot) renderPanopticon(lastPanopticonSnapshot, lastPanopticonPacket);
    });
    rosterPresence?.addEventListener("change", () => {
      rosterUiState.presence = rosterPresence.value;
      if (lastPanopticonSnapshot) renderPanopticon(lastPanopticonSnapshot, lastPanopticonPacket);
    });
    rosterSort?.addEventListener("change", () => {
      rosterUiState.sort = rosterSort.value;
      if (lastPanopticonSnapshot) renderPanopticon(lastPanopticonSnapshot, lastPanopticonPacket);
    });
    rosterList?.addEventListener("click", async (event) => {
      const row = event.target instanceof Element
        ? event.target.closest("[data-agent-id]")
        : null;
      if (!row) return;
      rosterUiState.selectedId = row.dataset.agentId;
      if (lastPanopticonSnapshot) renderPanopticon(lastPanopticonSnapshot, lastPanopticonPacket);
      try {
        const detail = await fetchRuntimeV3AgentDetail(getQueryApiBase(), rosterUiState.selectedId);
        const population = lastPanopticonSnapshot?.status?.agent_population;
        const selected = asArray(population?.sample).find((agent) => agent.id === detail.id);
        if (selected) Object.assign(selected, detail, { state: detail.presence });
        if (lastPanopticonSnapshot) renderPanopticon(lastPanopticonSnapshot, lastPanopticonPacket);
      } catch (error) {
        await renderLiveError(error);
      }
    });
    rosterLoadMore?.addEventListener("click", async () => {
      const population = lastPanopticonSnapshot?.status?.agent_population;
      if (!population?.next_page_token || rosterLoadMore.disabled) return;
      rosterLoadMore.disabled = true;
      try {
        const page = await fetchRuntimeV3AgentRosterPage(getQueryApiBase(), population.next_page_token);
        const known = new Map(asArray(population.sample).map((agent) => [agent.id, agent]));
        asArray(page.sample).forEach((agent) => known.set(agent.id, agent));
        population.sample = [...known.values()];
        population.rendered_sample_count = population.sample.length;
        population.has_more = page.has_more === true;
        population.next_page_token = page.next_page_token || null;
        population.revision = page.revision;
        renderPanopticon(lastPanopticonSnapshot, lastPanopticonPacket);
      } catch (error) {
        await renderLiveError(error);
      } finally {
        rosterLoadMore.disabled = false;
      }
    });
  }

  const renderMinimalFallback = (error) => {
    renderPanopticon({
      mode: "retained",
      fetchedAt: new Date().toISOString(),
      errors: {
        retained: error instanceof Error ? error.message : "unknown retained mirror error"
      }
    }, packet);
    setText("live-status", "retained fallback");
    setRuntimeTestStatus("retained fallback", error instanceof Error ? error.message : "Retained runtime mirror is available; live loopback is not connected.");
  };

  const refreshRetained = async (extraErrors = {}, requestGeneration = nextLiveGeneration()) => {
    try {
      const snapshot = await fetchRetainedRuntimeSnapshot(refs);
      if (!isCurrentLiveGeneration(requestGeneration)) {
        return;
      }
      const mergedSnapshot = {
        ...snapshot,
        errors: {
          ...(snapshot.errors || {}),
          ...(lastLiveError ? { live: lastLiveError } : {}),
          ...extraErrors
        }
      };
      renderPanopticon(mergedSnapshot, packet);
      const status = Object.keys(mergedSnapshot.errors || {}).length ? "published partial" : "published runtime mirror";
      setText("live-status", status);
      setRuntimeTestStatus(status, lastLiveError ? `Live loopback not proved: ${lastLiveError}` : "Using retained publishable CSM API artifacts until a loopback runtime is connected.");
    } catch (error) {
      if (!isCurrentLiveGeneration(requestGeneration)) {
        return;
      }
      renderMinimalFallback(error);
    }
  };

  const renderLiveError = async (error, requestGeneration) => {
    if (!isCurrentLiveGeneration(requestGeneration)) {
      return;
    }
    lastLiveError = error instanceof Error ? error.message : "unknown live polling error";
    if (requestedRuntimeSelection() === "v3") {
      const base = readApiBase() || getRuntimeV3Config().api_base;
      try {
        const snapshot = await fetchRuntimeSnapshot(base);
        if (!isCurrentLiveGeneration(requestGeneration)) {
          return;
        }
        const mergedSnapshot = {
          ...snapshot,
          errors: {
            ...(snapshot.errors || {}),
            websocket: lastLiveError
          }
        };
        renderPanopticon(mergedSnapshot, packet);
        setText("live-status", "live read / stream unavailable");
        setText("statusbar-websocket", "disconnected");
        setLiveConnectionState("live-read");
        setRuntimeTestStatus("live read / stream unavailable", `Runtime v3 GET feed is active; WebSocket stream not proved: ${lastLiveError}`);
        setWriteAccess(false, "signed post available", "Paste a signed Runtime v3 command and send it through /v1/control, or log in when WSS is available.");
        return;
      } catch (_refreshError) {
        // Fall through to retained evidence only when the Runtime v3 GET feed is also unavailable.
      }
    }
    await refreshRetained({
      live: lastLiveError
    }, requestGeneration);
  };

  const refreshLive = async () => {
    const requestGeneration = nextLiveGeneration();
    const base = readApiBase();
    if (!base) {
      runtimeBaseActive = false;
      await refreshRetained({}, requestGeneration);
      return;
    }
    runtimeBaseActive = true;
    if (communicationBase && base && !communicationBase.value) {
      communicationBase.value = base;
    }
    mirrorApiBase(base);
    setText("live-status", "polling loopback");
    setRuntimeTestStatus(
      "polling loopback",
      requestedRuntimeSelection() === "v3"
        ? `Checking ${base}${getRuntimeV3Config().observatory_endpoint} and ${getRuntimeV3Config().readiness_endpoint}.`
        : `Checking ${base}/status, /health, /ready, /metrics, and /events.`
    );
    try {
      const snapshot = await fetchRuntimeSnapshot(base);
      if (liveStoppedByOperator || !isCurrentLiveGeneration(requestGeneration)) {
        return;
      }
      if (snapshot.runtimeSelection === "runtime_v3_explicit_opt_in") {
        runtimeV3Readiness = snapshot.ready || null;
        await authenticateRuntimeRosterSuccessor(base, snapshot);
      }
      const endpointKeys = ["status", "health", "ready", "metrics", "events"];
      const successfulEndpoints = endpointKeys.filter((key) => snapshot[key]);
      if (successfulEndpoints.length === 0) {
        throw new Error("No CSM runtime API endpoints responded from the browser context.");
      }
      lastLiveError = null;
      if (snapshot.runtimeSelection !== "runtime_v3_explicit_opt_in" || acceptRuntimeRosterSnapshot(snapshot)) {
        renderPanopticon(snapshot, packet);
      }
      const status = Object.keys(snapshot.errors || {}).length ? "live partial" : "live loopback";
      setText("live-status", status);
      const runtimeKind = snapshot.runtimeSelection === "runtime_v3_explicit_opt_in" ? "Runtime v3 observatory feed" : "loopback CSM server";
      setRuntimeTestStatus(status, Object.keys(snapshot.errors || {}).length ? "Runtime reached, but one or more endpoints failed." : `Runtime API endpoints responded from the ${runtimeKind}.`);
      setWriteAccess(false, "signed post available", "Paste a signed Runtime v3 command and send it through /v1/control, or log in when WSS is available.");
    } catch (error) {
      if (liveStoppedByOperator || !isCurrentLiveGeneration(requestGeneration)) {
        return;
      }
      await renderLiveError(error, requestGeneration);
    }
  };

  const stopPolling = ({ resetReconnect = true } = {}) => {
    liveStoppedByOperator = true;
    nextLiveGeneration();
    setLiveConnectionState("stopped");
    if (liveSocket) {
      liveSocket.close(1000, "operator_stop");
      liveSocket = null;
    }
    if (livePollTimer) {
      clearInterval(livePollTimer);
      livePollTimer = null;
    }
    if (retainedPollTimer) {
      clearInterval(retainedPollTimer);
      retainedPollTimer = null;
    }
    if (liveReconnectTimer) {
      clearTimeout(liveReconnectTimer);
      liveReconnectTimer = null;
    }
    if (resetReconnect) liveReconnectAttempt = 0;
    lastLiveError = null;
    runtimeV3Readiness = null;
    liveRuntimeIncarnationId = null;
    runtimeBaseActive = false;
    setText("live-status", "polling stopped");
    setText("statusbar-websocket", "stopped");
    setRuntimeTestStatus("polling stopped", "Live polling is stopped; retained mirror remains available.");
  };

  const connectLive = async ({ reconnecting = false } = {}) => {
    stopPolling({ resetReconnect: !reconnecting });
    liveStoppedByOperator = false;
    const requestGeneration = nextLiveGeneration();
    setLiveConnectionState("connecting");
    if (requestedRuntimeSelection() === "v3") {
      runtimeBaseActive = true;
      setText("live-status", "connecting secure stream");
      setText("statusbar-websocket", "connecting");
      try {
        const base = readApiBase();
        const socketEndpoint = new URL(`${base}${getRuntimeV3Config().observatory_websocket_endpoint}`);
        socketEndpoint.protocol = "wss:";
        setRuntimeTestStatus("connecting secure stream", `Opening ${socketEndpoint}.`);
        try {
          runtimeV3Readiness = await fetchCurrentRuntimeV3Readiness(base);
        } catch (error) {
          runtimeV3Readiness = {
            status: "pending",
            ready: false,
            state: "pending",
            blocking_reasons: [error instanceof Error ? error.message : "readiness_unavailable"]
          };
        }
        let socket;
        socket = connectRuntimeV3ObservatoryWebSocket(
          base,
          async (snapshot) => {
            if (liveStoppedByOperator || liveSocket !== socket || !isCurrentLiveGeneration(requestGeneration)) {
              return;
            }
            lastLiveError = null;
            const streamSnapshot = runtimeV3Readiness
              ? { ...snapshot, ready: runtimeV3Readiness }
              : snapshot;
            liveRuntimeIncarnationId = streamSnapshot.status?.runtime_incarnation_id || null;
            try {
              await authenticateRuntimeRosterSuccessor(base, streamSnapshot);
            } catch (error) {
              await renderLiveError(error, requestGeneration);
              return;
            }
            replayPendingConversationsAfterAuthentication();
            if (!acceptRuntimeRosterSnapshot(streamSnapshot)) return;
            renderPanopticon(streamSnapshot, packet);
            updateConversationRoster(streamSnapshot.status?.agent_population);
            updateGovernedRoomRoster(streamSnapshot.status?.agent_population);
            if (runtimeV3Readiness?.ready !== true && !runtimeV3ReadinessRefresh) {
              runtimeV3ReadinessRefresh = fetchCurrentRuntimeV3Readiness(base)
                .then((readiness) => {
                  if (liveStoppedByOperator || liveSocket !== socket || !isCurrentLiveGeneration(requestGeneration)) return;
                  runtimeV3Readiness = readiness;
                  renderPanopticon({ ...snapshot, ready: readiness }, packet);
                })
                .catch(() => {})
                .finally(() => {
                 runtimeV3ReadinessRefresh = null;
               });
            }
            liveReconnectAttempt = 0;
            setText("live-status", "live secure stream");
            setText("statusbar-websocket", "connected");
            setLiveConnectionState("connected");
            setRuntimeTestStatus("live secure stream", "Runtime v3 public WebSocket feed is active; operator login is required only for writes.");
          },
          (error) => {
            if (liveStoppedByOperator || liveSocket !== socket || !isCurrentLiveGeneration(requestGeneration)) {
              return;
            }
            setText("statusbar-websocket", "disconnected");
            renderLiveError(error, requestGeneration);
          },
          (error) => {
            if (liveStoppedByOperator || !isCurrentLiveGeneration(requestGeneration)) {
              return;
            }
            if (liveSocket === socket) {
              markPendingConversationsDisconnected();
              liveSocket = null;
              setText("statusbar-websocket", "disconnected");
              setWriteAccess(false, "public read", "The live connection closed. Public monitoring can reconnect without login.");
              renderLiveError(error, requestGeneration);
              const delay = Math.min(250 * (2 ** liveReconnectAttempt), 4_000);
              liveReconnectAttempt = Math.min(liveReconnectAttempt + 1, 5);
              liveReconnectTimer = setTimeout(() => {
                liveReconnectTimer = null;
                if (!liveStoppedByOperator) connectLive({ reconnecting: true });
              }, delay);
            }
          },
          (frame) => {
            if (liveStoppedByOperator || liveSocket !== socket || !isCurrentLiveGeneration(requestGeneration)) {
              return;
            }
            renderControlFrame(frame);
          }
        );
        liveSocket = socket;
      } catch (error) {
        if (!isCurrentLiveGeneration(requestGeneration)) {
          return;
        }
        setText("statusbar-websocket", "disconnected");
        renderLiveError(error, requestGeneration);
      }
      return;
    }
    refreshLive();
    livePollTimer = setInterval(refreshLive, 3000);
  };

  if (connect) connect.onclick = () => connectLive();
  if (refresh) refresh.onclick = refreshLive;
  if (stop) stop.onclick = stopPolling;
  if (dashboardConnect) dashboardConnect.onclick = () => connectLive();
  if (dashboardRefresh) dashboardRefresh.onclick = refreshLive;
  if (dashboardStop) dashboardStop.onclick = stopPolling;
  modeSelect?.addEventListener("change", () => {
    if (modeSelect.value === "live") {
      connectLive();
      return;
    }
    stopPolling();
    if (modeSelect.value === "published") {
      refreshRetained();
      return;
    }
    renderPanopticon({
      mode: "retained",
      fetchedAt: new Date().toISOString(),
      status: {},
      health: {},
      ready: {},
      metrics: {},
      events: [],
      errors: {}
    }, packet);
    setText("live-status", "retained mirror");
    setRuntimeTestStatus("retained mirror", "Showing the retained proof packet without live or published endpoint polling.");
  });
  operatorLogin?.addEventListener("click", () => {
    const token = operatorToken?.value.trim() || "";
    if (!token) {
      setWriteAccess(false, "login required", "Enter the operator write token.");
      return;
    }
    globalThis.sessionStorage?.setItem("adl.runtimeV3.observatoryToken", token);
    if (!liveSocket || liveSocket.readyState !== WebSocket.OPEN) {
      setWriteAccess(false, "connecting", "Opening the public stream before operator login.");
      connectLive();
      return;
    }
    setWriteAccess(false, "logging in", "Authenticating write access...");
    authenticateRuntimeV3ObservatorySocket(liveSocket, token);
  });
  operatorLogout?.addEventListener("click", () => {
    globalThis.sessionStorage?.removeItem("adl.runtimeV3.observatoryToken");
    if (operatorToken) {
      operatorToken.value = "";
    }
    liveSocket?.close(1000, "operator_logout");
    liveSocket = null;
    setWriteAccess(false, "public read", "Write access cleared. Public monitoring remains available.");
    connectLive();
  });
  sendSignedCommand?.addEventListener("click", () => {
    const base = readApiBase() || getRuntimeV3Config().api_base;
    try {
      const command = JSON.parse(signedControlCommand?.value || "");
      if (command.schema !== "adl.runtime.control_command.v1") {
        throw new Error("Expected an adl.runtime.control_command.v1 signed envelope.");
      }
      if (requestedRuntimeSelection() === "v3") {
        if (operatorControlResult) {
          operatorControlResult.textContent = "Submitting signed command through /v1/control...";
        }
        submitRuntimeV3SignedControlCommand(base, command)
          .then((response) => {
            if (operatorControlResult) {
              operatorControlResult.textContent = JSON.stringify(response, null, 2);
            }
            setCommunicationStatus("signed command sent");
          })
          .catch((error) => {
            if (operatorControlResult) {
              operatorControlResult.textContent = error instanceof Error ? error.message : "Signed command failed.";
            }
            setCommunicationStatus("signed command rejected");
          });
        return;
      }
      if (!liveSocket || liveSocket.readyState !== WebSocket.OPEN) {
        setWriteAccess(false, "connection required", "Connect to the Runtime v3 Observatory before sending a command.");
        return;
      }
      liveSocket.send(JSON.stringify(command));
      if (operatorControlResult) {
        operatorControlResult.textContent = "Signed command submitted; awaiting runtime verification.";
      }
    } catch (error) {
      if (operatorControlResult) {
        operatorControlResult.textContent = error instanceof Error ? error.message : "Invalid signed command.";
      }
    }
  });
  conversationRecipient?.addEventListener("change", () => {
    if (conversationSend) {
      conversationSend.disabled = !conversationAuthorized || !conversationRecipient.value;
    }
  });
  roomRecipients?.addEventListener("change", updateRoomSendState);
  roomMessage?.addEventListener("input", updateRoomSendState);
  conversationSend?.addEventListener("click", () => {
    const message = conversationMessage?.value.trim() || "";
    const recipientId = conversationRecipient?.value || "";
    if (!conversationAuthorized || !liveSocket || liveSocket.readyState !== WebSocket.OPEN) {
      if (conversationStatus) conversationStatus.textContent = "login required";
      return;
    }
    if (!recipientId || !message || message.length > 4096) {
      if (conversationStatus) conversationStatus.textContent = "message required";
      return;
    }
    const randomId = globalThis.crypto?.randomUUID?.().replaceAll("-", "") || `${Date.now().toString(16).padStart(32, "0")}`;
    const turnId = `turn-${randomId}`;
    const conversationId = `conversation-${recipientId}`;
    if (!liveRuntimeIncarnationId) {
      if (conversationStatus) conversationStatus.textContent = "runtime incarnation unavailable";
      return;
    }
    const intent = {
      schema: "adl.runtime_v3.observatory_conversation_intent.v1",
      conversation_id: conversationId,
      turn_id: turnId,
      recipient_id: recipientId,
      correlation_id: randomId,
      message
    };
    pendingConversationTurns.set(turnId, {
      conversationId,
      turnId,
      recipientId,
      correlationId: randomId,
      message,
      intent,
      runtimeIncarnationId: liveRuntimeIncarnationId,
      operatorRendered: false,
      disconnected: false,
      reconnectReplayCount: 0,
      terminal: false,
      cancelRequested: false,
      cancelButton: null
    });
    liveSocket.send(JSON.stringify(intent));
    if (conversationMessage) conversationMessage.value = "";
    if (conversationStatus) conversationStatus.textContent = "awaiting runtime";
    conversationSend.disabled = true;
  });
  roomSend?.addEventListener("click", () => {
    const message = roomMessage?.value.trim() || "";
    const recipients = selectedRoomRecipients();
    if (!conversationAuthorized || !liveSocket || liveSocket.readyState !== WebSocket.OPEN) {
      if (roomStatus) roomStatus.textContent = "login required";
      return;
    }
    if (!liveRuntimeIncarnationId) {
      if (roomStatus) roomStatus.textContent = "runtime incarnation unavailable";
      return;
    }
    try {
      const randomId = globalThis.crypto?.randomUUID?.().replaceAll("-", "") || `${Date.now().toString(16).padStart(32, "0")}`;
      const turnId = `room-turn-${randomId}`;
      const roomIdentity = governedRoomIdentityForRecipients(recipients);
      const turnSequence = nextGovernedRoomTurnSequence(governedRoomSequences, roomIdentity.roomId);
      const intent = buildGovernedRoomTurnIntent({
        roomId: roomIdentity.roomId,
        turnId,
        turnSequence,
        senderId: "operator",
        correlationId: randomId,
        recipients: roomIdentity.addressedRecipients,
        message
      });
      const envelope = {
        schema: "adl.runtime_v3.observatory_governed_room_intent.v1",
        runtime_incarnation_id: liveRuntimeIncarnationId,
        intent
      };
      liveSocket.send(JSON.stringify(envelope));
      const selectedLabels = new Map(Array.from(roomRecipients?.selectedOptions || [])
        .map((option) => [option.value, option.textContent || option.value]));
      appendRoomTurn("operator", message, turnId, "awaiting runtime", intent.addressed_recipients.map((recipientId) => ({
        recipientId,
        displayName: selectedLabels.get(recipientId) || recipientId,
        state: "prepared",
        detail: "explicit recipient"
      })));
      if (roomMessage) roomMessage.value = "";
      if (roomStatus) roomStatus.textContent = "awaiting runtime";
      roomSend.disabled = true;
    } catch (error) {
      if (roomStatus) roomStatus.textContent = error instanceof Error ? error.message : "invalid room turn";
    }
  });

  const queryApiBase = getQueryApiBase();
  if (queryApiBase) {
    refreshLive();
  } else {
    refreshRetained();
  }
  if (queryApiBase && shouldAutoConnectLive()) {
    connectLive();
  }
  if (!retainedPollTimer && !runtimeBaseActive && !queryApiBase) {
    retainedPollTimer = setInterval(refreshRetained, 3000);
  }
}

async function loadText(ref) {
  if (!ref) {
    return "";
  }
  const response = await fetch(ref);
  if (!response.ok) {
    throw new Error(`failed to load ${ref}: ${response.status}`);
  }
  return response.text();
}

async function loadJson(ref) {
  const text = await loadText(ref);
  return JSON.parse(text);
}

async function loadRuntimeV3Config(root) {
  const configRef = root?.dataset.runtimeV3ConfigRef || "./runtime-v3.config.json";
  try {
    return applyRuntimeV3Config(await loadJson(configRef));
  } catch (_error) {
    return applyRuntimeV3Config(RUNTIME_V3_DEFAULT_CONFIG);
  }
}

async function bootObservatory() {
  const root = document.querySelector(".observatory");
  const packetRef = root?.dataset.packetRef || "";
  const reportRef = root?.dataset.reportRef || "";
  const csmServiceRef = root?.dataset.csmServiceRef || "";
  const csmApiRef = root?.dataset.csmApiRef || "";
  const cloudwatchRef = root?.dataset.cloudwatchRef || "";
  const cloudwatchEventsRef = root?.dataset.cloudwatchEventsRef || "";
  const acipSnsRef = root?.dataset.acipSnsRef || "";
  const snsResourceRef = root?.dataset.snsResourceRef || "";
  setHref("packet-link", packetRef);
  setHref("report-link", reportRef);
  const runtimeConfig = await loadRuntimeV3Config(root);
  const runtimeApiBase = getQueryApiBase();
  if (requestedRuntimeSelection() === "v3" && runtimeApiBase) {
    setHref("packet-link", `${runtimeApiBase}${runtimeConfig.observatory_endpoint}`);
    setHref("report-link", `${runtimeApiBase}${runtimeConfig.observatory_docs_endpoint}`);
  }

  try {
    const [packet, reportText, serviceManifest, apiText, cloudwatchSummary, cloudwatchEvents, acipSnsSummary, snsResourceSummary] = await Promise.all([
      loadJson(packetRef),
      loadText(reportRef).catch(() => ""),
      loadJson(csmServiceRef).catch(() => ({})),
      loadText(csmApiRef).catch(() => ""),
      loadJson(cloudwatchRef).catch(() => ({})),
      loadJson(cloudwatchEventsRef).catch(() => ({})),
      loadJson(acipSnsRef).catch(() => ({})),
      loadJson(snsResourceRef).catch(() => ({}))
    ]);
    renderObservatory(packet, reportText, "ok");
    renderIntegrations({ serviceManifest, apiText, cloudwatchSummary, cloudwatchEvents, acipSnsSummary, snsResourceSummary });
    renderLayer8DeliveryPanel(packet.layer8_delivery_states || packet.layer8_acknowledgements || []);
    renderOperatorAttentionInbox(packet);
    bindDashboardNavigation(packet);
    bindCommunication(packet, acipSnsSummary, snsResourceSummary);
    bindLivePanopticon(packet);
  } catch (_error) {
    renderObservatory(FALLBACK_PACKET, "", "fallback");
    renderIntegrations();
    renderLayer8DeliveryPanel([]);
    renderOperatorAttentionInbox(FALLBACK_PACKET);
    bindDashboardNavigation(FALLBACK_PACKET);
    bindCommunication(FALLBACK_PACKET);
    bindLivePanopticon(FALLBACK_PACKET);
  }
}

if (typeof document !== "undefined") {
  bootObservatory();
}

globalThis.AdlHtmlObservatory = {
  FALLBACK_PACKET,
  AWS_LINKAGES,
  formatLabel,
  parseCloudWatchEventMessage,
  buildOperatorEnvelope,
  normalizeApiBase,
  isLoopbackApiBase,
  getQueryApiBase,
  checkEventsEndpoint,
  fetchRuntimeSnapshot,
  fetchRuntimeV3ObservatorySnapshot,
  fetchRuntimeV3Health,
  fetchRuntimeV3AgentRosterPage,
  fetchRuntimeV3AgentDetail,
  authenticateRuntimeRosterSuccessor,
  submitRuntimeV3SignedControlCommand,
  runtimeV3SnapshotFromFeed,
  projectPolisIdentity,
  connectRuntimeV3ObservatoryWebSocket,
  authenticateRuntimeV3ObservatorySocket,
  conversationFrameTransition,
  conversationFrameProvesAcceptance,
  conversationReconnectIntent,
  conversationReplyFromFrame,
  normalizeRuntimeConversationHistorySnapshot,
  restoreConversationTranscriptFromRuntimeHistory,
  safeConversationHistoryText,
  isSafeGovernedRoomIdentifier,
  normalizeGovernedRoomParticipants,
  normalizeExplicitGovernedRoomRecipients,
  governedRoomIdentityForRecipients,
  nextGovernedRoomTurnSequence,
  buildGovernedRoomTurnIntent,
  normalizeGovernedRoomRoute,
  buildGovernedRoomRows,
  LARGE_POLIS_LIMITS,
  retainedLargePolisWindow,
  pruneLargePolisDomWindow,
  largePolisRecoveryViewModel,
  largePolisRecoverySequence,
  estimateLargePolisResourceMetrics,
  buildLargePolisPerformanceRecoveryFixture,
  evaluateLargePolisPerformanceRecovery,
  LAYER8_RECIPIENT_ACK_ENDPOINT,
  normalizeLayer8DeliveryState,
  layer8DeliveryRows,
  renderLayer8DeliveryPanel,
  submitLayer8RecipientAcknowledgement,
  normalizeOperatorAttentionRequest,
  operatorAttentionRows,
  operatorAttentionViewModel,
  operatorAttentionActionPayload,
  renderOperatorAttentionInbox,
  hasForbiddenLayer8Disclosure,
  fetchRetainedRuntimeSnapshot,
  requestedRuntimeSelection,
  getRuntimeV3Config,
  applyRuntimeV3Config,
  isRuntimeV3ApiBase,
  normalizeTrustedRuntimeV3ApiBase,
  buildRuntimeAgentRows,
  acceptRuntimeRosterSnapshot,
  runtimeRosterCursorState,
  buildViewModel,
  buildIntegrationViewModel,
  buildPanopticonViewModel,
  normalizeEventEntries,
  renderObservatory,
  renderIntegrations,
  bindDashboardNavigation,
  updateDashboardFocus,
  bindLivePanopticon,
  renderPanopticon
};
