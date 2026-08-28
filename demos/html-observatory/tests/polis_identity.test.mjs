import assert from "node:assert/strict";
import test from "node:test";
import { readFile } from "node:fs/promises";

await import(new URL("../app.js", import.meta.url));
const { projectPolisIdentity, runtimeV3SnapshotFromFeed } = globalThis.AdlHtmlObservatory;

const identity = {
  polis_id: "agent-logic-main",
  display_name: "Agent Logic",
  public_domain: "runtime.agent-logic.ai",
  runtime_api_base: "https://runtime.agent-logic.ai",
  observatory_public_origin: "https://observatory.agent-logic.ai"
};

test("polis identity projection accepts only feed-owned canonical values", () => {
  assert.deepEqual(projectPolisIdentity(identity), {
    polisId: "agent-logic-main",
    displayName: "Agent Logic",
    publicDomain: "runtime.agent-logic.ai",
    runtimeApiBase: "https://runtime.agent-logic.ai",
    observatoryPublicOrigin: "https://observatory.agent-logic.ai"
  });
  for (const invalid of [
    { ...identity, polis_id: "" },
    { ...identity, display_name: " Agent Logic" },
    { ...identity, public_domain: "other.example", runtime_api_base: "https://runtime.agent-logic.ai" },
    { ...identity, observatory_public_origin: "http://observatory.agent-logic.ai" }
  ]) {
    assert.throws(() => projectPolisIdentity(invalid), /invalid Polis identity/);
  }
});

test("HTML has no hard-coded production Polis fallback", async () => {
  const html = await readFile(new URL("../index.html", import.meta.url), "utf8");
  assert.match(html, /id="polis-display-name">Unavailable</);
  assert.doesNotMatch(html, /prod-polis/);
  assert.doesNotMatch(projectPolisIdentity.toString(), /window\.location|location\?\.|URLSearchParams/);
});

test("v3 snapshot requires explicit Polis identity and rejects legacy schemas", () => {
  const feed = {
    schema: "adl.runtime_v3.observatory_feed.v3",
    polis_identity: identity,
    runtime_selection: "runtime_v3_explicit_opt_in",
    runtime_instance_id: "runtime-1",
    runtime_incarnation_id: "incarnation-1",
    health: { snapshot: {}, observability_ready: true },
    agents: { total_count: 0 },
    events: []
  };
  assert.equal(runtimeV3SnapshotFromFeed(feed).polisIdentity.displayName, "Agent Logic");
  assert.throws(
    () => runtimeV3SnapshotFromFeed({ ...feed, schema: "adl.runtime_v3.observatory_feed.v2" }),
    /Unsupported Runtime v3 Observatory schema/
  );
});
