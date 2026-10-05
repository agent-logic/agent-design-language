#!/usr/bin/env python3
"""Read-only RD11 structure/evidence correspondence; never grants acceptance."""
import argparse
import json
import hashlib
import tarfile
from pathlib import Path, PurePosixPath
import re
from verify_install import digest

ROLES=('public_adl','csdlc','runtime','infrastructure','enterprise_adapter','codefriend','website')
REPOSITORIES=dict(zip(ROLES,('agent-design-language','cognitive-sdlc','agent-logic-runtime','agent-logic-infrastructure','agent-logic-enterprise-security','codefriend','codefriend.ai')))
IDENTITY={'source_commit','asset_identity','sha256','package_version','dependency_lock','build_provenance','supported_consumer_versions'}
REQUIRED_INTERFACES={
    ('runtime','public_adl','distribution','adl-engine'),
    ('runtime','public_adl','distribution','adl-records'),
    ('infrastructure','public_adl','build','adl-remote-validation'),
    ('infrastructure','runtime','distribution','runtime-candidate.tar'),
    ('codefriend','runtime','build','runtime-source'),
    ('enterprise_adapter','runtime','build','adl-runtime-policy'),
}
DISPOSITIONABLE_INTERFACES={('codefriend','runtime','build','runtime-source')}
NEGATIVES={'missing_product_proof','corrupt_artifact','incompatible_version','unproven_artifact','hidden_sibling_source','undeclared_build_cycle','standalone_csdlc_runtime_dependency','optional_adapter_absent_or_invalid','mixed_active_owners','unqualified_shared_tool_activation'}

def require(value,label):
    if not value:raise ValueError(label)

def unique(pairs):
    result={}
    for key,value in pairs:
        require(key not in result,'duplicate JSON key');result[key]=value
    return result

def load(path):
    require(path.is_file() and not path.is_symlink(),'regular JSON input required')
    return json.loads(path.read_bytes(),object_pairs_hook=unique)

def keys(value,expected,label):
    require(isinstance(value,dict) and set(value)==set(expected),label+' fields')

def text(value,label):
    require(isinstance(value,str) and bool(value.strip()),label+' missing')

def sha(value,length,label):
    require(isinstance(value,str) and re.fullmatch('[0-9a-f]{'+str(length)+'}',value),label+' digest')

def portable(value):
    if isinstance(value,str):
        require(not any(x in value for x in ('/Users/','/Volumes/','/private/tmp/','file://','BEGIN PRIVATE KEY')),'private host content forbidden')
    elif isinstance(value,dict):
        for item in value.values():portable(item)
    elif isinstance(value,list):
        for item in value:portable(item)

def reference(value,evidence_root,required,label):
    if value is None:
        require(not required,label+' missing');return
    keys(value,('path','sha256'),label);sha(value['sha256'],64,label)
    part=PurePosixPath(value['path']);require(not part.is_absolute() and '..' not in part.parts and str(part)==value['path'] and bool(part.parts),label+' relative path')
    if required:
        require(evidence_root is not None,label+' evidence root required')
        root=evidence_root.resolve(strict=True);path=root/value['path']
        require(path.resolve(strict=True).is_relative_to(root) and not any(p.is_symlink() for p in [path,*path.parents] if p==root or root in p.parents),label+' escaped evidence root')
        require(path.is_file() and digest(path)==value['sha256'],label+' bytes mismatch')

def identity(value,root,required,label):
    if value is None:
        require(not required,label+' selection missing');return
    keys(value,IDENTITY,label)
    for key,length in [('source_commit',40),('sha256',64)]:sha(value[key],length,label)
    text(value['package_version'],label)
    asset=value['asset_identity']
    require(isinstance(asset,dict) and set(asset) in ({'name','path'},{'name','path','member'}),label+' asset identity fields')
    text(asset['name'],label+' asset name')
    require(not re.search(r'(?i)(?:^|[/@:])(latest|main|master)(?:$|[/@:])',value['asset_identity']['name']),'mutable artifact identity')
    reference({'path':value['asset_identity']['path'],'sha256':value['sha256']},root,required,label+' actual artifact')
    if 'member' in asset:
        # A bundled product must name and hash its actual regular tar member.
        member=asset['member'];reference(member,root,False,label+' bundle member')
        if required:
            try:
                with tarfile.open(root/asset['path'],'r:*') as archive:
                    matches=[m for m in archive.getmembers() if m.name==member['path']]
                    require(len(matches)==1 and matches[0].isfile(),label+' bundle member absent/duplicate/nonregular')
                    with archive.extractfile(matches[0]) as stream:
                        actual=hashlib.file_digest(stream,'sha256').hexdigest()
                    require(actual==member['sha256'],label+' bundle member bytes mismatch')
            except tarfile.TarError as error:raise ValueError(label+' invalid bundle archive') from error
    versions=value['supported_consumer_versions'];require(isinstance(versions,list) and versions and all(isinstance(v,str) and v.strip() and v!='*' for v in versions) and len(set(versions))==len(versions),label+' explicit consumer versions')
    for key in ('dependency_lock','build_provenance'):reference(value[key],root,required,label+' '+key)

def generation_point(value,root,required,label):
    keys(value,('source_commit','generation','platform','artifacts','receipt','supported_consumer_versions','replay_authority'),label)
    sha(value['source_commit'],40,label);text(value['generation'],label+' generation');text(value['platform'],label+' platform')
    artifacts=value['artifacts'];require(isinstance(artifacts,list) and artifacts,'installed generation artifacts missing')
    names=set()
    for artifact in artifacts:
        keys(artifact,('name','path','sha256'),'installed generation artifact');text(artifact['name'],'installed generation artifact name')
        require(artifact['name'] not in names,'duplicate installed generation artifact');names.add(artifact['name'])
        reference({'path':artifact['path'],'sha256':artifact['sha256']},root,required,'installed generation artifact')
    require(names=={'adl-runtime-kernel','adl-runtime-guardian','csm'},'exact installed generation artifacts required')
    reference(value['receipt'],root,required,label+' receipt')
    versions=value['supported_consumer_versions'];require(isinstance(versions,list) and versions and all(isinstance(v,str) and v.strip() and v!='*' for v in versions),'installed generation explicit consumer versions')
    require(value['replay_authority'] is False,'installed generation replay authority forbidden')
    if required:
        receipt=load(root/value['receipt']['path'])
        require(receipt.get('source_revision')==value['source_commit'] and receipt.get('generation')==value['generation'] and receipt.get('platform')==value['platform'],'installed generation receipt identity mismatch')
        receipt_artifacts=receipt.get('artifacts');require(isinstance(receipt_artifacts,dict),'installed generation receipt artifacts missing')
        mapping={'adl-runtime-kernel':'kernel','adl-runtime-guardian':'guardian','csm':'csm'}
        actual={artifact['name']:artifact['sha256'] for artifact in artifacts}
        require(set(receipt_artifacts)==set(mapping.values()) and all(isinstance(receipt_artifacts[key],dict) and receipt_artifacts[key].get('sha256')==actual[name] for name,key in mapping.items()),'installed generation receipt artifact mismatch')

def selected_asset_key(selected):
    # Local transport path is not identity: copying a bundle cannot evade uniqueness.
    asset=selected['asset_identity'];member=asset.get('member')
    return (selected['source_commit'],asset['name'],selected['sha256'],
            None if member is None else (member['path'],member['sha256']))

def qualify(row,selected,root,role):
    common={'target','source_commit','artifact_sha256','independent_clean','distributable_only','evidence'}
    traditional=common|{'operations'}
    adapter=common|{'adapter_invocation','phase_outcomes'}
    retained=common|{'retained_acceptance'}
    require(isinstance(row,dict) and set(row) in (traditional,adapter,retained),'platform fields')
    text(row['target'],'platform');require(row['source_commit']==selected['source_commit'] and row['artifact_sha256']==selected['sha256'],'platform artifact/source mismatch')
    require(row['independent_clean'] is True and row['distributable_only'] is True,'independent clean distributable proof missing')
    if set(row)==traditional:
        operations=row['operations'];require(isinstance(operations,list) and len(operations)==3 and {x['operation'] for x in operations}=={'build','test','install'},'three original operations required')
        for op in operations:
            keys(op,('operation','argv','exit_code','executed','skipped'),'operation')
            require(isinstance(op['argv'],list) and op['argv'] and all(isinstance(v,str) and v for v in op['argv']),'exact operation argv missing')
            require(type(op['exit_code']) is int and op['exit_code']==0 and type(op['executed']) is int and op['executed']>0 and type(op['skipped']) is int and op['skipped']==0,'failed/zero/skipped operation cannot qualify')
    elif set(row)==adapter:
        invocation=row['adapter_invocation'];keys(invocation,('argv','exit_code','executed','skipped'),'adapter invocation')
        require(isinstance(invocation['argv'],list) and invocation['argv'] and all(isinstance(v,str) and v for v in invocation['argv']),'exact adapter argv missing')
        require(type(invocation['exit_code']) is int and invocation['exit_code']==0 and type(invocation['executed']) is int and invocation['executed']==1 and type(invocation['skipped']) is int and invocation['skipped']==0,'adapter must record exactly one successful invocation')
        phases=row['phase_outcomes'];require(isinstance(phases,list) and len(phases)==3 and {x['phase'] for x in phases}=={'build','test','install'},'adapter build/test/install phase outcomes required')
        for phase in phases:
            keys(phase,('phase','executed','failed','skipped','claim'),'adapter phase')
            text(phase['claim'],'adapter phase claim')
            require(type(phase['executed']) is int and phase['executed']>0 and type(phase['failed']) is int and phase['failed']==0 and type(phase['skipped']) is int and phase['skipped']==0,'failed/zero/skipped adapter phase cannot qualify')
    else:
        require(role=='runtime','retained acceptance is Runtime-only')
        retained_acceptance=row['retained_acceptance']
        keys(retained_acceptance,('build_provenance','installation_identity','scenario_acceptance','independent_review','scope_decision','scenarios_passed','exact_install_invocation_retained','current_qualification_status','limitations'),'retained acceptance')
        require(type(retained_acceptance['scenarios_passed']) is int and retained_acceptance['scenarios_passed']==7,'retained Runtime scenario denominator')
        require(retained_acceptance['exact_install_invocation_retained'] is False,'retained Runtime install argv must remain explicitly unavailable')
        require(retained_acceptance['current_qualification_status']=='operator_deferred_outside_sprint1','retained Runtime qualification disposition')
        limitations=retained_acceptance['limitations'];require(isinstance(limitations,list) and limitations and all(isinstance(v,str) and v.strip() for v in limitations),'retained Runtime limitations')
        for name in ('build_provenance','installation_identity','scenario_acceptance','independent_review','scope_decision'):
            reference(retained_acceptance[name],root,True,'retained Runtime '+name)
    reference(row['evidence'],root,True,'platform qualification')

def verify(lockset,graph,rollback,evidence_root=None,require_qualified=False):
    portable(lockset);portable(graph);portable(rollback)
    keys(lockset,('schema','state','roles','rd07_acceptance','negative_evidence','independent_review'),'lockset')
    require(lockset['schema']=='adl.compatibility_lockset.v1' and lockset['state'] in ('candidate','accepted'),'lockset schema/state')
    qualified=require_qualified or lockset['state']=='accepted'
    rows=lockset['roles'];require(isinstance(rows,list) and len(rows)==7 and {r['role'] for r in rows}==set(ROLES),'exact seven unique roles required')
    by_role={};missing=[];selected_assets=set()
    for row in rows:
        keys(row,('role','repository','available','selected','active_source_owners','platforms','website_disposition','authentication','activation'),'role')
        role=row['role'];by_role[role]=row
        require(row['repository']=='agent-logic/'+REPOSITORIES[role],'role/repository mismatch')
        require(isinstance(row['available'],list),'available manifest observations required')
        for available in row['available']:
            keys(available,('description','evidence'),'available input');text(available['description'],'available description');reference(available['evidence'],evidence_root,False,'available evidence')
        identity(row['selected'],evidence_root,qualified,role)
        if row['selected'] is None:missing.append(role+': selected distribution')
        else:
            key=selected_asset_key(row['selected'])
            require(key not in selected_assets,'duplicate selected asset across roles')
            require(not any(key[:3]==prior[:3] and (key[3] is None or prior[3] is None) for prior in selected_assets),'whole bundle overlaps selected member')
            selected_assets.add(key)
        owners=row['active_source_owners'];require(isinstance(owners,list) and len(owners)<=1 and all(isinstance(o,str) and o for o in owners),'mixed active source owners')
        require(not qualified or len(owners)==1,'qualified role owner missing')
        require(row['activation']=='none','shared activation is not authorized by lockset')
        reference(row['authentication'],evidence_root,qualified,role+' independent producer authentication')
        require(isinstance(row['platforms'],list),'platform list')
        targets=[r['target'] for r in row['platforms']];require(len(set(targets))==len(targets),'duplicate platform row')
        if role=='website':
            require(row['website_disposition'] in (None,'bundle_api_compatible','preserved_no_deployed_baseline'),'website disposition')
            if qualified:require(row['website_disposition'] is not None,'website compatibility/absence disposition missing')
            if row['website_disposition'] is not None:
                # Website uses its owner evidence reference, never fictitious software build counts.
                reference(row['authentication'],evidence_root,qualified,'website disposition evidence')
            require(not row['platforms'],'website does not inherit software platform proofs')
        else:
            require(row['website_disposition'] is None,'software website disposition')
            require(not qualified or bool(targets),'supported software platform proof missing')
            if row['selected'] is not None:
                for platform in row['platforms']:qualify(platform,row['selected'],evidence_root,role)
            else:require(not row['platforms'],'unselected artifact cannot have platform proof')
    require(isinstance(graph,dict),'graph fields')
    if graph.get('schema')=='adl.compatibility_graph.v1':
        keys(graph,('schema','edges'),'graph');dispositions=[]
    elif graph.get('schema')=='adl.compatibility_graph.v2':
        keys(graph,('schema','edges','dependency_dispositions'),'graph');dispositions=graph['dependency_dispositions']
        require(isinstance(dispositions,list),'dependency dispositions list')
    else:raise ValueError('graph schema')
    edges=set();build={r:set() for r in ROLES}
    for edge in graph['edges']:
        keys(edge,('consumer','producer','kind','interface','version','optional','evidence'),'edge')
        consumer,producer=edge['consumer'],edge['producer'];require(consumer in ROLES and producer in ROLES and consumer!=producer,'unknown/self dependency')
        require(edge['kind'] in ('build','distribution','runtime','api') and type(edge['optional']) is bool,'dependency kind/optionality')
        key=(consumer,producer,edge['kind'],edge['interface']);require(key not in edges,'duplicate dependency edge');edges.add(key)
        require(not (consumer=='csdlc' and producer=='runtime'),'standalone CSDLC Runtime dependency')
        require(not (consumer=='runtime' and producer=='enterprise_adapter') or (edge['optional'] and edge['kind']=='runtime'),'mandatory enterprise dependency')
        if qualified:text(edge['interface'],'versioned interface');text(edge['version'],'interface version')
        elif edge['interface'] is not None:text(edge['interface'],'interface')
        reference(edge['evidence'],evidence_root,qualified,'declared interface evidence')
        if edge['kind'] in ('build','distribution'):build[consumer].add(producer)
    disposition_keys=set()
    for disposition in dispositions:
        keys(disposition,('schema','consumer','producer','kind','interface','disposition','selected_source_commit','selected_artifact_sha256','dependency_lock','build_provenance','authentication','evidence'),'dependency disposition')
        require(disposition['schema']=='adl.compatibility_dependency_disposition.v1','dependency disposition schema')
        key=tuple(disposition[name] for name in ('consumer','producer','kind','interface'))
        require(key in DISPOSITIONABLE_INTERFACES and key not in disposition_keys,'unsupported/duplicate dependency disposition')
        require(key not in edges,'dependency edge and disposition conflict')
        require(disposition['disposition']=='independently_built_no_dependency','dependency disposition')
        selected=by_role[disposition['consumer']]['selected']
        require(selected is not None,'dependency disposition selected distribution missing')
        require(disposition['selected_source_commit']==selected['source_commit'] and disposition['selected_artifact_sha256']==selected['sha256'],'dependency disposition selected identity mismatch')
        require(disposition['dependency_lock']==selected['dependency_lock'] and disposition['build_provenance']==selected['build_provenance'],'dependency disposition source declaration mismatch')
        require(disposition['authentication']==by_role[disposition['consumer']]['authentication'],'dependency disposition authentication mismatch')
        for name in ('dependency_lock','build_provenance','authentication','evidence'):
            reference(disposition[name],evidence_root,qualified,'dependency disposition '+name)
        disposition_keys.add(key)
    require(REQUIRED_INTERFACES <= edges|disposition_keys,'original concrete interface/kind obligation missing')
    require(all(not edge['optional'] for edge in graph['edges'] if (edge['consumer'],edge['producer'],edge['kind'],edge['interface']) in REQUIRED_INTERFACES),'required interface cannot be optional')
    if qualified and by_role['website']['website_disposition']=='bundle_api_compatible':
        require(any(a=='website' and b=='codefriend' and k=='api' for a,b,k,_ in edges),'website API compatibility edge missing')
    def walk(node,active,done):
        require(node not in active,'undeclared build-order cycle')
        if node in done:return
        active.add(node)
        for dependency in build[node]:walk(dependency,active,done)
        active.remove(node);done.add(node)
    done=set()
    for node in ROLES:walk(node,set(),done)
    keys(rollback,('schema','roles'),'rollback');require(rollback['schema']=='adl.compatibility_rollback.v1','rollback schema')
    require(len(rollback['roles'])==7 and {r['role'] for r in rollback['roles']}==set(ROLES),'exact seven rollback rows')
    for row in rollback['roles']:
        keys(row,('role','disposition','prior','prior_observations','restore_argv','recoverability'),'rollback row')
        require(isinstance(row['prior_observations'],list),'prior observations list')
        for prior in row['prior_observations']:
            keys(prior,('description','evidence'),'prior observation');text(prior['description'],'prior description');reference(prior['evidence'],evidence_root,False,'prior observation')
        require(row['disposition'] in ('pending','retained_prior','retained_installed_generation','no_prior_accepted_baseline'),'rollback disposition')
        require(not qualified or row['disposition']!='pending','rollback unresolved')
        if row['disposition']=='retained_prior':
            identity(row['prior'],evidence_root,qualified,'prior distribution')
            require(isinstance(row['restore_argv'],list) and row['restore_argv'] and all(isinstance(s,str) and s for s in row['restore_argv']),'existing restore command missing')
        elif row['disposition']=='retained_installed_generation':
            generation_point(row['prior'],evidence_root,qualified,'prior installed generation')
            require(row['restore_argv']==[],'historical installed generation cannot imply replay argv')
        else:require(row['prior'] is None and row['restore_argv']==[],'absence/pending cannot imply prior restore')
        reference(row['recoverability'],evidence_root,qualified,'rollback recovery or baseline absence evidence')
    reference(lockset['rd07_acceptance'],evidence_root,qualified,'accepted RD07')
    reference(lockset['independent_review'],evidence_root,qualified,'independent exact lockset review')
    negatives=lockset['negative_evidence'];require(isinstance(negatives,list),'negative evidence list')
    names=set()
    for case in negatives:
        keys(case,('case','observed_refusal','executed','evidence'),'negative')
        require(case['case'] in NEGATIVES and case['case'] not in names,'unknown/duplicate negative');names.add(case['case']);text(case['observed_refusal'],'actual refusal')
        require(type(case['executed']) is int and case['executed']>0,'zero negative proof');reference(case['evidence'],evidence_root,qualified,'negative evidence')
    if qualified:require(names==NEGATIVES,'original negative proof missing')
    return {'status':'qualified_evidence_correspondence' if qualified else 'candidate_structure_valid','roles':7,'pending_selections':missing,'acceptance_authority':False,'activation_performed':False,'limits':'Hashes/normalized claims do not authenticate producers or independently review proofs; execution owner supplies authentic evidence and native acceptance.'}

def main():
    p=argparse.ArgumentParser(description=__doc__)
    for name in ('lockset','graph','rollback'):p.add_argument('--'+name,type=Path,required=True)
    p.add_argument('--evidence-root',type=Path);p.add_argument('--require-qualified',action='store_true');a=p.parse_args()
    try:result=verify(load(a.lockset),load(a.graph),load(a.rollback),a.evidence_root,a.require_qualified)
    except OSError:p.exit(2,'adl_event component=compatibility_lockset outcome=refused reason=evidence_unavailable\n')
    except (ValueError,KeyError,TypeError) as error:p.exit(2,'adl_event component=compatibility_lockset outcome=refused reason='+str(error)+'\n')
    print(json.dumps(result,sort_keys=True))
if __name__=='__main__':main()
