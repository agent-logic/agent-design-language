import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const manifestPath = new URL(
  "../../../.csdlc/evidence/1150/QUALIFICATION_EVIDENCE_MANIFEST.json",
  import.meta.url,
);
const manifestText = await readFile(manifestPath, "utf8");
const manifest = JSON.parse(manifestText);

const unique = (values) => new Set(values).size === values.length;
const cellId = ({ surface, repository, platform }) =>
  `${surface}/${repository}/${platform}`;

test("CF-05 checkpoint preserves the complete qualified denominator", () => {
  assert.equal(manifest.schema, "adl.cf05.qualification_checkpoint.v1");
  assert.equal(manifest.issue, 1150);
  assert.equal(manifest.decision, "QUALIFIED");

  const obligations = manifest.obligations;
  assert.equal(obligations.length, 24);
  assert.ok(unique(obligations.map(({ id }) => id)));
  assert.deepEqual(
    obligations.map(({ id }) => id),
    Array.from({ length: 24 }, (_, index) => `Q${String(index + 1).padStart(2, "0")}`),
  );

  const counts = obligations.reduce(
    (result, { status }) => ({ ...result, [status]: (result[status] ?? 0) + 1 }),
    {},
  );
  assert.deepEqual(counts, { PASS: 24 });
  assert.deepEqual(manifest.original_obligations, {
    pass: 24,
    fail: 0,
    incomplete: 0,
    incomplete_ids: [],
  });
  assert.deepEqual(
    obligations.filter(({ status }) => status === "INCOMPLETE").map(({ id }) => id),
    [],
  );

  const cells = manifest.cells;
  assert.equal(cells.length, 12);
  assert.ok(unique(cells.map(cellId)));
  const expectedCells = ["cli", "hosted_website", "local_agent_website"].flatMap(
    (surface) =>
      ["adl", "vector"].flatMap((repository) =>
        ["linux", "macos"].map(
          (platform) => `${surface}/${repository}/${platform}`,
        ),
      ),
  );
  assert.deepEqual(cells.map(cellId).sort(), expectedCells.sort());
  assert.ok(cells.every(({ semantic_status }) => semantic_status === "PASS"));
  assert.deepEqual(manifest.semantic_cells, { pass: 12, fail: 0 });
  assert.deepEqual(manifest.release_accepted_cells, {
    accepted: 12,
    denominator: 12,
    reason: "all_required_obligations_pass",
  });
  assert.deepEqual(manifest.execution_groups, { pass: 23, fail: 1, incomplete: 0 });

  const adverse = cells.filter(({ first }) => first === "FAIL");
  assert.deepEqual(adverse.map(cellId), ["local_agent_website/vector/linux"]);
  assert.equal(adverse[0].repeat, "PASS");

  assert.equal(manifest.provider_accounting.actual_posts, 126);
  assert.equal(manifest.provider_accounting.unknown_outcomes, 0);
  assert.equal(manifest.provider_accounting.ambiguous_outcomes_replayed, 0);
  assert.equal(manifest.provider_accounting.operator_amended_ceiling_usd, 70);
  assert.equal(manifest.provider_accounting.conservative_spend_usd, 64.744);
  assert.equal(manifest.provider_accounting.exact_billed_cost_claimed, false);
  assert.equal(
    manifest.candidate.deployable_product_candidate,
    "aa2cca757ba4e6c37cef6c1a6e013d3896cd1de1",
  );
  assert.equal(
    manifest.candidate.deployable_product_tree,
    "89ca46e84d9f1060c80e47c97704eb9ab369af2a",
  );
  assert.equal(
    manifest.candidate.product_archive_sha256,
    "2bf22aa1d0f354c5a719d26a07c92f0fbd39a890733f4d7336bed80b943c2c0c",
  );
});

test("CF-05 checkpoint publishes stable identities without private absolute paths", () => {
  const sha256 = /^[0-9a-f]{64}$/;
  for (const [name, digest] of Object.entries(manifest.private_evidence_sha256)) {
    assert.match(digest, sha256, `${name} must be a SHA-256 digest`);
  }
  assert.match(manifest.candidate.execution_installed_codefriend_sha256, sha256);
  assert.match(manifest.candidate.deployable_macos_arm64_codefriend_sha256, sha256);
  assert.match(manifest.candidate.deployable_linux_x86_64_codefriend_sha256, sha256);

  assert.doesNotMatch(manifestText, /\/Users\//);
  assert.doesNotMatch(manifestText, /\/Volumes\/FastWork/);
  assert.doesNotMatch(manifestText, /OPENAI_API_KEY|GITHUB_TOKEN|AWS_SECRET_ACCESS_KEY/);
});
