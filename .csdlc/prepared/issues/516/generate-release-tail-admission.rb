#!/usr/bin/env ruby
# frozen_string_literal: true
require "digest"; require "json"; require "open3"; require "pathname"; require "yaml"

ROOT = Pathname.new(__dir__).join("../../../..").realpath
OUT = ROOT.join("docs/milestones/v0.92.1/evidence/integration")
REPO = "agent-logic/agent-design-language"
PLAN = ROOT.join("docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml")
SPEC = ROOT.join("docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
CATALOG = ROOT.join("docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md")

# Reviewed WP-01 allocation. New milestone labels cannot silently alter admission.
PLANNED = {
  "WP-01"=>480,"CORP-A"=>482,"CORP-B"=>483,"AWS-A"=>484,"AWS-B"=>485,"AWS-C"=>486,"AWS-D"=>487,"AWS-E"=>488,"AWS-F"=>489,
  "GCP-A"=>490,"GCP-B"=>491,"GCP-C"=>492,"GCP-D"=>493,"GCP-E"=>494,"XCL-01"=>495,"AWS-G"=>496,"CORP-C"=>497,"CORP-D"=>498,
  "RUST-01"=>499,"V3-A"=>500,"V3-B"=>501,"V3-C"=>502,"V3-D"=>503,"V3-E"=>504,"V3-F"=>505,"DRT-A"=>506,"DRT-B"=>507,
  "DRT-C"=>508,"DRT-D"=>509,"HOT-01"=>510,"OBS-A"=>511,"OBS-B"=>512,"DEC-01"=>513,"PROV-A"=>514,"PROV-B"=>515
}.freeze
EXISTING = {"POD-COORD"=>51,"POD-51A"=>261,"POD-51B"=>262,"POD-51C"=>263,"POD-51D"=>264,"POD-STUDIO"=>342,"AWS-GPU-SIDECAR"=>345,"OBS-PUBLIC"=>122}.freeze
AMENDMENTS = {
  "PROV-C"=>528,"CSDLC-BOOTSTRAP-DEFECT"=>544,"LEARNER-COVERAGE-GATE"=>558,"RUNTIME-COVERAGE-GATE"=>560,"CSDLC-STALE-BINARY-GUARD"=>563,
  "GCP-PROVIDER-CONFIG"=>592,"CANONICAL-AGENT-NAMES"=>617,"RUNTIME-ATOMIC-GENERATION"=>656,"RUNTIME-CONVERGENCE-DEADLINE"=>659,
  "PODCAST-EXPOSURE-REPAIR"=>660,"SHEPHERD-PROVIDER-REPLY"=>661,"A2A-INITIATION"=>662,"CSDLC-EMERGENCY-RECOVERY"=>665,
  "A2A-ACTION-RELIABILITY"=>693,"POLIS-WELCOME-PACKAGE"=>708
}.freeze
BACKLOG = {84=>"github:issue-84:track-backlog",251=>"github:issue-251:track-backlog"}.freeze
RETAINED = {
  "CORP-A"=>[153,154,155],"CORP-B"=>[156],"CORP-C"=>[157,158,159],"CORP-D"=>[160],"V3-A"=>[161,162,163],
  "V3-B"=>[164,165,166,167],"V3-C"=>[168,169,170],"V3-D"=>[171,172,173],"V3-E"=>[174,175,176,177,178],
  "V3-F"=>[179,180],"DRT-A"=>[181,182],"DRT-B"=>[183,184],"DRT-C"=>[185,186,187],"INT-01"=>[188]
}.freeze
# Explicit no-PR closure policy: absorption is conditional on its delivery owner.
ABSORBED = {
  51=>{"kind"=>"coordination_closeout","authority"=>"github:issue-51:terminal-child-coordination-closeout"},
  497=>{"kind"=>"operator_terminal_closeout","authority"=>"github:issue-497:operator-closeout-after-pr-613"},
  511=>{"kind"=>"absorbed","owner"=>512,"authority"=>"github:issue-511:absorption-closeout-comment-v1"}
}.freeze

def capture(*argv)
  out, err, status = Open3.capture3(*argv, chdir: ROOT.to_s)
  abort("#{argv.join(' ')} failed: #{err}") unless status.success?
  out
end
def sha(path)
  Digest::SHA256.file(path).hexdigest
end
def ancestor?(oid, candidate)
  system("git","merge-base","--is-ancestor",oid,candidate,chdir:ROOT.to_s,out:File::NULL,err:File::NULL)
end
def acceptance(body)
  section = body.to_s[/^## (?:Acceptance(?: Criteria| criteria)?|Exit Criteria)\s*$\n(.*?)(?=^## |\z)/m,1]
  return [] unless section
  section.lines.map { |line| line.match(/^\s*(?:-|\d+\.)\s*(?:\[[ xX]\]\s*)?(.+)/)&.captures&.first&.strip }.compact
end

candidate = capture("gh","api","repos/#{REPO}/git/ref/heads/main","--jq",".object.sha").strip
abort("local origin/main differs from captured remote main") unless capture("git","rev-parse","origin/main").strip == candidate
pages = JSON.parse(capture("gh","api","--paginate","--slurp","repos/#{REPO}/issues?milestone=1&state=all&per_page=100"))
captured = pages.flatten.reject { |e| e.key?("pull_request") }.to_h { |e| [e.fetch("number"),e] }
mapping = PLANNED.merge(EXISTING).merge(AMENDMENTS)
missing = (mapping.values + BACKLOG.keys).reject { |n| captured.key?(n) }
abort("captured denominator missing #{missing.inspect}") unless missing.empty?

declared=[]; walk=lambda{|x| x.is_a?(Hash) ? (declared << x["id"] if x["id"]; x.each_value{|v|walk.call(v)}) : (x.each{|v|walk.call(v)} if x.is_a?(Array))}; walk.call(YAML.safe_load(PLAN.read).fetch("work_packages"))
abort("planned IDs absent from issue wave") unless (PLANNED.keys-declared).empty?

rows = mapping.sort_by { |_id,n| n }.map do |planned_id,number|
  issue = JSON.parse(capture("gh","issue","view",number.to_s,"--repo",REPO,"--json","number,title,state,url,body,labels,closedByPullRequestsReferences"))
  prs = issue.fetch("closedByPullRequestsReferences").map do |ref|
    JSON.parse(capture("gh","pr","view",ref.fetch("number").to_s,"--repo",REPO,"--json","number,url,state,mergedAt,headRefOid,mergeCommit"))
  end
  prs.each { |pr| pr["ancestral"] = !!(pr.dig("mergeCommit","oid") && ancestor?(pr.dig("mergeCommit","oid"),candidate)) }
  canonical = prs.select { |pr| pr["mergedAt"] && pr["ancestral"] }.max_by { |pr| [pr["mergedAt"],pr["number"]] }
  absorbed = ABSORBED[number]
  owner_closed = absorbed && (!absorbed["owner"] || captured.fetch(absorbed.fetch("owner")).fetch("state") == "closed")
  disposition = if absorbed then owner_closed ? "satisfied_by_explicit_no_pr_closure" : "release_blocker"
                elsif issue["state"] == "CLOSED" && canonical then "satisfied" else "release_blocker" end
  record=ROOT.join(".csdlc/issues/#{number}"); srp=record.join("cards/srp.md"); sor=record.join("cards/sor.md")
  evidence = canonical ? [canonical.fetch("url"),canonical.fetch("headRefOid")] : (absorbed ? [absorbed.fetch("authority"), absorbed["owner"] ? "issue ##{absorbed['owner']}" : nil].compact : [])
  evidence += [srp,sor].select(&:file?).map{|p|"#{p.relative_path_from(ROOT)}@sha256:#{sha(p)}"}
  spp_values=record.join("cards/spp.values.json")
  owned_paths = if spp_values.file?
                  JSON.parse(spp_values.read).dig("content","values","affected_areas") || []
                else [] end
  {
    "kind"=>"execution_issue","planned_id"=>planned_id,"issue"=>number,"title"=>issue["title"],"acceptance_authority"=>"#{issue['url']}#issue-body",
    "acceptance_rows"=>acceptance(issue["body"]).each_with_index.map{|text,i|{"id"=>"issue-#{number}-ac-#{i+1}","text"=>text,"evidence_status"=>evidence.empty? ? "missing" : "evidence_linked","evidence"=>evidence}},
    "linked_prs"=>prs.map{|pr|{"number"=>pr["number"],"url"=>pr["url"],"head_oid"=>pr["headRefOid"],"merge_oid"=>pr.dig("mergeCommit","oid"),"merged_at"=>pr["mergedAt"],"ancestral"=>pr["ancestral"]}},
    "canonical_pr"=>canonical&.fetch("number",nil),"revision"=>canonical&.fetch("headRefOid",nil),"merge_revision"=>canonical&.dig("mergeCommit","oid"),
    "merge_ancestry"=>canonical ? "ancestor" : (absorbed ? "not_applicable_absorbed" : "not_proven"),
    "artifacts"=>[issue["url"],srp.file? ? srp.relative_path_from(ROOT).to_s : nil,sor.file? ? sor.relative_path_from(ROOT).to_s : nil].compact,
    "review_evidence"=>srp.file? ? {"path"=>srp.relative_path_from(ROOT).to_s,"sha256"=>sha(srp)} : nil,
    "validation_evidence"=>sor.file? ? {"path"=>sor.relative_path_from(ROOT).to_s,"sha256"=>sha(sor)} : nil,
    "closure_disposition"=>absorbed,"closeout_state"=>issue["state"].downcase,"disposition"=>disposition
  }.merge("owned_paths"=>owned_paths)
end

backlog = BACKLOG.sort.map do |number,authority|
  issue=captured.fetch(number); labels=issue.fetch("labels").map{|x|x["name"]}
  abort("backlog authority missing live label for ##{number}") unless labels.include?("track:backlog")
  {"kind"=>"operator_deferred_backlog","issue"=>number,"title"=>issue["title"],"disposition_authority"=>authority,"disposition"=>"routed_to_backlog","owner"=>"issue ##{number}"}
end
retained = RETAINED.flat_map do |planned_id,numbers|
  numbers.map do |number|
    path=ROOT.join("docs/milestones/v0.92.1/planned-issue-packets/issues/#{number}/cards/stp.md"); abort("missing retained ##{number}") unless path.file?
    {"kind"=>"retained_predecessor","planned_id"=>planned_id,"issue"=>number,"path"=>path.relative_path_from(ROOT).to_s,"sha256"=>sha(path),
     "acceptance_rows"=>acceptance(path.read).each_with_index.map{|text,i|{"id"=>"retained-#{number}-ac-#{i+1}","text"=>text}}}
  end
end

findings=rows.map do |row|
  next if %w[satisfied satisfied_by_explicit_no_pr_closure].include?(row["disposition"])
  {"id"=>"issue-#{row['issue']}-not-terminal","type"=>"closeout_drift","severity"=>"P1","classification"=>"release_blockers",
   "summary"=>"#{row['planned_id']} / ##{row['issue']} lacks reviewed merged ancestral authority.","evidence"=>row["artifacts"],"uncertainty"=>"none","disposition"=>"open","owner"=>"issue ##{row['issue']}"}
end.compact
backlog.each{|row|findings<<{"id"=>"issue-#{row['issue']}-operator-deferred","type"=>"scope_ambiguity","severity"=>"P2","classification"=>"routed_work","summary"=>"#{row['title']} is explicitly routed outside the release gate.","evidence"=>[row["disposition_authority"]],"uncertainty"=>"planning documentation requires reconciliation","disposition"=>"routed_to_backlog","owner"=>row["owner"]}}
decision=findings.any?{|f|%w[P0 P1].include?(f["severity"])&&f["disposition"]!="resolved"} ? "blocked" : "admitted"
observations=rows.map{|r|r.slice("planned_id","issue","title","acceptance_authority","acceptance_rows","linked_prs","closeout_state","closure_disposition","owned_paths","review_evidence","validation_evidence")}
source={"schema"=>"adl.v0921.release_tail_input.v1","candidate"=>candidate,"planning"=>[PLAN,SPEC,CATALOG].map{|p|{"path"=>p.relative_path_from(ROOT).to_s,"sha256"=>sha(p)}},"mapping"=>mapping,"backlog"=>BACKLOG,"retained"=>RETAINED,"observations"=>observations,"captured_issue_count"=>captured.length,"captured_pages"=>pages.length}
digest=Digest::SHA256.hexdigest(JSON.generate(source))
claims=Hash.new{|h,k|h[k]=[]}; rows.each{|r|r["owned_paths"].each{|path|claims[path]<<r["issue"]}}
collisions=claims.map do |path,owners|
  unique=owners.uniq; next unless unique.length>1
  owner_rows=rows.select{|r|unique.include?(r["issue"])}
  resolved=owner_rows.all?{|r|%w[satisfied satisfied_by_explicit_no_pr_closure].include?(r["disposition"])}
  {"path"=>path,"owners"=>unique,"status"=>resolved ? "resolved_by_ancestral_history" : "unresolved"}
end.compact
collisions.select{|c|c["status"]=="unresolved"}.each{|c|findings<<{"id"=>"owned-path-collision-#{Digest::SHA256.hexdigest(c['path'])[0,12]}","type"=>"implementation_gap","severity"=>"P1","classification"=>"release_blockers","summary"=>"Owned path #{c['path']} has multiple unresolved owners.","evidence"=>c["owners"].map{|n|".csdlc/issues/#{n}/cards/spp.values.json"},"uncertainty"=>"none","disposition"=>"open","owner"=>c["owners"].map{|n|"issue ##{n}"}.join(", ")}}
decision=findings.any?{|f|%w[P0 P1].include?(f["severity"])&&f["disposition"]!="resolved"} ? "blocked" : "admitted"
counts={"execution_issues"=>rows.length,"backlog"=>backlog.length,"retained_predecessors"=>retained.length,"acceptance_rows"=>rows.sum{|r|r["acceptance_rows"].length}+retained.sum{|r|r["acceptance_rows"].length}}
admission={"schema"=>"adl.v0921.release_tail_admission.v2","candidate"=>candidate,"source_digest"=>digest,"denominator_policy"=>"canonical_plan_plus_explicit_amendments_backlog_and_retained_predecessors","counts"=>counts,"execution_issues"=>rows,"backlog"=>backlog,"retained_predecessors"=>retained,"ownership_collisions"=>collisions,"findings"=>findings,"decision"=>decision}
gap={"schema"=>"adl.gap_analysis_report.v2","mode"=>"compare_canonical_plan_to_immutable_evidence","candidate"=>candidate,"source_digest"=>digest,"execution_issues"=>rows.map{|r|r.slice("planned_id","issue","revision","merge_revision","merge_ancestry","disposition","acceptance_rows")},"backlog"=>backlog,"retained_predecessors"=>retained,"findings"=>findings,"decision"=>decision}

OUT.mkpath
stem=candidate; paths={source:OUT.join("release-tail-input.#{stem}.json"),admission:OUT.join("release-tail-admission.json"),gap:OUT.join("gap_analysis_report.json"),md:OUT.join("gap_analysis_report.md")}
paths[:source].write(JSON.generate(source)+"\n"); paths[:admission].write(JSON.generate(admission)+"\n"); paths[:gap].write(JSON.generate(gap)+"\n")
lines=rows.map{|r|"| #{r['planned_id']} | ##{r['issue']} | #{r['revision']||'none'} | #{r['merge_revision']||'none'} | #{r['merge_ancestry']} | #{r['disposition']} |"}
fl=findings.empty? ? ["No unresolved findings."] : findings.map{|f|"- **#{f['severity']} #{f['id']}** — #{f['summary']} Evidence: #{f['evidence'].join(', ')}. Owner: #{f['owner']}. Disposition: #{f['disposition']}."}
md="# v0.92.1 Release-tail Gap Analysis\n\nCandidate: `#{candidate}`\n\nCaptured-input digest: `#{digest}`\n\n## Findings\n\n#{fl.join("\n")}\n\n## Denominator\n\nExecution issues: #{rows.length}; retained predecessors: #{retained.length}; backlog dispositions: #{backlog.length}; acceptance rows: #{counts['acceptance_rows']}.\n\n| Planned ID | Issue | Head revision | Merge revision | Ancestry | Disposition |\n|---|---:|---|---|---|---|\n#{lines.join("\n")}\n\n## Backlog and retained authority\n\n#{backlog.map{|r|"- ##{r['issue']}: #{r['disposition_authority']}"}.join("\n")}\n- Retained predecessor packets are indexed with SHA-256 digests in `#{paths[:gap].basename}`.\n\n## Decision\n\n**#{decision.upcase}**\n\nThis is an admission decision only; it is not release approval.\n"
paths[:md].write(md)
puts JSON.generate(schema:"adl.v0921.release_tail_generation.v2",status:"pass",candidate:candidate,source_digest:digest,counts:counts,decision:decision)
