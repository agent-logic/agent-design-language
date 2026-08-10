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
assert.equal(url.hostname, "observatory.dev.agent-logic.ai", "Observatory proof requires the canonical DNS identity");
assert.equal(runtimeUrl.hostname, "runtime.dev.agent-logic.ai", "Runtime proof requires the canonical DNS identity");
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
const consoleErrors = [];
const badResponses = [];
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
    (pathname === "/v1/control" && [400, 403].includes(response.status()));
  if (!expected) badResponses.push({ pathname, status: response.status() });
});
page.on("request", (request) => {
  if (new URL(request.url()).pathname === "/v1/control") {
    lastControlCommand = request.postDataJSON();
  }
});

const report = {
  schema: "adl.v092.html_observatory_live_validation.v1",
  observatory_url: url.toString(),
  runtime_api_base: runtimeUrl.origin,
  assertions: {},
  artifacts: {}
};

try {
  await page.goto(url.toString(), { waitUntil: "networkidle", timeout: 30_000 });
  await page.getByRole("link", { name: "Chat", exact: true }).click();
  await page.locator("#agent-chat-target:not([disabled])").waitFor({ timeout: 20_000 });
  const agents = await page.locator("#agent-chat-target option").allTextContents();
  assert(agents.length > 0 && !agents.includes("No live agents"), "live Runtime roster did not provide an agent");
  report.assertions.live_agent_roster = { passed: true, agents };

  await page.locator("#agent-chat-key-file").setInputFiles(operatorKeyFile);
  await page.locator("#operator-message").fill("Happy birthday. I am glad you are here.");
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

  const beforeReconnect = await page.locator(".chat-message").count();
  await page.getByRole("link", { name: "Runtime", exact: true }).click();
  await page.locator("#dashboard-stop-live").click();
  await page.getByRole("link", { name: "Chat", exact: true }).click();
  assert(await page.locator("#send-agent-message").isDisabled(), "stopped Runtime left chat writes enabled");
  assert(await page.locator("#agent-chat-target").isDisabled(), "stopped Runtime retained an addressable roster");
  await page.getByRole("link", { name: "Runtime", exact: true }).click();
  await page.locator("#dashboard-connect-live").click();
  await page.getByRole("link", { name: "Chat", exact: true }).click();
  await page.locator("#agent-chat-target:not([disabled])").waitFor({ timeout: 20_000 });
  const afterReconnect = await page.locator(".chat-message").count();
  assert.equal(afterReconnect, beforeReconnect, "reconnect duplicated the conversation transcript");
  report.assertions.bounded_reconnect_without_chat_duplication = { passed: true };

  const pageText = await page.locator("body").innerText();
  assert(!pageText.includes(signingSeed), "operator signing seed appeared in visible DOM text");
  assert(!pageText.includes(path.basename(operatorKeyFile)), "operator key filename appeared in visible DOM text");
  report.assertions.signing_material_not_rendered = { passed: true };

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
await fs.writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`, { mode: 0o600 });
console.log(JSON.stringify({ schema: report.schema, result: "pass", report: reportPath }));
