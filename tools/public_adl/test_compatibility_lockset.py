"""Synthetic schema/refusal fixtures, never product qualification or acceptance."""
import copy
import json
import io
import tarfile
import hashlib
from pathlib import Path
import tempfile
import unittest
import verify_compatibility_lockset as v

DOC=Path(__file__).resolve().parents[2]/'docs/milestones/v0.93.1/repository-decomposition/rd11-1193'
class Lockset(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory();self.addCleanup(self.temp.cleanup);self.root=Path(self.temp.name)
        self.receipt=self.root/'fixture.json';self.receipt.write_text('{"fixture_only":true}\n')
        self.ref={'path':'fixture.json','sha256':v.digest(self.receipt)}
        self.lock=v.load(DOC/'candidate-lockset.json');self.graph=v.load(DOC/'interface-graph.json');self.rollback=v.load(DOC/'rollback-catalog.json')
    def qualified_fixture(self):
        # Structural fixture only: normalized claims cannot authenticate the external receipts.
        self.lock['state']='accepted';self.lock['rd07_acceptance']=self.ref;self.lock['independent_review']=self.ref
        self.lock['negative_evidence']=[{'case':name,'observed_refusal':'synthetic fixture refusal','executed':1,'evidence':self.ref} for name in sorted(v.NEGATIVES)]
        for row in self.lock['roles']:
            row['selected']={'source_commit':'a'*40,'asset_identity':{'name':'fixture:'+row['role'],'path':'fixture.json'},'sha256':self.ref['sha256'],'package_version':'0.1.0','dependency_lock':self.ref,'build_provenance':self.ref,'supported_consumer_versions':['=0.1.0']}
            row['active_source_owners']=['fixture-owner'];row['authentication']=self.ref
            if row['role']=='website':row['website_disposition']='preserved_no_deployed_baseline'
            else:row['platforms']=[{'target':'fixture-platform','source_commit':'a'*40,'artifact_sha256':self.ref['sha256'],'independent_clean':True,'distributable_only':True,'operations':[{'operation':op,'argv':['fixture',op],'exit_code':0,'executed':1,'skipped':0} for op in ['build','test','install']],'evidence':self.ref}]
        for edge in self.graph['edges']:edge.update(interface=edge['interface'] or 'fixture-interface',version='1',evidence=self.ref)
        for row in self.rollback['roles']:
            row.update(disposition='no_prior_accepted_baseline',prior=None,restore_argv=[],recoverability=self.ref)
        for disposition in self.graph.get('dependency_dispositions',[]):
            selected=next(row for row in self.lock['roles'] if row['role']==disposition['consumer'])['selected']
            authentication=next(row for row in self.lock['roles'] if row['role']==disposition['consumer'])['authentication']
            disposition.update(
                selected_source_commit=selected['source_commit'],
                selected_artifact_sha256=selected['sha256'],
                dependency_lock=selected['dependency_lock'],
                build_provenance=selected['build_provenance'],
                authentication=authentication,
                evidence=self.ref,
            )
    def candidate_fixture(self):
        # Preserve the original all-pending structural fixture independently of
        # the increasingly complete real candidate loaded from DOC.
        for row in self.lock['roles']:
            row.update(selected=None,active_source_owners=[],platforms=[],authentication=None)
        self.graph['schema']='adl.compatibility_graph.v1'
        self.graph.pop('dependency_dispositions',None)
        if not any((edge['consumer'],edge['producer'],edge['kind'],edge['interface'])==('codefriend','runtime','build','runtime-source') for edge in self.graph['edges']):
            self.graph['edges'].append({
                'consumer':'codefriend','producer':'runtime','kind':'build',
                'interface':'runtime-source','version':'0.92.1','optional':False,
                'evidence':self.ref,
            })
    def check(self):return v.verify(self.lock,self.graph,self.rollback,self.root)
    def independent_codefriend_fixture(self):
        self.qualified_fixture()
        key=('codefriend','runtime','build','runtime-source')
        self.graph['schema']='adl.compatibility_graph.v2'
        self.graph['edges']=[edge for edge in self.graph['edges'] if tuple(edge[name] for name in ('consumer','producer','kind','interface'))!=key]
        selected=next(row for row in self.lock['roles'] if row['role']=='codefriend')['selected']
        authentication=next(row for row in self.lock['roles'] if row['role']=='codefriend')['authentication']
        self.graph['dependency_dispositions']=[{
            'schema':'adl.compatibility_dependency_disposition.v1',
            'consumer':'codefriend','producer':'runtime','kind':'build','interface':'runtime-source',
            'disposition':'independently_built_no_dependency',
            'selected_source_commit':selected['source_commit'],
            'selected_artifact_sha256':selected['sha256'],
            'dependency_lock':selected['dependency_lock'],
            'build_provenance':selected['build_provenance'],
            'authentication':authentication,
            'evidence':self.ref,
        }]
    def test_candidate_preserves_seven_pending_roles(self):
        self.candidate_fixture()
        result=self.check();self.assertEqual(len(result['pending_selections']),7);self.assertFalse(result['acceptance_authority'])
        with self.assertRaises(ValueError):v.verify(self.lock,self.graph,self.rollback,self.root,True)
    def test_complete_synthetic_correspondence_never_grants_authority(self):
        self.qualified_fixture();result=self.check();self.assertEqual(result['status'],'qualified_evidence_correspondence');self.assertFalse(result['acceptance_authority']);self.assertFalse(result['activation_performed'])
    def test_independently_built_codefriend_disposition(self):
        self.independent_codefriend_fixture();result=self.check();self.assertEqual(result['status'],'qualified_evidence_correspondence');self.assertFalse(result['acceptance_authority'])
    def test_independent_disposition_requires_evidence(self):
        self.independent_codefriend_fixture();self.graph['dependency_dispositions'][0]['evidence']=None
        with self.assertRaisesRegex(ValueError,'dependency disposition evidence missing'):self.check()
    def test_independent_disposition_matches_selected_identity_and_sources(self):
        for field,value,message in (
            ('selected_artifact_sha256','0'*64,'selected identity mismatch'),
            ('dependency_lock',{'path':'other.json','sha256':'0'*64},'source declaration mismatch'),
        ):
            self.independent_codefriend_fixture();self.graph['dependency_dispositions'][0][field]=value
            with self.subTest(field=field),self.assertRaisesRegex(ValueError,message):self.check()
    def test_missing_product_proof(self):
        self.qualified_fixture();self.lock['roles'][0]['authentication']=None
        with self.assertRaises(ValueError):self.check()
    def test_mixed_active_owners(self):
        self.lock['roles'][0]['active_source_owners']=['a','b']
        with self.assertRaises(ValueError):self.check()
    def test_unqualified_shared_activation(self):
        self.lock['roles'][0]['activation']='install-shared'
        with self.assertRaises(ValueError):self.check()
    def test_duplicate_or_missing_role(self):
        self.lock['roles'][-1]=copy.deepcopy(self.lock['roles'][0])
        with self.assertRaises(ValueError):self.check()
    def test_duplicate_selected_asset_even_at_another_transport_path(self):
        self.qualified_fixture()
        first,second=self.lock['roles'][:2]
        second['selected']=copy.deepcopy(first['selected'])
        with self.assertRaisesRegex(ValueError,'duplicate selected asset'):self.check()
        (self.root/'copy.json').write_bytes(self.receipt.read_bytes())
        second['selected']['asset_identity']['path']='copy.json'
        with self.assertRaisesRegex(ValueError,'duplicate selected asset'):self.check()
    def bundled_fixture(self):
        self.qualified_fixture();archive=self.root/'bundle.tar'
        with tarfile.open(archive,'w') as out:
            for name in ('engine.tar','records.tar'):
                data=name.encode();info=tarfile.TarInfo(name);info.size=len(data);out.addfile(info,io.BytesIO(data))
        for row,name in zip(self.lock['roles'][:2],('engine.tar','records.tar')):
            row['selected']['asset_identity']={'name':'fixture-bundle:1','path':'bundle.tar','member':{'path':name,'sha256':hashlib.sha256(name.encode()).hexdigest()}}
            row['selected']['sha256']=v.digest(archive);row['platforms'][0]['artifact_sha256']=v.digest(archive)
    def test_distinct_verified_members_of_same_bundle(self):
        self.bundled_fixture();self.assertFalse(self.check()['acceptance_authority'])
    def test_duplicate_bundle_member(self):
        self.bundled_fixture();a,b=self.lock['roles'][:2];b['selected']=copy.deepcopy(a['selected'])
        with self.assertRaisesRegex(ValueError,'duplicate selected asset'):self.check()
    def test_invented_or_corrupt_bundle_member(self):
        for change in ({'path':'missing.tar'},{'sha256':'0'*64}):
            self.bundled_fixture();self.lock['roles'][0]['selected']['asset_identity']['member'].update(change)
            with self.subTest(change=change),self.assertRaisesRegex(ValueError,'bundle member'):self.check()
    def test_whole_bundle_overlaps_member(self):
        self.bundled_fixture();del self.lock['roles'][0]['selected']['asset_identity']['member']
        with self.assertRaisesRegex(ValueError,'whole bundle overlaps'):self.check()
    def test_each_concrete_interface_is_required(self):
        self.qualified_fixture();original_edges=copy.deepcopy(self.graph['edges']);original_dispositions=copy.deepcopy(self.graph.get('dependency_dispositions',[]))
        for obligation in v.REQUIRED_INTERFACES:
            self.graph['edges']=[e for e in copy.deepcopy(original_edges) if (e['consumer'],e['producer'],e['kind'],e['interface'])!=obligation]
            self.graph['dependency_dispositions']=[d for d in copy.deepcopy(original_dispositions) if (d['consumer'],d['producer'],d['kind'],d['interface'])!=obligation]
            with self.subTest(obligation=obligation),self.assertRaisesRegex(ValueError,'interface/kind obligation'):self.check()
    def test_required_kind_cannot_be_downgraded_or_optional(self):
        self.qualified_fixture();original_edges=copy.deepcopy(self.graph['edges']);original_dispositions=copy.deepcopy(self.graph.get('dependency_dispositions',[]))
        for obligation in v.REQUIRED_INTERFACES:
            for change in ({'kind':'runtime'},{'optional':True}):
                self.graph['edges']=copy.deepcopy(original_edges);self.graph['dependency_dispositions']=copy.deepcopy(original_dispositions)
                surfaces=[e for e in self.graph['edges'] if (e['consumer'],e['producer'],e['kind'],e['interface'])==obligation]
                surfaces += [d for d in self.graph['dependency_dispositions'] if (d['consumer'],d['producer'],d['kind'],d['interface'])==obligation]
                self.assertEqual(len(surfaces),1);surfaces[0].update(change)
                with self.subTest(obligation=obligation,change=change),self.assertRaises(ValueError):self.check()
    def test_artifact_platform_mismatch(self):
        self.qualified_fixture();self.lock['roles'][0]['platforms'][0]['artifact_sha256']='c'*64
        with self.assertRaises(ValueError):self.check()
    def test_zero_skipped_and_failed_operation(self):
        for changes in [{'executed':0},{'skipped':1},{'exit_code':1}]:
            self.qualified_fixture();self.lock['roles'][0]['platforms'][0]['operations'][1].update(changes)
            with self.subTest(changes=changes),self.assertRaises(ValueError):self.check()
    def test_single_adapter_invocation_with_phase_outcomes(self):
        self.qualified_fixture();row=self.lock['roles'][0]
        adapter_row={
            'target':'fixture-platform','source_commit':'a'*40,'artifact_sha256':self.ref['sha256'],
            'independent_clean':True,'distributable_only':True,
            'adapter_invocation':{'argv':['fixture','adapter'],'exit_code':0,'executed':1,'skipped':0},
            'phase_outcomes':[{'phase':phase,'executed':1,'failed':0,'skipped':0,'claim':'fixture phase'} for phase in ('build','test','install')],
            'evidence':self.ref,
        }
        row['platforms']=[copy.deepcopy(adapter_row)]
        self.assertEqual(self.check()['status'],'qualified_evidence_correspondence')
        for field,value in (('executed',2),('executed',True),('skipped',1),('skipped',False),('exit_code',1),('exit_code',False)):
            self.qualified_fixture();self.lock['roles'][0]['platforms']=[copy.deepcopy(adapter_row)]
            self.lock['roles'][0]['platforms'][0]['adapter_invocation'][field]=value
            with self.subTest(field=field),self.assertRaisesRegex(ValueError,'exactly one successful invocation'):self.check()
        for field,value in (('executed',0),('executed',True),('failed',1),('failed',False),('skipped',1),('skipped',False)):
            self.qualified_fixture();self.lock['roles'][0]['platforms']=[copy.deepcopy(adapter_row)]
            self.lock['roles'][0]['platforms'][0]['phase_outcomes'][0][field]=value
            with self.subTest(field=field,value=value),self.assertRaisesRegex(ValueError,'adapter phase'):self.check()
    def test_runtime_retained_acceptance_preserves_deferred_qualification(self):
        self.qualified_fixture();row=next(r for r in self.lock['roles'] if r['role']=='runtime')
        retained={
            'build_provenance':self.ref,'installation_identity':self.ref,
            'scenario_acceptance':self.ref,'independent_review':self.ref,
            'scope_decision':self.ref,'scenarios_passed':7,
            'exact_install_invocation_retained':False,
            'current_qualification_status':'operator_deferred_outside_sprint1',
            'limitations':['historical aggregate install argv was not retained'],
        }
        row['platforms']=[{
            'target':'fixture-platform','source_commit':'a'*40,
            'artifact_sha256':self.ref['sha256'],'independent_clean':True,
            'distributable_only':True,'retained_acceptance':retained,
            'evidence':self.ref,
        }]
        self.assertEqual(self.check()['status'],'qualified_evidence_correspondence')
        for field,value,message in (
            ('scenarios_passed',0,'scenario denominator'),
            ('scenarios_passed',True,'scenario denominator'),
            ('exact_install_invocation_retained',True,'explicitly unavailable'),
            ('current_qualification_status','qualified','qualification disposition'),
        ):
            self.qualified_fixture();row=next(r for r in self.lock['roles'] if r['role']=='runtime')
            changed=copy.deepcopy(retained);changed[field]=value
            row['platforms']=[{'target':'fixture-platform','source_commit':'a'*40,'artifact_sha256':self.ref['sha256'],'independent_clean':True,'distributable_only':True,'retained_acceptance':changed,'evidence':self.ref}]
            with self.subTest(field=field),self.assertRaisesRegex(ValueError,message):self.check()
        self.qualified_fixture();row=self.lock['roles'][0]
        row['platforms']=[{'target':'fixture-platform','source_commit':'a'*40,'artifact_sha256':self.ref['sha256'],'independent_clean':True,'distributable_only':True,'retained_acceptance':copy.deepcopy(retained),'evidence':self.ref}]
        with self.assertRaisesRegex(ValueError,'Runtime-only'):self.check()
    def test_mutated_evidence(self):
        self.qualified_fixture();self.receipt.write_text('changed')
        with self.assertRaises(ValueError):self.check()
    def test_evidence_escape_and_symlink(self):
        for path in ['../outside','/absolute','link']:
            self.qualified_fixture()
            if path=='link':(self.root/'link').symlink_to(self.receipt)
            self.lock['rd07_acceptance']={'path':path,'sha256':self.ref['sha256']}
            with self.subTest(path=path),self.assertRaises(ValueError):self.check()
    def test_standalone_csdlc_runtime_forbidden(self):
        self.candidate_fixture()
        edge=copy.deepcopy(self.graph['edges'][0]);edge.update(consumer='csdlc',producer='runtime');self.graph['edges'].append(edge)
        with self.assertRaises(ValueError):self.check()
    def test_build_cycle_and_missing_graph(self):
        self.candidate_fixture()
        saved=copy.deepcopy(self.graph['edges']);edge=copy.deepcopy(saved[0]);edge.update(consumer='public_adl',producer='runtime',kind='build');self.graph['edges'].append(edge)
        with self.assertRaises(ValueError):self.check()
        self.graph['edges']=[]
        with self.assertRaises(ValueError):self.check()
    def test_mandatory_enterprise_dependency_forbidden(self):
        self.candidate_fixture()
        edge=next(e for e in self.graph['edges'] if e['consumer']=='runtime' and e['producer']=='enterprise_adapter');edge['optional']=False
        with self.assertRaises(ValueError):self.check()
    def test_unresolved_rollback_and_original_negatives(self):
        self.qualified_fixture();self.rollback['roles'][0]['disposition']='pending'
        with self.assertRaises(ValueError):self.check()
        self.qualified_fixture();self.lock['negative_evidence'].pop()
        with self.assertRaises(ValueError):self.check()
    def test_retained_installed_generation_point(self):
        self.qualified_fixture();row=next(x for x in self.rollback['roles'] if x['role']=='runtime')
        receipt=self.root/'generation.json';receipt.write_text(json.dumps({
            'source_revision':'b'*40,'generation':'fixture-generation','platform':'fixture-platform',
            'artifacts':{key:{'sha256':self.ref['sha256']} for key in ('kernel','guardian','csm')},
        }))
        receipt_ref={'path':'generation.json','sha256':v.digest(receipt)}
        point={'source_commit':'b'*40,'generation':'fixture-generation','platform':'fixture-platform',
               'artifacts':[{'name':name,'path':'fixture.json','sha256':self.ref['sha256']} for name in ('adl-runtime-kernel','adl-runtime-guardian','csm')],
               'receipt':receipt_ref,'supported_consumer_versions':['fixture runtime generation'],'replay_authority':False}
        row.update(disposition='retained_installed_generation',prior=copy.deepcopy(point),restore_argv=[],recoverability=self.ref)
        self.assertEqual(self.check()['status'],'qualified_evidence_correspondence')
        for change,message in (
            ({'replay_authority':True},'replay authority forbidden'),
            ({'artifacts':[]},'artifacts missing'),
        ):
            self.qualified_fixture();row=next(x for x in self.rollback['roles'] if x['role']=='runtime')
            changed=copy.deepcopy(point);changed.update(change)
            row.update(disposition='retained_installed_generation',prior=changed,restore_argv=[],recoverability=self.ref)
            with self.subTest(change=change),self.assertRaisesRegex(ValueError,message):self.check()
        for change,message in (
            ({'generation':'other-generation'},'receipt identity mismatch'),
            ({'artifacts':[{'name':'kernel','path':'fixture.json','sha256':self.ref['sha256']}]},'exact installed generation artifacts'),
            ({'artifacts':point['artifacts']+[{'name':'extra','path':'fixture.json','sha256':self.ref['sha256']}]},'exact installed generation artifacts'),
        ):
            self.qualified_fixture();row=next(x for x in self.rollback['roles'] if x['role']=='runtime')
            changed=copy.deepcopy(point);changed.update(change)
            row.update(disposition='retained_installed_generation',prior=changed,restore_argv=[],recoverability=self.ref)
            with self.subTest(change=change),self.assertRaisesRegex(ValueError,message):self.check()
        mutated=json.loads(receipt.read_text());mutated['artifacts']['kernel']['sha256']='0'*64;receipt.write_text(json.dumps(mutated))
        receipt_ref['sha256']=v.digest(receipt);point['receipt']=receipt_ref
        self.qualified_fixture();row=next(x for x in self.rollback['roles'] if x['role']=='runtime')
        row.update(disposition='retained_installed_generation',prior=copy.deepcopy(point),restore_argv=[],recoverability=self.ref)
        with self.assertRaisesRegex(ValueError,'receipt artifact mismatch'):self.check()
        mutated['artifacts']['kernel']['sha256']=self.ref['sha256'];receipt.write_text(json.dumps(mutated))
        point['receipt']={'path':'generation.json','sha256':v.digest(receipt)}
        self.qualified_fixture();row=next(x for x in self.rollback['roles'] if x['role']=='runtime')
        row.update(disposition='retained_installed_generation',prior=copy.deepcopy(point),restore_argv=['python3','missing.py'],recoverability=self.ref)
        with self.assertRaisesRegex(ValueError,'cannot imply replay argv'):self.check()
    def test_private_paths_and_mutable_selection(self):
        self.lock['roles'][0]['available'][0]['description']='/Users/private/source'
        with self.assertRaises(ValueError):self.check()
        self.lock['roles'][0]['available'][0]['description']='fixture';self.qualified_fixture();self.lock['roles'][0]['selected']['asset_identity']['name']='https://example.com/releases/latest'
        with self.assertRaises(ValueError):self.check()
    def test_duplicate_json_key(self):
        self.receipt.write_text('{"schema":1,"schema":2}')
        with self.assertRaises(ValueError):v.load(self.receipt)
if __name__=='__main__':unittest.main()
