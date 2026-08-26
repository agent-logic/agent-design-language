#!/usr/bin/env bash
set -euo pipefail

node <<'NODE'
const fs = require("fs");
const path = require("path");
const vm = require("vm");

const root = process.cwd();
const appPath = path.join(root, "demos/html-observatory/app.js");
const handoffPath = path.join(root, ".csdlc/evidence/271/authentic-handler-output.json");
const source = fs.readFileSync(appPath, "utf8");
const handoff = JSON.parse(fs.readFileSync(handoffPath, "utf8"));
const context = {
  console,
  URL,
  Blob: class Blob {},
  setInterval() { return 0; },
  clearInterval() {},
  setTimeout(callback) { if (typeof callback === "function") callback(); return 0; },
  clearTimeout() {}
};
context.globalThis = context;
vm.createContext(context);
vm.runInContext(source, context, { filename: appPath });

const api = context.AdlHtmlObservatory;
if (!api) throw new Error("Observatory API was not exported");

const schema = "adl.runtime_v3.layer8.recipient_acknowledgement_response.v1";
if (handoff.schema !== "adl.issue271.authentic_handler_output_handoff.v1") {
  throw new Error("invalid handler-output handoff schema");
}
if (handoff.source?.runtime_route !== "/v1/layer8/recipient-acknowledgement") {
  throw new Error("handler-output handoff does not bind the #270 recipient-acknowledgement route");
}
if (handoff.source?.response_schema !== schema) {
  throw new Error("handler-output handoff does not bind the Runtime response schema");
}
const fixtureByCase = Object.fromEntries(
  (handoff.public_response_fixtures || []).map((fixture) => [fixture.case, fixture.response])
);
const base = {
  schema,
  recipient_id: "shepherd",
  correlation_hash: "sha256:abcdef0123456789",
  acknowledgement_message_id: "ack-message-1"
};

const cases = [
  ["delivered", { ...fixtureByCase.delivered, action_released: false }, "delivered"],
  ["signed_refusal", fixtureByCase.signed_refusal, "refused"],
  ["malformed_response_failure", { schema: "wrong", status: "delivered", correlation_hash: "sha256:bad" }, "failed"],
  ["unavailable_runtime_recovery", { runtime_unavailable: true, error: "runtime_unavailable" }, "recovery"],
  ["revoked_demotion", { ...base, status: "revoked", error: "credential_revoked" }, "revoked"],
  ["action_release", { ...fixtureByCase.delivered, action_released: true }, "delivered"],
  ["keyboard_live_region_accessibility", fixtureByCase.signed_refusal, "refused"],
  ["forbidden_field_non_disclosure", { ...base, status: "delivered", signed_request: { signature: "secret" }, correlation_id: "raw-correlation" }, "failed"]
];

if (cases.length !== 8) throw new Error(`expected exactly eight cases, got ${cases.length}`);

const outcomes = cases.map(([name, input, expected]) => {
  const row = api.normalizeLayer8DeliveryState(input);
  if (row.state !== expected) {
    throw new Error(`${name} expected ${expected}, got ${row.state}`);
  }
  const rendered = JSON.stringify(row);
  for (const forbidden of ["raw-correlation", "secret", "signed_request", "signature"]) {
    if (rendered.includes(forbidden)) {
      throw new Error(`${name} disclosed forbidden material: ${forbidden}`);
    }
  }
  return { name, state: row.state, terminal: row.terminal, actionEnabled: row.actionEnabled };
});

if (!outcomes.some((item) => item.name === "delivered" && item.terminal)) {
  throw new Error("delivered case did not prove terminal delivery");
}
if (!outcomes.some((item) => item.name === "unavailable_runtime_recovery" && !item.terminal)) {
  throw new Error("recovery case did not remain non-terminal");
}
if (!outcomes.some((item) => item.name === "action_release" && item.actionEnabled)) {
  throw new Error("action release case did not enable action release");
}

const elements = new Map();
function element(id = "") {
  if (!elements.has(id)) {
    const item = {
      id,
      dataset: {},
      attributes: {},
      children: [],
      textContent: "",
      _innerHTML: "",
      setAttribute(name, value) { this.attributes[name] = String(value); },
      append(child) { this.children.push(child); }
    };
    Object.defineProperty(item, "innerHTML", {
      get() { return this._innerHTML; },
      set(value) {
        this._innerHTML = String(value);
        if (this._innerHTML.includes('id="layer8-delivery-count"')) element("layer8-delivery-count");
        if (this._innerHTML.includes('id="layer8-delivery-list"')) {
          const list = element("layer8-delivery-list");
          const match = this._innerHTML.match(/id="layer8-delivery-list"[^>]*aria-live="([^"]+)"/);
          if (match) list.attributes["aria-live"] = match[1];
        }
      }
    });
    elements.set(id, item);
  }
  return elements.get(id);
}
element("root");
context.document = {
  querySelector(selector) { return selector === ".ops-command" ? element("root") : null; },
  getElementById(id) { return elements.get(id) || null; },
  createElement(tag) { const created = element(`created-${tag}-${elements.size}`); created.tagName = tag; return created; }
};

const renderedRows = api.renderLayer8DeliveryPanel(cases.map(([, input]) => input));
if (renderedRows.length !== 8) throw new Error(`rendered ${renderedRows.length} rows instead of eight`);
const list = element("layer8-delivery-list");
if (list.attributes["aria-live"] !== "polite") {
  throw new Error("app-created Layer 8 list is missing aria-live=polite");
}
const count = element("layer8-delivery-count");
if (count.textContent !== "8 states") {
  throw new Error(`app-created Layer 8 count text mismatch: ${count.textContent}`);
}
if (!list.innerHTML.includes("Delivered") || !list.innerHTML.includes("Signed refusal")) {
  throw new Error("rendered Layer 8 panel is missing visible delivery/refusal states");
}
for (const forbidden of ["raw-correlation", "secret", "signed_request", "signature"]) {
  if (list.innerHTML.includes(forbidden)) {
    throw new Error(`rendered DOM disclosed forbidden material: ${forbidden}`);
  }
}

console.log(JSON.stringify({
  schema: "adl.issue271.layer8_observatory_ui.validation.v1",
  status: "passed",
  cases: outcomes,
  endpoint: api.LAYER8_RECIPIENT_ACK_ENDPOINT
}));
NODE
