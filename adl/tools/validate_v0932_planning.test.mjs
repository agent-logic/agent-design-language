// PVF: deterministic planning-contract proof; bounded local files/Node, no builds/network.
// Release gate: allocation consistency only; never product qualification.
import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
const root='docs/milestones/';
const read=(v,n)=>JSON.parse(fs.readFileSync(root+v+'/'+n,'utf8'));
const versions=['v0.93.2','v0.93.3'];
const plans=Object.fromEntries(versions.map(v=>[v,read(v,`EXECUTION_PLAN_${v}.json`)]));
const allocation=read('v0.93.2','REALLOCATION_1296.json');
function validate(ps,alloc){
 const all={};
 for(const [v,p] of Object.entries(ps)){
  const ids=p.work_packages.map(w=>w.id);assert.equal(new Set(ids).size,ids.length);assert.equal(ids.length,p.counts.core_work_packages);
  const assigned={};for(const [i,s] of p.sprint_plan.sprints.entries()){assert.equal(s.number,i+1);for(const id of s.work_packages){assert.ok(!assigned[id]);assigned[id]=s.number;}}
  assert.deepEqual(Object.keys(assigned).sort(),[...ids].sort());
  for(const w of p.work_packages){assert.equal(w.sprint,assigned[w.id]);assert.ok(w.result&&w.acceptance.length&&w.negative_cases.length);all[v+'/'+w.id]=w;}
  for(const w of p.work_packages)for(const dep of w.depends_on){if(ids.includes(dep))assert.ok(assigned[dep]<=w.sprint);else assert.ok(p.external_dependencies[dep],`${v}/${w.id}: ${dep}`);}
  for(let i=2;i<=10;i++)assert.deepEqual(p.work_packages.find(w=>w.id===`TAIL-${String(i).padStart(2,'0')}`).depends_on,[`TAIL-${String(i-1).padStart(2,'0')}`]);
 }
 function ancestors(key,stack=[]){assert.ok(!stack.includes(key),'cycle');const w=all[key];assert.ok(w);const v=key.split('/')[0],out=new Set();for(const dep of w.depends_on){const target=dep.includes('/')?dep:v+'/'+dep;if(!all[target])continue;out.add(target);for(const a of ancestors(target,[...stack,key]))out.add(a);}return out;}
 for(const key of Object.keys(all))ancestors(key);
 const current=ps['v0.93.2'];assert.deepEqual(current.opening.execution_authorized,['WP-01','CSDLC-296','CSDLC-297','CSDLC-298','CSDLC-299']);assert.equal(current.opening.downstream_execution_authorized,true);assert.equal(current.opening.deployment_authorized,false);assert.equal(ps['v0.93.3'].status,'planned_not_open');assert.ok(!ps['v0.93.3'].opening);assert.equal(ps['v0.93.3'].predecessor_reconciliation.status,'pending_v0932_acceptance');assert.equal(ps['v0.93.3'].predecessor_reconciliation.opening_authorized,false);assert.equal(current.counts.core_work_packages,31);assert.equal(current.counts.distinct_planned_issue_identities,33);assert.deepEqual(current.sidecars.map(x=>x.issue),[1275,304]);assert.ok(current.sidecars.every(x=>x.execution_authorized===false));
 assert.equal(alloc.allocations.length,67);assert.equal(new Set(alloc.allocations.map(x=>x.id)).size,67);
 for(const row of alloc.allocations){assert.ok(all[row.to+'/'+row.id]);assert.equal(row.original.id,row.id);if(row.to==='v0.93.3')assert.ok(!all['v0.93.2/'+row.id]);}
 assert.equal(alloc.allocations.filter(x=>x.to==='v0.93.3').length,46);
 const launch=ancestors('v0.93.2/CF-92');assert.ok(launch.has('v0.93.2/QUALIFY'));
 for(const key of launch)assert.ok(!key.startsWith('v0.93.3/')&&!/\/(RV-|GHB-|GOV-|CM-)/.test(key),'future platform blocks launch');
 for(const id of ['CF-B1-01','CF-B1-02','CF-B1-03','CF-B1-04','CF-B1-05',...Array.from({length:6},(_,i)=>'WP-S'+(i+1))])assert.ok(launch.has('v0.93.2/'+id));
 for(const key of ['v0.93.2/TAIL-10','v0.93.3/TAIL-10']){const required=ancestors(key);for(const w of ps[key.split('/')[0]].work_packages)if(w.id!=='TAIL-10')assert.ok(required.has(key.split('/')[0]+'/'+w.id),`release omits ${w.id}`);}
 assert.equal(all['v0.93.2/CF-92'].existing_issue,92);assert.equal(all['v0.93.3/GHB-WP07'].existing_issue,112);
 for(const id of ['WP-S1','WP-S2','WP-S4','WP-S5']){assert.ok(alloc.security_dependency_replacements[id].required);assert.ok(all['v0.93.2/'+id].acceptance.some(x=>x.includes('revocation')&&x.includes('tenant isolation')));}
 assert.ok(!Object.keys(all).some(x=>x.endsWith('/PY-01')));
}
test('current allocation retains all prior identities without coupling Beta 1 to platform evolution',()=>validate(plans,allocation));
test('mirrors, feature paths and WBS match each current graph',()=>{
 for(const [v,p] of Object.entries(plans)){
  const spec=read(v,`WP_EXECUTION_SPECIFICATIONS_${v}.yaml`),wave=read(v,`WP_ISSUE_WAVE_${v}.yaml`);
  assert.deepEqual(spec.work_packages,p.work_packages);assert.deepEqual(spec.sprint_plan,p.sprint_plan);assert.deepEqual(wave.sprint_plan,p.sprint_plan);
  assert.equal(wave.work_packages.length,p.work_packages.length);
  for(const [i,w] of p.work_packages.entries()){
   const q=wave.work_packages[i];for(const [a,b] of [['id','wp'],['result','outcome'],['depends_on','depends_on'],['acceptance','acceptance'],['negative_cases','negative_cases'],['sprint','sprint'],['existing_issue','existing_issue']])assert.deepEqual(w[a],q[b]);
   assert.ok(fs.existsSync(root+v+'/'+w.source));
  }
  const text=fs.readFileSync(root+v+`/WBS_${v}.md`,'utf8');const rows=[...text.matchAll(/^\| ([A-Z0-9-]+) \| ([1-5]) \|/gm)];assert.equal(rows.length,p.work_packages.length);assert.deepEqual(Object.fromEntries(rows.map(m=>[m[1],Number(m[2])])),Object.fromEntries(p.work_packages.map(w=>[w.id,w.sprint])));
 }
 const baseline=read('v0.93.2','baseline-before-reorganization/EXECUTION_PLAN_v0.93.2.json');assert.deepEqual(allocation.allocations.map(x=>x.original),baseline.work_packages);
});
test('reject lost work, future dependencies, launch before qualification and unapproved execution',()=>{
 const bad=[(p,a)=>a.allocations.pop(),(p)=>p['v0.93.2'].work_packages.find(w=>w.id==='WP-S1').depends_on.push('v0.93.3/GOV-11'),(p)=>p['v0.93.2'].work_packages.find(w=>w.id==='CF-92').depends_on=['ADL-1150'],(p)=>p['v0.93.2'].opening.deployment_authorized=true,(p)=>p['v0.93.3'].status='open',(p)=>p['v0.93.2'].work_packages.find(w=>w.id==='WP-S2').acceptance=['scope only']];
 for(const mutate of bad){const p=structuredClone(plans),a=structuredClone(allocation);mutate(p,a);assert.throws(()=>validate(p,a));}
});
