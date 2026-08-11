#!/usr/bin/env node

import assert from "node:assert/strict";
import { createRequire } from "node:module";

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
assert(observatoryUrl, "ADL_OBSERVATORY_URL must name the HTML Observatory URL");
assert(runtimeApiBase, "ADL_RUNTIME_API_BASE must name the Runtime v3 API base");
assert(/^[0-9a-f]{40}$/.test(sourceRevision || ""), "ADL_SOURCE_REVISION must name the exact candidate");

const observatory = new URL(observatoryUrl);
const runtime = new URL(runtimeApiBase);
assert.equal(observatory.protocol, "https:", "Observatory roster proof requires trusted HTTPS");
assert.equal(runtime.protocol, "https:", "Runtime roster proof requires trusted HTTPS");
assert.equal(observatory.hostname, runtime.hostname, "Observatory and Runtime must share the instance DNS identity");

const response = await fetch(new URL("/v1/observatory", runtime));
assert.equal(response.status, 200, "Runtime Observatory feed must be available");
const feed = await response.json();
assert.equal(feed.schema, "adl.runtime_v3.observatory_feed.v2");
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
assert(shepherd.freshness_deadline_unix_millis >= shepherd.observed_at_unix_millis);
assert(/^[0-9a-f]{40}$/.test(shepherd.source_revision));
assert.equal(shepherd.source_revision, sourceRevision, "roster evidence must name the exact Runtime build revision");
const rosterResponse = await fetch(new URL("/v1/agents?page_size=1", runtime));
assert.equal(rosterResponse.status, 200);
const rosterPage = await rosterResponse.json();
assert.equal(rosterPage.schema, "adl.runtime_v3.agent_roster_page.v1");
assert.equal(rosterPage.sample.length, 1);
assert.equal(rosterPage.sample[0].id, shepherd.id);

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

  await page.setViewportSize({ width: 390, height: 844 });
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth), true);
  assert.deepEqual(pageErrors, []);
  await context.close();
} finally {
  await browser.close();
}

process.stdout.write("PASS: Runtime-backed local Shepherd roster and browser presence proof\n");
