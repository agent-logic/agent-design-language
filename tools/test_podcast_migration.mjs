// PVF: deterministic local migration/reference contract; tiny CPU/filesystem;
// required for this migration, no media/provider/deployment qualification.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync, existsSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
const root=fileURLToPath(new URL('../',import.meta.url));
const read=p=>readFileSync(new URL('../'+p,import.meta.url));
const hash=b=>createHash('sha256').update(b).digest('hex');
const migration=()=>JSON.parse(read('tools/podcast_migration/transition.json'));

test('every removed asset has exact committed destination provenance',()=>{
 const m=migration();
 const bytes=read('tools/podcast_migration/source-manifest.json');
 assert.equal(hash(bytes),m.manifest_sha256);
 const imported=JSON.parse(bytes).entries;
 const byPath=new Map(imported.filter(e=>e.source_kind==='adl_git').map(e=>[e.source_path,e]));
 assert.match(m.destination_commit,/^[a-f0-9]{40}$/);
 assert.equal(m.remote_verified_commit,m.destination_commit);
 assert.equal(m.destination_private,true);
 assert.ok(m.removed.length>=182);
 for(const p of m.removed){
  assert.equal(existsSync(new URL('../'+p,import.meta.url)),false,p);
  const e=byPath.get(p);assert.ok(e,p);
  const data=execFileSync('git',['show',`${m.source_commit}:${p}`],{cwd:root,maxBuffer:70*1024*1024});
  assert.equal(hash(data),e.sha256,p);
 }
});

test('published feed and pending episode retain distinct status',()=>{
 const m=migration();const entries=JSON.parse(read('tools/podcast_migration/source-manifest.json')).entries;
 assert.equal(m.public_feed,'https://agent-logic.ai/podcast/feed.xml');
 assert.equal(m.deployed,false);
 assert.ok(entries.find(e=>e.destination==='demos/podcast/feed.xml'));
 assert.ok(entries.find(e=>e.destination==='production/2026-10-16-ai-goes-to-hawkins/script.md'));
 assert.match(read('docs/repository-split/PODCAST_MIGRATION_1285.md').toString(),/awaits script approval/);
});

test('active ADL entrypoints point at the migration contract',()=>{
 const config=JSON.parse(read('adl/config/validation_lane_selector.public.v1.json'));
 const lane=config.lanes.find(x=>x.id==='podcast_migration_contract');
 assert.equal(lane.run_command,'node --test tools/test_podcast_migration.mjs');
 for(const p of ['adl/tools/test_install_adl_operational_skills.sh','adl/tools/run_v0913_proof_validation_lane.sh']){
  const t=read(p).toString();assert.match(t,/tools\/test_podcast_migration\.mjs/);
  assert.doesNotMatch(t,/adl\/tools\/(?:test_multiagent_podcast_audio_policy|test_podcast_studio_v2_packet|validate_podcast_studio_v2_packet)\.(?:sh|py)/);
 }
 assert.match(read('docs/REPOSITORIES.md').toString(),/agent-logic\/the-cognitive-stack/);
});

test('copied historical podcast evidence stays byte-identical',()=>{
 const entries=JSON.parse(read('tools/podcast_migration/source-manifest.json')).entries;
 const historical=entries.filter(e=>e.source_kind==='adl_git'&&e.source_path.startsWith('docs/milestones/'));
 assert.ok(historical.length>0);
 for(const e of historical) assert.equal(hash(read(e.source_path)),e.sha256,e.source_path);
});

test('retirement transition rejects changed or resurrected migrated inputs',()=>{
 const script=`import importlib.util,json,subprocess
from pathlib import Path
root=Path.cwd()
spec=importlib.util.spec_from_file_location('guard',root/'tools/podcast_migration/guard.py')
g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g)
m=json.loads((root/'tools/podcast_migration/transition.json').read_text())
base={r['path']:r['before'] for r in m['changes']}
candidate={r['path']:r['after'] for r in m['changes'] if r['after'] is not None}
assert g.normalize_candidate(root,candidate,base)==base
for row in m['changes']:
 bad=dict(candidate);bad[row['path']]=['100644','0'*40]
 try:g.normalize_candidate(root,bad,base)
 except ValueError:pass
 else:raise AssertionError('changed migration input accepted: '+row['path'])
print('transition negative cases passed')`;
 assert.match(execFileSync('python3',['-B','-c',script],{cwd:root,encoding:'utf8'}),/negative cases passed/);
});
