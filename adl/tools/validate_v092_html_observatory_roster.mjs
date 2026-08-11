#!/usr/bin/env node

import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { createPrivateKey, randomBytes, sign } from "node:crypto";
import { readFileSync } from "node:fs";

const require = createRequire(import.meta.url);
let chromium;
try {
  ({ chromium } = require("playwright"));
} catch (error) {
  throw new Error(`Playwright is required for the roster proof: ${error.message}`);
}

const observatoryUrl = process.env.ADL_OBSERVATORY_URL;
const runtimeApiBase = process.env.ADL_RUNTIME_API_BASE;
const sourceRevision = process.env.ADL_SOURCE_REVISION;
const controlPrivateKeyPath = process.env.ADL_CONTROL_PRIVATE_KEY_PATH;
const allowRestartProof = process.env.ADL_ALLOW_RUNTIME_RESTART_PROOF === "1";
assert(observatoryUrl, "ADL_OBSERVATORY_URL must name the HTML Observatory URL");
assert(runtimeApiBase, "ADL_RUNTIME_API_BASE must name the Runtime v3 API base");
assert(/^[0-9a-f]{40}$/.test(sourceRevision || ""), "ADL_SOURCE_REVISION must name the exact candidate");
assert(controlPrivateKeyPath, "ADL_CONTROL_PRIVATE_KEY_PATH must name the external proof control key");
assert(allowRestartProof, "ADL_ALLOW_RUNTIME_RESTART_PROOF=1 is required for the isolated Guardian restart proof");

const sleep = (millis) => new Promise((resolve) => setTimeout(resolve, millis));
const signedRestart = (feed) => {
  const seed = Buffer.from(readFileSync(controlPrivateKeyPath, "utf8").trim(), "hex");
  assert.equal(seed.length, 32, "control signing seed must be 32 bytes");
  const privateKey = createPrivateKey({
    key: Buffer.concat([Buffer.from("302e020100300506032b657004220420", "hex"), seed]),
    format: "der",
    type: "pkcs8"
  });
  const id = randomBytes(16).toString("hex");
  const command = {
    schema: "adl.runtime.control_command.v1",
    runtime_instance_id: feed.runtime_instance_id,
    command_id: `restart-${id}`,
    correlation_id: id,
    principal: "operator",
    action: { action: "restart", expected_incarnation_id: feed.runtime_incarnation_id, grace_millis: 5_000 },
    signing_algorithm: "ed25519",
    signing_key_id: "operator-key",
    signature: ""
  };
  command.signature = sign(null, Buffer.from(JSON.stringify(command)), privateKey).toString("hex");
  return command;
};
const fetchFeed = async () => {
  const response = await fetch(new URL("/v1/observatory", runtime));
  assert.equal(response.status, 200, "Runtime Observatory feed must be available");
  return response.json();
};

const observatory = new URL(observatoryUrl);
const runtime = new URL(runtimeApiBase);
assert.equal(observatory.protocol, "https:", "Observatory roster proof requires trusted HTTPS");
assert.equal(runtime.protocol, "https:", "Runtime roster proof requires trusted HTTPS");
assert.equal(observatory.hostname, runtime.hostname, "Observatory and Runtime must share the instance DNS identity");

const feed = await fetchFeed();
assert.equal(feed.schema, "adl.runtime_v3.observatory_feed.v2");
assert.equal(typeof feed.runtime_incarnation_id, "string");
assert(feed.runtime_incarnation_id.length > 0);
assert.equal(feed.agents?.scope, "local_runtime");
assert.equal(feed.agents?.population_complete, false);
assert.equal(feed.agents?.sample?.length, 1, "local production proof requires one visible resident Shepherd");
const shepherd = feed.agents.sample[0];
assert.equal(shepherd.id, "shepherd");
assert.equal(shepherd.label, "Shepherd");
assert.equal(shepherd.provenance, "runtime_component_state");
assert.equal(shepherd.state, "ready");
assert.equal(shepherd.health, "healthy");
assert.equal(shepherd.availability, "available");
assert.equal(shepherd.communication_eligible, true);
assert(Number.isSafeInteger(shepherd.observed_at_unix_millis));
assert(shepherd.freshness_deadline_unix_millis > shepherd.observed_at_unix_millis);
assert(shepherd.freshness_deadline_unix_millis - shepherd.observed_at_unix_millis <= 5_000);
assert(/^[0-9a-f]{40}$/.test(shepherd.source_revision));
assert.equal(shepherd.source_revision, sourceRevision, "roster evidence must name the exact Runtime build revision");
const rosterResponse = await fetch(new URL("/v1/agents?page_size=1", runtime));
assert.equal(rosterResponse.status, 200);
const rosterPage = await rosterResponse.json();
assert.equal(rosterPage.schema, "adl.runtime_v3.agent_roster_page.v1");
assert.equal(rosterPage.sample.length, 1);
assert.equal(rosterPage.sample[0].id, shepherd.id);

await sleep(1_200);
const heartbeatFeed = await fetchFeed();
const heartbeatShepherd = heartbeatFeed.agents.sample[0];
assert(heartbeatShepherd.observed_at_unix_millis > shepherd.observed_at_unix_millis, "qualified component heartbeat must advance admission freshness");
assert(heartbeatShepherd.freshness_deadline_unix_millis > shepherd.freshness_deadline_unix_millis, "heartbeat must advance a bounded deadline");
assert.equal(heartbeatFeed.runtime_incarnation_id, feed.runtime_incarnation_id);

const proofUrl = new URL(observatory);
proofUrl.searchParams.set("runtime", "v3");
proofUrl.searchParams.set("runtimeApiBase", runtime.origin);
proofUrl.searchParams.set("live", "1");

const browser = await chromium.launch({ headless: true });
try {
  const context = await browser.newContext({ viewport: { width: 1440, height: 1000 } });
  const page = await context.newPage();
  const pageErrors = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));
  await page.goto(proofUrl.href, { waitUntil: "networkidle" });
  await page.locator('[data-dashboard-link="agents"]').first().click();
  const row = page.locator('[data-agent-id="shepherd"]');
  await row.waitFor({ state: "visible" });
  assert.equal(await page.locator("#agent-count").textContent(), "1 of 1 visible");
  assert.equal(await row.getAttribute("aria-pressed"), "false");
  await row.focus();
  assert.equal(await page.evaluate(() => document.activeElement?.dataset.agentId), "shepherd");
  await row.press("Enter");
  assert.equal(await row.getAttribute("aria-pressed"), "true");
  await page.locator("#roster-detail").getByText("Runtime Component State").waitFor();
  await page.locator("#roster-detail").getByText("Eligible").waitFor();

  await page.locator("#roster-search").fill("not-a-visible-agent");
  assert.equal(await page.locator("#agent-count").textContent(), "0 of 1 visible");
  assert.equal(await page.locator('[data-agent-id="shepherd"]').count(), 0);
  await page.locator("#roster-search").fill("Shepherd");
  await row.waitFor({ state: "visible" });
  await page.locator("#roster-presence-filter").selectOption("degraded");
  assert.equal(await page.locator("#agent-count").textContent(), "0 of 1 visible");
  await page.locator("#roster-presence-filter").selectOption("ready");
  await row.waitFor({ state: "visible" });

  await context.setOffline(true);
  await page.locator("#statusbar-websocket").getByText("disconnected", { exact: true }).waitFor({ timeout: 10_000 });
  await context.setOffline(false);
  await page.locator("#statusbar-websocket").getByText("connected", { exact: true }).waitFor({ timeout: 12_000 });
  await row.waitFor({ state: "visible" });
  assert.equal(await page.locator('[data-agent-id="shepherd"]').count(), 1, "reconnect must not duplicate roster rows");

  const uiPage = await context.newPage();
  let uiRevision = 20;
  let uiState = "ready";
  let uiLocation = "node-a";
  await uiPage.route("**/v1/observatory", async (route) => {
    const shaped = structuredClone(feed);
    shaped.agents = {
      ...shaped.agents,
      revision: uiRevision,
      total_count: 3,
      rendered_sample_count: 1,
      has_more: true,
      next_page_token: "proof-continuation",
      sample: [{ ...shaped.agents.sample[0], state: uiState, location: uiLocation }]
    };
    await route.fulfill({ status: 200, contentType: "application/json", body: JSON.stringify(shaped) });
  });
  await uiPage.route("**/v1/agents?page_size=50&page_token=proof-continuation", async (route) => {
    await route.fulfill({
      status: 200,
      contentType: "application/json",
      body: JSON.stringify({
        ...rosterPage,
        revision: uiRevision,
        total_count: 3,
        rendered_sample_count: 2,
        sample: [
          { ...rosterPage.sample[0], id: "proof-agent-2", label: "Proof Agent Two", location: "node-b" },
          rosterPage.sample[0]
        ],
        has_more: false,
        next_page_token: null
      })
    });
  });
  await uiPage.goto(proofUrl.href.replace("&live=1", ""), { waitUntil: "networkidle" });
  await uiPage.locator('[data-dashboard-link="agents"]').first().click();
  await uiPage.locator('[data-agent-id="shepherd"]').waitFor({ state: "visible" });
  await uiPage.locator("#roster-load-more").click();
  await uiPage.locator('[data-agent-id="proof-agent-2"]').waitFor({ state: "visible" });
  assert.equal(await uiPage.locator('[data-agent-id="shepherd"]').count(), 1);
  assert.equal(await uiPage.locator("#live-agent-list [data-agent-id]").count(), 2);
  assert.equal(await uiPage.locator("#agent-count").textContent(), "2 of 3 visible");
  assert.equal(await uiPage.locator("#roster-load-more").isHidden(), true);
  await uiPage.locator('[data-agent-id="proof-agent-2"]').click();
  await uiPage.locator("#roster-detail").getByText("Proof Agent Two").waitFor();
  assert((await uiPage.locator("#live-agent-list [data-agent-id]").count()) <= 50);

  for (const transition of [
    { revision: 21, state: "migrating", location: "node-b" },
    { revision: 22, state: "degraded", location: "node-b" },
    { revision: 23, state: "unreachable", location: "node-b" },
    { revision: 25, state: "ready", location: "node-c" }
  ]) {
    uiRevision = transition.revision;
    uiState = transition.state;
    uiLocation = transition.location;
    await uiPage.locator("#refresh-live").click();
    await uiPage.locator('[data-agent-id="shepherd"]').getByText(new RegExp(transition.state, "i")).waitFor();
  }
  uiRevision = 24;
  uiState = "degraded";
  await uiPage.locator("#refresh-live").click();
  assert.match(await uiPage.locator('[data-agent-id="shepherd"]').textContent(), /ready/i);
  await uiPage.close();

  const restartResponse = await fetch(new URL("/v1/control", runtime), {
    method: "POST",
    headers: { "content-type": "application/json", origin: observatory.origin },
    body: JSON.stringify(signedRestart(feed))
  });
  assert.equal(restartResponse.status, 200, `signed incarnation-bound restart failed: ${await restartResponse.text()}`);

  let restartedFeed = null;
  const restartDeadline = Date.now() + 15_000;
  while (Date.now() < restartDeadline) {
    try {
      const candidate = await fetchFeed();
      if (candidate.runtime_incarnation_id !== feed.runtime_incarnation_id) {
        restartedFeed = candidate;
        break;
      }
    } catch (_error) {
      // The isolated Guardian owns this bounded restart interval.
    }
    await sleep(100);
  }
  assert(restartedFeed, "Guardian must restore Runtime with a new incarnation");
  assert.equal(restartedFeed.agents.sample[0].source_revision, sourceRevision);
  await page.locator("#statusbar-websocket").getByText("connected", { exact: true }).waitFor({ timeout: 12_000 });
  await row.waitFor({ state: "visible" });
  assert.equal(await page.locator('[data-agent-id="shepherd"]').count(), 1, "Runtime reincarnation must reset cursor without duplicate rows");

  await page.setViewportSize({ width: 390, height: 844 });
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth), true);
  assert.deepEqual(pageErrors, []);
  await context.close();
} finally {
  await browser.close();
}

process.stdout.write("PASS: Runtime-backed local Shepherd roster, bounded presence, pagination, transition, reconnect, and reincarnation proof\n");
