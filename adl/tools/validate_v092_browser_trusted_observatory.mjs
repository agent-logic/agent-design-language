#!/usr/bin/env node

import { createHash, X509Certificate } from "node:crypto";
import { createReadStream, promises as fs } from "node:fs";
import { createServer } from "node:https";
import { createRequire } from "node:module";
import { dirname, extname, isAbsolute, join, normalize, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { spawn } from "node:child_process";
import { connect as tlsConnect } from "node:tls";

const PLAYWRIGHT_VERSION = "1.60.0";
const EVIDENCE_SCHEMA = "adl.v092.browser_trusted_observatory.evidence.v1";
const FASTWORK_ROOT = "/Volumes/FastWork";
const CONCURRENT_RUNTIME_CONNECTIONS = 50;
const SCOPED_PRODUCT_PATHS = [
  "adl-runtime/src/local_tls.rs",
  "adl-runtime/src/bin/adl-runtime-local-tls-bootstrap.rs",
  "adl-runtime/tests/local_tls.rs",
  "demos/html-observatory/runtime-v3.config.json",
  "demos/html-observatory/README.md",
  "adl/tools/validate_v092_browser_trusted_observatory.mjs",
];
const args = parseArgs(process.argv.slice(2));

if (args.nativePlatforms && !args.requireTrustedTls) {
  emit({
    schema: EVIDENCE_SCHEMA,
    mode: "platform_disposition",
    platforms: platformDispositions(args.nativePlatforms),
  });
  process.exit(0);
}

if (!args.requireTrustedTls) {
  fail("--require-trusted-tls is required for live proof");
}

const repoRoot = await canonicalExistingFastWorkPath(
  resolve(dirname(new URL(import.meta.url).pathname), "../.."),
  "repository root",
);
const certificate = await requiredPath("ADL_V092_TLS_CERT");
const privateKey = await requiredPath("ADL_V092_TLS_KEY");
const runtimeCommand = parseRuntimeCommand(process.env.ADL_V092_RUNTIME_COMMAND_JSON);
const runtimeGuardian = await requiredPath("ADL_V092_RUNTIME_GUARDIAN");
const runtimeKernel = await requiredPath("ADL_V092_RUNTIME_KERNEL");
const runtimeBootstrap = await requiredPath("ADL_V092_TLS_BOOTSTRAP");
const runtimeBootstrapConfig = await requiredPath("ADL_V092_TLS_BOOTSTRAP_CONFIG");
const exactBuild = await buildExactHeadBinaries({
  repoRoot,
  guardian: runtimeGuardian,
  kernel: runtimeKernel,
  bootstrap: runtimeBootstrap,
});
const tlsBootstrap = await verifyDurableTlsBootstrap({
  bootstrap: runtimeBootstrap,
  config: runtimeBootstrapConfig,
  certificate,
  privateKey,
});
const runtimeTlsPaths = await verifyRuntimeTlsPaths({
  command: runtimeCommand,
  certificate: tlsBootstrap.certificate_path,
  privateKey: tlsBootstrap.private_key_path,
});
if (await fs.realpath(runtimeCommand[0]) !== runtimeGuardian) {
  fail("Runtime candidate command must launch the declared Guardian binary directly");
}
const evidencePath = args.evidence ? await canonicalFastWorkOutput(args.evidence) : null;
const sourceBefore = await gitScopedIdentity(repoRoot);
const observatory = new URL(args.observatoryUrl);
const runtime = new URL(args.runtimeUrl);
assertHttpsLocalhost(observatory, "Observatory");
assertHttpsLocalhost(runtime, "Runtime");
if (observatory.port === runtime.port) {
  fail("Observatory and Runtime must remain separate HTTPS listeners");
}

const tlsErrors = [];
let staticServer;
let runtimeProcess;
let runtimeProcessId;
let runtimeOwnership;
let browser;

try {
  runtimeProcess = startRuntime(runtimeCommand, repoRoot);
  staticServer = await startStaticServer({
    certificate,
    privateKey,
    hostname: observatory.hostname,
    port: Number(observatory.port),
    root: repoRoot,
  });
  await waitForTrustedEndpoint(new URL("/v1/health", runtime), certificate, runtimeProcess);
  await waitForRuntimeReady(new URL("/v1/ready", runtime), certificate, runtimeProcess);
  const listenerCertificateSha256 = {
    observatory: await peerCertificateSha256(observatory, certificate),
    runtime: await peerCertificateSha256(runtime, certificate),
  };
  if (new Set(Object.values(listenerCertificateSha256)).size !== 1
      || listenerCertificateSha256.runtime !== await certificateDerSha256(certificate)) {
    fail("all HTTPS listeners must present the one current manifest certificate identity");
  }
  runtimeOwnership = await proveRuntimeCandidateOwnership({
    feed: await curlTrustedJson(new URL("/v1/observatory", runtime), certificate),
    guardian: runtimeGuardian,
    guardianProcess: runtimeProcess,
    kernel: runtimeKernel,
    runtime,
  });
  runtimeProcessId = runtimeOwnership.runtime_pid;

  const { chromium, version } = await loadPinnedPlaywright();
  browser = await chromium.launch({ channel: args.browser, headless: true });
  const context = await browser.newContext({ ignoreHTTPSErrors: false });
  const page = await context.newPage();
  const dashboardRuntimeRequests = [];
  if (runtime.origin !== "https://localhost:20997") {
    await page.route("https://localhost:20997/**", async (route) => {
      const requested = new URL(route.request().url());
      const candidate = new URL(`${requested.pathname}${requested.search}`, runtime);
      dashboardRuntimeRequests.push({ requested: requested.href, candidate: candidate.href });
      await route.continue({ url: candidate.href });
    });
  }
  page.on("requestfailed", (request) => {
    const text = request.failure()?.errorText ?? "request failed";
    if (isTlsError(text)) tlsErrors.push(`network:${request.url()}:${text}`);
  });

  const dashboard = new URL("/demos/html-observatory/", observatory);
  dashboard.searchParams.set("runtime", "v3");
  dashboard.searchParams.set("runtimeApiBase", "https://localhost:20997");
  const response = await page.goto(dashboard.href, { waitUntil: "domcontentloaded" });
  if (!response || !response.ok()) fail(`Observatory HTML returned ${response?.status() ?? "no response"}`);
  const title = await page.title();
  if (/privacy error|not secure|certificate/i.test(title)) {
    fail(`browser certificate interstitial detected: ${title}`);
  }
  await page.waitForFunction(
    () => document.querySelector("#live-status")?.textContent?.trim() === "live loopback",
    null,
    { timeout: 10_000 },
  );
  const dashboardLiveStatus = await page.locator("#live-status").textContent();
  const dashboardRuntimePaths = new Set(
    dashboardRuntimeRequests.map(({ requested }) => new URL(requested).pathname),
  );
  for (const path of ["/v1/ready", "/v1/observatory"]) {
    if (runtime.origin !== "https://localhost:20997" && !dashboardRuntimePaths.has(path)) {
      fail(`Observatory dashboard did not request ${path} from the isolated Runtime candidate`);
    }
  }

  const browserEndpoints = await page.evaluate(async ({ base }) => {
    const paths = ["/v1/ready", "/v1/observatory"];
    return Promise.all(paths.map(async (path) => {
      try {
        const response = await fetch(new URL(path, base));
        const body = await response.json();
        return { path, status: response.status, ok: response.ok, body };
      } catch (error) {
        return { path, status: 0, ok: false, error: String(error) };
      }
    }));
  }, { base: runtime.href });
  for (const endpoint of browserEndpoints) {
    if (!endpoint.ok) fail(`browser Runtime request failed for ${endpoint.path}: ${endpoint.error ?? endpoint.status}`);
  }
  const ready = browserEndpoints.find((entry) => entry.path === "/v1/ready");
  if (ready?.body?.ready !== true) fail("Runtime readiness response did not report ready=true");
  const observatoryFeed = browserEndpoints.find((entry) => entry.path === "/v1/observatory")?.body;
  const browserRuntimeOwnership = await proveRuntimeCandidateOwnership({
    feed: observatoryFeed,
    guardian: runtimeGuardian,
    guardianProcess: runtimeProcess,
    kernel: runtimeKernel,
    runtime,
  });
  if (browserRuntimeOwnership.runtime_pid !== runtimeProcessId) {
    fail("Runtime process identity changed between readiness and browser proof");
  }

  const healthPage = await context.newPage();
  const browserHealth = await healthPage.goto(new URL("/v1/health", runtime).href, {
    waitUntil: "domcontentloaded",
  });
  if (!browserHealth || !browserHealth.ok()) {
    fail(`browser Runtime health navigation returned ${browserHealth?.status() ?? "no response"}`);
  }
  await healthPage.close();
  if (tlsErrors.length) fail(`browser reported TLS errors: ${tlsErrors.join("; ")}`);

  const curlEndpoints = [
    dashboard,
    new URL("/v1/health", runtime),
    new URL("/v1/ready", runtime),
    new URL("/v1/observatory", runtime),
  ];
  for (const endpoint of curlEndpoints) await curlTrusted(endpoint, certificate);
  const concurrentRuntimeProof = await proveConcurrentTrustedConnections(
    new URL("/v1/health", runtime),
    certificate,
    CONCURRENT_RUNTIME_CONNECTIONS,
  );

  const source = await gitScopedIdentity(repoRoot);
  if (source.head !== sourceBefore.head || source.scoped_tree_sha256 !== sourceBefore.scoped_tree_sha256) {
    fail("scoped product identity changed during validation");
  }
  const evidence = {
    schema: EVIDENCE_SCHEMA,
    status: "pass",
    head: source.head,
    source,
    playwright_version: version,
    browser: args.browser,
    tls_verification: "required",
    certificate_sha256: await certificateDerSha256(certificate),
    listeners: {
      observatory: `${observatory.protocol}//${observatory.hostname}:${observatory.port}`,
      runtime: `${runtime.protocol}//${runtime.hostname}:${runtime.port}`,
    },
    listener_certificate_sha256: listenerCertificateSha256,
    browser_endpoints: browserEndpoints.map(({ path, status }) => ({ path, status })),
    browser_direct_health_status: browserHealth.status(),
    dashboard_live_status: dashboardLiveStatus?.trim(),
    dashboard_runtime_requests: dashboardRuntimeRequests,
    curl_endpoints: curlEndpoints.map((endpoint) => endpoint.pathname),
    concurrent_runtime_connections: concurrentRuntimeProof,
    runtime_candidate: runtimeOwnership,
    runtime_tls_paths: runtimeTlsPaths,
    exact_head_build: exactBuild,
    tls_bootstrap: tlsBootstrap,
    platforms: platformDispositions(args.nativePlatforms ?? [process.platform]),
  };
  if (evidencePath) await writeEvidence(evidencePath, evidence);
  emit(evidence);
} finally {
  if (browser) await browser.close();
  if (staticServer) await new Promise((done) => staticServer.close(done));
  await terminateRuntimeCandidate(runtimeProcess, runtimeOwnership, runtimeKernel);
}

function parseArgs(argv) {
  const parsed = {
    browser: "chrome",
    runtimeUrl: "https://localhost:20997",
    observatoryUrl: "https://localhost:8765",
    requireTrustedTls: false,
    evidence: null,
    nativePlatforms: null,
  };
  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    const value = () => {
      index += 1;
      if (index >= argv.length) fail(`${argument} requires a value`);
      return argv[index];
    };
    if (argument === "--browser") parsed.browser = value();
    else if (argument === "--runtime-url") parsed.runtimeUrl = value();
    else if (argument === "--observatory-url") parsed.observatoryUrl = value();
    else if (argument === "--evidence") parsed.evidence = value();
    else if (argument === "--require-trusted-tls") parsed.requireTrustedTls = true;
    else if (argument === "--require-native-platform-evidence") {
      parsed.nativePlatforms = value().split(",").map((entry) => entry.trim());
    } else fail(`unknown argument: ${argument}`);
  }
  return parsed;
}

async function loadPinnedPlaywright() {
  const require = createRequire(import.meta.url);
  const modulePath = await canonicalExistingFastWorkPath(
    process.env.ADL_PLAYWRIGHT_MODULE || require.resolve("playwright"),
    "Playwright module",
  );
  const moduleRequire = createRequire(modulePath);
  const packagePath = await canonicalExistingFastWorkPath(
    moduleRequire.resolve("playwright/package.json"),
    "Playwright package",
  );
  const packageJson = JSON.parse(await fs.readFile(packagePath, "utf8"));
  if (packageJson.version !== PLAYWRIGHT_VERSION) {
    fail(`Playwright ${PLAYWRIGHT_VERSION} is required; found ${packageJson.version}`);
  }
  const playwright = await import(pathToFileURL(modulePath));
  return { chromium: playwright.chromium, version: packageJson.version };
}

async function buildExactHeadBinaries({ repoRoot, guardian, kernel, bootstrap }) {
  const expected = {
    guardian: await fs.realpath(join(repoRoot, "adl-runtime/target/debug/adl-runtime-guardian")),
    kernel: await fs.realpath(join(repoRoot, "adl-runtime-kernel/target/debug/adl-runtime-kernel")),
    bootstrap: await fs.realpath(join(repoRoot, "adl-runtime/target/debug/adl-runtime-local-tls-bootstrap")),
  };
  if (guardian !== expected.guardian || kernel !== expected.kernel || bootstrap !== expected.bootstrap) {
    fail("Runtime proof binaries must be the exact issue-worktree Cargo outputs");
  }
  const commands = [
    ["cargo", "build", "--locked", "--manifest-path", "adl-runtime/Cargo.toml", "--bin", "adl-runtime-guardian", "--bin", "adl-runtime-local-tls-bootstrap"],
    ["cargo", "build", "--locked", "--manifest-path", "adl-runtime-kernel/Cargo.toml", "--bin", "adl-runtime-kernel"],
  ];
  for (const [executable, ...argv] of commands) await runChecked(executable, argv, repoRoot);
  return {
    head: (await capture("git", ["-C", repoRoot, "rev-parse", "HEAD"])).trim(),
    commands: commands.map((command) => command.join(" ")),
    guardian_sha256: await fileSha256(guardian),
    kernel_sha256: await fileSha256(kernel),
    bootstrap_sha256: await fileSha256(bootstrap),
  };
}

async function verifyDurableTlsBootstrap({ bootstrap, config, certificate, privateKey }) {
  const output = await capture(bootstrap, ["--config", config, "--operation", "bootstrap"]);
  let outcome;
  try {
    outcome = JSON.parse(output);
  } catch (error) {
    fail(`TLS bootstrap returned invalid JSON: ${error.message}`);
  }
  if (outcome.manifest_durable !== true) fail("TLS bootstrap manifest is not durably synchronized");
  if (await fs.realpath(outcome.certificate_chain_path) !== certificate
      || await fs.realpath(outcome.private_key_path) !== privateKey) {
    fail("TLS proof paths do not match the durable current-generation manifest");
  }
  const marker = `${join("tls", "generations")}/`;
  const markerIndex = certificate.lastIndexOf(marker);
  if (markerIndex < 0) fail("TLS certificate is not inside a generated manifest identity");
  const tlsRoot = certificate.slice(0, markerIndex + "tls".length);
  const manifestPath = join(tlsRoot, "current-generation.json");
  const manifest = JSON.parse(await fs.readFile(manifestPath, "utf8"));
  const manifestCertificate = await fs.realpath(join(tlsRoot, manifest.certificate_chain_path));
  const manifestPrivateKey = await fs.realpath(join(tlsRoot, manifest.private_key_path));
  const certificateSha256 = (await certificateDerSha256(certificate)).toUpperCase();
  if (manifestCertificate !== certificate
      || manifestPrivateKey !== privateKey
      || manifest.certificate_sha256 !== certificateSha256
      || outcome.certificate_sha256 !== certificateSha256) {
    fail("TLS manifest, bootstrap outcome, and supplied identity do not match");
  }
  return {
    manifest_durable: true,
    manifest_path: manifestPath,
    manifest_sha256: await fileSha256(manifestPath),
    certificate_sha256: certificateSha256.toLowerCase(),
    certificate_path: manifestCertificate,
    private_key_path: manifestPrivateKey,
    bootstrap_sha256: await fileSha256(bootstrap),
    bootstrap_config_sha256: await fileSha256(config),
  };
}

async function verifyRuntimeTlsPaths({ command, certificate, privateKey }) {
  const initMarkers = command
    .map((part, index) => part === "--init" ? index : -1)
    .filter((index) => index >= 0);
  if (initMarkers.length !== 1 || !command[initMarkers[0] + 1]) {
    fail("Runtime candidate command must contain exactly one --init <path> argument");
  }
  const initPath = await canonicalExistingFastWorkPath(
    command[initMarkers[0] + 1],
    "Runtime init",
  );
  const init = await fs.readFile(initPath, "utf8");
  const configuredCertificate = await canonicalExistingFastWorkPath(
    tomlSectionString(init, "api.tls", "certificate_chain_path"),
    "Runtime TLS certificate",
  );
  const configuredPrivateKey = await canonicalExistingFastWorkPath(
    tomlSectionString(init, "api.tls", "private_key_path"),
    "Runtime TLS private key",
  );
  if (configuredCertificate !== certificate || configuredPrivateKey !== privateKey) {
    fail("Runtime init TLS paths do not match the one durable manifest identity");
  }
  return {
    init_path: initPath,
    certificate_path: configuredCertificate,
    private_key_path: configuredPrivateKey,
    matches_manifest: true,
  };
}

function tomlSectionString(document, section, key) {
  let currentSection = null;
  const matches = [];
  for (const rawLine of document.split(/\r?\n/u)) {
    const line = rawLine.trim();
    const sectionMatch = /^\[([^\]]+)\]$/u.exec(line);
    if (sectionMatch) {
      currentSection = sectionMatch[1].trim();
      continue;
    }
    if (currentSection !== section || !line || line.startsWith("#")) continue;
    const valueMatch = new RegExp(`^${key}\\s*=\\s*("(?:[^"\\\\]|\\\\.)*")\\s*(?:#.*)?$`, "u").exec(line);
    if (valueMatch) matches.push(JSON.parse(valueMatch[1]));
  }
  if (matches.length !== 1 || typeof matches[0] !== "string" || !matches[0]) {
    fail(`Runtime init must declare exactly one string ${section}.${key}`);
  }
  return matches[0];
}

async function runChecked(executable, argv, cwd) {
  const child = spawn(executable, argv, { cwd, env: process.env, stdio: ["ignore", "ignore", "pipe"] });
  let stderr = "";
  child.stderr.on("data", (chunk) => { stderr += chunk; });
  const code = await onceExit(child);
  if (code !== 0) fail(`${executable} failed with status ${code}: ${stderr.trim()}`);
}

function startRuntime(command, cwd) {
  const child = spawn(command[0], command.slice(1), {
    cwd,
    detached: true,
    env: process.env,
    stdio: ["ignore", "pipe", "pipe"],
  });
  child.stdout.on("data", () => {});
  child.stderr.on("data", () => {});
  child.on("error", (error) => fail(`Runtime candidate launch failed: ${error.message}`));
  return child;
}

async function startStaticServer({ certificate, privateKey, hostname, port, root }) {
  const server = createServer(
    { cert: await fs.readFile(certificate), key: await fs.readFile(privateKey) },
    async (request, response) => {
      try {
        const pathname = decodeURIComponent(new URL(request.url, "https://localhost").pathname);
        const requested = pathname.endsWith("/") ? `${pathname}index.html` : pathname;
        const target = normalize(join(root, requested));
        if (relative(root, target).startsWith("..")) throw new Error("path escaped root");
        const stat = await fs.stat(target);
        if (!stat.isFile()) throw new Error("not a file");
        response.writeHead(200, { "content-type": contentType(target) });
        createReadStream(target).pipe(response);
      } catch {
        response.writeHead(404, { "content-type": "text/plain" });
        response.end("not found\n");
      }
    },
  );
  await new Promise((resolveListen, rejectListen) => {
    server.once("error", (error) => rejectListen(new Error(`HTTPS static listener refused ${hostname}:${port}: ${error.message}`)));
    server.listen(port, hostname, resolveListen);
  });
  return server;
}

async function waitForTrustedEndpoint(url, certificate, child) {
  const deadline = Date.now() + 30000;
  while (Date.now() < deadline) {
    if (child.exitCode !== null) fail(`Runtime candidate exited early with status ${child.exitCode}`);
    try {
      await curlTrusted(url, certificate);
      return;
    } catch {
      await delay(250);
    }
  }
  fail(`Runtime candidate did not become healthy at ${url.pathname}`);
}

async function waitForRuntimeReady(url, certificate, child) {
  const deadline = Date.now() + 30000;
  let lastObservation = "no readiness response";
  while (Date.now() < deadline) {
    if (child.exitCode !== null) fail(`Runtime candidate exited early with status ${child.exitCode}`);
    try {
      const readiness = await curlTrustedJson(url, certificate);
      lastObservation = JSON.stringify(readiness);
      if (readiness.ready === true) return;
    } catch (error) {
      lastObservation = error.message;
    }
    await delay(250);
  }
  fail(`Runtime candidate did not become ready: ${lastObservation}`);
}

async function curlTrusted(url, certificate) {
  const child = spawn("curl", ["--fail", "--silent", "--show-error", "--cacert", certificate, url.href], {
    stdio: ["ignore", "ignore", "pipe"],
  });
  let stderr = "";
  child.stderr.on("data", (chunk) => { stderr += chunk; });
  const code = await onceExit(child);
  if (code !== 0) throw new Error(`curl verified probe failed for ${url.pathname}: ${stderr.trim()}`);
}

async function curlTrustedJson(url, certificate) {
  const child = spawn("curl", ["--fail", "--silent", "--show-error", "--cacert", certificate, url.href], {
    stdio: ["ignore", "pipe", "pipe"],
  });
  let stdout = "";
  let stderr = "";
  child.stdout.on("data", (chunk) => { stdout += chunk; });
  child.stderr.on("data", (chunk) => { stderr += chunk; });
  const code = await onceExit(child);
  if (code !== 0) throw new Error(`curl verified probe failed for ${url.pathname}: ${stderr.trim()}`);
  try {
    return JSON.parse(stdout);
  } catch (error) {
    throw new Error(`curl verified probe returned invalid JSON for ${url.pathname}: ${error.message}`);
  }
}

async function proveConcurrentTrustedConnections(url, certificate, count) {
  const ca = await fs.readFile(certificate);
  const expectedPeerSha256 = new X509Certificate(ca).fingerprint256.replaceAll(":", "").toLowerCase();
  const sessions = await Promise.all(Array.from({ length: count }, (_, index) => new Promise((resolveSession, rejectSession) => {
    const socket = tlsConnect({
      host: url.hostname,
      port: Number(url.port),
      ca,
      rejectUnauthorized: true,
      servername: "localhost",
      ALPNProtocols: ["http/1.1"],
    });
    socket.setTimeout(10_000, () => socket.destroy(new Error(`connection ${index + 1} timed out`)));
    socket.once("secureConnect", () => {
      const peerCertificate = socket.getPeerCertificate();
      const peerSha256 = peerCertificate.raw
        ? createHash("sha256").update(peerCertificate.raw).digest("hex")
        : null;
      const authorized = socket.authorized === true;
      if (!authorized || peerSha256 !== expectedPeerSha256) {
        socket.destroy();
        rejectSession(new Error(
          `connection ${index + 1} failed TLS verification: authorized=${authorized} peer=${peerSha256}`,
        ));
        return;
      }
      resolveSession({ index, socket, peerSha256 });
    });
    socket.once("error", rejectSession);
  })));
  const simultaneousOpenConnections = sessions.filter(({ socket }) => !socket.destroyed).length;
  if (simultaneousOpenConnections !== count) {
    sessions.forEach(({ socket }) => socket.destroy());
    fail(`only ${simultaneousOpenConnections} of ${count} TLS sessions remained open at the barrier`);
  }
  const results = await Promise.all(sessions.map(({ index, socket, peerSha256 }) => new Promise((resolveResponse, rejectResponse) => {
    let response = "";
    socket.on("data", (chunk) => { response += chunk; });
    socket.once("end", () => {
      const status = Number(response.match(/^HTTP\/1\.[01] (\d{3})/m)?.[1] ?? 0);
      if (status < 200 || status >= 300) {
        rejectResponse(new Error(`connection ${index + 1} returned HTTP ${status}`));
        return;
      }
      resolveResponse({ peerSha256, status });
    });
    socket.once("error", rejectResponse);
    socket.write(`GET ${url.pathname}${url.search} HTTP/1.1\r\nHost: localhost:${url.port}\r\nConnection: close\r\n\r\n`);
  })));
  return {
    requested: count,
    completed: results.length,
    simultaneous_open_connections_at_barrier: simultaneousOpenConnections,
    certificate_verification: "required",
    connection_reuse: "disabled",
    peer_certificate_sha256: expectedPeerSha256,
    statuses: [...new Set(results.map((result) => result.status))],
  };
}

async function proveRuntimeCandidateOwnership({ feed, guardian, guardianProcess, kernel, runtime }) {
  const runtimePid = Number(feed?.runtime_process_id);
  if (!Number.isSafeInteger(runtimePid) || runtimePid <= 0) {
    fail("Runtime observatory feed did not expose a valid runtime_process_id");
  }
  const parentPid = Number((await capture("/bin/ps", ["-p", String(runtimePid), "-o", "ppid="])).trim());
  if (parentPid !== guardianProcess.pid) {
    fail(`Runtime process ${runtimePid} is not a direct child of Guardian ${guardianProcess.pid}`);
  }
  const observedKernel = await fs.realpath(
    (await capture("/bin/ps", ["-p", String(runtimePid), "-o", "comm="])).trim(),
  );
  const processStartedAt = (await capture("/bin/ps", ["-p", String(runtimePid), "-o", "lstart="])).trim();
  if (observedKernel !== kernel) {
    fail(`Runtime process ${runtimePid} executable does not match the declared kernel`);
  }
  const listener = await capture("/usr/sbin/lsof", [
    "-nP",
    "-a",
    "-p",
    String(runtimePid),
    `-iTCP:${runtime.port}`,
    "-sTCP:LISTEN",
    "-Fpn",
  ]);
  if (!listener.split("\n").includes(`p${runtimePid}`) || !listener.includes(`:${runtime.port}`)) {
    fail(`Runtime process ${runtimePid} does not own the declared HTTPS listener`);
  }
  return {
    guardian_pid: guardianProcess.pid,
    guardian_sha256: await fileSha256(guardian),
    kernel_sha256: await fileSha256(kernel),
    runtime_pid: runtimePid,
    runtime_parent_pid: parentPid,
    runtime_process_started_at: processStartedAt,
    runtime_listener_port: Number(runtime.port),
    runtime_process_matches_declared_kernel: true,
  };
}

async function capture(executable, argv) {
  const child = spawn(executable, argv, { stdio: ["ignore", "pipe", "pipe"] });
  let stdout = "";
  let stderr = "";
  child.stdout.on("data", (chunk) => { stdout += chunk; });
  child.stderr.on("data", (chunk) => { stderr += chunk; });
  const code = await onceExit(child);
  if (code !== 0) fail(`${executable} failed with status ${code}: ${stderr.trim()}`);
  return stdout;
}

async function terminateRuntimeCandidate(guardianProcess, ownership, kernel) {
  if (guardianProcess) {
    signalProcessGroup(guardianProcess.pid, "SIGTERM");
    await Promise.race([onceExit(guardianProcess), delay(5000)]);
    if (guardianProcess.exitCode === null) signalProcessGroup(guardianProcess.pid, "SIGKILL");
  }
  const runtimePid = ownership?.runtime_pid;
  if (!runtimePid) return;
  const exited = await waitForPidExit(runtimePid, 5000);
  if (exited) return;
  const currentKernel = await tryCapture("/bin/ps", ["-p", String(runtimePid), "-o", "comm="]);
  const currentStartedAt = await tryCapture("/bin/ps", ["-p", String(runtimePid), "-o", "lstart="]);
  if (!currentKernel
      || await fs.realpath(currentKernel.trim()) !== kernel
      || currentStartedAt?.trim() !== ownership.runtime_process_started_at) {
    fail(`Runtime candidate PID ${runtimePid} identity changed before cleanup; refusing to signal it`);
  }
  process.kill(runtimePid, "SIGTERM");
  if (await waitForPidExit(runtimePid, 3000)) return;
  process.kill(runtimePid, "SIGKILL");
  if (!(await waitForPidExit(runtimePid, 3000))) {
    fail(`verified Runtime candidate process ${runtimePid} survived cleanup`);
  }
}

function signalProcessGroup(pid, signal) {
  try {
    process.kill(-pid, signal);
  } catch (error) {
    if (error.code !== "ESRCH") throw error;
  }
}

async function tryCapture(executable, argv) {
  const child = spawn(executable, argv, { stdio: ["ignore", "pipe", "pipe"] });
  let stdout = "";
  child.stdout.on("data", (chunk) => { stdout += chunk; });
  return (await onceExit(child)) === 0 ? stdout : null;
}

async function waitForPidExit(pid, timeoutMilliseconds) {
  const deadline = Date.now() + timeoutMilliseconds;
  while (Date.now() < deadline) {
    try {
      process.kill(pid, 0);
    } catch (error) {
      if (error.code === "ESRCH") return true;
      throw error;
    }
    await delay(100);
  }
  return false;
}

async function peerCertificateSha256(url, certificate) {
  const ca = await fs.readFile(certificate);
  return new Promise((resolvePeer, rejectPeer) => {
    const socket = tlsConnect({
      host: url.hostname,
      port: Number(url.port),
      ca,
      rejectUnauthorized: true,
      servername: "localhost",
    });
    socket.setTimeout(10_000, () => socket.destroy(new Error(`TLS probe timed out for ${url.origin}`)));
    socket.once("secureConnect", () => {
      const peer = socket.getPeerCertificate();
      const digest = peer.raw ? createHash("sha256").update(peer.raw).digest("hex") : null;
      socket.end();
      if (!socket.authorized || !digest) {
        rejectPeer(new Error(`TLS listener ${url.origin} did not present an authorized certificate`));
        return;
      }
      resolvePeer(digest);
    });
    socket.once("error", rejectPeer);
  });
}

async function certificateDerSha256(path) {
  const certificate = new X509Certificate(await fs.readFile(path));
  return createHash("sha256").update(certificate.raw).digest("hex");
}

async function fileSha256(path) {
  return createHash("sha256").update(await fs.readFile(path)).digest("hex");
}

function parseRuntimeCommand(text) {
  if (!text) fail("ADL_V092_RUNTIME_COMMAND_JSON must contain the isolated Runtime candidate argv array");
  let command;
  try { command = JSON.parse(text); } catch { fail("ADL_V092_RUNTIME_COMMAND_JSON is not valid JSON"); }
  if (!Array.isArray(command) || command.length === 0 || command.some((part) => typeof part !== "string" || !part)) {
    fail("ADL_V092_RUNTIME_COMMAND_JSON must be a non-empty string argv array");
  }
  return command;
}

async function requiredPath(name) {
  const value = process.env[name];
  if (!value) fail(`${name} is required`);
  return canonicalExistingFastWorkPath(value, name);
}

async function canonicalExistingFastWorkPath(value, label) {
  let canonical;
  try {
    canonical = await fs.realpath(resolve(value));
  } catch (error) {
    fail(`${label} could not be resolved: ${error.message}`);
  }
  await assertCanonicalFastWorkPath(canonical, label);
  return canonical;
}

async function canonicalFastWorkOutput(value) {
  const requested = resolve(value);
  let ancestor = requested;
  while (true) {
    let exists = false;
    try {
      await fs.lstat(ancestor);
      exists = true;
    } catch (error) {
      if (error.code !== "ENOENT") fail(`--evidence parent could not be resolved: ${error.message}`);
    }
    if (exists) {
      let canonicalAncestor;
      try {
        canonicalAncestor = await fs.realpath(ancestor);
      } catch (error) {
        fail(`--evidence parent could not be resolved: ${error.message}`);
      }
      await assertCanonicalFastWorkPath(canonicalAncestor, "--evidence parent");
      if (ancestor === requested) return canonicalAncestor;
      const target = resolve(canonicalAncestor, relative(ancestor, requested));
      await assertCanonicalFastWorkPath(target, "--evidence");
      return target;
    }
    const parent = dirname(ancestor);
    if (parent === ancestor) fail("--evidence has no existing parent");
    ancestor = parent;
  }
}

async function assertCanonicalFastWorkPath(path, label) {
  const canonicalRoot = await fs.realpath(FASTWORK_ROOT);
  const contained = relative(canonicalRoot, path);
  if (!contained || contained.startsWith("..") || isAbsolute(contained)) {
    fail(`${label} must resolve beneath ${canonicalRoot}`);
  }
}

function assertHttpsLocalhost(url, label) {
  if (url.protocol !== "https:" || url.hostname !== "localhost" || !url.port) {
    fail(`${label} URL must be an explicit https://localhost:<port> URL`);
  }
}

function platformDispositions(platforms) {
  const normalized = platforms.map((platform) => platform === "darwin" ? "macos" : platform);
  const supported = {
    macos: { status: "supported", trust: "explicit user-keychain security(1) transaction" },
    linux: { status: "blocked", reason: "Chrome NSS trust and system curl trust are separate stores; no single reversible native transaction is implemented" },
    windows: { status: "blocked", reason: "CurrentUser Root import/removal has not received native Windows execution proof" },
  };
  return normalized.map((platform) => ({ platform, ...(supported[platform] ?? { status: "blocked", reason: "unknown platform" }) }));
}

function contentType(path) {
  return ({ ".html": "text/html; charset=utf-8", ".js": "text/javascript; charset=utf-8", ".json": "application/json", ".css": "text/css; charset=utf-8", ".svg": "image/svg+xml" })[extname(path)] ?? "application/octet-stream";
}

async function gitScopedIdentity(cwd) {
  const status = await gitCapture(cwd, [
    "status",
    "--porcelain=v1",
    "--untracked-files=all",
    "--",
    ...SCOPED_PRODUCT_PATHS,
  ]);
  if (status.trim()) {
    fail(`scoped product paths are dirty; commit them before retaining exact-head evidence: ${status.trim().split("\n").join(", ")}`);
  }
  const head = (await gitCapture(cwd, ["rev-parse", "HEAD"])).trim();
  const tree = await gitCapture(cwd, [
    "ls-tree",
    "-r",
    "--full-tree",
    head,
    "--",
    ...SCOPED_PRODUCT_PATHS,
  ]);
  const entries = tree.trim().split("\n").filter(Boolean);
  if (entries.length !== SCOPED_PRODUCT_PATHS.length) {
    fail("exact-head tree does not contain every scoped product path");
  }
  return {
    head,
    scoped_paths: SCOPED_PRODUCT_PATHS,
    scoped_tree_sha256: createHash("sha256").update(tree).digest("hex"),
  };
}

async function gitCapture(cwd, args) {
  const child = spawn("git", args, { cwd, stdio: ["ignore", "pipe", "pipe"] });
  let stdout = "";
  let stderr = "";
  child.stdout.on("data", (chunk) => { stdout += chunk; });
  child.stderr.on("data", (chunk) => { stderr += chunk; });
  if (await onceExit(child) !== 0) fail(`git ${args[0]} failed: ${stderr.trim()}`);
  return stdout;
}

async function writeEvidence(path, evidence) {
  await fs.mkdir(dirname(path), { recursive: true });
  await fs.writeFile(path, `${JSON.stringify(evidence, null, 2)}\n`, { mode: 0o600 });
}

function isTlsError(text) {
  return /certificate|cert_|ssl|tls|net::err_cert|authority_invalid/i.test(text);
}

function onceExit(child) {
  if (child.exitCode !== null) return Promise.resolve(child.exitCode);
  return new Promise((resolveExit) => child.once("exit", (code) => resolveExit(code ?? 1)));
}

function delay(milliseconds) {
  return new Promise((resolveDelay) => setTimeout(resolveDelay, milliseconds));
}

function emit(value) {
  process.stdout.write(`${JSON.stringify(value, null, 2)}\n`);
}

function fail(message) {
  throw new Error(message);
}
