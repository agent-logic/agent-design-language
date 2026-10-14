import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
const root = 'docs/milestones/v0.93.2/';
const files = ['WP_ISSUE_WAVE_v0.93.2.yaml','WP_EXECUTION_SPECIFICATIONS_v0.93.2.yaml','EXECUTION_PLAN_v0.93.2.json'];
test('v0.93.2 graphs agree on the complete allocated scope', () => {
  const graphs = files.map((file) => JSON.parse(fs.readFileSync(root + file, 'utf8')));
  const ids = graphs.map((graph) => graph.work_packages.map((wp) => wp.wp ?? wp.id));
  assert.deepEqual(ids[0], ids[1]); assert.deepEqual(ids[1], ids[2]);
  assert.equal(ids[0].length, 68); assert.equal(new Set(ids[0]).size, 68);
  for (const id of ['CF-92','GHB-SPRINT','GHB-WP00','GHB-WP12']) assert.ok(ids[0].includes(id));
  for (const graph of graphs) {
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
