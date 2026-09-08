#!/usr/bin/env ruby
require "digest"; require "fileutils"; require "json"; require "tmpdir"
require_relative "validate-remediation-ledger"
def wj(p,v); FileUtils.mkdir_p(File.dirname(p)); File.write(p,JSON.generate(v)); end
def sh!(*a); system(*a) or abort("failed #{a.join(' ')}"); end
Dir.mktmpdir("issue-522-production-",File.expand_path("../../../../.adl",__dir__)) do |repo|
 Dir.chdir(repo) do
  sh!("git","init","-q"); sh!("git","config","user.email","f@invalid"); sh!("git","config","user.name","fixture")
  File.write("base","base"); sh!("git","add","."); sh!("git","commit","-qm","base"); candidate=`git rev-parse HEAD`.strip
  reports=[]
  [[520,900],[521,901]].each do |issue,pr|
   path="sources/#{issue}/report.json"; wj(path,{"candidate_sha"=>candidate,"outcome"=>"passed","findings"=>[]})
   mp="sources/#{issue}/manifest.json"; wj(mp,{"entries"=>[{"path"=>path,"sha256"=>Digest::SHA256.file(path).hexdigest}]})
   sh!("git","add","."); sh!("git","commit","-qm","source #{issue}"); merge=`git rev-parse HEAD`.strip
   reports << {"issue"=>issue,"pull_request"=>pr,"merge_sha"=>merge,"path"=>path,"sha256"=>Digest::SHA256.hexdigest(`git show #{merge}:#{path}`),"packet_manifest_path"=>mp,"packet_manifest_sha256"=>Digest::SHA256.hexdigest(`git show #{merge}:#{mp}`),"reviewed_revision"=>candidate,"finding_ids"=>[],"finding_digests"=>{}}
  end
  File.write("ledger","candidate"); sh!("git","add","."); sh!("git","commit","-qm","ledger"); ledger=`git rev-parse HEAD`.strip
  root=File.join(repo,"packet"); FileUtils.mkdir_p(root)
  wj(File.join(root,"source-findings.json"),{"ledger_candidate_sha"=>ledger,"reports"=>reports,"findings"=>[],"zero_findings_proof"=>"both merged exact-head reports passed with no findings"})
  wj(File.join(root,"dispositions.json"),{"dispositions"=>[]}); wj(File.join(root,"release-blockers.json"),{"unresolved"=>[]})
  paths=%w[source-findings.json dispositions.json release-blockers.json].map{|n|File.join(root,n)}
  wj(File.join(root,"packet-manifest.json"),{"entries"=>paths.map{|p|{"path"=>p,"sha256"=>Digest::SHA256.file(p).hexdigest}}})
  bin=File.join(repo,"bin"); FileUtils.mkdir_p(bin); gh=File.join(bin,"gh")
  File.write(gh,"#!/usr/bin/env ruby\nrequire 'json'; issue=ARGV[2].to_i; if ARGV[0]=='issue'; puts JSON.generate(state:'CLOSED',closedByPullRequestsReferences:[{number:{520=>900,521=>901}[issue]}]); else; pr=issue; sha={900=>'#{reports[0]["merge_sha"]}',901=>'#{reports[1]["merge_sha"]}'}[pr]; puts JSON.generate(state:'MERGED',mergeCommit:{oid:sha}); end\n")
  FileUtils.chmod(0755,gh); ENV["PATH"]="#{bin}:#{ENV["PATH"]}"
  abort("valid production packet failed") unless validate_packet!(root:root)[:status]=="passed"
  doc=JSON.parse(File.read(File.join(root,"source-findings.json"))); doc["findings"]=[{"id"=>"F-X","severity"=>"P1","status"=>"blocking","evidence"=>"x","revision"=>candidate}]; wj(File.join(root,"source-findings.json"),doc)
  begin; validate_packet!(root:root); abort("invented production finding passed"); rescue SystemExit,KeyError; puts JSON.generate(status:"passed",production_negative:"invented_finding"); end
 end
end
