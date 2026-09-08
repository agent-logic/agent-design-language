import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("issue 708 orientation source and Observatory surface are present", async () => {
  const [welcomePackage, observatory] = await Promise.all([
    readFile("docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md", "utf8"),
    readFile("demos/html-observatory/app.js", "utf8"),
  ]);

  assert.match(welcomePackage, /AXIOMA POLIS WELCOME PACKAGE/i);
  assert.match(observatory, /Orientation package/);
  assert.match(observatory, /non-authoritative/);
});

test("Observatory preserves exact per-agent orientation provenance", async () => {
  await import("../app.js");
  const {
    buildRuntimeAgentRows,
    formatAgentOrientation,
    normalizeAgentOrientation,
  } = globalThis.AdlHtmlObservatory;
  const orientation = {
    schema: "adl.runtime_v3.agent_orientation_delivery.v1",
    version: "v1",
    digest_algorithm: "blake3",
    digest: "a".repeat(64),
    source_path: "docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md",
    projection: "full",
  };

  assert.deepEqual(normalizeAgentOrientation(orientation), {
    version: "v1",
    digestAlgorithm: "blake3",
    digest: "a".repeat(64),
    sourcePath: "docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md",
    projection: "full",
  });
  assert.equal(normalizeAgentOrientation({ ...orientation, digest: "not-a-digest" }), null);
  assert.equal(normalizeAgentOrientation({ ...orientation, schema: "wrong.schema" }), null);

  const [agent] = buildRuntimeAgentRows({
    status: {
      schema: "adl.runtime_v3.observatory_feed.v3",
      agent_population: {
        total_count: 1,
        sample: [{
          id: "ember",
          name: "ember.axioma",
          label: "Ember Axioma",
          role: "resident agent",
          state: "ready",
          provider: "ollama",
          model: "gemma4:e4b-mlx",
          communication_eligible: true,
          source_revision: "configured",
          provenance: "runtime_dynamic_admission",
          orientation,
        }],
      },
    },
  });
  assert.equal(agent.orientation.version, "v1");
  assert.equal(agent.orientation.digest, "a".repeat(64));
  assert.match(formatAgentOrientation(agent.orientation), /v1 \/ blake3:aaaaaaaaaaaa \/ full \/ non-authoritative/);
});
