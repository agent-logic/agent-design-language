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
  cloudwatchEvents = {}
} = {}) {
  const events = asArray(cloudwatchEvents.events);
  const parsedEvents = events.map(parseCloudWatchEventMessage).filter((event) => Object.keys(event).length > 0);
  const latestEvent = parsedEvents.at(-1) || {};
  const cloudwatch = cloudwatchSummary.cloudwatch || {};
  const heartbeat = cloudwatchSummary.heartbeat || {};
  const redaction = cloudwatchSummary.redaction || {};

  return {
    serviceManifest,
    apiText,
    cloudwatchSummary,
    cloudwatchEvents,
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
    ]
  };
}

function setText(id, value) {
  const target = document.getElementById(id);
  if (target) {
    target.textContent = value;
  }
}

function setHref(id, value) {
  const target = document.getElementById(id);
  if (target) {
    target.href = value;
  }
}

function renderRows(targetId, rows) {
  const target = document.getElementById(targetId);
  if (target) {
    target.innerHTML = rows.join("");
  }
}

function buildOperatorEnvelope({ channel = "events", message = "", packetId = "" } = {}) {
  return {
    schema: "adl.html_observatory.operator_message.v1",
    channel,
    intent: "operator_communication",
    delivery: "prepared_client_side",
    runtime_mutation_claimed: false,
    packet_id: packetId,
    message: String(message || "").slice(0, 800),
    allowed_live_check: channel === "events" ? "/events" : null
  };
}

function renderEnvelope(envelope) {
  const target = document.getElementById("message-envelope");
  if (target) {
    target.textContent = JSON.stringify(envelope, null, 2);
  }
}

function normalizeApiBase(value) {
  return String(value || "").trim().replace(/\/+$/, "");
}

function isLoopbackApiBase(value) {
  const base = normalizeApiBase(value);
  try {
    const parsed = new URL(base);
    return (
      ["http:", "https:"].includes(parsed.protocol) &&
      ["127.0.0.1", "localhost", "[::1]", "::1"].includes(parsed.hostname)
    );
  } catch (_error) {
    return false;
  }
}

async function checkEventsEndpoint(apiBase) {
  const base = normalizeApiBase(apiBase);
  if (!base) {
    throw new Error("Enter a loopback CSM API base first.");
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

function renderObservatory(packet, reportText = "", state = "ok") {
  const vm = buildViewModel(packet, reportText);
  const source = vm.packet.source || {};
  const manifold = vm.packet.manifold || {};
  const pulse = vm.packet.kernel?.pulse || {};

  setText("packet-status", state === "ok" ? "Runtime packet loaded" : "Fallback shell");
  document.getElementById("packet-status")?.setAttribute("data-state", state);
  setText("claim-boundary", source.claim_boundary || "No claim boundary recorded.");
  setText("evidence-level", formatLabel(source.evidence_level));
  document.getElementById("evidence-level")?.setAttribute("data-tone", state === "ok" ? "ok" : "warn");
  setText("packet-heading", manifold.display_name || "Runtime / Ops Soak");
  setText("manifold-id", manifold.manifold_id || "unknown");
  setText("manifold-state", formatLabel(manifold.state));
  setText("manifold-tick", String(manifold.current_tick ?? 0));
  setText("packet-id", vm.packet.packet_id || "unknown");
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
        <span class="row-kicker">available / ${formatLabel(action.mode)}</span>
        <strong>${formatLabel(action.action)}</strong>
        <p class="row-detail">${formatLabel(action.status)}</p>
      </article>
    `),
    ...vm.disabledActions.map((action) => `
      <article class="action-row">
        <span class="row-kicker">disabled</span>
        <strong>${formatLabel(action.action)}</strong>
        <p class="row-detail">${action.reason}</p>
      </article>
    `)
  ]);
}

function renderIntegrations(integrationInputs = {}) {
  const vm = buildIntegrationViewModel(integrationInputs);

  setText("csm-api-status", vm.serviceRows.every((row) => row.state === "closed") ? "wired" : "check evidence");
  setText("cloudwatch-status", vm.cloudwatchSummary.status === "passed" ? "live proof" : "pending");
  setText("cloudwatch-event-count", `${vm.parsedEvents.length} events`);

  renderRows("csm-api-list", vm.serviceRows.map((row) => `
    <article class="integration-row" data-state="${row.state}">
      <span class="row-kicker">${formatLabel(row.label)}</span>
      <strong>${formatLabel(row.value)}</strong>
      <p class="row-detail">${row.detail}</p>
    </article>
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
}

function bindCommunication(packet = FALLBACK_PACKET) {
  const channel = document.getElementById("operator-channel");
  const message = document.getElementById("operator-message");
  const apiBase = document.getElementById("runtime-api-base");
  const prepare = document.getElementById("prepare-envelope");
  const checkEvents = document.getElementById("check-events");
  const packetId = packet.packet_id || "";

  const updateEnvelope = () => {
    const envelope = buildOperatorEnvelope({
      channel: channel?.value || "events",
      message: message?.value || "",
      packetId
    });
    renderEnvelope(envelope);
    setText("communication-status", "envelope ready");
  };

  prepare?.addEventListener("click", updateEnvelope);
  checkEvents?.addEventListener("click", async () => {
    setText("communication-status", "checking /events");
    try {
      const events = await checkEventsEndpoint(apiBase?.value || "");
      const envelope = buildOperatorEnvelope({
        channel: "events",
        message: `Read ${asArray(events.events).length} retained CSM events from live API.`,
        packetId
      });
      renderEnvelope({ ...envelope, live_event_count: asArray(events.events).length });
      setText("communication-status", "events reachable");
    } catch (error) {
      renderEnvelope({
        ...buildOperatorEnvelope({
          channel: channel?.value || "events",
          message: message?.value || "",
          packetId
        }),
        live_check_error: error instanceof Error ? error.message : "unknown error"
      });
      setText("communication-status", "offline draft");
    }
  });

  updateEnvelope();
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

async function bootObservatory() {
  const root = document.querySelector(".observatory");
  const packetRef = root?.dataset.packetRef || "";
  const reportRef = root?.dataset.reportRef || "";
  const csmServiceRef = root?.dataset.csmServiceRef || "";
  const csmApiRef = root?.dataset.csmApiRef || "";
  const cloudwatchRef = root?.dataset.cloudwatchRef || "";
  const cloudwatchEventsRef = root?.dataset.cloudwatchEventsRef || "";
  setHref("packet-link", packetRef);
  setHref("report-link", reportRef);

  try {
    const [packet, reportText, serviceManifest, apiText, cloudwatchSummary, cloudwatchEvents] = await Promise.all([
      loadJson(packetRef),
      loadText(reportRef).catch(() => ""),
      loadJson(csmServiceRef).catch(() => ({})),
      loadText(csmApiRef).catch(() => ""),
      loadJson(cloudwatchRef).catch(() => ({})),
      loadJson(cloudwatchEventsRef).catch(() => ({}))
    ]);
    renderObservatory(packet, reportText, "ok");
    renderIntegrations({ serviceManifest, apiText, cloudwatchSummary, cloudwatchEvents });
    bindCommunication(packet);
  } catch (_error) {
    renderObservatory(FALLBACK_PACKET, "", "fallback");
    renderIntegrations();
    bindCommunication(FALLBACK_PACKET);
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
  buildViewModel,
  buildIntegrationViewModel,
  renderObservatory,
  renderIntegrations
};
