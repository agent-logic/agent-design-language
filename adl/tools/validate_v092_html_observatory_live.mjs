#!/usr/bin/env node

import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
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

assert(observatoryUrl, "ADL_OBSERVATORY_URL must name the served HTML Observatory URL");
assert(runtimeApiBase, "ADL_RUNTIME_API_BASE must name the exact Runtime candidate URL");
assert(operatorKeyFile, "ADL_OPERATOR_KEY_FILE must name the trusted operator Ed25519 seed file");
assert(evidenceRoot, "ADL_OBSERVATORY_EVIDENCE_DIR must name a retained FastWork evidence directory");
assert(
  path.resolve(evidenceRoot).startsWith("/Volumes/FastWork/"),
  "ADL_OBSERVATORY_EVIDENCE_DIR must be under /Volumes/FastWork"
);

const signingSeed = (await fs.readFile(operatorKeyFile, "utf8")).trim();
assert(/^(?:0x)?[0-9a-fA-F]{64}$/.test(signingSeed), "operator key file must contain one hex Ed25519 seed");
await fs.mkdir(evidenceRoot, { recursive: true });

const url = new URL(observatoryUrl);
const runtimeUrl = new URL(runtimeApiBase);
assert.equal(url.protocol, "https:", "Observatory proof requires HTTPS");
assert.equal(runtimeUrl.protocol, "https:", "Runtime proof requires HTTPS");
const observatoryHosts = new Set(["observatory.dev.agent-logic.ai", "localhost", "127.0.0.1"]);
const runtimeHosts = new Set(["runtime.dev.agent-logic.ai", "localhost", "127.0.0.1"]);
assert(observatoryHosts.has(url.hostname), "Observatory proof requires a canonical or loopback-only identity");
assert(runtimeHosts.has(runtimeUrl.hostname), "Runtime proof requires a canonical or loopback-only identity");
assert.equal(url.protocol, "https:", "Observatory proof requires browser-trusted HTTPS");
assert.equal(runtimeUrl.protocol, "https:", "Runtime proof requires browser-trusted HTTPS");
const observatoryIsLoopback = ["localhost", "127.0.0.1"].includes(url.hostname);
const runtimeIsLoopback = ["localhost", "127.0.0.1"].includes(runtimeUrl.hostname);
assert.equal(observatoryIsLoopback, runtimeIsLoopback, "local proof cannot mix public and loopback identities");
assert.notEqual(url.origin, runtimeUrl.origin, "Observatory and Runtime must use distinct HTTPS origins");
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
const controlRequests = [];
let lastControlCommand = null;
page.on("console", (message) => {
  if (message.type() === "error" && !message.text().startsWith("Failed to load resource:")) {
    consoleErrors.push(message.text());
  }
});
page.on("pageerror", (error) => consoleErrors.push(error.message));
page.on("response", (response) => {
  if (response.status() < 400) return;
  const pathname = new URL(response.url()).pathname;
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
  observatory_url: url.toString(),
  runtime_api_base: runtimeUrl.origin,
  tls_trust: "platform_trust_store",
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
  assert.equal(await page.locator("#environment-label").textContent(), "Local Runtime");
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
  await page.locator("#compact-operator-message").fill("Prepare a bounded operator envelope.");
  await page.locator("#compact-prepare-envelope").click();
  await page.locator("#message-envelope").filter({ hasText: "Prepare a bounded operator envelope." }).waitFor();
  report.assertions.visible_navigation_and_envelope_control = { passed: true, views: navigation };

  await railLink("Chat").click();
  await page.locator("#agent-chat-target:not([disabled])").waitFor({ timeout: 20_000 });
  const agents = await page.locator("#agent-chat-target option").allTextContents();
  assert.equal(agents.length, 1, `live Runtime roster must contain only the resident Shepherd: ${JSON.stringify(agents)}`);
  assert(agents[0].includes("Shepherd"), "live Runtime roster did not provide the admitted Shepherd");
  assert.equal(await page.locator("#agent-chat-target").inputValue(), "shepherd", "Shepherd was not the selected live agent");
  report.assertions.live_agent_roster = { passed: true, agents };

  await page.locator("#agent-chat-key-file").setInputFiles(operatorKeyFile);
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

  const forbiddenOriginResponse = await context.request.post(`${runtimeUrl.origin}/v1/control`, {
    headers: {
      "Content-Type": "application/json",
      Origin: "https://forbidden.example.test"
    },
    data: lastControlCommand
  });
  assert.equal(forbiddenOriginResponse.status(), 403, "forbidden browser origin was not refused");
  report.assertions.forbidden_origin_refused = {
    passed: true,
    response_status: forbiddenOriginResponse.status()
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
  await context.setOffline(true);
  await page.waitForFunction(() => {
    const state = document.querySelector(".observatory")?.dataset.liveConnection;
    const status = document.getElementById("statusbar-websocket")?.textContent || "";
    return state === "reconnecting" && /reconnecting in (500|1000|2000|4000)ms/.test(status);
  }, null, { timeout: 20_000 });
  const observedBackoff = await page.locator("#statusbar-websocket").textContent();
  await context.setOffline(false);
  await page.waitForFunction(() => {
    return document.querySelector(".observatory")?.dataset.liveConnection === "connected";
  }, null, { timeout: 20_000 });
  const automaticReconnectAfter = {
    transcript: await page.locator(".chat-message").count(),
    control_posts: controlRequests.length,
    runtime_instance: await page.locator(".observatory").getAttribute("data-stream-runtime-instance"),
    last_sequence: Number(await page.locator(".observatory").getAttribute("data-stream-last-sequence")),
    applied_events: Number(await page.locator(".observatory").getAttribute("data-stream-applied-events"))
  };
  assert.equal(automaticReconnectAfter.transcript, automaticReconnectBefore.transcript, "automatic reconnect duplicated chat");
  assert.equal(automaticReconnectAfter.control_posts, automaticReconnectBefore.control_posts, "automatic reconnect replayed a control POST");
  assert.equal(automaticReconnectAfter.runtime_instance, automaticReconnectBefore.runtime_instance, "automatic reconnect lost Runtime identity");
  assert(automaticReconnectAfter.last_sequence >= automaticReconnectBefore.last_sequence, "automatic reconnect regressed the event cursor");
  assert(automaticReconnectAfter.applied_events >= automaticReconnectBefore.applied_events, "automatic reconnect lost applied events");
  report.assertions.automatic_bounded_reconnect = {
    passed: true,
    observed_backoff: observedBackoff,
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
  await page.locator("#dashboard-live-api-base").fill("https://localhost:21984");
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
  await page.locator("#dashboard-live-api-base").fill(runtimeUrl.origin);
  await page.locator("#dashboard-connect-live").click();
  await railLink("Chat").click();
  await page.locator("#agent-chat-target:not([disabled])").waitFor({ timeout: 20_000 });
  const afterReconnect = await page.locator(".chat-message").count();
  assert.equal(afterReconnect, beforeReconnect, "reconnect duplicated the conversation transcript");
  const postsAfterReconnect = controlRequests.length;
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
  assert.equal(ingressAfterReconnectDenial, ingressBeforeDeniedAuthority, "denied authority reached ingress after reconnect");
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

  await page.setViewportSize({ width: 390, height: 844 });
  await railLink("Chat").click();
  const mobileLayout = await page.evaluate(() => ({
    viewport: document.documentElement.clientWidth,
    document: document.documentElement.scrollWidth,
    sendHeight: document.getElementById("send-agent-message")?.getBoundingClientRect().height || 0
  }));
  assert(mobileLayout.document <= mobileLayout.viewport + 1, `mobile layout overflows: ${JSON.stringify(mobileLayout)}`);
  assert(mobileLayout.sendHeight >= 44, `mobile primary action is too small: ${mobileLayout.sendHeight}`);
  report.assertions.mobile_layout = { passed: true, ...mobileLayout };

  const screenshot = path.join(evidenceRoot, "observatory-layer8-chat.png");
  await page.screenshot({ path: screenshot, fullPage: true });
  report.artifacts.screenshot = screenshot;
  assert.equal(consoleErrors.length, 0, `browser errors: ${consoleErrors.join("; ")}`);
  assert.deepEqual(badResponses, [], `unexpected HTTP failures: ${JSON.stringify(badResponses)}`);
  report.assertions.browser_console_clean = { passed: true };
} finally {
  await context.close();
  await browser.close();
}

const reportPath = path.join(evidenceRoot, "observatory-layer8-chat-report.json");
const serializedReport = `${JSON.stringify(report, null, 2)}\n`;
assert(!serializedReport.includes(signingSeed), "operator signing seed appeared in retained report");
assert(!serializedReport.includes(path.basename(operatorKeyFile)), "operator key filename appeared in retained report");
await fs.writeFile(reportPath, serializedReport, { mode: 0o600 });
console.log(JSON.stringify({ schema: report.schema, result: "pass", report: reportPath }));
