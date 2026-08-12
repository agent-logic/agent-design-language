#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export ADL_LAYER8_REPO_ROOT="$ROOT_DIR"

for path in \
  "$ROOT_DIR/demos/html-observatory/index.html" \
  "$ROOT_DIR/demos/html-observatory/app.js" \
  "$ROOT_DIR/demos/html-observatory/styles.css" \
  "$ROOT_DIR/docs/milestones/v0.92/features/LAYER8_CONVERSATION_AUTHORITY.md"; do
  if [[ ! -f "$path" ]]; then
    printf 'missing Layer 8 Observatory browser-proof input: %s\n' "$path" >&2
    exit 1
  fi
done

node --input-type=module <<'NODE'
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtemp, readFile, realpath, rm } from "node:fs/promises";
import { createServer } from "node:https";
import { createRequire } from "node:module";
import { extname, join, normalize, resolve, sep } from "node:path";
import { pathToFileURL } from "node:url";
import { tmpdir } from "node:os";

const repoRoot = await realpath(process.env.ADL_LAYER8_REPO_ROOT || "");
const require = createRequire(import.meta.url);
let playwright;
try {
  const requested = process.env.ADL_PLAYWRIGHT_MODULE;
  playwright = requested
    ? require(resolve(requested))
    : require("playwright");
} catch (error) {
  throw new Error(
    `Playwright is required for the Layer 8 Observatory browser proof; set ADL_PLAYWRIGHT_MODULE or install the repo-native module: ${error.message}`
  );
}
assert(playwright?.chromium, "resolved Playwright module does not export chromium");

const tempRoot = await mkdtemp(join(tmpdir(), "adl-layer8-observatory-"));
const keyPath = join(tempRoot, "localhost-key.pem");
const certPath = join(tempRoot, "localhost-cert.pem");
let server;
let browser;

const feed = (revision) => ({
  schema: "adl.runtime_v3.observatory_feed.v2",
  runtime_selection: "runtime_v3_explicit_opt_in",
  runtime_instance_id: "layer8-browser-runtime",
  runtime_incarnation_id: "layer8-browser-incarnation-a",
  health: {
    observability_ready: true,
    snapshot: {
      lifecycle: "ready",
      observability: "ready",
      observability_ready: true,
      topology_generation: 1,
      event_count: 0,
      components: {},
      queues: {}
    }
  },
  agents: {
    scope: "local_runtime",
    population_complete: true,
    revision,
    event_cursor: `layer8-cursor-${revision}`,
    total_count: 1,
    rendered_sample_count: 1,
    has_more: false,
    sample: [{
      id: "agent-0001",
      label: "Authority Proof Agent",
      role: "conversation_agent",
      state: "ready",
      health: "healthy",
      availability: "available",
      communication_eligible: true,
      provenance: "runtime_component_state",
      detail: "deterministic local browser fixture",
      source_revision: "0".repeat(40)
    }]
  },
  events: [],
  control: { browser_mutation_authority: true },
  proof: { mode: "deterministic_local_browser" }
});

const contentTypes = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".md": "text/markdown; charset=utf-8"
};

try {
  try {
    execFileSync("openssl", [
      "req", "-x509", "-newkey", "rsa:2048", "-nodes", "-sha256", "-days", "1",
      "-subj", "/CN=127.0.0.1", "-addext", "subjectAltName=IP:127.0.0.1",
      "-keyout", keyPath, "-out", certPath
    ], { stdio: "ignore" });
  } catch (error) {
    throw new Error(`openssl is required to create the ephemeral loopback certificate: ${error.message}`);
  }

  server = createServer({
    key: await readFile(keyPath),
    cert: await readFile(certPath)
  }, async (request, response) => {
    try {
      const url = new URL(request.url || "/", "https://127.0.0.1");
      if (url.pathname === "/v1/ready") {
        response.writeHead(200, { "content-type": "application/json", "access-control-allow-origin": "*" });
        response.end(JSON.stringify({
          schema: "adl.runtime_v3.readiness.v1",
          ready: true,
          degraded_reasons: []
        }));
        return;
      }
      if (url.pathname === "/v1/observatory") {
        response.writeHead(200, { "content-type": "application/json", "access-control-allow-origin": "*" });
        response.end(JSON.stringify(feed(1)));
        return;
      }

      const relativePath = url.pathname === "/demos/html-observatory/"
        ? "demos/html-observatory/index.html"
        : decodeURIComponent(url.pathname).replace(/^\/+/, "");
      const candidate = resolve(repoRoot, normalize(relativePath));
      if (candidate !== repoRoot && !candidate.startsWith(`${repoRoot}${sep}`)) {
        response.writeHead(403).end("forbidden");
        return;
      }
      const body = await readFile(candidate);
      response.writeHead(200, { "content-type": contentTypes[extname(candidate)] || "application/octet-stream" });
      response.end(body);
    } catch (_error) {
      response.writeHead(404).end("not found");
    }
  });
  await new Promise((resolveListen, rejectListen) => {
    server.once("error", rejectListen);
    server.listen(0, "127.0.0.1", resolveListen);
  });
  const address = server.address();
  assert(address && typeof address === "object", "ephemeral loopback server did not expose an address");
  const origin = `https://127.0.0.1:${address.port}`;

  const executablePath = process.env.ADL_CHROMIUM_EXECUTABLE || undefined;
  browser = await playwright.chromium.launch({ headless: true, executablePath });
  const context = await browser.newContext({
    ignoreHTTPSErrors: true,
    viewport: { width: 1440, height: 1000 }
  });
  const page = await context.newPage();
  const loadedProductPaths = new Set();
  const pageErrors = [];
  page.on("response", (response) => {
    const path = new URL(response.url()).pathname;
    if ([
      "/demos/html-observatory/",
      "/demos/html-observatory/app.js",
      "/demos/html-observatory/styles.css"
    ].includes(path) && response.ok()) loadedProductPaths.add(path);
  });
  page.on("pageerror", (error) => pageErrors.push(error.message));

  await page.addInitScript(({ websocketFeed }) => {
    const sockets = [];
    class DeterministicWebSocket extends EventTarget {
      static CONNECTING = 0;
      static OPEN = 1;
      static CLOSING = 2;
      static CLOSED = 3;
      constructor(url) {
        super();
        this.url = String(url);
        this.readyState = DeterministicWebSocket.CONNECTING;
        sockets.push(this);
        setTimeout(() => {
          this.readyState = DeterministicWebSocket.OPEN;
          this.dispatchEvent(new Event("open"));
          this.emit(websocketFeed);
        }, 0);
      }
      emit(frame) {
        this.dispatchEvent(new MessageEvent("message", { data: JSON.stringify(frame) }));
      }
      send(payload) {
        const frame = JSON.parse(String(payload));
        if (frame.schema === "adl.runtime_v3.observatory_ws_auth.v1") {
          setTimeout(() => this.emit({
            schema: "adl.runtime_v3.observatory_ws_control_result.v1",
            status: "authenticated"
          }), 0);
          return;
        }
        if (frame.schema !== "adl.runtime_v3.observatory_conversation_intent.v1") return;
        const common = {
          schema: "adl.runtime_v3.observatory_conversation_result.v1",
          conversation_id: frame.conversation_id,
          turn_id: frame.turn_id,
          recipient_id: frame.recipient_id,
          correlation_id: frame.correlation_id
        };
        if (frame.message.includes("REFUSE")) {
          setTimeout(() => this.emit({
            ...common,
            status: "refused",
            error: "capability_scope_refused",
            result_hash: "FORBIDDEN-REFUSAL-HASH",
            private_policy: "FORBIDDEN-PRIVATE-POLICY"
          }), 0);
          return;
        }
        setTimeout(() => this.emit({ ...common, status: "accepted" }), 0);
        setTimeout(() => this.emit({
          ...common,
          status: "delivered",
          turn_sequence: 1,
          reply: "AUTHORIZED AGENT REPLY",
          result_hash: "FORBIDDEN-DELIVERY-HASH",
          provider_payload: "FORBIDDEN-PROVIDER-PAYLOAD"
        }), 5);
      }
      close(code = 1000, reason = "") {
        if (this.readyState === DeterministicWebSocket.CLOSED) return;
        this.readyState = DeterministicWebSocket.CLOSED;
        this.dispatchEvent(new CloseEvent("close", { code, reason, wasClean: true }));
      }
    }
    globalThis.WebSocket = DeterministicWebSocket;
    globalThis.__adlLayer8Emit = (frame) => sockets.at(-1)?.emit(frame);
  }, { websocketFeed: feed(2) });

  const proofUrl = new URL("/demos/html-observatory/", origin);
  proofUrl.searchParams.set("runtime", "v3");
  proofUrl.searchParams.set("runtimeApiBase", origin);
  proofUrl.searchParams.set("live", "1");
  const navigation = await page.goto(proofUrl.href, { waitUntil: "domcontentloaded" });
  assert(navigation?.ok(), `Observatory navigation failed: ${navigation?.status() ?? "no response"}`);
  await page.locator("#statusbar-websocket").getByText("connected", { exact: true }).waitFor();
  await page.evaluate(() => document.querySelector('[data-dashboard-link="communication"]')?.click());
  await page.locator('.observatory[data-dashboard-surface="communication"] #communication').waitFor({ state: "visible" });
  await page.locator("#agent-conversation-recipient").selectOption("agent-0001");
  assert.deepEqual(
    [...loadedProductPaths].sort(),
    [
      "/demos/html-observatory/",
      "/demos/html-observatory/app.js",
      "/demos/html-observatory/styles.css"
    ],
    "browser must load the actual Observatory index, app, and styles"
  );
  assert.equal(await page.locator("#agent-conversation-transcript").evaluate((element) => {
    const style = getComputedStyle(element);
    return style.display !== "none" && style.visibility !== "hidden";
  }), true, "conversation transcript must be visibly rendered by the actual stylesheet");

  await page.locator("#operator-write-token").fill("LOCAL-BROWSER-TOKEN-MUST-NOT-RENDER");
  await page.locator("#operator-login").click();
  await page.locator("#operator-auth-status").getByText("write access enabled", { exact: true }).waitFor();

  await page.locator("#agent-conversation-message").fill("AUTHORIZED OPERATOR TURN");
  await page.locator("#send-agent-conversation").click();
  await page.locator('.conversation-turn[data-speaker="operator"]').getByText("AUTHORIZED OPERATOR TURN", { exact: true }).waitFor();
  await page.locator('.conversation-turn[data-speaker="agent"]').getByText("AUTHORIZED AGENT REPLY", { exact: true }).waitFor();
  await page.locator('.conversation-turn[data-speaker="agent"] .conversation-turn-status').getByText("delivered", { exact: true }).waitFor();
  console.log("PASS authorized visible state: accepted operator turn and delivered agent reply rendered");

  await page.locator("#agent-conversation-message").fill("REFUSE PRIVATE OPERATOR TEXT");
  await page.locator("#send-agent-conversation").click();
  await page.locator('.conversation-turn[data-speaker="runtime"]').getByText(
    "Turn refused: capability_scope_refused",
    { exact: true }
  ).waitFor();
  assert.equal(await page.getByText("REFUSE PRIVATE OPERATOR TEXT", { exact: true }).count(), 0);
  assert.equal(await page.locator("#send-agent-conversation").isEnabled(), true);
  console.log("PASS refused visible state: bounded refusal rendered, operator content withheld, action released");

  await page.evaluate(() => globalThis.__adlLayer8Emit({
    schema: "adl.runtime_v3.observatory_ws_control_result.v1",
    status: "rejected",
    error: "credential_revoked"
  }));
  await page.locator("#operator-auth-status").getByText("public read", { exact: true }).waitFor();
  assert.equal(await page.locator("#send-agent-conversation").isDisabled(), true);
  console.log("PASS revoked visible state: Observatory demoted to public read and send disabled");

  const visibleText = await page.locator("body").innerText();
  for (const forbidden of [
    "LOCAL-BROWSER-TOKEN-MUST-NOT-RENDER",
    "FORBIDDEN-REFUSAL-HASH",
    "FORBIDDEN-PRIVATE-POLICY",
    "FORBIDDEN-DELIVERY-HASH",
    "FORBIDDEN-PROVIDER-PAYLOAD"
  ]) {
    assert.equal(visibleText.includes(forbidden), false, `forbidden field reached visible UI: ${forbidden}`);
  }
  assert.equal(pageErrors.length, 0, `browser page errors: ${pageErrors.join("; ")}`);
  console.log("PASS disclosure-safe visible state: token, proof hashes, policy, and provider payload stayed hidden");
  console.log(`PASS browser evidence: ${origin}/demos/html-observatory/ (ephemeral loopback HTTPS)`);
} finally {
  if (browser) await browser.close();
  if (server) await new Promise((resolveClose) => server.close(resolveClose));
  await rm(tempRoot, { recursive: true, force: true });
}
NODE

printf 'PASS: Layer 8 Observatory real-browser authority presentation contract\n'
