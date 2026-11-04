// PVF planning_contract: deterministic bounded local files/Node, no builds/network.
// Release gate: allocation consistency and negative mutations, not product qualification.
import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
const root = 'docs/milestones/';
const read = (v, n) => JSON.parse(fs.readFileSync(root + v + '/' + n, 'utf8'));
const versions = ['v0.93.2', 'v0.93.3', 'v0.93.4', 'v0.93.5', 'v0.93.6', 'v0.93.7'];
const plans = Object.fromEntries(versions.map(v => [v, read(v, `EXECUTION_PLAN_${v}.json`)]));
const allocation = read('v0.93.2', 'REALLOCATION_1296.json');
const split = read('v0.93.3', 'REALLOCATION_1405.json');
const reallocation = read('v0.93.3', 'REALLOCATION_1416.json');
function validate(ps, alloc, routing, currentRouting) {
  const all = {}, streams = {};
  for (const [v, p] of Object.entries(ps)) {
    const ids = p.work_packages.map(w => w.id);
    assert.equal(new Set(ids).size, ids.length);
    assert.equal(ids.length, p.counts.core_work_packages);
    const assigned = {}, numbers = p.sprint_plan.sprints.map(s => s.number);
    assert.ok(numbers.every(n => Number.isInteger(n) && n > 0));
    assert.equal(new Set(numbers).size, numbers.length);
    assert.deepEqual([...numbers].sort((a, b) => a - b), numbers);
    assert.equal(p.sprint_plan.sprint_count, numbers.length);
    // Numbers identify milestone allocation groups. GHB is a separately scheduled
    // stream; it can precede the Beta release tail despite its appended index 6.
    for (const s of p.sprint_plan.sprints) for (const id of s.work_packages) {
      assert.ok(!assigned[id]); assigned[id] = s.number;
      streams[v + '/' + id] = s.stream || 'milestone';
    }
    assert.deepEqual(Object.keys(assigned).sort(), [...ids].sort());
    for (const w of p.work_packages) {
      assert.equal(w.sprint, assigned[w.id]);
      assert.ok(w.result && w.acceptance.length && w.negative_cases.length);
      all[v + '/' + w.id] = w;
    }
    for (const w of p.work_packages) for (const dep of w.depends_on) {
      if (ids.includes(dep) && streams[v + '/' + dep] === streams[v + '/' + w.id])
        assert.ok(assigned[dep] <= w.sprint, `later sprint dependency: ${v}/${w.id}: ${dep}`);
    }
    if (ids.includes('TAIL-10')) for (let i = 2; i <= 10; i++)
      assert.deepEqual(all[v + `/TAIL-${String(i).padStart(2, '0')}`].depends_on, [`TAIL-${String(i - 1).padStart(2, '0')}`]);
  }
  const moved = Object.fromEntries(currentRouting.allocations.map(r => [r.from, r.to]));
  const now = key => moved[key] || key;
  const memo = new Map();
  function ancestors(key, stack = []) {
    assert.ok(!stack.includes(key), 'cycle');
    if (memo.has(key)) return memo.get(key);
    const w = all[key]; assert.ok(w, `missing ${key}`);
    const v = key.split('/')[0], out = new Set();
    for (const dep of w.depends_on) {
      const target = dep.includes('/') ? dep : v + '/' + dep;
      assert.ok(target.split('/')[0] <= v, `forward dependency: ${key}: ${target}`);
      if (!all[target]) {
        assert.ok(ps[v].external_dependencies[dep], `unknown dependency ${key}: ${dep}`); continue;
      }
      // Program/sprint umbrellas collect completion; children cannot depend on
      // their own enclosing completion. Only the milestone tail waits on GHB S1.
      if (/\/GHB-(PROGRAM|S[123])$/.test(target)) assert.equal(key, 'v0.93.2/TAIL-01');
      out.add(target);
      for (const a of ancestors(target, [...stack, key])) out.add(a);
    }
    memo.set(key, out); return out;
  }
  for (const key of Object.keys(all)) ancestors(key);
  for (const v of versions.slice(1)) for (const w of ps[v].work_packages) if (w.id !== 'WP-01' && streams[v + '/' + w.id] !== 'GHB')
    assert.ok(ancestors(v + '/' + w.id).has(v + '/WP-01'), `activation gate omitted: ${v}/${w.id}`);
  const current = ps['v0.93.2'];
  assert.deepEqual(current.opening.execution_authorized, ['WP-01', 'CSDLC-296', 'CSDLC-297', 'CSDLC-298', 'CSDLC-299']);
  assert.equal(current.opening.downstream_execution_authorized, true);
  assert.equal(current.opening.deployment_authorized, false);
  assert.equal(current.counts.distinct_planned_issue_identities, 38);
  assert.deepEqual(current.sidecars.map(x => x.issue), [1275, 304]);
  assert.ok(current.sidecars.every(x => x.execution_authorized === false));
  for (const v of versions.slice(1)) {
    assert.equal(ps[v].status, v === 'v0.93.3' ? 'setup_prepared_execution_not_started' : v === 'v0.93.5' ? 'superseded_vacated_schedule' : 'planned_not_open');
    assert.ok(!ps[v].opening);
    assert.equal(ps[v].predecessor_reconciliation.opening_authorized, false);
  }
  const active = Object.keys(all).filter(k => k.split('/')[0] <= 'v0.93.5');
  assert.equal(currentRouting.original_count, 80);
  assert.equal(currentRouting.active_count, 83);
  assert.equal(currentRouting.allocations.length, 80);
  assert.equal(new Set(currentRouting.allocations.map(r => r.from)).size, 80);
  assert.equal(new Set(currentRouting.allocations.map(r => r.to)).size, 80);
  assert.deepEqual(currentRouting.new_coordination_identities, ['v0.93.2/GHB-S1', 'v0.93.3/GHB-S2', 'v0.93.3/GHB-S3']);
  assert.deepEqual([...currentRouting.allocations.map(r => r.to), ...currentRouting.new_coordination_identities].sort(), active.sort());
  for (const r of currentRouting.allocations) {
    const w = all[r.to], o = r.original;
    assert.equal(r.from.split('/')[1], o.id);
    // The former GHB opening gate becomes whole-program coordination; all
    // substantive requirements, adverse cases and resource limits stay intact.
    if (r.from !== 'v0.93.5/WP-01') for (const a of o.acceptance) assert.ok(w.acceptance.includes(a), `lost ${r.from} acceptance`);
    assert.deepEqual(w.negative_cases, o.negative_cases);
    assert.equal(w.resources, o.resources);
    for (const dep of o.depends_on) {
      const oldKey = dep.includes('/') ? dep : r.from.split('/')[0] + '/' + dep;
      const target = dep === 'WP-01' && r.from.startsWith('v0.93.4/') && r.to.startsWith('v0.93.3/') ? 'v0.93.3/WP-01' : now(oldKey);
      if (oldKey === 'v0.93.5/GHB-SPRINT') continue; // Containment is no longer an execution dependency.
      if (r.from === 'v0.93.5/GHB-SPRINT' || r.from.endsWith('/WP-01') || /\/(TAIL-01|REVIEW|RELEASE|FOUNDATION-REVIEW|FOUNDATION-CLOSE)$/.test(r.from)) continue;
      assert.ok(ancestors(r.to).has(target) || (!all[target] && w.depends_on.includes(dep)), `lost edge ${r.to}: ${target}`);
    }
  }
  // Preserve the prior 1296/1405 registers as historical provenance, while
  // resolving their destinations through the approved 1416 allocation.
  assert.equal(routing.execution_authorized, false);
  assert.equal(routing.allocations.length, 59);
  assert.equal(routing.original_count, 59);
  assert.equal(new Set(routing.allocations.map(r => r.id)).size, 59);
  const dest = Object.fromEntries(routing.allocations.map(r => [r.id, r.to]));
  for (const r of routing.allocations) {
    assert.equal(r.original.id, r.id);
    const w = all[now(r.to + '/' + r.id)]; assert.ok(w);
    if (r.id !== 'WP-01') for (const dep of r.original.depends_on) {
      if (dep === 'GHB-SPRINT') continue; // Removed parent-completion cycle.
      const oldTarget = dep.includes('/') ? dep : dest[dep] ? dest[dep] + '/' + dep : r.to + '/' + dep;
      assert.ok(ancestors(now(r.to + '/' + r.id)).has(now(oldTarget)) || w.depends_on.includes(dep), `lost historical edge ${r.id}: ${dep}`);
    }
    if (r.id !== 'WP-01') for (const field of ['acceptance', 'negative_cases'])
      for (const value of r.original[field]) assert.ok(w[field].includes(value), `lost ${r.id} ${field}`);
  }
  assert.equal(routing.supplemental_allocations.length, 12);
  for (const r of routing.supplemental_allocations) {
    assert.equal(r.to, 'v0.93.7'); assert.equal(r.original.id, r.id);
    for (const field of ['acceptance', 'negative_cases']) assert.deepEqual(all[r.to + '/' + r.id][field], r.original[field]);
    for (const dep of r.original.depends_on) assert.ok(ancestors(r.to + '/' + r.id).has(now(dep.includes('/') ? dep : r.to + '/' + dep)));
  }
  assert.equal(alloc.allocations.length, 67);
  assert.equal(new Set(alloc.allocations.map(x => x.id)).size, 67);
  for (const row of alloc.allocations) {
    const v = row.to === 'v0.93.3' ? dest[row.id] : row.to;
    assert.ok(all[now(v + '/' + row.id)]); assert.equal(row.original.id, row.id);
  }
  assert.equal(alloc.allocations.filter(x => x.to === 'v0.93.3').length, 46);
  const launch = ancestors('v0.93.2/CF-92'); assert.ok(launch.has('v0.93.2/QUALIFY'));
  for (const key of launch) assert.ok(key.startsWith('v0.93.2/') && !/\/(RV-|GHB-|GOV-|CM-|AF-|HU-|CS4-)/.test(key), 'future platform blocks Beta 1 launch');
  for (const id of ['CF-B1-01', 'CF-B1-02', 'CF-B1-03', 'CF-B1-04', 'CF-B1-05', ...Array.from({length: 6}, (_, i) => 'WP-S' + (i + 1))]) assert.ok(launch.has('v0.93.2/' + id));
  const exits = [['v0.93.2', 'TAIL-10', current.work_packages], ['v0.93.3', 'FOUNDATION-CLOSE', ps['v0.93.3'].work_packages.filter(w => w.sprint <= 3)], ['v0.93.4', 'RELEASE', ps['v0.93.4'].work_packages], ['v0.93.6', 'GOV-CLOSE', ps['v0.93.6'].work_packages], ['v0.93.7', 'TAIL-10', ps['v0.93.7'].work_packages]];
  for (const [v, end, rows] of exits) for (const w of rows) if (w.id !== end) assert.ok(ancestors(v + '/' + end).has(v + '/' + w.id), `release omits ${v}/${w.id}`);
  for (const [v, n] of [['v0.93.2', 36], ['v0.93.3', 38], ['v0.93.4', 9], ['v0.93.5', 0], ['v0.93.6', 17], ['v0.93.7', 25]]) assert.equal(ps[v].work_packages.length, n);
  for (const [v, num, exit] of [['v0.93.2', 6, 'GHB-S1'], ['v0.93.3', 4, 'GHB-S2'], ['v0.93.3', 5, 'GHB-S3']]) {
    const s = ps[v].sprint_plan.sprints.find(s => s.number === num);
    assert.equal(s.stream, 'GHB'); assert.equal(s.program_sprint, exit.at(-1) * 1);
    for (const id of s.work_packages) if (id !== exit && id !== 'GHB-PROGRAM') assert.ok(ancestors(v + '/' + exit).has(v + '/' + id), `${exit} omits ${id}`);
  }
  assert.deepEqual(all['v0.93.3/GHB-S2'].depends_on, ['GHB-WP08', 'GHB-WP04']);
  for (const key of ancestors('v0.93.6/GOV-CLOSE')) assert.ok(!key.startsWith('v0.93.7/'), 'affect blocks governance');
  for (const key of ancestors('v0.93.3/GHB-CLOSE')) assert.ok(!key.startsWith('v0.93.4/') && !key.startsWith('v0.93.6/') && !key.startsWith('v0.93.7/'), 'later work blocks GHB');
  assert.ok(ancestors('v0.93.4/RELEASE').has('v0.93.3/CM-04'));
  assert.equal(all['v0.93.2/CF-92'].existing_issue, 92);
  assert.equal(all['v0.93.3/GHB-WP07'].existing_issue, 112);
  assert.equal(all['v0.93.3/ADL-1352'].existing_issue, 1352);
  for (const id of ['WP-S1', 'WP-S2', 'WP-S4', 'WP-S5']) {
    assert.ok(alloc.security_dependency_replacements[id].required);
    assert.ok(all['v0.93.2/' + id].acceptance.some(x => x.includes('revocation') && x.includes('tenant isolation')));
  }
  for (const [prefix, n] of [['AF', 8], ['HU', 4]]) for (let i = 1; i <= n; i++) assert.ok(all[`v0.93.7/${prefix}-${String(i).padStart(2, '0')}`]);
  assert.ok(all['v0.93.7/AF-04'].depends_on.includes('v0.93.3/GHB-WP12'));
  assert.ok(!ancestors('v0.93.3/GHB-WP12').has('v0.93.7/AF-04'));
}
test('approved successor allocation preserves prior scope, dependencies and Beta 1 independence', () => validate(plans, allocation, split, reallocation));
test('mirrors, feature paths and WBS match each current graph', () => {
  for (const [v, p] of Object.entries(plans)) {
    const spec = read(v, `WP_EXECUTION_SPECIFICATIONS_${v}.yaml`), wave = read(v, `WP_ISSUE_WAVE_${v}.yaml`);
    assert.deepEqual(spec.work_packages, p.work_packages);
    assert.deepEqual(spec.sprint_plan, p.sprint_plan); assert.deepEqual(wave.sprint_plan, p.sprint_plan);
    assert.equal(wave.work_packages.length, p.work_packages.length);
    for (const [i, w] of p.work_packages.entries()) {
      const q = wave.work_packages[i];
      for (const [a, b] of [['id', 'wp'], ['result', 'outcome'], ['depends_on', 'depends_on'], ['acceptance', 'acceptance'], ['negative_cases', 'negative_cases'], ['sprint', 'sprint'], ['existing_issue', 'existing_issue']]) assert.deepEqual(w[a], q[b]);
      assert.ok(fs.existsSync(root + v + '/' + w.source));
    }
    const text = fs.readFileSync(root + v + `/WBS_${v}.md`, 'utf8');
    const table = text.split('## Work Packages\n')[1]?.split('\n## ')[0] || '';
    const rows = [...table.matchAll(/^\| ([A-Z0-9-]+) \| ([1-9][0-9]*) \|/gm)];
    assert.equal(rows.length, p.work_packages.length);
    assert.deepEqual(Object.fromEntries(rows.map(m => [m[1], Number(m[2])])), Object.fromEntries(p.work_packages.map(w => [w.id, w.sprint])));
  }
  assert.deepEqual(split.allocations.map(x => x.original), read('v0.93.3', 'REALLOCATION_1403.json').allocations.map(x => x.original));
  const baseline = read('v0.93.2', 'baseline-before-reorganization/EXECUTION_PLAN_v0.93.2.json');
  assert.deepEqual(allocation.allocations.map(x => x.original), baseline.work_packages);
});
test('reject dropped work, weakened acceptance, cycles, early launch and unapproved execution', () => {
  const bad = [
    p => p['v0.93.7'].work_packages.find(w => w.id === 'AF-02').acceptance = ['weakened'],
    p => p['v0.93.6'].work_packages.find(w => w.id === 'GOV-INTEGRATE').depends_on.push('v0.93.7/AF-08'),
    p => p['v0.93.4'].work_packages.find(w => w.id === 'REVIEW').depends_on = ['CS4-06'],
    p => p['v0.93.6'].work_packages.find(w => w.id === 'GOV-06').depends_on = p['v0.93.6'].work_packages.find(w => w.id === 'GOV-06').depends_on.filter(x => x !== 'WP-01'),
    (p, a) => a.allocations.pop(),
    p => p['v0.93.2'].work_packages.find(w => w.id === 'WP-S1').depends_on.push('v0.93.3/GOV-11'),
    p => p['v0.93.2'].work_packages.find(w => w.id === 'CF-92').depends_on = ['ADL-1150'],
    p => p['v0.93.2'].opening.deployment_authorized = true,
    p => p['v0.93.3'].status = 'open',
    p => p['v0.93.2'].work_packages.find(w => w.id === 'WP-S2').acceptance = ['scope only'],
    (p, a, r) => r.allocations.pop(),
    p => p['v0.93.3'].work_packages.find(w => w.id === 'GOV-02').acceptance = ['weaker'],
    p => p['v0.93.3'].work_packages.find(w => w.id === 'GHB-WP12').depends_on.push('v0.93.7/AF-04'),
    p => p['v0.93.3'].work_packages.find(w => w.id === 'FOUNDATION-REVIEW').depends_on = ['RV-08'],
    p => p['v0.93.7'].work_packages.find(w => w.id === 'INTEGRATE').depends_on = p['v0.93.7'].work_packages.find(w => w.id === 'INTEGRATE').depends_on.filter(x => x !== 'HU-04'),
    (p, a, r) => r.execution_authorized = true,
    (p, a, r, c) => c.allocations.pop(),
    p => p['v0.93.3'].work_packages.find(w => w.id === 'GHB-S2').depends_on = ['GHB-WP08'],
    p => p['v0.93.2'].work_packages.find(w => w.id === 'GHB-WP00').depends_on.push('GHB-S1'),
  ];
  const names = ['weaken affect acceptance', 'forward governance dependency', 'drop tooling review prerequisite', 'remove governance activation gate', 'drop original allocation', 'future platform blocks Beta launch', 'drop Beta qualification', 'authorize deployment', 'open successor without approval', 'weaken security acceptance', 'drop historical routing', 'weaken moved governance acceptance', 'later affect blocks GHB', 'omit foundation review inputs', 'omit humour integration', 'authorize historical execution', 'drop current allocation', 'omit WP04 from Sprint2 exit', 'child waits on enclosing sprint'];
  assert.equal(names.length, bad.length);
  for (const [i, mutate] of bad.entries()) {
    const p = structuredClone(plans), a = structuredClone(allocation), r = structuredClone(split), c = structuredClone(reallocation);
    mutate(p, a, r, c); assert.throws(() => validate(p, a, r, c), names[i]);
  }
});
