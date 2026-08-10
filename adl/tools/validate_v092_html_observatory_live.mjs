#!/usr/bin/env node

import assert from "node:assert/strict";
import fs from "node:fs/promises";
import https from "node:https";
import net from "node:net";
import path from "node:path";
import tls from "node:tls";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
let chromium;
try {
  ({ chromium } = require("playwright"));
} catch (error) {
  throw new Error(
    `Playwright is required. Run with a Node environment that provides the playwright package: ${error.message}`
  );
}

const observatoryUrl = process.env.ADL_OBSERVATORY_URL;
const runtimeApiBase = process.env.ADL_RUNTIME_API_BASE;
const operatorKeyFile = process.env.ADL_OPERATOR_KEY_FILE;
const evidenceRoot = process.env.ADL_OBSERVATORY_EVIDENCE_DIR;
const tlsConnectHost = process.env.ADL_TLS_PROOF_CONNECT_HOST || null;
const allowRuntimeRestartProof = process.env.ADL_ALLOW_RUNTIME_RESTART_PROOF === "1";
const sourceRevision = process.env.ADL_SOURCE_REVISION;
const expectedRuntimePid = Number(process.env.ADL_EXPECTED_RUNTIME_PID);

assert(observatoryUrl, "ADL_OBSERVATORY_URL must name the served HTML Observatory URL");
assert(runtimeApiBase, "ADL_RUNTIME_API_BASE must name the exact Runtime candidate URL");
assert(operatorKeyFile, "ADL_OPERATOR_KEY_FILE must name the trusted operator Ed25519 seed file");
assert(evidenceRoot, "ADL_OBSERVATORY_EVIDENCE_DIR must name a retained FastWork evidence directory");
assert(allowRuntimeRestartProof, "ADL_ALLOW_RUNTIME_RESTART_PROOF=1 is required for the isolated Guardian restart proof");
assert(/^[0-9a-f]{40}$/.test(sourceRevision || ""), "ADL_SOURCE_REVISION must name the exact 40-character source commit");
assert(
  Number.isSafeInteger(expectedRuntimePid) && expectedRuntimePid > 1,
  "ADL_EXPECTED_RUNTIME_PID must independently name the exact Runtime child approved for restart"
);
const fastWorkRoot = await fs.realpath("/Volumes/FastWork");
const requestedEvidenceRoot = path.resolve(evidenceRoot);
assert(
  requestedEvidenceRoot.startsWith(`${fastWorkRoot}${path.sep}`),
  "ADL_OBSERVATORY_EVIDENCE_DIR must be under /Volumes/FastWork"
);
assert(
  tlsConnectHost === null || ["127.0.0.1", "::1"].includes(tlsConnectHost),
  "ADL_TLS_PROOF_CONNECT_HOST may only map the proof connection to loopback"
);

const signingSeed = (await fs.readFile(operatorKeyFile, "utf8")).trim();
assert(/^(?:0x)?[0-9a-fA-F]{64}$/.test(signingSeed), "operator key file must contain one hex Ed25519 seed");
const retainedEvidenceRoot = await fs.realpath(requestedEvidenceRoot);
assert(
  retainedEvidenceRoot.startsWith(`${fastWorkRoot}${path.sep}`),
  "ADL_OBSERVATORY_EVIDENCE_DIR must not escape FastWork through a symlink"
);
assert((await fs.stat(retainedEvidenceRoot)).isDirectory(), "ADL_OBSERVATORY_EVIDENCE_DIR must already be a directory");

const url = new URL(observatoryUrl);
const runtimeUrl = new URL(runtimeApiBase);
assert.equal(url.protocol, "https:", "Observatory proof requires HTTPS");
assert.equal(runtimeUrl.protocol, "https:", "Runtime proof requires HTTPS");
assert.equal(url.hostname, runtimeUrl.hostname, "Observatory and Runtime must share one TLS DNS identity");
assert(!net.isIP(url.hostname), "Observatory proof requires a public DNS certificate identity");
assert.notEqual(url.hostname, "localhost", "Observatory proof does not accept a local certificate identity");
assert.notEqual(url.origin, runtimeUrl.origin, "Observatory and Runtime must use distinct HTTPS origins");

function inspectTrustedPeer(endpoint) {
  return new Promise((resolve, reject) => {
    const socket = tls.connect({
      host: tlsConnectHost || endpoint.hostname,
      port: Number(endpoint.port || 443),
      ...(net.isIP(endpoint.hostname) ? {} : { servername: endpoint.hostname }),
      rejectUnauthorized: true,
      ca: tls.rootCertificates
    });
    socket.setTimeout(10_000);
    socket.once("secureConnect", () => {
      const certificate = socket.getPeerCertificate();
      const result = {
        fingerprint_sha256: certificate.fingerprint256,
        subject_cn: certificate.subject?.CN || null,
        issuer_cn: certificate.issuer?.CN || null,
        valid_from: certificate.valid_from,
        valid_to: certificate.valid_to,
        authorized: socket.authorized
      };
      socket.end();
      resolve(result);
    });
    socket.once("timeout", () => socket.destroy(new Error(`TLS inspection timed out for ${endpoint.origin}`)));
    socket.once("error", reject);
  });
}

function postTrustedJson(endpoint, pathname, headers, value) {
  const body = JSON.stringify(value);
  return new Promise((resolve, reject) => {
    const request = https.request({
      hostname: tlsConnectHost || endpoint.hostname,
      port: Number(endpoint.port || 443),
      path: pathname,
      method: "POST",
      servername: endpoint.hostname,
      rejectUnauthorized: true,
      ca: tls.rootCertificates,
      headers: {
        Host: endpoint.host,
        "Content-Type": "application/json",
        "Content-Length": Buffer.byteLength(body),
        ...headers
      }
    }, (response) => {
      response.resume();
      response.once("end", () => resolve({ status: response.statusCode }));
    });
    request.setTimeout(10_000, () => request.destroy(new Error(`HTTPS POST timed out for ${endpoint.origin}${pathname}`)));
    request.once("error", reject);
    request.end(body);
  });
}

const observatoryPeer = await inspectTrustedPeer(url);
const runtimePeer = await inspectTrustedPeer(runtimeUrl);
assert(observatoryPeer.authorized, "Observatory peer certificate is not trusted");
assert(runtimePeer.authorized, "Runtime peer certificate is not trusted");
assert.equal(
  observatoryPeer.fingerprint_sha256,
  runtimePeer.fingerprint_sha256,
  "Observatory and Runtime listeners must present the same certificate"
);
url.searchParams.set("runtime", "v3");
url.searchParams.set("runtimeApiBase", runtimeApiBase);
url.searchParams.set("live", "1");

const browser = await chromium.launch({
  headless: true,
  ...(process.env.ADL_PLAYWRIGHT_HOST_RESOLVER_RULES
    ? { args: [`--host-resolver-rules=${process.env.ADL_PLAYWRIGHT_HOST_RESOLVER_RULES}`] }
    : {}),
  ...(process.env.ADL_PLAYWRIGHT_CHROMIUM_EXECUTABLE
    ? { executablePath: process.env.ADL_PLAYWRIGHT_CHROMIUM_EXECUTABLE }
    : {})
});
const context = await browser.newContext({
  viewport: { width: 1440, height: 1000 },
  ignoreHTTPSErrors: false
});
const page = await context.newPage();
const railLink = (name) => page.locator(".dashboard-rail").getByRole("link", { name, exact: true });
const consoleErrors = [];
const badResponses = [];
const expectedTransientResponses = [];
const controlRequests = [];
let lastControlCommand = null;
let guardianRestartInProgress = false;
let proofPhase = "baseline";
page.on("console", (message) => {
  if (message.type() === "error" && !message.text().startsWith("Failed to load resource:")) {
    consoleErrors.push({ message: message.text(), phase: proofPhase });
  }
});
page.on("pageerror", (error) => consoleErrors.push({ message: error.message, phase: proofPhase }));
page.on("response", (response) => {
  if (response.status() < 400) return;
  const pathname = new URL(response.url()).pathname;
  if (guardianRestartInProgress && pathname === "/v1/ready" && response.status() === 503) {
    expectedTransientResponses.push({ pathname, status: response.status(), context: "guardian_restart" });
    return;
  }
  const expected =
    (pathname === "/favicon.ico" && response.status() === 404) ||
    (pathname === "/v1/control" && [400, 401, 403].includes(response.status()));
  if (!expected) badResponses.push({ pathname, status: response.status() });
});
page.on("request", (request) => {
  if (new URL(request.url()).pathname === "/v1/control") {
    lastControlCommand = request.postDataJSON();
    controlRequests.push(lastControlCommand);
  }
});

const report = {
  schema: "adl.v092.html_observatory_live_validation.v1",
  source_revision: sourceRevision,
  observatory_url: url.toString(),
  runtime_api_base: runtimeUrl.origin,
  tls_trust: "browser_platform_and_node_public_roots",
  tls_connect_host: tlsConnectHost || "dns",
  shared_certificate: {
    observatory: observatoryPeer,
    runtime: runtimePeer,
    fingerprint_match: true
  },
  assertions: {},
  artifacts: {}
};

try {
  await page.goto(url.toString(), { waitUntil: "networkidle", timeout: 30_000 });
  await page.locator("#statusbar-websocket").filter({ hasText: "connected" }).waitFor({ timeout: 20_000 });
  await page.waitForFunction(() => {
    const value = Number(document.querySelector(".observatory")?.dataset.streamLastSequence);
    return Number.isSafeInteger(value) && value >= 0;
  });
  const initialStreamSequence = Number(await page.locator(".observatory").getAttribute("data-stream-last-sequence"));
  assert.equal(await page.locator("#environment-label").textContent(), "Trusted Runtime");
  assert.equal(await page.locator("#operator-status-pill").textContent(), "Public read");
  report.assertions.live_wss_frame = { passed: true, initial_sequence: initialStreamSequence };

  const navigation = ["Runtime", "Agents", "Chat", "Events", "AWS", "Governance", "Evidence"];
  for (const name of navigation) {
    await railLink(name).click();
    assert.equal(
      await railLink(name).getAttribute("aria-current"),
      "page",
      `${name} navigation did not activate its dashboard view`
    );
  }
  await railLink("Runtime").click();
  await page.locator("#compact-operator-message").fill("Prepare a bounded operator envelope.");
  await page.locator("#compact-prepare-envelope").click();
  await page.locator("#compact-message-envelope").filter({ hasText: "Prepare a bounded operator envelope." }).waitFor();
  report.assertions.visible_navigation_and_envelope_control = { passed: true, views: navigation };

  await railLink("Chat").click();
  await page.locator("#agent-chat-target:not([disabled])").waitFor({ timeout: 20_000 });
  const agents = await page.locator("#agent-chat-target option").allTextContents();
  assert.equal(agents.length, 1, `live Runtime roster must contain only the resident Shepherd: ${JSON.stringify(agents)}`);
  assert(agents[0].includes("Shepherd"), "live Runtime roster did not provide the admitted Shepherd");
  assert.equal(await page.locator("#agent-chat-target").inputValue(), "shepherd", "Shepherd was not the selected live agent");
  report.assertions.live_agent_roster = { passed: true, agents };

  await page.locator("#agent-chat-key-file").setInputFiles(operatorKeyFile);
  await page.waitForFunction(() => {
    const input = document.getElementById("agent-chat-key-file");
    const status = document.getElementById("agent-chat-status")?.textContent || "";
    return input?.value === "" && status === "ready";
  });
  await page.locator("#agent-chat-key-file").evaluate((element) => {
    if (element.value !== "") throw new Error("operator key file input was not cleared after import");
  });
  report.assertions.native_key_input_cleared = { passed: true };
  await page.locator("#operator-message").fill("Hello Shepherd. Please confirm that you are present.");
  await page.locator("#send-agent-message:not([disabled])").click();
  await page.locator('.chat-message[data-role="agent"], .chat-message[data-role="system"]').waitFor({ timeout: 20_000 });
  const agentMessages = await page.locator('.chat-message[data-role="agent"]').count();
  if (agentMessages === 0) {
    const refusal = await page.locator('.chat-message[data-role="system"] span').last().textContent();
    const diagnostic = lastControlCommand
      ? { ...lastControlCommand, signature: `<${String(lastControlCommand.signature || "").length} hex characters>` }
      : null;
    throw new Error(`signed selected-agent chat was refused: ${refusal}; command=${JSON.stringify(diagnostic)}`);
  }
  const deliveredStatus = await page.locator("#agent-chat-status").textContent();
  assert.equal(deliveredStatus, "delivered");
  report.assertions.signed_selected_agent_chat = {
    passed: true,
    status: deliveredStatus,
    transcript_messages: await page.locator(".chat-message").count()
  };
  await page.waitForFunction((previous) => {
    const value = Number(document.querySelector(".observatory")?.dataset.streamLastSequence);
    return Number.isSafeInteger(value) && value > previous;
  }, initialStreamSequence);
  report.assertions.wss_observed_correlated_runtime_progress = {
    passed: true,
    sequence: Number(await page.locator(".observatory").getAttribute("data-stream-last-sequence"))
  };

  const forbiddenOriginResponse = await postTrustedJson(
    runtimeUrl,
    "/v1/control",
    { Origin: "https://forbidden.example.test" },
    lastControlCommand
  );
  assert.equal(forbiddenOriginResponse.status, 403, "forbidden browser origin was not refused");
  report.assertions.forbidden_origin_refused = {
    passed: true,
    response_status: forbiddenOriginResponse.status
  };

  const failureStates = await page.evaluate(() => {
    const classify = globalThis.AdlHtmlObservatory.classifyRuntimeV3Failure;
    return [
      "unsupported runtime v3 observatory schema",
      "backpressure",
      "invalid_request",
      "403 origin denied",
      "temporarily_unavailable",
      "certificate failure",
      "connection reset"
    ].map((message) => classify(new Error(message)).state);
  });
  assert.deepEqual(failureStates, [
    "incompatible",
    "backpressure",
    "malformed",
    "denied",
    "unavailable",
    "tls-or-origin",
    "offline"
  ]);
  report.assertions.operator_failure_states = { passed: true, states: failureStates };

  const systemMessagesBefore = await page.locator('.chat-message[data-role="system"]').count();
  await page.locator("#operator-message").evaluate((element) => {
    element.value = "This message contains a forbidden control character.\u0001";
    element.dispatchEvent(new Event("input", { bubbles: true }));
  });
  const refusalResponsePromise = page.waitForResponse(
    (response) => new URL(response.url()).pathname === "/v1/control"
  );
  await page.locator("#send-agent-message:not([disabled])").click();
  const refusalResponse = await refusalResponsePromise;
  const refusalPayload = await refusalResponse.json();
  assert.equal(refusalResponse.status(), 400, "malformed Layer 8 content was not rejected as invalid input");
  assert.equal(refusalPayload?.code, "invalid_request", "malformed Layer 8 content returned the wrong refusal code");
  await page.locator('.chat-message[data-role="system"]').nth(systemMessagesBefore).waitFor({ timeout: 20_000 });
  const refusalStatus = await page.locator("#agent-chat-status").textContent();
  assert.equal(refusalStatus, "malformed runtime data", "Runtime refusal was not presented as malformed input");
  report.assertions.runtime_refusal_remains_denied = {
    passed: true,
    response_status: refusalResponse.status(),
    response_code: refusalPayload.code,
    ui_status: refusalStatus
  };

  const ingressBeforeDeniedAuthority = await page.evaluate(async (base) => {
    const response = await fetch(`${base}/v1/observatory`);
    return (await response.json()).ingress.accepted_through;
  }, runtimeUrl.origin);
  await page.locator("#agent-chat-key-file").setInputFiles({
    name: "invalid-operator.key",
    mimeType: "text/plain",
    buffer: Buffer.from("11".repeat(32))
  });
  const deniedBeforeReconnect = page.waitForResponse(
    (response) => new URL(response.url()).pathname === "/v1/control" && response.status() === 401
  );
  await page.locator("#operator-message").fill("This identity must be denied.");
  await page.locator("#send-agent-message:not([disabled])").click();
  await deniedBeforeReconnect;
  assert.equal(await page.locator("#agent-chat-status").textContent(), "origin or authority denied");
  const ingressAfterDeniedAuthority = await page.evaluate(async (base) => {
    const response = await fetch(`${base}/v1/observatory`);
    return (await response.json()).ingress.accepted_through;
  }, runtimeUrl.origin);
  assert.equal(ingressAfterDeniedAuthority, ingressBeforeDeniedAuthority, "denied authority reached canonical ingress");

  const automaticReconnectBefore = {
    transcript: await page.locator(".chat-message").count(),
    control_posts: controlRequests.length,
    runtime_instance: await page.locator(".observatory").getAttribute("data-stream-runtime-instance"),
    last_sequence: Number(await page.locator(".observatory").getAttribute("data-stream-last-sequence")),
    applied_events: Number(await page.locator(".observatory").getAttribute("data-stream-applied-events"))
  };
  const restartTarget = await page.evaluate(async (base) => {
    const response = await fetch(`${base}/v1/observatory`);
    if (!response.ok) throw new Error(`Runtime feed returned ${response.status}`);
    const feed = await response.json();
    return {
      pid: feed.runtime_process_id,
      runtime_instance_id: feed.runtime_instance_id
    };
  }, runtimeUrl.origin);
  assert(Number.isSafeInteger(restartTarget.pid) && restartTarget.pid > 1, "Runtime feed did not expose an exact candidate PID");
  assert.equal(
    restartTarget.pid,
    expectedRuntimePid,
    "public Runtime feed PID did not match the independently approved restart target"
  );
  assert.equal(restartTarget.runtime_instance_id, automaticReconnectBefore.runtime_instance, "restart target identity drifted before the proof");
  proofPhase = "guardian_restart";
  guardianRestartInProgress = true;
  process.kill(expectedRuntimePid, "SIGKILL");
  await page.waitForFunction(() => {
    const state = document.querySelector(".observatory")?.dataset.liveConnection;
    const status = document.getElementById("statusbar-websocket")?.textContent || "";
    return state === "reconnecting" && /reconnecting in (250|500|1000|2000|4000)ms/.test(status);
  }, null, { timeout: 20_000 });
  const observedBackoff = await page.locator("#statusbar-websocket").textContent();
  await page.waitForFunction(() => {
    return document.querySelector(".observatory")?.dataset.liveConnection === "connected";
  }, null, { timeout: 30_000 });
  const automaticReconnectAfter = {
    transcript: await page.locator(".chat-message").count(),
    control_posts: controlRequests.length,
    runtime_instance: await page.locator(".observatory").getAttribute("data-stream-runtime-instance"),
    last_sequence: Number(await page.locator(".observatory").getAttribute("data-stream-last-sequence")),
    applied_events: Number(await page.locator(".observatory").getAttribute("data-stream-applied-events"))
  };
  guardianRestartInProgress = false;
  proofPhase = "baseline";
  assert.equal(automaticReconnectAfter.transcript, automaticReconnectBefore.transcript, "automatic reconnect duplicated chat");
  assert.equal(automaticReconnectAfter.control_posts, automaticReconnectBefore.control_posts, "automatic reconnect replayed a control POST");
  assert.notEqual(automaticReconnectAfter.runtime_instance, automaticReconnectBefore.runtime_instance, "Guardian restart did not establish a new Runtime instance");
  assert(automaticReconnectAfter.last_sequence >= 0, "automatic reconnect did not establish a valid event cursor");
  assert(automaticReconnectAfter.applied_events >= 0, "automatic reconnect did not restore event accounting");
  report.assertions.automatic_bounded_reconnect = {
    passed: true,
    observed_backoff: observedBackoff,
    healthy_reset_millis: 10_000,
    guardian_restart: {
      previous_pid: restartTarget.pid,
      previous_runtime_instance: automaticReconnectBefore.runtime_instance,
      next_runtime_instance: automaticReconnectAfter.runtime_instance
    },
    before: automaticReconnectBefore,
    after: automaticReconnectAfter,
    command_replay_count: 0
  };

  const beforeReconnect = await page.locator(".chat-message").count();
  await railLink("Runtime").click();
  await page.locator("#dashboard-stop-live").click();
  await railLink("Chat").click();
  assert(await page.locator("#send-agent-message").isDisabled(), "stopped Runtime left chat writes enabled");
  assert(await page.locator("#agent-chat-target").isDisabled(), "stopped Runtime retained an addressable roster");

  await railLink("Runtime").click();
  const unavailableRuntime = new URL(runtimeUrl.origin);
  unavailableRuntime.port = "21984";
  proofPhase = "unbound_runtime_negative_test";
  await page.locator("#dashboard-live-api-base").fill(unavailableRuntime.origin);
  await page.locator("#dashboard-connect-live").click();
  await page.waitForFunction(() => {
    const root = document.querySelector(".observatory");
    const status = document.getElementById("live-status")?.textContent || "";
    return root?.dataset.liveConnection === "reconnecting" && !status.startsWith("live ");
  }, null, { timeout: 20_000 });
  await railLink("Chat").click();
  assert(await page.locator("#send-agent-message").isDisabled(), "unavailable Runtime left chat writes enabled");
  assert(await page.locator("#agent-chat-target").isDisabled(), "unavailable Runtime retained an addressable roster");
  report.assertions.runtime_unavailability_not_presented_as_live = {
    passed: true,
    live_status: await page.locator("#live-status").textContent(),
    connection_state: await page.locator(".observatory").getAttribute("data-live-connection")
  };

  await railLink("Runtime").click();
  await page.locator("#dashboard-stop-live").click();
  proofPhase = "baseline";
  await page.locator("#dashboard-live-api-base").fill(runtimeUrl.origin);
  await page.locator("#dashboard-connect-live").click();
  await railLink("Chat").click();
  await page.locator("#agent-chat-target:not([disabled])").waitFor({ timeout: 20_000 });
  const afterReconnect = await page.locator(".chat-message").count();
  assert.equal(afterReconnect, beforeReconnect, "reconnect duplicated the conversation transcript");
  const postsAfterReconnect = controlRequests.length;
  const ingressBeforeReconnectDenial = await page.evaluate(async (base) => {
    const response = await fetch(`${base}/v1/observatory`);
    return (await response.json()).ingress.accepted_through;
  }, runtimeUrl.origin);
  await page.locator("#operator-message").fill("This identity must remain denied after reconnect.");
  const deniedAfterReconnect = page.waitForResponse(
    (response) => new URL(response.url()).pathname === "/v1/control" && response.status() === 401
  );
  await page.locator("#send-agent-message:not([disabled])").click();
  await deniedAfterReconnect;
  assert.equal(controlRequests.length, postsAfterReconnect + 1, "reconnect replayed a control POST");
  const ingressAfterReconnectDenial = await page.evaluate(async (base) => {
    const response = await fetch(`${base}/v1/observatory`);
    return (await response.json()).ingress.accepted_through;
  }, runtimeUrl.origin);
  assert.equal(ingressAfterReconnectDenial, ingressBeforeReconnectDenial, "denied authority reached ingress after reconnect");
  report.assertions.bounded_reconnect_without_chat_duplication = {
    passed: true,
    authority_denied_before_and_after: true,
    command_replay_count: 0
  };

  const pageText = await page.locator("body").innerText();
  assert(!pageText.includes(signingSeed), "operator signing seed appeared in visible DOM text");
  assert(!pageText.includes(path.basename(operatorKeyFile)), "operator key filename appeared in visible DOM text");
  const browserStorage = await page.evaluate(() => ({
    local: { ...localStorage },
    session: { ...sessionStorage }
  }));
  const redactionSurface = JSON.stringify({ browserStorage, consoleErrors, controlRequests });
  assert(!redactionSurface.includes(signingSeed), "operator signing seed appeared in browser state or network capture");
  assert(!redactionSurface.includes(path.basename(operatorKeyFile)), "operator key filename appeared in browser state or network capture");
  report.assertions.signing_material_not_rendered = {
    passed: true,
    browser_storage_scanned: true,
    console_scanned: true,
    control_requests_scanned: true
  };

  const hostileRoster = await page.evaluate(() => {
    globalThis.__adlHostileRosterExecuted = false;
    globalThis.AdlHtmlObservatory.renderPanopticon({
      mode: "live",
      fetchedAt: "2026-08-10T00:00:00Z",
      status: {
        runtime_instance_id: "hostile-payload-proof",
        agent_population: {
          total_count: 1,
          sample: [{
            id: '<img src=x onerror="globalThis.__adlHostileRosterExecuted=true">',
            label: '<script>globalThis.__adlHostileRosterExecuted=true</script>',
            role: "runtime agent",
            state: "running",
            detail: '<a href="javascript:globalThis.__adlHostileRosterExecuted=true">unsafe</a>'
          }]
        }
      },
      health: { status: "ok" },
      ready: { status: "ready" },
      metrics: {},
      events: []
    }, {}, { chatLive: false });
    return {
      executed: globalThis.__adlHostileRosterExecuted,
      scriptCount: document.querySelectorAll("#panopticon-map script, #live-agent-list script").length,
      handlerCount: document.querySelectorAll("#panopticon-map [onerror], #live-agent-list [onerror]").length,
      javascriptLinkCount: [...document.querySelectorAll("#panopticon-map a, #live-agent-list a")]
        .filter((element) => String(element.getAttribute("href") || "").toLowerCase().startsWith("javascript:"))
        .length
    };
  });
  await page.waitForTimeout(50);
  hostileRoster.executed = await page.evaluate(() => globalThis.__adlHostileRosterExecuted);
  assert.deepEqual(hostileRoster, {
    executed: false,
    scriptCount: 0,
    handlerCount: 0,
    javascriptLinkCount: 0
  });
  report.assertions.hostile_runtime_roster_rendered_inertly = { passed: true, ...hostileRoster };

  await railLink("Runtime").click();
  await page.locator("#dashboard-connect-live").click();
  await page.locator("#statusbar-websocket").filter({ hasText: "connected" }).waitFor({ timeout: 20_000 });
  await railLink("Chat").click();
  const desktopScreenshot = path.join(retainedEvidenceRoot, "observatory-layer8-chat-desktop.png");
  await page.screenshot({ path: desktopScreenshot, fullPage: true });
  report.artifacts.desktop_screenshot = desktopScreenshot;

  await page.setViewportSize({ width: 390, height: 844 });
  const mobileLayout = await page.evaluate(() => ({
    viewport: document.documentElement.clientWidth,
    document: document.documentElement.scrollWidth,
    sendHeight: document.getElementById("send-agent-message")?.getBoundingClientRect().height || 0
  }));
  assert(mobileLayout.document <= mobileLayout.viewport + 1, `mobile layout overflows: ${JSON.stringify(mobileLayout)}`);
  assert(mobileLayout.sendHeight >= 44, `mobile primary action is too small: ${mobileLayout.sendHeight}`);
  report.assertions.mobile_layout = { passed: true, ...mobileLayout };

  const mobileScreenshot = path.join(retainedEvidenceRoot, "observatory-layer8-chat-mobile.png");
  await page.screenshot({ path: mobileScreenshot, fullPage: true });
  report.artifacts.mobile_screenshot = mobileScreenshot;
  const expectedTransientWebSocketEndpoints = [runtimeUrl, unavailableRuntime].map((endpoint) => {
    const websocket = new URL(`${endpoint.origin}/v1/observatory/ws`);
    websocket.protocol = "wss:";
    return websocket.toString();
  });
  const expectedTransientWebSocketErrors = consoleErrors.filter(({ message, phase }) => {
    const expectedEndpoint = phase === "guardian_restart"
      ? expectedTransientWebSocketEndpoints[0]
      : phase === "unbound_runtime_negative_test"
        ? expectedTransientWebSocketEndpoints[1]
        : null;
    return expectedEndpoint !== null &&
      message.includes(`WebSocket connection to '${expectedEndpoint}' failed:`) &&
      message.includes("net::ERR_CONNECTION_REFUSED");
  });
  const unexpectedConsoleErrors = consoleErrors.filter(
    (entry) => !expectedTransientWebSocketErrors.includes(entry)
  );
  report.assertions.transient_websocket_failures_are_bounded = {
    passed: true,
    observed: expectedTransientWebSocketErrors.length,
    contexts: ["guardian_restart", "unbound_runtime_negative_test"]
  };
  report.assertions.restart_readiness_fails_closed = {
    passed: true,
    responses: expectedTransientResponses
  };
  assert.equal(unexpectedConsoleErrors.length, 0, `browser errors: ${JSON.stringify(unexpectedConsoleErrors)}`);
  assert.deepEqual(badResponses, [], `unexpected HTTP failures: ${JSON.stringify(badResponses)}`);
  report.assertions.browser_console_clean = { passed: true };
} finally {
  await context.close();
  await browser.close();
}

report.result = "pass";
const reportPath = path.join(retainedEvidenceRoot, "observatory-layer8-chat-report.json");
const serializedReport = `${JSON.stringify(report, null, 2)}\n`;
assert(!serializedReport.includes(signingSeed), "operator signing seed appeared in retained report");
assert(!serializedReport.includes(path.basename(operatorKeyFile)), "operator key filename appeared in retained report");
await fs.writeFile(reportPath, serializedReport, { mode: 0o600 });
console.log(JSON.stringify({ schema: report.schema, result: "pass", report: reportPath }));
