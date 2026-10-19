import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
const root = 'docs/milestones/v0.93.2/';
const files = ['WP_ISSUE_WAVE_v0.93.2.yaml','WP_EXECUTION_SPECIFICATIONS_v0.93.2.yaml','EXECUTION_PLAN_v0.93.2.json'];
test('v0.93.2 graphs agree on the complete allocated scope', () => {
  const graphs = files.map((file) => JSON.parse(fs.readFileSync(root + file, 'utf8')));
  const ids = graphs.map((graph) => graph.work_packages.map((wp) => wp.wp ?? wp.id));
  assert.deepEqual(ids[0], ids[1]); assert.deepEqual(ids[1], ids[2]);
  assert.equal(ids[0].length, 67); assert.equal(new Set(ids[0]).size, 67);
  for (const id of ['CF-92','GHB-SPRINT','GHB-WP00','GHB-WP12']) assert.ok(ids[0].includes(id));
  const successor = JSON.parse(fs.readFileSync(root + 'tail07-1242/SUCCESSOR_PLAN.json', 'utf8'));
  const python = successor.deferred.find((row) => row.id === 'PY-01');
  assert.equal(python.original.id, 'PY-01');
  assert.ok(python.original.acceptance.length > 0);
  for (const graph of graphs) {
    assert.ok(!graph.work_packages.some((wp) => (wp.wp ?? wp.id) === 'PY-01'));
    assert.ok(graph.work_packages.every((wp) => !wp.depends_on.includes('PY-01')));
    assert.ok(graph.sprint_plan.sprints.every((sprint) => !sprint.work_packages.includes('PY-01')));
    const byId = Object.fromEntries(graph.work_packages.map((wp) => [wp.wp ?? wp.id, wp]));
    assert.ok(byId.QUALIFY.depends_on.includes('GHB-WP12'));
    assert.ok(byId.QUALIFY.depends_on.includes('CF-92'));
    assert.equal(byId['GHB-WP12'].sprint, 4);
    assert.equal(byId['CF-92'].sprint, 5);
    const finalOrder = graph.sprint_plan.sprints[4].work_packages;
    assert.ok(finalOrder.indexOf('CF-92') < finalOrder.indexOf('QUALIFY'));
    for (const dep of ['ADL-1150','CodeFriend-45','CodeFriend-46','codefriend.ai-21']) assert.ok(graph.external_dependencies[dep]);
  }
});

test('opening WBS covers each allocated identity once and preserves the sprint assignment', () => {
  const wave = JSON.parse(fs.readFileSync(root + files[0], 'utf8'));
  const table = fs.readFileSync(root + 'WBS_v0.93.2.md', 'utf8');
  const rows = [...table.matchAll(/^\| ([A-Z0-9-]+) \| ([1-5]) \|/gm)].map((m) => [m[1], Number(m[2])]);
  assert.equal(rows.length, 67);
  assert.equal(new Set(rows.map(([id]) => id)).size, 67);
  assert.deepEqual(Object.fromEntries(rows), Object.fromEntries(wave.work_packages.map((w) => [w.wp, w.sprint])));
  assert.deepEqual(wave.opening.execution_authorized, ['WP-01']);
  assert.equal(wave.opening.downstream_execution_authorized, false);
  assert.equal(wave.opening.deployment_authorized, false);
});
