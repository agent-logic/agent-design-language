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

test("CF-05 checkpoint preserves the complete blocked qualification denominator", () => {
  assert.equal(manifest.schema, "adl.cf05.qualification_checkpoint.v1");
  assert.equal(manifest.issue, 1150);
  assert.equal(manifest.decision, "NOT_QUALIFIED");

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
  assert.deepEqual(counts, { PASS: 21, INCOMPLETE: 3 });
  assert.deepEqual(manifest.original_obligations, {
    pass: 21,
    fail: 0,
    incomplete: 3,
    incomplete_ids: ["Q02", "Q03", "Q19"],
  });
  assert.deepEqual(
    obligations.filter(({ status }) => status === "INCOMPLETE").map(({ id }) => id),
    ["Q02", "Q03", "Q19"],
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
    accepted: 0,
    denominator: 12,
    reason: "global_required_obligations_incomplete",
  });

  const adverse = cells.filter(({ first }) => first === "FAIL");
  assert.deepEqual(adverse.map(cellId), ["local_agent_website/vector/linux"]);
  assert.equal(adverse[0].repeat, "PASS");

  assert.deepEqual(manifest.provider_accounting, {
    actual_posts: 122,
    unknown_outcomes: 0,
    ambiguous_outcomes_replayed: 0,
    operator_amended_ceiling_usd: 70,
    exact_billed_cost_claimed: false,
  });
});

test("CF-05 checkpoint publishes stable identities without private absolute paths", () => {
  const sha256 = /^[0-9a-f]{64}$/;
  for (const [name, digest] of Object.entries(manifest.private_evidence_sha256)) {
    assert.match(digest, sha256, `${name} must be a SHA-256 digest`);
  }
  assert.match(manifest.candidate.installed_codefriend_sha256, sha256);

  assert.doesNotMatch(manifestText, /\/Users\//);
  assert.doesNotMatch(manifestText, /\/Volumes\/FastWork/);
  assert.doesNotMatch(manifestText, /OPENAI_API_KEY|GITHUB_TOKEN|AWS_SECRET_ACCESS_KEY/);
});
