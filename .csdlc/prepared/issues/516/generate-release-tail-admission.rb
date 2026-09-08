#!/usr/bin/env ruby
# frozen_string_literal: true
require "digest"; require "json"; require "open3"; require "pathname"; require "yaml"

ROOT = Pathname.new(__dir__).join("../../../..").realpath
OUT = ROOT.join("docs/milestones/v0.92.1/evidence/integration")
REPO = "agent-logic/agent-design-language"
PLAN = ROOT.join("docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml")
SPEC = ROOT.join("docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
CATALOG = ROOT.join("docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md")

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
def git_at(revision,path)
  out, _err, status=Open3.capture3("git","show","#{revision}:#{path}",chdir:ROOT.to_s)
  status.success? ? out : nil
end
def git_blob(revision,path)
  out,_err,status=Open3.capture3("git","rev-parse","#{revision}:#{path}",chdir:ROOT.to_s)
  status.success? ? out.strip : nil
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
plan_doc=YAML.safe_load(PLAN.read); nodes=[]; walk=lambda{|x|x.is_a?(Hash) ? (nodes<<x if x["id"];x.each_value{|v|walk.call(v)}) : (x.each{|v|walk.call(v)} if x.is_a?(Array))};walk.call(plan_doc.fetch("work_packages"))
specs=YAML.safe_load(SPEC.read).fetch("issue_specifications").to_h{|s|[s.fetch("id"),s]}; canonical_ids=specs.keys
declared=nodes.map{|n|n["id"]}&canonical_ids; catalog_ids=CATALOG.read.scan(/^\| ([A-Z][A-Z0-9-]+) \|/).flatten.select{|id|canonical_ids.include?(id)}
abort("wave/catalog/spec planned-ID parity") unless declared.sort==canonical_ids.sort && catalog_ids.sort==canonical_ids.sort
matches=canonical_ids.to_h{|id|[id,captured.values.select{|i|i["title"].match?(/\[#{Regexp.escape(id)}\]/)}]};bad=matches.select{|_,v|v.length!=1};abort("exact live mapping failed #{bad.transform_values(&:length)}") unless bad.empty?
all_mapping=matches.transform_values{|v|v.first["number"]}; tail_ids=canonical_ids.select{|id|id=="INT-01"||id.match?(/\ATAIL-\d+\z/)}; mapping=all_mapping.reject{|id,_|tail_ids.include?(id)}; tail_mapping=all_mapping.select{|id,_|tail_ids.include?(id)}
retained_mapping=nodes.select{|n|canonical_ids.include?(n["id"])&&n["predecessor_issues"]}.to_h{|n|[n["id"],n["predecessor_issues"].map(&:to_i)]}
backlog_numbers=[]; walk_all=lambda{|x|if x.is_a?(Hash);backlog_numbers.concat(Array(x["operator_deferred_backlog"]).flat_map{|v|v.is_a?(Integer) ? v : v.to_s.scan(/\d+/).map(&:to_i)});x.each_value{|v|walk_all.call(v)}elsif x.is_a?(Array);x.each{|v|walk_all.call(v)}end};walk_all.call(plan_doc);backlog_numbers.uniq!
spec_acceptance=mapping.to_h{|id,_|spec=specs.fetch(id);[id,{"acceptance_criteria"=>spec.fetch("acceptance_criteria"),"digest"=>Digest::SHA256.hexdigest(JSON.generate(spec))}]}

rows = mapping.sort_by { |_id,n| n }.map do |planned_id,number|
  issue = JSON.parse(capture("gh","issue","view",number.to_s,"--repo",REPO,"--json","number,title,state,url,body,labels,comments,closedByPullRequestsReferences"))
  prs = issue.fetch("closedByPullRequestsReferences").map do |ref|
    JSON.parse(capture("gh","pr","view",ref.fetch("number").to_s,"--repo",REPO,"--json","number,url,state,mergedAt,headRefOid,mergeCommit,files,statusCheckRollup"))
  end
  prs.each { |pr| pr["ancestral"] = !!(pr.dig("mergeCommit","oid") && ancestor?(pr.dig("mergeCommit","oid"),candidate)) }
  canonical = prs.select { |pr| pr["mergedAt"] && pr["ancestral"] }.max_by { |pr| [pr["mergedAt"],pr["number"]] }
  closure_comment=issue["comments"].find{|c|c["body"].to_s.match?(/Closed as absorbed into #(\d+)/)&&c["body"].to_s.include?("csdlc-github-operation:")}
  absorbed=if closure_comment
    target=closure_comment["body"].match(/Closed as absorbed into #(\d+)/)[1].to_i; target_issue=captured[target]
    {"kind"=>"captured_absorption_closeout","comment_url"=>closure_comment["url"],"comment_body_sha256"=>Digest::SHA256.hexdigest(closure_comment["body"]),"target_issue"=>target,"target_state"=>target_issue&&target_issue["state"]&.downcase,"authority_marker"=>"csdlc-github-operation"}
  end
  owner_closed=absorbed&&absorbed["target_state"]=="closed"; disposition="release_blocker"
  record=ROOT.join(".csdlc/issues/#{number}"); srp=record.join("cards/srp.md"); sor=record.join("cards/sor.md")
  files=canonical ? canonical.fetch("files").map{|f|f["path"]} : []
  product=files.reject{|p|p.start_with?(".csdlc/")||p.include?("/tests/")||p.match?(/(?:^|\/)(?:test|validate)[^\/]*$/)||p.start_with?("docs/")}
  noncode=files.select{|p|p.start_with?("docs/")||p.start_with?(".csdlc/evidence/")}
  behavioral=files.select{|p|p.include?("test")||p.include?("validate")||p.start_with?(".csdlc/evidence/")}
  review_files=files.select{|p|p.include?("/cards/srp.")||p.include?("review-record")||p.include?("review_record")}
  docs_demo=files.select{|p|p.start_with?("docs/")||p.start_with?("demos/")}
  successful_checks=canonical ? canonical.fetch("statusCheckRollup").select{|c|c["conclusion"]=="SUCCESS"}.map{|c|c["name"]}.uniq.sort : []
  srp_path=review_files.find{|p|p.end_with?("/cards/srp.md")}; srp_content=canonical&&srp_path ? git_at(canonical["headRefOid"],srp_path) : nil
  reviewed_revision=srp_content&.match(/Revision:.*?([0-9a-f]{40})/)&.captures&.first
  post_review=reviewed_revision&&canonical ? capture("git","diff","--name-only","#{reviewed_revision}..#{canonical['headRefOid']}").lines.map(&:strip).reject(&:empty?) : []
  review_current=reviewed_revision&&canonical&&ancestor?(reviewed_revision,canonical["headRefOid"])&&post_review.empty?
  semantic={"production_call_path_or_noncode"=>(product+noncode).uniq,"behavioral_validation"=>behavioral,"exact_head_review"=>review_files,"review_basis"=>{"reviewed_revision"=>reviewed_revision,"post_review_paths"=>post_review,"current"=>!!review_current},"docs_demo_relevance"=>docs_demo.empty? ? ["explicit:not_applicable"] : docs_demo,"successful_checks"=>successful_checks}
  semantic_complete=false
  evidence = canonical ? [canonical.fetch("url"),canonical.fetch("headRefOid")] : (absorbed ? [absorbed.fetch("comment_url"), "issue ##{absorbed['target_issue']}"] : [])
  evidence += [srp,sor].select(&:file?).map{|p|"#{p.relative_path_from(ROOT)}@sha256:#{sha(p)}"}
  spp_values=record.join("cards/spp.values.json")
  owned_paths = if spp_values.file?
                  JSON.parse(spp_values.read).dig("content","values","affected_areas") || []
                else [] end
  {
    "kind"=>"execution_issue","planned_id"=>planned_id,"issue"=>number,"title"=>issue["title"],"acceptance_authority"=>"#{issue['url']}#issue-body","issue_body_sha256"=>Digest::SHA256.hexdigest(issue["body"]),
    "acceptance_rows"=>specs.fetch(planned_id).fetch("acceptance_criteria").each_with_index.map{|text,i|{"id"=>"#{planned_id}-ac-#{i+1}","text"=>text,"text_digest"=>Digest::SHA256.hexdigest(text),"evidence_status"=>"gap_missing_explicit_criterion_evidence","evidence"=>[],"proof"=>nil}},
    "linked_prs"=>prs.map{|pr|{"number"=>pr["number"],"url"=>pr["url"],"head_oid"=>pr["headRefOid"],"merge_oid"=>pr.dig("mergeCommit","oid"),"merged_at"=>pr["mergedAt"],"ancestral"=>pr["ancestral"],"files_digest"=>Digest::SHA256.hexdigest(JSON.generate(pr.fetch("files").map{|f|f["path"]}.sort))}},
    "canonical_pr"=>canonical&.fetch("number",nil),"revision"=>canonical&.fetch("headRefOid",nil),"merge_revision"=>canonical&.dig("mergeCommit","oid"),
    "merge_ancestry"=>canonical ? "ancestor" : (absorbed ? "not_applicable_absorbed" : "not_proven"),
    "artifacts"=>[issue["url"],srp.file? ? srp.relative_path_from(ROOT).to_s : nil,sor.file? ? sor.relative_path_from(ROOT).to_s : nil].compact,
    "review_evidence"=>srp.file? ? {"path"=>srp.relative_path_from(ROOT).to_s,"sha256"=>sha(srp)} : nil,
    "validation_evidence"=>sor.file? ? {"path"=>sor.relative_path_from(ROOT).to_s,"sha256"=>sha(sor)} : nil,
    "closure_disposition"=>absorbed,"closeout_state"=>issue["state"].downcase,"disposition"=>disposition
  }.merge("owned_paths"=>owned_paths)
end

tail_rows=tail_mapping.sort_by{|_id,n|n}.map do |planned_id,number|
  issue=JSON.parse(capture("gh","issue","view",number.to_s,"--repo",REPO,"--json","number,title,state,url,body"))
  expected=planned_id=="INT-01" ? "active_admission_work" : "future_serial_stage"
  {"kind"=>"release_tail_stage","planned_id"=>planned_id,"issue"=>number,"issue_url"=>issue["url"],"issue_body_sha256"=>Digest::SHA256.hexdigest(issue["body"]),"acceptance_rows"=>specs.fetch(planned_id).fetch("acceptance_criteria"),"observed_state"=>issue["state"].downcase,"expected_lifecycle"=>expected,"gate_role"=>"denominator_only_not_execution_root"}
end

rows.each do |row|
  next unless spec_acceptance[row["planned_id"]]
  expected_ac=spec_acceptance.fetch(row["planned_id"]).fetch("acceptance_criteria")
  live=acceptance(captured.fetch(row["issue"]).fetch("body")); mismatch=live!=expected_ac
  row["spec_acceptance"]={"digest"=>spec_acceptance[row["planned_id"]]["digest"],"expected"=>expected_ac,"live_digest"=>Digest::SHA256.hexdigest(JSON.generate(live)),"status"=>mismatch ? "mismatch" : "match"}
end

backlog = backlog_numbers.sort.map do |number|
  issue=captured.fetch(number); labels=issue.fetch("labels").map{|x|x["name"]}
  abort("backlog authority missing live label for ##{number}") unless labels.include?("track:backlog")
  authority={"source"=>"github_label_and_canonical_plan","label"=>"track:backlog","issue_body_sha256"=>Digest::SHA256.hexdigest(issue.fetch("body")),"candidate"=>candidate}
  {"kind"=>"operator_deferred_backlog","issue"=>number,"title"=>issue["title"],"disposition_authority"=>authority,"disposition"=>"routed_to_backlog","owner"=>"issue ##{number}"}
end
retained = retained_mapping.flat_map do |planned_id,numbers|
  owner=rows.find{|r|r["planned_id"]==planned_id}
  numbers.map do |number|
    path=ROOT.join("docs/milestones/v0.92.1/planned-issue-packets/issues/#{number}/cards/stp.md"); abort("missing retained ##{number}") unless path.file?
    status="gap_missing_explicit_semantic_successor_mapping"
    {"kind"=>"retained_predecessor","planned_id"=>planned_id,"issue"=>number,"path"=>path.relative_path_from(ROOT).to_s,"sha256"=>sha(path),"observed_status"=>status,"observed_owner_issue"=>owner&&owner["issue"],"observed_revision"=>owner&&owner["revision"],
     "acceptance_rows"=>acceptance(path.read).each_with_index.map{|text,i|{"id"=>"retained-#{number}-ac-#{i+1}","text"=>text,"text_digest"=>Digest::SHA256.hexdigest(text),"observed_status"=>status,"observed_evidence"=>nil}}}
  end
end

findings=rows.map do |row|
  reason="lacks explicit criterion-level production, behavior, validation, and exact-head review mappings"
  {"id"=>"issue-#{row['issue']}-review-or-terminal-gap","type"=>"missing_evidence","severity"=>"P1","classification"=>"release_blockers",
   "summary"=>"#{row['planned_id']} / ##{row['issue']} #{reason}.","evidence"=>row["artifacts"],"uncertainty"=>"none","disposition"=>"open","owner"=>"issue ##{row['issue']}"}
end.compact
rows.select{|r|r.dig("spec_acceptance","status")=="mismatch"}.each{|r|findings<<{"id"=>"issue-#{r['issue']}-spec-ac-drift","type"=>"docs_drift","severity"=>"P1","classification"=>"release_blockers","summary"=>"#{r['planned_id']} live acceptance criteria do not cover the exact execution specification.","evidence"=>[r["acceptance_authority"],r.dig("spec_acceptance","digest")],"uncertainty"=>"none","disposition"=>"open","owner"=>"issue ##{r['issue']}"}}
retained.each{|r|findings<<{"id"=>"retained-#{r['issue']}-observed-gap","type"=>"missing_evidence","severity"=>"P1","classification"=>"release_blockers","summary"=>"Retained predecessor ##{r['issue']} lacks an explicit semantic successor mapping.","evidence"=>[r["path"]],"uncertainty"=>"none","disposition"=>"open","owner"=>"issue ##{r['observed_owner_issue']}"}}
backlog.each{|row|findings<<{"id"=>"issue-#{row['issue']}-operator-deferred","type"=>"scope_ambiguity","severity"=>"P2","classification"=>"routed_work","summary"=>"#{row['title']} is explicitly routed outside the release gate.","evidence"=>[row["disposition_authority"]],"uncertainty"=>"planning documentation requires reconciliation","disposition"=>"routed_to_backlog","owner"=>row["owner"]}}
decision=findings.any?{|f|%w[P0 P1].include?(f["severity"])&&f["disposition"]!="resolved"} ? "blocked" : "admitted"
observations=rows.map{|r|r.slice("planned_id","issue","title","acceptance_authority","issue_body_sha256","acceptance_rows","linked_prs","closeout_state","closure_disposition","owned_paths","review_evidence","validation_evidence")}
source={"schema"=>"adl.v0921.release_tail_input.v1","candidate"=>candidate,"planning"=>[PLAN,SPEC,CATALOG].map{|p|{"path"=>p.relative_path_from(ROOT).to_s,"sha256"=>sha(p)}},"canonical_planned_ids"=>mapping.keys,"spec_acceptance"=>spec_acceptance,"mapping"=>mapping,"tail_mapping"=>tail_mapping,"amendment_authority"=>{},"backlog"=>backlog_numbers,"retained"=>retained_mapping,"observations"=>observations,"tail_observations"=>tail_rows,"captured_issue_count"=>captured.length,"captured_pages"=>pages.length,"generator_contract"=>"explicit-evidence-only-v4"}
digest=Digest::SHA256.hexdigest(JSON.generate(source))
claims=Hash.new{|h,k|h[k]=[]}; rows.each{|r|r["owned_paths"].each{|path|claims[path]<<r["issue"]}}
collisions=claims.map do |path,owners|
  unique=owners.uniq; next unless unique.length>1
  owner_rows=rows.select{|r|unique.include?(r["issue"])}
  history=owner_rows.map do |r|
    blob=r["revision"] ? git_blob(r["revision"],path) : nil
    link=r["linked_prs"].find{|pr|pr["number"]==r["canonical_pr"]}
    {"issue"=>r["issue"],"head"=>r["revision"],"merge"=>r["merge_revision"],"merged_at"=>link&&link["merged_at"],"blob"=>blob}
  end
  final_blob=git_blob(candidate,path)
  ordered=history.select{|h|h["merged_at"]}.sort_by{|h|h["merged_at"]}
  final_writer=capture("git","log","-1","--format=%H","#{candidate}","--",path).strip
  {"path"=>path,"owners"=>unique,"history"=>history,"final_blob"=>final_blob,"final_writer"=>final_writer,"resolution_authority"=>nil,"status"=>"unresolved"}
end.compact
collisions.select{|c|c["status"]=="unresolved"}.each{|c|findings<<{"id"=>"owned-path-collision-#{Digest::SHA256.hexdigest(c['path'])[0,12]}","type"=>"implementation_gap","severity"=>"P1","classification"=>"release_blockers","summary"=>"Owned path #{c['path']} has multiple unresolved owners.","evidence"=>c["owners"].map{|n|".csdlc/issues/#{n}/cards/spp.values.json"},"uncertainty"=>"none","disposition"=>"open","owner"=>c["owners"].map{|n|"issue ##{n}"}.join(", ")}}
decision=findings.any?{|f|%w[P0 P1].include?(f["severity"])&&f["disposition"]!="resolved"} ? "blocked" : "admitted"
counts={"execution_issues"=>rows.length,"release_tail_stages"=>tail_rows.length,"backlog"=>backlog.length,"retained_predecessors"=>retained.length,"acceptance_rows"=>rows.sum{|r|r["acceptance_rows"].length}+retained.sum{|r|r["acceptance_rows"].length}+tail_rows.sum{|r|r["acceptance_rows"].length}}
versioned_admission="release-tail-admission.#{candidate}.#{digest}.json"; versioned_gap="gap-analysis.#{candidate}.#{digest}.json"
admission={"schema"=>"adl.v0921.release_tail_admission.v2","candidate"=>candidate,"source_digest"=>digest,"output_identity"=>versioned_admission,"denominator_policy"=>"canonical_plan_plus_explicit_amendments_backlog_retained_and_release_tail","counts"=>counts,"execution_issues"=>rows,"release_tail_stages"=>tail_rows,"backlog"=>backlog,"retained_predecessors"=>retained,"ownership_collisions"=>collisions,"findings"=>findings,"decision"=>decision}
gap={"schema"=>"adl.gap_analysis_report.v2","mode"=>"compare_canonical_plan_to_immutable_evidence","candidate"=>candidate,"source_digest"=>digest,"execution_issues"=>rows.map{|r|r.slice("planned_id","issue","revision","merge_revision","merge_ancestry","disposition","acceptance_rows")},"backlog"=>backlog,"retained_predecessors"=>retained,"findings"=>findings,"decision"=>decision}
gap["release_tail_stages"]=tail_rows
projection={"counts"=>counts,"execution_issues"=>gap["execution_issues"],"release_tail_stages"=>tail_rows,"backlog"=>backlog,"retained_predecessors"=>retained,"ownership_collisions"=>collisions,"findings"=>findings,"decision"=>decision}
projection_digest=Digest::SHA256.hexdigest(JSON.generate(projection)); admission["projection_digest"]=projection_digest; gap["projection_digest"]=projection_digest

OUT.mkpath
stem=candidate; paths={source:OUT.join("release-tail-input.#{stem}.#{digest}.json"),admission:OUT.join("release-tail-admission.json"),gap:OUT.join("gap_analysis_report.json"),md:OUT.join("gap_analysis_report.md"),versioned_admission:OUT.join(versioned_admission),versioned_gap:OUT.join(versioned_gap)}
write_immutable=lambda do |path,content|
  abort("immutable evidence drift: #{path}") if path.exist? && path.read!=content
  path.write(content) unless path.exist?
end
source_json=JSON.generate(source)+"\n"; admission_json=JSON.generate(admission)+"\n"; gap_json=JSON.generate(gap)+"\n"
write_immutable.call(paths[:source],source_json); write_immutable.call(paths[:versioned_admission],admission_json); write_immutable.call(paths[:versioned_gap],gap_json)
paths[:admission].write(admission_json); paths[:gap].write(gap_json)
lines=rows.map{|r|"| #{r['planned_id']} | ##{r['issue']} | #{r['revision']||'none'} | #{r['merge_revision']||'none'} | #{r['merge_ancestry']} | #{r['disposition']} |"}
tail_lines=tail_rows.map{|r|"| #{r['planned_id']} | ##{r['issue']} | #{r['observed_state']} | #{r['expected_lifecycle']} | #{r['gate_role']} |"}
fl=findings.empty? ? ["No unresolved findings."] : findings.map{|f|"- **#{f['severity']} #{f['id']}** — #{f['summary']} Evidence: #{f['evidence'].join(', ')}. Owner: #{f['owner']}. Disposition: #{f['disposition']}."}
md="# v0.92.1 Release-tail Gap Analysis\n\nCandidate: `#{candidate}`\n\nCaptured-input digest: `#{digest}`\n\nCanonical projection digest: `#{projection_digest}`\n\n## Findings\n\n#{fl.join("\n")}\n\n## Denominator\n\nExecution roots: #{rows.length}; release-tail stages: #{tail_rows.length}; retained predecessors: #{retained.length}; backlog dispositions: #{backlog.length}; acceptance rows: #{counts['acceptance_rows']}.\n\n| Planned ID | Issue | Head revision | Merge revision | Ancestry | Disposition |\n|---|---:|---|---|---|---|\n#{lines.join("\n")}\n\n### Release-tail lifecycle denominator\n\n| Planned ID | Issue | Observed state | Expected lifecycle | Gate role |\n|---|---:|---|---|---|\n#{tail_lines.join("\n")}\n\n## Backlog and retained authority\n\n#{backlog.map{|r|"- ##{r['issue']}: `#{Digest::SHA256.hexdigest(JSON.generate(r['disposition_authority']))}`"}.join("\n")}\n- Retained predecessor packets are indexed with SHA-256 digests in `#{paths[:gap].basename}`.\n\n## Decision\n\n**#{decision.upcase}**\n\nThis is an admission decision only; it is not release approval.\n"
ac_lines=rows.flat_map{|r|r["acceptance_rows"].map{|a|"| #{r['planned_id']} | ##{r['issue']} | #{a['id']} | #{a['evidence_status']} | #{a['text'].gsub('|','/')} |"}}
retained_lines=retained.flat_map{|r|r["acceptance_rows"].map{|a|"| #{r['planned_id']} | ##{r['issue']} | #{a['id']} | #{a['observed_status']} | #{a['text'].gsub('|','/')} |"}}
collision_lines=collisions.map{|c|"| #{c['path'].gsub('|','/')} | #{c['owners'].join(',')} | #{c['status']} |"}; backlog_lines=backlog.map{|r|"| ##{r['issue']} | #{r['disposition']} | #{Digest::SHA256.hexdigest(JSON.generate(r['disposition_authority']))} |"}
md += "\n## Complete acceptance projection\n\n| Planned ID | Issue | Criterion ID | Status | Criterion |\n|---|---:|---|---|---|\n#{ac_lines.join("\n")}\n\n## Complete retained projection\n\n| Successor | Retained | Criterion ID | Status | Criterion |\n|---|---:|---|---|---|\n#{retained_lines.join("\n")}\n\n## Complete collision projection\n\n| Path | Owners | Status |\n|---|---|---|\n#{collision_lines.join("\n")}\n\n## Complete backlog projection\n\n| Issue | Disposition | Authority digest |\n|---|---|---|\n#{backlog_lines.join("\n")}\n"
paths[:md].write(md)
puts JSON.generate(schema:"adl.v0921.release_tail_generation.v2",status:"pass",candidate:candidate,source_digest:digest,counts:counts,decision:decision)
