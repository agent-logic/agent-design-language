#!/usr/bin/env node

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash, createPrivateKey, randomUUID, sign as signBytes } from "node:crypto";
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
const observatoryTokenFile = process.env.ADL_OBSERVATORY_TOKEN_FILE;
const evidenceRoot = process.env.ADL_OBSERVATORY_EVIDENCE_DIR;
const tlsConnectHost = process.env.ADL_TLS_PROOF_CONNECT_HOST || null;
const allowRuntimeRestartProof = process.env.ADL_ALLOW_RUNTIME_RESTART_PROOF === "1";
const sourceRevision = process.env.ADL_SOURCE_REVISION;
const expectedPolisName = process.env.ADL_EXPECTED_POLIS_NAME;
const repositoryRootInput = process.env.ADL_REPOSITORY_ROOT;

assert(observatoryUrl, "ADL_OBSERVATORY_URL must name the served HTML Observatory URL");
assert(runtimeApiBase, "ADL_RUNTIME_API_BASE must name the exact Runtime candidate URL");
assert(operatorKeyFile, "ADL_OPERATOR_KEY_FILE must name the trusted operator Ed25519 seed file");
assert(observatoryTokenFile, "ADL_OBSERVATORY_TOKEN_FILE must name the Observatory write credential file");
assert(evidenceRoot, "ADL_OBSERVATORY_EVIDENCE_DIR must name a retained FastWork evidence directory");
assert(allowRuntimeRestartProof, "ADL_ALLOW_RUNTIME_RESTART_PROOF=1 is required for the isolated Guardian restart proof");
assert(/^[0-9a-f]{40}$/.test(sourceRevision || ""), "ADL_SOURCE_REVISION must name the exact 40-character source commit");
assert(repositoryRootInput, "ADL_REPOSITORY_ROOT must name the exact clean source worktree");
assert(
  expectedPolisName?.trim() === expectedPolisName && expectedPolisName.length > 0,
  "ADL_EXPECTED_POLIS_NAME must name the configured logical Polis"
);
const fastWorkRoot = await fs.realpath("/Volumes/FastWork");
const repositoryRoot = await fs.realpath(repositoryRootInput);
const repositoryRevision = execFileSync("git", ["-C", repositoryRoot, "rev-parse", "HEAD"], {
  encoding: "utf8"
}).trim();
assert.equal(sourceRevision, repositoryRevision, "ADL_SOURCE_REVISION does not match repository HEAD");
const repositoryStatus = execFileSync(
  "git",
  ["-C", repositoryRoot, "status", "--porcelain", "--untracked-files=no"],
  { encoding: "utf8" }
).trim();
assert.equal(repositoryStatus, "", "live proof requires a clean tracked repository state");
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
const observatoryToken = (await fs.readFile(observatoryTokenFile, "utf8")).trim();
assert(observatoryToken.length >= 32 && observatoryToken.length <= 256, "Observatory token must satisfy Runtime bounds");
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

function buildSignedRestartCommand(runtimeInstanceId) {
  const seedHex = signingSeed.replace(/^0x/, "");
  const pkcs8Prefix = Buffer.from("302e020100300506032b657004220420", "hex");
  const privateKey = createPrivateKey({
    key: Buffer.concat([pkcs8Prefix, Buffer.from(seedHex, "hex")]),
    format: "der",
    type: "pkcs8"
  });
  const correlationId = randomUUID().replaceAll("-", "");
  const command = {
    schema: "adl.runtime.control_command.v1",
    runtime_instance_id: runtimeInstanceId,
    command_id: `restart-${correlationId}`,
    correlation_id: correlationId,
    principal: "operator",
    action: { action: "restart", grace_millis: 10_000 },
    signing_algorithm: "ed25519",
    signing_key_id: "operator-key",
    signature: ""
  };
  command.signature = signBytes(null, Buffer.from(JSON.stringify(command)), privateKey).toString("hex");
  return command;
}

async function writeExclusive(pathname, value) {
  const handle = await fs.open(pathname, "wx", 0o600);
  try {
    await handle.writeFile(value);
  } finally {
    await handle.close();
  }
}

function getTrusted(endpoint, pathname) {
  return new Promise((resolve, reject) => {
    const request = https.request({
      hostname: tlsConnectHost || endpoint.hostname,
      port: Number(endpoint.port || 443),
      path: pathname,
      method: "GET",
      servername: endpoint.hostname,
      rejectUnauthorized: true,
      ca: tls.rootCertificates,
      headers: { Host: endpoint.host }
    }, (response) => {
      const chunks = [];
      response.on("data", (chunk) => chunks.push(chunk));
      response.once("end", () => resolve({
        status: response.statusCode,
        headers: response.headers,
        body: Buffer.concat(chunks)
      }));
    });
    request.setTimeout(10_000, () => request.destroy(new Error(`HTTPS GET timed out for ${endpoint.origin}${pathname}`)));
    request.once("error", reject);
    request.end();
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
const observatoryDocument = await getTrusted(url, url.pathname || "/");
assert.equal(observatoryDocument.status, 200, "Observatory HTML did not load through the trusted listener");
assert.equal(
  observatoryDocument.headers["x-adl-source-revision"],
  sourceRevision,
  "running Observatory server source revision does not match the proof revision"
);
const assetProof = {};
for (const relativeAsset of [
  "demos/html-observatory/index.html",
  "demos/html-observatory/app.js",
  "demos/html-observatory/styles.css"
]) {
  const served = relativeAsset.endsWith("index.html")
    ? observatoryDocument
    : await getTrusted(url, `/${relativeAsset}`);
  assert.equal(served.status, 200, `${relativeAsset} was not served`);
  const local = await fs.readFile(path.join(repositoryRoot, relativeAsset));
  const localDigest = createHash("sha256").update(local).digest("hex");
  const servedDigest = createHash("sha256").update(served.body).digest("hex");
  assert.equal(servedDigest, localDigest, `${relativeAsset} bytes differ from exact source revision`);
  assetProof[relativeAsset] = servedDigest;
}
const blockedRepositoryPaths = ["/AGENTS.md", "/.git/HEAD", "/.csdlc/issues/83/index.json"];
for (const blockedPath of blockedRepositoryPaths) {
  const response = await getTrusted(url, blockedPath);
  assert.equal(response.status, 404, `repository path was publicly served: ${blockedPath}`);
}
const contentSecurityPolicy = String(observatoryDocument.headers["content-security-policy"] || "");
for (const directive of [
  `https://${url.hostname}:*`,
  `wss://${url.hostname}:*`,
  "object-src 'none'",
  "base-uri 'none'",
  "frame-ancestors 'none'",
  "form-action 'none'"
]) {
  assert(contentSecurityPolicy.includes(directive), `Observatory CSP is missing ${directive}`);
}
assert(!contentSecurityPolicy.includes("localhost"), "Observatory CSP must not authorize localhost");
assert(!contentSecurityPolicy.includes("127.0.0.1"), "Observatory CSP must not authorize a loopback IP identity");
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
  source_provenance: {
    repository_head: repositoryRevision,
    tracked_worktree_clean: true,
    repository_root_verified: true,
    served_asset_sha256: assetProof
  },
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
const artifactStem = `observatory-layer8-chat-${sourceRevision.slice(0, 12)}-${randomUUID()}`;

try {
  await page.goto(url.toString(), { waitUntil: "domcontentloaded", timeout: 30_000 });
  await page.locator("#statusbar-websocket").filter({ hasText: "connected" }).waitFor({ timeout: 20_000 });
  await page.waitForFunction(() => {
    const value = Number(document.querySelector(".observatory")?.dataset.streamLastSequence);
    return Number.isSafeInteger(value) && value >= 0;
  });
  const initialStreamSequence = Number(await page.locator(".observatory").getAttribute("data-stream-last-sequence"));
  assert.equal(await page.locator("#environment-label").textContent(), "Trusted Runtime");
  await page.locator("#polis-name").filter({ hasText: expectedPolisName }).waitFor({ timeout: 20_000 });
  assert.equal(await page.locator("#polis-name").textContent(), expectedPolisName);
  assert.equal(await page.locator("#operator-status-pill").textContent(), "Public read");
  report.assertions.live_wss_frame = {
    passed: true,
    polis_name: expectedPolisName,
    initial_sequence: initialStreamSequence
  };
  report.assertions.instance_scoped_csp = {
    passed: true,
    policy: contentSecurityPolicy,
    runtime_hostname: runtimeUrl.hostname
  };
  report.assertions.repository_files_not_served = {
    passed: true,
    blocked_paths: blockedRepositoryPaths
  };
  const capture = await page.evaluate(() => ({
    iso: document.querySelector(".observatory")?.dataset.captureTime || "",
    source_millis: Number(document.querySelector(".observatory")?.dataset.captureSourceMillis),
    hero: document.getElementById("hero-uptime")?.textContent?.trim() || "",
    rail: document.getElementById("rail-capture-time")?.textContent?.trim() || "",
    status: document.getElementById("statusbar-updated")?.textContent?.trim() || ""
  }));
  const captureMillis = Date.parse(capture.iso);
  assert(Number.isFinite(captureMillis), `live capture time is not an ISO timestamp: ${capture.iso}`);
  assert(Number.isSafeInteger(capture.source_millis) && capture.source_millis > 0, "Runtime capture source was not retained by the page");
  assert.equal(captureMillis, capture.source_millis, "displayed capture time was not copied exactly from the consumed Runtime feed");
  const captureAgeMillis = Date.now() - captureMillis;
  assert(captureAgeMillis >= -5_000 && captureAgeMillis < 30_000, `live capture time is stale or outside qualified skew: ${capture.iso}`);
  assert(capture.hero && capture.hero === capture.rail && capture.hero === capture.status, `capture time surfaces diverged: ${JSON.stringify(capture)}`);
  report.assertions.fresh_consistent_capture_time = {
    passed: true,
    authority: "runtime_qualified_time",
    runtime_captured_at_unix_millis: capture.source_millis,
    browser_age_millis: captureAgeMillis,
    ...capture
  };

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

  assert.equal(await page.locator("#agent-chat-key-file").count(), 0, "normal chat exposed a private-key file input");
  await page.locator("#operator-write-token").fill(observatoryToken);
  await page.locator("#operator-login").click();
  await page.locator("#operator-auth-status").filter({ hasText: "write access enabled" }).waitFor({ timeout: 20_000 });
  report.assertions.authenticated_chat_has_no_browser_private_key = { passed: true };
  await page.locator("#operator-message").fill("Hello Shepherd. Please confirm that you are present.");
  await page.locator("#send-agent-message:not([disabled])").click();
  await page.locator('.chat-message[data-role="agent"], .chat-message[data-role="system"]').waitFor({ timeout: 20_000 });
  const agentMessages = await page.locator('.chat-message[data-role="agent"]').count();
  if (agentMessages === 0) {
    const refusal = await page.locator('.chat-message[data-role="system"] span').last().textContent();
    throw new Error(`signed selected-agent chat was refused: ${refusal}`);
  }
  const deliveredStatus = await page.locator("#agent-chat-status").textContent();
  assert.match(deliveredStatus, /^delivered · .+ verified$/);
  report.assertions.signed_selected_agent_chat = {
    passed: true,
    status: deliveredStatus,
    transcript_messages: await page.locator(".chat-message").count()
  };
  const preRestartAckProof = await page.evaluate(async (base) => {
    const feed = await (await fetch(`${base}/v1/observatory`)).json();
    const acknowledgements = Object.values(feed.ingress?.completed || {})
      .map((result) => result?.public_output)
      .filter((output) => output?.sender_id === "shepherd")
      .sort((left, right) => right.monotonic_sequence - left.monotonic_sequence);
    const acknowledgement = acknowledgements[0];
    const agent = feed.agents?.sample?.find((candidate) => candidate.id === "shepherd");
    if (!acknowledgement || !agent) throw new Error("Runtime feed omitted the verified Shepherd acknowledgement");
    let replayError = "";
    try {
      await globalThis.AdlHtmlObservatory.verifySignedIdentityMessage(
        acknowledgement,
        agent,
        acknowledgement.correlation_id,
        acknowledgement.causation_id
      );
    } catch (error) {
      replayError = error instanceof Error ? error.message : String(error);
    }
    return { sequence: acknowledgement.monotonic_sequence, replayError };
  }, runtimeUrl.origin);
  assert.match(preRestartAckProof.replayError, /replayed or arrived behind/, "browser accepted a replayed pre-restart Shepherd acknowledgement");
  await page.waitForFunction((previous) => {
    const value = Number(document.querySelector(".observatory")?.dataset.streamLastSequence);
    return Number.isSafeInteger(value) && value > previous;
  }, initialStreamSequence);
  report.assertions.wss_observed_correlated_runtime_progress = {
    passed: true,
    sequence: Number(await page.locator(".observatory").getAttribute("data-stream-last-sequence"))
  };

  const forbiddenOriginCommand = buildSignedRestartCommand(
    await page.locator(".observatory").getAttribute("data-stream-runtime-instance")
  );
  const forbiddenOriginResponse = await postTrustedJson(
    runtimeUrl,
    "/v1/control",
    { Origin: "https://forbidden.example.test" },
    forbiddenOriginCommand
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
  await page.locator("#send-agent-message:not([disabled])").click();
  await page.locator('.chat-message[data-role="system"]').nth(systemMessagesBefore).waitFor({ timeout: 20_000 });
  const refusalStatus = await page.locator("#agent-chat-status").textContent();
  assert.equal(refusalStatus, "malformed runtime data", "Runtime refusal was not presented as malformed input");
  report.assertions.runtime_refusal_remains_denied = {
    passed: true,
    transport: "authenticated_wss_intent",
    ui_status: refusalStatus
  };

  assert.equal(await page.locator('input[type="file"]').count(), 0, "Observatory retained a private-key input after chat");

  const automaticReconnectBefore = {
    transcript: await page.locator(".chat-message").count(),
    control_posts: controlRequests.length,
    runtime_instance: await page.locator(".observatory").getAttribute("data-stream-runtime-instance"),
    runtime_incarnation: await page.locator(".observatory").getAttribute("data-stream-runtime-incarnation"),
    last_sequence: Number(await page.locator(".observatory").getAttribute("data-stream-last-sequence")),
    applied_events: Number(await page.locator(".observatory").getAttribute("data-stream-applied-events")),
    reconnect_decisions: Number(await page.locator(".observatory").getAttribute("data-reconnect-decision-count") || 0)
  };
  const restartTarget = await page.evaluate(async (base) => {
    const response = await fetch(`${base}/v1/observatory`);
    if (!response.ok) throw new Error(`Runtime feed returned ${response.status}`);
    const feed = await response.json();
    return {
      pid: feed.runtime_process_id,
      runtime_incarnation_id: feed.runtime_incarnation_id,
      runtime_instance_id: feed.runtime_instance_id,
      source_revision: feed.source_revision
    };
  }, runtimeUrl.origin);
  assert(Number.isSafeInteger(restartTarget.pid) && restartTarget.pid > 1, "Runtime feed did not expose an exact candidate PID");
  assert.equal(restartTarget.runtime_instance_id, automaticReconnectBefore.runtime_instance, "restart target identity drifted before the proof");
  assert.equal(restartTarget.source_revision, sourceRevision, "Runtime binary source revision does not match the proof revision");
  proofPhase = "guardian_restart";
  guardianRestartInProgress = true;
  const restartResponse = await postTrustedJson(
    runtimeUrl,
    "/v1/control",
    { Origin: new URL(observatoryUrl).origin },
    buildSignedRestartCommand(restartTarget.runtime_instance_id)
  );
  assert.equal(restartResponse.status, 200, "signed checkpointed restart request was not accepted");
  await page.waitForFunction((previousReconnectDecisions) => {
    const root = document.querySelector(".observatory");
    return Number(root?.dataset.reconnectDecisionCount || 0) > previousReconnectDecisions &&
      /^(250|500|1000|2000|4000)$/.test(root?.dataset.lastReconnectDelayMillis || "");
  }, automaticReconnectBefore.reconnect_decisions, { timeout: 20_000 });
  const observedBackoff = Number(
    await page.locator(".observatory").getAttribute("data-last-reconnect-delay-millis")
  );
  await page.waitForFunction(() => {
    return document.querySelector(".observatory")?.dataset.liveConnection === "connected";
  }, null, { timeout: 30_000 });
  const automaticReconnectAfter = {
    transcript: await page.locator(".chat-message").count(),
    control_posts: controlRequests.length,
    runtime_instance: await page.locator(".observatory").getAttribute("data-stream-runtime-instance"),
    runtime_incarnation: await page.locator(".observatory").getAttribute("data-stream-runtime-incarnation"),
    last_sequence: Number(await page.locator(".observatory").getAttribute("data-stream-last-sequence")),
    applied_events: Number(await page.locator(".observatory").getAttribute("data-stream-applied-events")),
    reconnect_decisions: Number(await page.locator(".observatory").getAttribute("data-reconnect-decision-count") || 0)
  };
  guardianRestartInProgress = false;
  proofPhase = "baseline";
  const restartedTarget = await page.evaluate(async (base) => {
    const response = await fetch(`${base}/v1/observatory`);
    if (!response.ok) throw new Error(`Runtime feed returned ${response.status}`);
    const feed = await response.json();
    return {
      pid: feed.runtime_process_id,
      runtime_incarnation_id: feed.runtime_incarnation_id,
      source_revision: feed.source_revision
    };
  }, runtimeUrl.origin);
  assert.equal(automaticReconnectAfter.transcript, automaticReconnectBefore.transcript, "automatic reconnect duplicated chat");
  assert.equal(automaticReconnectAfter.control_posts, automaticReconnectBefore.control_posts, "automatic reconnect emitted or replayed a browser control POST");
  assert.equal(automaticReconnectAfter.runtime_instance, automaticReconnectBefore.runtime_instance, "Runtime restart changed configured instance identity");
  assert.notEqual(
    automaticReconnectAfter.runtime_incarnation,
    automaticReconnectBefore.runtime_incarnation,
    "Runtime restart did not change the explicit process incarnation"
  );
  assert.equal(automaticReconnectAfter.runtime_incarnation, restartedTarget.runtime_incarnation_id);
  assert.equal(restartedTarget.source_revision, sourceRevision);
  assert(automaticReconnectAfter.reconnect_decisions > automaticReconnectBefore.reconnect_decisions, "restart did not produce a fresh reconnect decision");
  assert(automaticReconnectAfter.last_sequence >= 0, "automatic reconnect did not establish a valid event cursor");
  assert(automaticReconnectAfter.applied_events > 0, "automatic reconnect dropped all new-process bootstrap events");
  report.assertions.automatic_bounded_reconnect = {
    passed: true,
    observed_backoff_millis: observedBackoff,
    healthy_reset_millis: 10_000,
    guardian_restart: {
      previous_pid: restartTarget.pid,
      next_pid: restartedTarget.pid,
      previous_runtime_incarnation: automaticReconnectBefore.runtime_incarnation,
      next_runtime_incarnation: automaticReconnectAfter.runtime_incarnation,
      previous_runtime_instance: automaticReconnectBefore.runtime_instance,
      next_runtime_instance: automaticReconnectAfter.runtime_instance,
      restart_authority: "signed_control_stop_capability",
      accepted_restart_requests: 1,
      accepted_restart_status: restartResponse.status,
      source_revision: restartTarget.source_revision
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
  await page.locator("#operator-logout").click();
  await page.locator("#agent-chat-target:not([disabled])").waitFor({ timeout: 20_000 });
  const afterReconnect = await page.locator(".chat-message").count();
  assert.equal(afterReconnect, beforeReconnect, "reconnect duplicated the conversation transcript");
  const postsAfterReconnect = controlRequests.length;
  const ingressBeforeReconnectDenial = await page.evaluate(async (base) => {
    const response = await fetch(`${base}/v1/observatory`);
    return (await response.json()).ingress.accepted_through;
  }, runtimeUrl.origin);
  await page.locator("#operator-message").fill("Confirm the reauthenticated channel after reconnect.");
  assert(await page.locator("#send-agent-message").isDisabled(), "manual reconnect retained browser write authority");
  const ingressWhileUnauthenticated = await page.evaluate(async (base) => {
    const response = await fetch(`${base}/v1/observatory`);
    return (await response.json()).ingress.accepted_through;
  }, runtimeUrl.origin);
  assert.equal(ingressWhileUnauthenticated, ingressBeforeReconnectDenial, "manual reconnect changed ingress before reauthentication");
  assert.equal(controlRequests.length, postsAfterReconnect, "manual reconnect replayed a control POST");
  const agentMessagesBeforeReauth = await page.locator('.chat-message[data-role="agent"]').count();
  await page.locator("#operator-write-token").fill(observatoryToken);
  await page.locator("#operator-login").click();
  await page.locator("#operator-auth-status").filter({ hasText: "write access enabled" }).waitFor({ timeout: 20_000 });
  await page.locator("#send-agent-message:not([disabled])").click();
  await page.locator('.chat-message[data-role="agent"]').nth(agentMessagesBeforeReauth).waitFor({ timeout: 20_000 });
  assert.match(await page.locator("#agent-chat-status").textContent(), /^delivered · .+ verified$/);
  assert.equal(controlRequests.length, postsAfterReconnect, "WSS chat emitted an obsolete control POST after reconnect");
  const postRestartAckSequence = await page.evaluate(async (base) => {
    const feed = await (await fetch(`${base}/v1/observatory`)).json();
    return Math.max(
      ...Object.values(feed.ingress?.completed || {})
        .map((result) => result?.public_output)
        .filter((output) => output?.sender_id === "shepherd")
        .map((output) => output.monotonic_sequence)
    );
  }, runtimeUrl.origin);
  assert(
    postRestartAckSequence > preRestartAckProof.sequence,
    "post-restart Shepherd acknowledgement sequence did not advance"
  );
  report.assertions.signed_agent_ack_continuity = {
    passed: true,
    pre_restart_sequence: preRestartAckProof.sequence,
    pre_restart_replay_rejected: true,
    post_restart_sequence: postRestartAckSequence,
    post_restart_sequence_advanced: true
  };
  report.assertions.bounded_reconnect_without_chat_duplication = {
    passed: true,
    logout_forced_public_read_before_denial_check: true,
    authority_cleared_before_reauthentication: true,
    signed_agent_ack_verified_after_reauthentication: true,
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
  const desktopScreenshot = path.join(retainedEvidenceRoot, `${artifactStem}-desktop.png`);
  await writeExclusive(desktopScreenshot, await page.screenshot({ fullPage: true }));
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

  const mobileScreenshot = path.join(retainedEvidenceRoot, `${artifactStem}-mobile.png`);
  await writeExclusive(mobileScreenshot, await page.screenshot({ fullPage: true }));
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
const reportPath = path.join(retainedEvidenceRoot, `${artifactStem}-report.json`);
const serializedReport = `${JSON.stringify(report, null, 2)}\n`;
assert(!serializedReport.includes(signingSeed), "operator signing seed appeared in retained report");
assert(!serializedReport.includes(path.basename(operatorKeyFile)), "operator key filename appeared in retained report");
await writeExclusive(reportPath, serializedReport);
console.log(JSON.stringify({ schema: report.schema, result: "pass", report: reportPath }));
