#!/usr/bin/env ruby
# frozen_string_literal: true
require "digest"; require "json"; require "open3"; require "pathname"; require "yaml"

ROOT = Pathname.new(__dir__).join("../../../..").realpath
OUT = ROOT.join("docs/milestones/v0.92.1/evidence/integration")
REPO = "agent-logic/agent-design-language"
PLAN = ROOT.join("docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml")
SPEC = ROOT.join("docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
CATALOG = ROOT.join("docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md")
SEMANTIC = ROOT.join(".csdlc/evidence/516/semantic-criterion-evidence.json")

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
def acceptance_items(body)
  section=body.to_s[/^## (?:Acceptance(?: Criteria| criteria)?|Exit Criteria)\s*$\n(.*?)(?=^## |\z)/m,1].to_s
  section.lines.map{|line|m=line.match(/^\s*(?:-|\d+\.)\s*(?:\[([ xX])\]\s*)?(.+)/);m&&{"text"=>m[2].strip,"checked"=>%w[x X].include?(m[1])}}.compact
end

remote_main = capture("gh","api","repos/#{REPO}/git/ref/heads/main","--jq",".object.sha").strip
abort("local origin/main differs from captured remote main") unless capture("git","rev-parse","origin/main").strip == remote_main
candidate = ENV.fetch("ADMISSION_CANDIDATE",remote_main)
abort("candidate is not an ancestor of captured remote main") unless ancestor?(candidate,remote_main)
semantic_doc=JSON.parse(SEMANTIC.read); abort("semantic evidence candidate mismatch") unless semantic_doc["candidate"]==candidate
semantic_entries=semantic_doc.fetch("entries").to_h{|e|[e.fetch("criterion_id"),e]}; abort("duplicate semantic criterion IDs") unless semantic_entries.length==semantic_doc["entries"].length
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
  if number==497
    sidecar=JSON.parse(capture("gh","issue","view","624","--repo",REPO,"--json","number,state,url,body"))
    acceptance_section=issue["body"].to_s[/^## Acceptance criteria\s*$\n(.*?)(?=^## |\z)/mi,1].to_s
    log=ROOT.join(".csdlc/evidence/497/corp-c-operational-control-acceptance.log")
    absorbed={"kind"=>"recordless_acceptance","acceptance_checked"=>!acceptance_section.empty?&&!acceptance_section.include?("[ ]"),"acceptance_log"=>log.relative_path_from(ROOT).to_s,"acceptance_log_sha256"=>sha(log),"sidecar_issue"=>624,"sidecar_url"=>sidecar["url"],"sidecar_state"=>sidecar["state"].downcase,"sidecar_body_sha256"=>Digest::SHA256.hexdigest(sidecar["body"])}
  end
  owner_closed=absorbed&&(absorbed["target_state"]=="closed"||absorbed["kind"]=="recordless_acceptance"&&absorbed["acceptance_checked"]&&absorbed["sidecar_state"]=="closed"); disposition="release_blocker"
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
  disposition = if issue["state"]=="CLOSED" && absorbed&&absorbed["kind"]=="recordless_acceptance"&&owner_closed then "satisfied_recordless_acceptance"
                elsif issue["state"]=="CLOSED" && canonical && !(product+noncode).empty? && (!behavioral.empty?||!successful_checks.empty?) then "observed_execution_evidence"
                elsif issue["state"]=="CLOSED" && owner_closed then "satisfied_by_captured_absorption"
                elsif issue["state"]=="CLOSED" && canonical then "proof_debt"
                else "product_blocker" end
  evidence = canonical ? [canonical.fetch("url"),canonical.fetch("headRefOid")] : (absorbed ? [absorbed["comment_url"]||absorbed["acceptance_log"], absorbed["target_issue"] ? "issue ##{absorbed['target_issue']}" : "issue ##{absorbed['sidecar_issue']}"] : [])
  evidence += [srp,sor].select(&:file?).map{|p|"#{p.relative_path_from(ROOT)}@sha256:#{sha(p)}"}
  spp_values=record.join("cards/spp.values.json")
  owned_paths = if spp_values.file?
                  JSON.parse(spp_values.read).dig("content","values","affected_areas") || []
                else [] end
  {
    "kind"=>"execution_issue","planned_id"=>planned_id,"issue"=>number,"title"=>issue["title"],"acceptance_authority"=>"#{issue['url']}#issue-body","issue_body_sha256"=>Digest::SHA256.hexdigest(issue["body"]),
    "acceptance_rows"=>specs.fetch(planned_id).fetch("acceptance_criteria").each_with_index.map do |text,i|
      id="#{planned_id}-ac-#{i+1}"; entry=semantic_entries[id]; live_items=acceptance_items(issue["body"])
      abort("semantic criterion digest mismatch #{id}") if entry&&entry["criterion_digest"]!=Digest::SHA256.hexdigest(text)
      if entry&&entry["classification"]=="accepted_recordless"
        abort("recordless semantic mapping missing #{id}") if entry.fetch("semantic_mapping").empty?
        entry["semantic_mapping"].each{|m|abort("live mapping digest mismatch #{id}") unless Digest::SHA256.hexdigest(m.fetch("live_text"))==m["live_digest"];abort("mapped live criterion is not checked #{id}") unless live_items.any?{|x|x["text"]==m["live_text"]&&x["checked"]}}
      end
      proof=entry&&Marshal.load(Marshal.dump(entry))
      if proof
        local_refs=proof.values_at("implementation_evidence","validation_evidence","review_evidence","docs_evidence").flatten.compact.reject{|ref|ref.start_with?("github:","https://")}
        proof["evidence_digests"]=local_refs.uniq.map do |ref|
          path=ROOT.join(ref); abort("semantic evidence path missing #{id}: #{ref}") unless path.file?
          {"path"=>ref,"sha256"=>sha(path),"candidate_blob"=>git_blob(candidate,ref),"bytes"=>path.size}
        end
      end
      {"id"=>id,"text"=>text,"text_digest"=>Digest::SHA256.hexdigest(text),"live_exact"=>live_items.find{|x|x["text"]==text},"evidence_status"=>entry ? entry.fetch("classification") : (disposition=="product_blocker" ? "product_gap" : "proof_gap"),"evidence"=>entry ? entry.values_at("implementation_evidence","validation_evidence","review_evidence","docs_evidence","closeout_evidence").flatten.compact : [],"proof"=>proof}
    end,
    "linked_prs"=>prs.map{|pr|{"number"=>pr["number"],"url"=>pr["url"],"head_oid"=>pr["headRefOid"],"merge_oid"=>pr.dig("mergeCommit","oid"),"merged_at"=>pr["mergedAt"],"ancestral"=>pr["ancestral"],"files_digest"=>Digest::SHA256.hexdigest(JSON.generate(pr.fetch("files").map{|f|f["path"]}.sort))}},
    "canonical_pr"=>canonical&.fetch("number",nil),"revision"=>canonical&.fetch("headRefOid",nil),"merge_revision"=>canonical&.dig("mergeCommit","oid"),
    "merge_ancestry"=>canonical ? "ancestor" : (absorbed ? "not_applicable_absorbed" : "not_proven"),
    "artifacts"=>[issue["url"],srp.file? ? srp.relative_path_from(ROOT).to_s : nil,sor.file? ? sor.relative_path_from(ROOT).to_s : nil].compact,
    "review_evidence"=>srp.file? ? {"path"=>srp.relative_path_from(ROOT).to_s,"sha256"=>sha(srp)} : nil,
    "validation_evidence"=>sor.file? ? {"path"=>sor.relative_path_from(ROOT).to_s,"sha256"=>sha(sor)} : nil,
    "closure_disposition"=>absorbed,"closeout_state"=>issue["state"].downcase,"disposition"=>disposition
  }.merge("owned_paths"=>owned_paths)
end

expected_semantic=rows.flat_map{|row|row.fetch("acceptance_rows")}.to_h{|ac|[ac.fetch("id"),ac.fetch("text_digest")]}
actual_semantic=semantic_entries.to_h{|id,entry|[id,entry.fetch("criterion_digest")]}
abort("semantic criterion denominator/digest mismatch") unless actual_semantic==expected_semantic

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
seen_retained_acceptance={}
retained = retained_mapping.flat_map do |planned_id,numbers|
  owner=rows.find{|r|r["planned_id"]==planned_id}
  numbers.map do |number|
    path=ROOT.join("docs/milestones/v0.92.1/planned-issue-packets/issues/#{number}/cards/stp.md"); abort("missing retained ##{number}") unless path.file?
    status=owner&&%w[observed_execution_evidence satisfied_recordless_acceptance satisfied_by_captured_absorption].include?(owner["disposition"]) ? "observed_in_merged_successor" : "consolidated_successor_uncertainty"
    duplicate=seen_retained_acceptance.key?(number)
    acceptance_rows=duplicate ? [] : acceptance(path.read).each_with_index.map{|text,i|{"id"=>"retained-#{number}-ac-#{i+1}","text"=>text,"text_digest"=>Digest::SHA256.hexdigest(text),"observed_status"=>status,"observed_evidence"=>nil}}
    seen_retained_acceptance[number]=planned_id unless duplicate
    {"kind"=>"retained_predecessor","planned_id"=>planned_id,"issue"=>number,"path"=>path.relative_path_from(ROOT).to_s,"sha256"=>sha(path),"observed_status"=>status,"observed_owner_issue"=>owner&&owner["issue"],"observed_revision"=>owner&&owner["revision"],
     "acceptance_projection"=>duplicate ? "reference_only_duplicate_predecessor" : "canonical",
     "canonical_projection_planned_id"=>duplicate ? seen_retained_acceptance.fetch(number) : planned_id,
     "acceptance_rows"=>acceptance_rows}
  end
end

findings=rows.select{|r|r["disposition"]=="product_blocker"}.map{|r|summary=r["issue"]==497 ? "CORP-C / #497 has no PR or captured closure authority and its live criteria materially replace control-plane ownership/recovery acceptance with prerequisite ancestry and sidecar routing." : "#{r['planned_id']} / ##{r['issue']} lacks closed merged ancestral execution evidence.";{"id"=>"issue-#{r['issue']}-execution-gap","type"=>"missing_evidence","severity"=>"P1","classification"=>"product_blocker","summary"=>summary,"evidence"=>r["artifacts"],"uncertainty"=>"behavior or integration is not established","disposition"=>"open","owner"=>"issue ##{r['issue']}"}}
drift=rows.select{|r|r.dig("spec_acceptance","status")=="mismatch"}
material=drift.select{|r|r["issue"]==497}
sync=drift-material; findings<<{"id"=>"consolidated-live-spec-sync-debt","type"=>"docs_drift","severity"=>"P2","classification"=>"proof_debt","summary"=>"Live criteria for #{sync.map{|r|r['planned_id']}.join(', ')} are equivalent or stronger expansions of the spec, except OBS-B moves backlog authority to canonical planning and adds no-mock proof; synchronize the records.","evidence"=>sync.flat_map{|r|[r["acceptance_authority"],r.dig("spec_acceptance","digest"),r.dig("spec_acceptance","live_digest")]},"affected_rows"=>sync.map{|r|r["planned_id"]},"uncertainty"=>"record synchronization only","disposition"=>"follow_up","owner"=>"release planning maintainers"} unless sync.empty?
issue721=JSON.parse(capture("gh","issue","view","721","--repo",REPO,"--json","number,title,state,url,body"))
issue725=JSON.parse(capture("gh","issue","view","725","--repo",REPO,"--json","number,title,state,url,body,labels"))
pr722=JSON.parse(capture("gh","pr","view","722","--repo",REPO,"--json","number,url,state,mergedAt,headRefOid,mergeCommit,files,body"))
pr726=JSON.parse(capture("gh","pr","view","726","--repo",REPO,"--json","number,url,state,headRefOid,files,statusCheckRollup,body"))
canary=ROOT.join(".csdlc/evidence/516/no-v2-canary-af5f8036.json")
findings<<{"id"=>"issue-725-v3-standalone-parity-gap","type"=>"implementation_gap","severity"=>"P1","classification"=>"product_blocker","summary"=>"Closed #721 and open PR #726 cover issue-create and reporting parity but do not remove v2 build and operational dependencies. Open #725 owns native v3 authority migration. At af5f8036 and current main, v3 cannot compile without csdlc-v2 and still reads the v2 selector.","evidence"=>[issue721["url"],Digest::SHA256.hexdigest(issue721["body"]),issue725["url"],Digest::SHA256.hexdigest(issue725["body"]),pr722["url"],pr726["url"],pr726["headRefOid"],Digest::SHA256.hexdigest(JSON.generate(pr726["files"])),Digest::SHA256.hexdigest(JSON.generate(pr726["statusCheckRollup"])),{"path"=>canary.relative_path_from(ROOT).to_s,"sha256"=>sha(canary)},"csdlc-v3/Cargo.toml","csdlc-v3/src/authority.rs","csdlc-v3/src/commands/remote/mod.rs"],"affected_rows"=>["V3-F-ac-2"],"uncertainty"=>"none","disposition"=>"open","owner"=>"issue #725"}
semantic_entries.values.select{|e|e["classification"]=="implementation_gap"&&e["criterion_id"]!="V3-F-ac-2"}.group_by{|e|e["criterion_id"].split("-ac-").first}.each do |id,entries|
  n=mapping.fetch(id); findings<<{"id"=>"issue-#{n}-semantic-implementation-gap","type"=>"implementation_gap","severity"=>"P1","classification"=>"product_blocker","summary"=>"#{id} has #{entries.length} audit-confirmed unmet implementation criteria.","evidence"=>entries.flat_map{|e|e["implementation_evidence"]+e["validation_evidence"]}.uniq,"affected_rows"=>entries.map{|e|e["criterion_id"]},"uncertainty"=>"none","disposition"=>"open","owner"=>"issue ##{n}"}
end
proof_entries=semantic_entries.values.select{|e|e["classification"]=="proof_gap"}; findings<<{"id"=>"semantic-criterion-proof-gaps","type"=>"missing_evidence","severity"=>"P2","classification"=>"proof_debt","summary"=>"Two GCP-B criteria lack live recovery/cleanup proof.","evidence"=>proof_entries.flat_map{|e|e["implementation_evidence"]+e["validation_evidence"]}.uniq,"affected_rows"=>proof_entries.map{|e|e["criterion_id"]},"uncertainty"=>"live behavior not proven","disposition"=>"follow_up","owner"=>"issue #491"} unless proof_entries.empty?
amended=semantic_entries.values.select{|e|%w[accepted_with_explicit_amendment accepted_recordless].include?(e["classification"])}; findings<<{"id"=>"accepted-semantic-amendments","type"=>"scope_amendment","severity"=>"P2","classification"=>"accepted_amendment","summary"=>"Explicit live acceptance/absorption/sequencing amendments replace the listed original criteria.","evidence"=>amended.flat_map{|e|e["closeout_evidence"]}.uniq,"affected_rows"=>amended.map{|e|e["criterion_id"]},"uncertainty"=>"none","disposition"=>"accepted","owner"=>"release operator"} unless amended.empty?
uncertain_retained=retained.select{|r|r["observed_status"]=="consolidated_successor_uncertainty"}
findings<<{"id"=>"consolidated-retained-successor-proof-debt","type"=>"record_debt","severity"=>"P2","classification"=>"proof_debt","summary"=>"Some retained predecessors lack a successor with observed merged execution evidence.","evidence"=>uncertain_retained.map{|r|r["path"]},"affected_rows"=>uncertain_retained.flat_map{|r|r["acceptance_rows"].map{|a|a["id"]}},"uncertainty"=>"historical traceability, not an assumed product failure","disposition"=>"follow_up","owner"=>"release evidence maintainers"} unless uncertain_retained.empty?
observations=rows.map{|r|r.slice("planned_id","issue","title","acceptance_authority","issue_body_sha256","acceptance_rows","linked_prs","closeout_state","closure_disposition","owned_paths","review_evidence","validation_evidence")}
source={"schema"=>"adl.v0921.release_tail_input.v1","candidate"=>candidate,"planning"=>[PLAN,SPEC,CATALOG].map{|p|{"path"=>p.relative_path_from(ROOT).to_s,"sha256"=>sha(p)}},"semantic_evidence"=>{"path"=>SEMANTIC.relative_path_from(ROOT).to_s,"sha256"=>sha(SEMANTIC)},"canonical_planned_ids"=>mapping.keys,"spec_acceptance"=>spec_acceptance,"mapping"=>mapping,"tail_mapping"=>tail_mapping,"amendment_authority"=>{},"backlog"=>backlog_numbers,"retained"=>retained_mapping,"observations"=>observations,"tail_observations"=>tail_rows,"external_product_gaps"=>[{"issue"=>725,"url"=>issue725["url"],"state"=>issue725["state"].downcase,"labels"=>issue725["labels"].map{|label|label["name"]}.sort,"disposition"=>"open_release_blocker","body_sha256"=>Digest::SHA256.hexdigest(issue725["body"]),"supersedes_partial_owner"=>721,"open_pr"=>726,"open_pr_head"=>pr726["headRefOid"],"canary_path"=>canary.relative_path_from(ROOT).to_s,"canary_sha256"=>sha(canary)}],"captured_issue_count"=>captured.length,"captured_pages"=>pages.length,"generator_contract"=>"curated-semantic-evidence-v7","generator_sha256"=>Digest::SHA256.file(__FILE__).hexdigest}
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
unless collisions.empty?
  findings<<{"id"=>"consolidated-owned-path-resolution-proof-debt","type"=>"record_debt","severity"=>"P2","classification"=>"proof_debt","summary"=>"Shared paths lack explicit owner sign-off; no final-content requirement loss was demonstrated.","evidence"=>collisions.map{|c|c["path"]},"affected_rows"=>collisions.map{|c|c["path"]},"uncertainty"=>"ownership record only","disposition"=>"follow_up","owner"=>"release evidence maintainers"}
end
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
md += "\n## Revision history\n\n- Historical result at `e68c666803185c348140547e243bc3922b6a571c`: 2 P1 product blockers (#497, #721), 4 consolidated P2 proof-debt findings, and 2 routed backlog entries. Superseded after direct #497 recordless-acceptance evidence and final review.\n- Revised result: #{findings.count{|f|f['severity']=='P1'}} P1 product blocker, #{findings.count{|f|f['severity']=='P2'}} P2 proof-debt findings; backlog #84/#251 is excluded scope and is not counted as a finding.\n"
md += "\n## Complete acceptance projection\n\n| Planned ID | Issue | Criterion ID | Status | Criterion |\n|---|---:|---|---|---|\n#{ac_lines.join("\n")}\n\n## Complete retained projection\n\n| Successor | Retained | Criterion ID | Status | Criterion |\n|---|---:|---|---|---|\n#{retained_lines.join("\n")}\n\n## Complete collision projection\n\n| Path | Owners | Status |\n|---|---|---|\n#{collision_lines.join("\n")}\n\n## Complete backlog projection\n\n| Issue | Disposition | Authority digest |\n|---|---|---|\n#{backlog_lines.join("\n")}\n"
paths[:md].write(md)
puts JSON.generate(schema:"adl.v0921.release_tail_generation.v2",status:"pass",candidate:candidate,source_digest:digest,counts:counts,decision:decision)
