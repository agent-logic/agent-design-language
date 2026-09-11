#!/usr/bin/env ruby
require "digest"; require "fileutils"; require "json"; require "tmpdir"
require_relative "validate-remediation-ledger"
def wj(p,v); FileUtils.mkdir_p(File.dirname(p)); File.write(p,JSON.generate(v)); end
def sh!(*a); system(*a) or abort("failed #{a.join(' ')}"); end
Dir.mktmpdir("issue-522-production-",File.expand_path("../../../../.adl",__dir__)) do |repo|
 Dir.chdir(repo) do
  sh!("git","init","-q"); sh!("git","config","user.email","f@invalid"); sh!("git","config","user.name","fixture")
  File.write("base","base"); sh!("git","add","."); sh!("git","commit","-qm","base"); internal_candidate=`git rev-parse HEAD`.strip
  internal_findings = INTERNAL_FINDING_IDS.each_with_index.map do |id, index|
    {"id"=>id,"severity"=>(index < 3 ? "P1" : "P2"),"status"=>"blocking","evidence"=>"internal.rb:#{index + 1}","revision"=>internal_candidate,"title"=>"internal gap #{index + 1}"}
  end
  reports=[]
  [[520,900,internal_findings,"findings"]].each do |issue,pr,findings,outcome|
   path="sources/#{issue}/report.json"; wj(path,{"candidate_sha"=>internal_candidate,"outcome"=>outcome,"findings"=>findings}); mp="sources/#{issue}/manifest.json"; wj(mp,{"entries"=>[{"path"=>path,"sha256"=>Digest::SHA256.file(path).hexdigest}]})
   sh!("git","add","."); sh!("git","commit","-qm","source #{issue}"); merge=`git rev-parse HEAD`.strip
   reports << {"issue"=>issue,"pull_request"=>pr,"merge_sha"=>merge,"path"=>path,"sha256"=>Digest::SHA256.hexdigest(`git show #{merge}:#{path}`),"packet_manifest_path"=>mp,"packet_manifest_sha256"=>Digest::SHA256.hexdigest(`git show #{merge}:#{mp}`),"reviewed_revision"=>internal_candidate,"finding_ids"=>findings.map{|f|f["id"]},"finding_digests"=>findings.to_h{|f|[f["id"],Digest::SHA256.hexdigest(canonical_json(f))]}}
  end
  external_findings = EXTERNAL_FINDING_IDS.each_with_index.map do |id, index|
    {"id"=>id,"severity"=>(index < 3 ? "P1" : "P2"),"status"=>"blocking","evidence"=>"source.rb:#{index + 1}","title"=>"gap #{index + 1}"}
  end
  issue=521; pr=901; path="sources/#{issue}/report.json"; wj(path,{"candidate_sha"=>nil,"outcome"=>"failed","non_proving"=>true,"findings"=>external_findings}); mp="sources/#{issue}/manifest.json"; wj(mp,{"entries"=>[{"path"=>path,"sha256"=>Digest::SHA256.file(path).hexdigest}]})
  sh!("git","add","."); sh!("git","commit","-qm","source #{issue}"); merge=`git rev-parse HEAD`.strip
  reports[0]["binding"]="exact_revision"
  reports << {"issue"=>issue,"pull_request"=>pr,"merge_sha"=>merge,"path"=>path,"sha256"=>Digest::SHA256.hexdigest(`git show #{merge}:#{path}`),"packet_manifest_path"=>mp,"packet_manifest_sha256"=>Digest::SHA256.hexdigest(`git show #{merge}:#{mp}`),"binding"=>"missing_revision","reviewed_revision"=>nil,"finding_ids"=>external_findings.map{|f|f["id"]},"finding_digests"=>external_findings.to_h{|f|[f["id"],Digest::SHA256.hexdigest(canonical_json(f))]}}
  findings = internal_findings + external_findings
  returned_findings = RETURNED_FINDING_IDS.each_with_index.map do |id, index|
    {"id"=>id,"severity"=>([0, 3, 4].include?(index) ? "P1" : "P2"),"title"=>"returned gap #{index + 1}","evidence"=>"review evidence #{index + 1}","source_artifact"=>"review/returned.md","source_heading"=>"Finding #{index + 1}"}
  end
  FileUtils.mkdir_p("review"); File.write("review/returned.md",returned_findings.each_with_index.map { |finding,index| "## Finding #{index + 1}\n#{finding.fetch('title')}\n" }.join("\n"))
  FileUtils.mkdir_p("remediation"); File.write("remediation/fix.txt","fixed\n")
  sh!("git","add","."); sh!("git","commit","-qm","remediation"); head=`git rev-parse HEAD`.strip
  File.write("ledger","candidate"); sh!("git","add","."); sh!("git","commit","-qm","ledger"); ledger=`git rev-parse HEAD`.strip
  root=File.join(repo,"packet"); FileUtils.mkdir_p(root)
  review_path=File.join(root,"review.json"); wj(review_path,{"outcome"=>"passed","candidate_sha"=>head,"findings"=>[],"blockers"=>[],"resolved_finding_ids"=>INTERNAL_FINDING_IDS + EXTERNAL_FINDING_IDS + RETURNED_FINDING_IDS})
  fix_digest=Digest::SHA256.hexdigest(`git show #{head}:remediation/fix.txt`)
  validation_path=File.join(root,"validation.json"); wj(validation_path,{"outcome"=>"passed","head_sha"=>head,"failures"=>[],"observations"=>[{"artifact_path"=>"remediation/fix.txt","artifact_sha256"=>fix_digest,"result"=>"verified","behavior"=>"remediation output is present at exact head"}]})
  source_path=File.join(root,"source-findings.json"); wj(source_path,{"ledger_candidate_sha"=>ledger,"source_bindings"=>{"520"=>internal_candidate,"521"=>nil},"reports"=>reports,"findings"=>findings})
  returned_path=File.join(root,"returned-findings.json"); wj(returned_path,{"review_issue"=>833,"review_pull_request"=>853,"review_head_sha"=>head,"review_merge_sha"=>head,"artifacts"=>[{"path"=>"review/returned.md","sha256"=>Digest::SHA256.hexdigest(`git show #{head}:review/returned.md`)}],"findings"=>returned_findings})
  review={
    "reviewer"=>"reviewer", "reviewed_sha"=>head, "observed_pr_head_sha"=>head,
    "observed_at"=>"now", "outcome"=>"passed", "findings"=>[], "blockers"=>[], "report_path"=>review_path,
    "sha256"=>Digest::SHA256.file(review_path).hexdigest
  }
  remediation_issues=EXPECTED_REMEDIATION_ISSUES.values.flatten.uniq.sort
  remediations=remediation_issues.map do |remediation_issue|
    {"issue"=>remediation_issue,"pull_request"=>remediation_issue + 1000,"head_sha"=>head,"merge_sha"=>head,"artifacts"=>[{"path"=>"remediation/fix.txt","sha256"=>fix_digest}],"review"=>review,"validation"=>[{"evidence"=>validation_path,"sha256"=>Digest::SHA256.file(validation_path).hexdigest,"outcome"=>"passed"}]}
  end
  disposition={"kind"=>"fixed","source_finding_ids"=>INTERNAL_FINDING_IDS + EXTERNAL_FINDING_IDS + RETURNED_FINDING_IDS,"remediations"=>remediations}
  disposition_path=File.join(root,"dispositions.json"); wj(disposition_path,{"dispositions"=>[disposition]}); blockers_path=File.join(root,"release-blockers.json"); wj(blockers_path,{"unresolved"=>[]})
  paths=[source_path,returned_path,disposition_path,blockers_path,review_path,validation_path]; manifest_path=File.join(root,"packet-manifest.json"); wj(manifest_path,{"entries"=>paths.map{|p|{"path"=>p,"sha256"=>Digest::SHA256.file(p).hexdigest}}})
  bin=File.join(repo,"bin"); FileUtils.mkdir_p(bin); gh=File.join(bin,"gh"); File.write(gh,"#!/usr/bin/env ruby\nrequire 'json'; n=ARGV[2].to_i; if ARGV[0]=='issue'; pr={520=>900,521=>901}.fetch(n,n+1000); puts JSON.generate(state:'CLOSED',closedByPullRequestsReferences:[{number:pr}]); else; sha={900=>'#{reports[0]["merge_sha"]}',901=>'#{reports[1]["merge_sha"]}'}.fetch(n,'#{head}'); puts JSON.generate(state:'MERGED',mergeCommit:{oid:sha},headRefOid:sha); end\n"); FileUtils.chmod(0755,gh); ENV["PATH"]="#{bin}:#{ENV["PATH"]}"; ENV["ADL_REMEDIATION_HEAD_SHA"]=head
  abort("valid fixed production packet failed") unless validate_packet!(root:root)[:status]=="passed"
  reject=lambda do |name,files,&mutation|
   originals=files.to_h{|p|[p,File.binread(p)]}; mutation.call; pm=JSON.parse(File.read(manifest_path)); pm["entries"].each{|e|e["sha256"]=Digest::SHA256.file(e["path"]).hexdigest}; wj(manifest_path,pm)
   rejected=false; begin; validate_packet!(root:root); rescue SystemExit,KeyError,TypeError,JSON::ParserError; rejected=true; ensure originals.each{|p,c|File.binwrite(p,c)}; end
   abort("#{name} production mutation passed") unless rejected; puts JSON.generate(status:"passed",production_negative:name)
  end
  reject.call("stale_review_identity",[disposition_path,manifest_path]){d=JSON.parse(File.read(disposition_path));d["dispositions"][0]["remediations"][0]["review"]["reviewed_sha"]=internal_candidate;wj(disposition_path,d)}
  reject.call("empty_remediation_list",[disposition_path,manifest_path]){d=JSON.parse(File.read(disposition_path));d["dispositions"][0]["remediations"]=[];wj(disposition_path,d)}
  reject.call("dropped_contributing_remediation",[disposition_path,manifest_path]){d=JSON.parse(File.read(disposition_path));d["dispositions"][0]["remediations"].reject!{|r|r["issue"]==821};wj(disposition_path,d)}
  reject.call("pass_with_findings",[review_path,manifest_path]){d=JSON.parse(File.read(review_path));d["findings"]=[{"id"=>"still-open"}];wj(review_path,d)}
  reject.call("invented_twentieth_finding",[source_path,manifest_path]){d=JSON.parse(File.read(source_path));d["findings"] << d["findings"].last.merge("id"=>"TPR-006");wj(source_path,d)}
  reject.call("invented_external_binding",[source_path,manifest_path]){d=JSON.parse(File.read(source_path));d["source_bindings"]["521"]=internal_candidate;wj(source_path,d)}
  reject.call("missing_returned_finding",[returned_path,manifest_path]){d=JSON.parse(File.read(returned_path));d["findings"].pop;wj(returned_path,d)}
  reject.call("returned_finding_not_in_source",[returned_path,manifest_path]){d=JSON.parse(File.read(returned_path));d["findings"][0]["title"]="invented title";wj(returned_path,d)}
  reject.call("digest_mismatch",[validation_path,manifest_path]){File.write(validation_path,"{}")}
  reject.call("stale_head",[disposition_path,manifest_path]){d=JSON.parse(File.read(disposition_path));d["dispositions"][0]["remediations"][0]["head_sha"]=internal_candidate;wj(disposition_path,d)}
  empty_tree=`git mktree </dev/null`.strip; other,err,status=Open3.capture3("git","commit-tree",empty_tree,stdin_data:"unrelated\n"); abort(err) unless status.success?; other=other.strip
  reject.call("non_ancestral_merge",[disposition_path,gh,manifest_path]) do
   d=JSON.parse(File.read(disposition_path)); d["dispositions"][0]["remediations"][0]["merge_sha"]=other; wj(disposition_path,d)
   File.write(gh,"#!/usr/bin/env ruby\nrequire 'json'; n=ARGV[2].to_i; if ARGV[0]=='issue'; pr={520=>900,521=>901}.fetch(n,n+1000); puts JSON.generate(state:'CLOSED',closedByPullRequestsReferences:[{number:pr}]); else; pair={900=>['#{reports[0]["merge_sha"]}','#{reports[0]["merge_sha"]}'],901=>['#{reports[1]["merge_sha"]}','#{reports[1]["merge_sha"]}']}.fetch(n,['#{head}','#{head}']); pair=['#{head}','#{other}'] if n==1814; puts JSON.generate(state:'MERGED',mergeCommit:{oid:pair[1]},headRefOid:pair[0]); end\n"); FileUtils.chmod(0755,gh)
  end
 end
end
