#!/usr/bin/env node

import { createReadStream } from "node:fs";
import { readFile, realpath, stat } from "node:fs/promises";
import { createServer } from "node:https";
import { extname, join, relative } from "node:path";

const rootInput = process.env.ADL_OBSERVATORY_ROOT;
const runtimeInitPath = process.env.ADL_RUNTIME_INIT_FILE;
const listenAddress = process.env.ADL_OBSERVATORY_LISTEN_ADDRESS || "127.0.0.1";
const listenPort = Number(process.env.ADL_OBSERVATORY_LISTEN_PORT || "18783");

if (!rootInput || !runtimeInitPath) {
  throw new Error("ADL_OBSERVATORY_ROOT and ADL_RUNTIME_INIT_FILE are required");
}
if (!Number.isSafeInteger(listenPort) || listenPort < 1 || listenPort > 65535) {
  throw new Error("ADL_OBSERVATORY_LISTEN_PORT must be a valid TCP port");
}

const root = await realpath(rootInput);
const runtimeInit = await readFile(runtimeInitPath, "utf8");

function sectionValue(sectionName, name) {
  const section = runtimeInit.split(`[${sectionName}]`)[1]?.split(/^\[/m)[0] || "";
  const match = new RegExp(`^${name}\\s*=\\s*"([^"]+)"$`, "m").exec(section);
  if (!match) throw new Error(`missing ${sectionName}.${name}`);
  return match[1];
}

const publicBaseUrl = new URL(sectionValue("api", "public_base_url"));
if (publicBaseUrl.protocol !== "https:") {
  throw new Error("api.public_base_url must use https");
}
const runtimeHostname = publicBaseUrl.hostname;
const cert = await readFile(sectionValue("api.tls", "certificate_chain_path"));
const key = await readFile(sectionValue("api.tls", "private_key_path"));
const contentSecurityPolicy = [
  "default-src 'self'",
  "script-src 'self'",
  "style-src 'self'",
  "img-src 'self' data:",
  `connect-src 'self' https://${runtimeHostname}:* wss://${runtimeHostname}:*`,
  "object-src 'none'",
  "base-uri 'none'",
  "frame-ancestors 'none'",
  "form-action 'none'"
].join("; ");
const contentTypes = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".svg": "image/svg+xml"
};

const server = createServer({ cert, key }, async (request, response) => {
  try {
    const pathname = decodeURIComponent(new URL(request.url, publicBaseUrl).pathname);
    const requested = pathname.endsWith("/") ? `${pathname}index.html` : pathname;
    const candidate = await realpath(join(root, requested));
    const fromRoot = relative(root, candidate);
    if (fromRoot.startsWith("..") || fromRoot === "" || (await stat(candidate)).isFile() === false) {
      throw new Error("not found");
    }
    const headers = {
      "cache-control": "no-store",
      "content-type": contentTypes[extname(candidate)] || "application/octet-stream",
      "x-content-type-options": "nosniff"
    };
    if (extname(candidate) === ".html") {
      headers["content-security-policy"] = contentSecurityPolicy;
    }
    response.writeHead(200, headers);
    createReadStream(candidate).pipe(response);
  } catch {
    response.writeHead(404, {
      "cache-control": "no-store",
      "content-type": "text/plain; charset=utf-8",
      "x-content-type-options": "nosniff"
    });
    response.end("not found\n");
  }
});

server.listen(listenPort, listenAddress, () => {
  process.stderr.write(`HTML Observatory listening on https://${listenAddress}:${listenPort}\n`);
});
for (const signal of ["SIGINT", "SIGTERM"]) {
  process.on(signal, () => server.close(() => process.exit(0)));
}
