#!/usr/bin/env ruby
# frozen_string_literal: true
require "digest"; require "json"; require "open3"; require "pathname"; require "yaml"
ROOT=Pathname.new(__dir__).join("../../../..").realpath
OUT=ROOT.join("docs/milestones/v0.92.1/evidence/integration")
MODE=ARGV.fetch(0,"all")
abort("usage: #{$PROGRAM_NAME} [denominator|gaps|decision|negative|all]") unless %w[denominator gaps decision negative all].include?(MODE)

def load_json(path)
  JSON.parse(path.read)
end
def validate!(source, admission, gap, markdown, require_admitted:)
  raise "wrong source schema" unless source["schema"]=="adl.v0921.release_tail_input.v1"
  raise "wrong admission schema" unless admission["schema"]=="adl.v0921.release_tail_admission.v2"
  raise "wrong gap schema" unless gap["schema"]=="adl.gap_analysis_report.v2"
  digest=Digest::SHA256.hexdigest(JSON.generate(source))
  raise "source digest mismatch" unless admission["source_digest"]==digest && gap["source_digest"]==digest
  raise "candidate mismatch" unless [source["candidate"],admission["candidate"],gap["candidate"]].uniq.one?
  source.fetch("planning").each do |entry|
    path=ROOT.join(entry.fetch("path")); raise "planning source missing" unless path.file?
    raise "planning source digest drift" unless Digest::SHA256.file(path).hexdigest==entry["sha256"]
  end
  spec_path=ROOT.join("docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml")
  specs=YAML.safe_load(spec_path.read).fetch("issue_specifications").to_h{|s|[s.fetch("id"),s]}
  plan_path=ROOT.join("docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml"); declared=[]; walk=lambda{|x|x.is_a?(Hash) ? (declared<<x["id"] if x["id"];x.each_value{|v|walk.call(v)}) : (x.each{|v|walk.call(v)} if x.is_a?(Array))};walk.call(YAML.safe_load(plan_path.read).fetch("work_packages"))
  catalog=ROOT.join("docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md").read; catalog_ids=catalog.scan(/^\| ([A-Z][A-Z0-9-]+) \|/).flatten.select{|id|specs.key?(id)}
  canonical_ids=source.fetch("canonical_planned_ids")+source.fetch("tail_mapping").keys
  raise "wave/catalog/spec planned-ID parity mismatch" unless canonical_ids.sort==specs.keys.sort && (declared&specs.keys).sort==specs.keys.sort && catalog_ids.sort==specs.keys.sort
  expected_specs=source.fetch("canonical_planned_ids").to_h{|id|spec=specs.fetch(id);[id,{"acceptance_criteria"=>spec.fetch("acceptance_criteria"),"digest"=>Digest::SHA256.hexdigest(JSON.generate(spec))}]}
  raise "spec acceptance denominator mismatch" unless source["spec_acceptance"]==expected_specs
  raise "amendment authority incomplete" unless source.fetch("amendment_authority").all?{|id,a|source.fetch("mapping")[id]==a["issue"] && a["url"]&.start_with?("https://github.com/") && a["issue_body_sha256"]&.match?(/\A[0-9a-f]{64}\z/) && a["source"]=="explicit_INT_01_amendment_mapping"}
  versioned=OUT.join(admission.fetch("output_identity")); raise "versioned admission missing" unless versioned.file? && versioned.read==JSON.generate(admission)+"\n"
  versioned_gap=OUT.join("gap-analysis.#{admission['candidate']}.#{digest}.json"); raise "versioned gap missing" unless versioned_gap.file? && versioned_gap.read==JSON.generate(gap)+"\n"
  rows=admission.fetch("execution_issues"); mapping=source.fetch("mapping")
  raise "execution denominator mismatch" unless rows.to_h{|r|[r["planned_id"],r["issue"]]}==mapping
  raise "duplicate issue mapping" unless rows.map{|r|r["issue"]}.uniq.length==rows.length
  observations=rows.map{|r|r.slice("planned_id","issue","title","acceptance_authority","issue_body_sha256","acceptance_rows","linked_prs","closeout_state","closure_disposition","owned_paths","review_evidence","validation_evidence")}
  raise "captured observation mismatch" unless source["observations"]==observations
  projection=rows.map{|r|r.slice("planned_id","issue","revision","merge_revision","merge_ancestry","disposition","acceptance_rows")}
  raise "gap execution projection mismatch" unless gap["execution_issues"]==projection
  tails=admission.fetch("release_tail_stages"); raise "tail denominator mismatch" unless tails.to_h{|r|[r["planned_id"],r["issue"]]}==source.fetch("tail_mapping") && tails==source["tail_observations"] && tails==gap["release_tail_stages"]
  raise "tail stage became circular gate" unless tails.all?{|r|r["gate_role"]=="denominator_only_not_execution_root" && %w[active_admission_work future_serial_stage].include?(r["expected_lifecycle"])}
  rows.each do |row|
    raise "empty acceptance denominator #{row['issue']}" if row.fetch("acceptance_rows").empty?
    if row["disposition"]=="satisfied"
      raise "satisfied row lacks canonical PR" unless row["canonical_pr"] && row["revision"]&.match?(/\A[0-9a-f]{40}\z/) && row["merge_revision"]&.match?(/\A[0-9a-f]{40}\z/)
      selected=row.fetch("linked_prs").find{|pr|pr["number"]==row["canonical_pr"]}
      raise "canonical PR missing from linked PR census" unless selected && selected["head_oid"]==row["revision"] && selected["merge_oid"]==row["merge_revision"] && selected["ancestral"]
      raise "satisfied row lacks ancestry" unless row["merge_ancestry"]=="ancestor"
      ancestral=system("git","merge-base","--is-ancestor",row["merge_revision"],admission["candidate"],chdir:ROOT.to_s,out:File::NULL,err:File::NULL)
      raise "recorded merge is not actually ancestral" unless ancestral
    elsif row["disposition"]=="satisfied_by_explicit_no_pr_closure"
      closure=row.fetch("closure_disposition")
      raise "explicit no-PR closure authority missing" unless closure["authority"] && closure["kind"] && closure["issue_url"]&.start_with?("https://github.com/") && closure["issue_state"]=="closed" && closure["body_sha256"]&.match?(/\A[0-9a-f]{64}\z/) && !closure["comments"].to_a.empty? && closure["comments"].all?{|c|c["url"]&.start_with?("https://github.com/")&&c["body_sha256"]&.match?(/\A[0-9a-f]{64}\z/)}
    end
    %w[review_evidence validation_evidence].each do |key|
      next unless row[key]
      path=ROOT.join(row[key].fetch("path")); raise "#{key} artifact missing" unless path.file?
      raise "#{key} digest mismatch" unless Digest::SHA256.file(path).hexdigest==row[key]["sha256"]
    end
    row["acceptance_rows"].each do |ac|
      raise "acceptance identity/text missing" if ac["id"].to_s.empty? || ac["text"].to_s.empty?
      linked=ac["evidence_status"]=="evidence_linked" && !ac.fetch("evidence").empty?
      if linked && row["canonical_pr"]
        proof=ac.fetch("proof")
        %w[production_call_path_or_noncode behavioral_validation exact_head_review docs_demo_relevance successful_checks].each{|key|raise "criterion proof missing #{key}" if proof[key].to_a.empty?}
        %w[criterion_content criterion_validation].each do |key|
          raise "criterion content mapping missing #{key}" if proof[key].to_a.empty?
          proof[key].each{|ref|raise "criterion blob reference invalid" unless ref["path"] && ref["blob"]&.match?(/\A[0-9a-f]{40}\z/)}
        end
        raise "criterion validation status missing" unless proof["check_status"].to_a.all?{|c|c["conclusion"]=="SUCCESS"} && !proof["check_status"].empty?
        raise "criterion review is stale or followed by substantive changes" unless proof.dig("review_basis","current")==true && proof.dig("review_basis","reviewed_revision")&.match?(/\A[0-9a-f]{40}\z/) && proof.dig("review_basis","post_review_paths").to_a.all?{|p|p.start_with?(".csdlc/")}
        raise "criterion proof is vacuous" if proof.values.flatten.any?{|value|value.to_s.match?(/(?:stub|placeholder|do[-_ ]?nothing)/i)}
      end
      explicitly_blocked=row["disposition"]=="release_blocker" && admission.fetch("findings").any?{|f|f["id"]=="issue-#{row['issue']}-review-or-terminal-gap" && %w[P0 P1].include?(f["severity"])}
      raise "acceptance has unclassified missing/placeholder/do-nothing evidence" unless linked || explicitly_blocked
    end
  end
  retained=admission.fetch("retained_predecessors")
  raise "retained projection mismatch" unless retained==gap["retained_predecessors"]
  retained.each do |row|
    path=ROOT.join(row.fetch("path")); raise "retained artifact missing" unless path.file?
    raise "retained digest mismatch" unless Digest::SHA256.file(path).hexdigest==row["sha256"]
    raise "retained acceptance empty" if row.fetch("acceptance_rows").empty?
    raise "retained observed status missing" unless %w[observed_in_successor successor_proof_gap carried_into_int_01].include?(row["observed_status"])
    if row["observed_status"]=="observed_in_successor"
      raise "retained observed evidence incomplete" unless row["acceptance_rows"].all?{|ac|ac.dig("observed_evidence","evidence_status")=="evidence_linked" && !ac.dig("observed_evidence","criterion_content").to_a.empty? && !ac.dig("observed_evidence","criterion_validation").to_a.empty?}
    end
  end
  raise "backlog projection mismatch" unless admission["backlog"]==gap["backlog"]
  admission.fetch("backlog").each{|r|raise "backlog authority missing" if r["disposition_authority"].to_s.empty?}
  claims=Hash.new{|h,k|h[k]=[]}; rows.each{|r|r.fetch("owned_paths").each{|path|claims[path]<<r["issue"]}}
  actual_collisions=admission.fetch("ownership_collisions")
  raise "ownership collision denominator mismatch" unless actual_collisions.map{|c|[c["path"],c["owners"]]}==claims.map{|path,owners|[path,owners.uniq] if owners.uniq.length>1}.compact
  actual_collisions.select{|c|c["status"]=="resolved_by_ordered_content"}.each do |collision|
    final_blob,_err,status=Open3.capture3("git","rev-parse","#{admission['candidate']}:#{collision['path']}",chdir:ROOT.to_s)
    final_blob=final_blob.strip
    writer,_writer_err,writer_status=Open3.capture3("git","log","-1","--format=%H",admission["candidate"],"--",collision["path"],chdir:ROOT.to_s)
    valid_history=collision.fetch("history").all?{|h|h["merge"]&&h["blob"]&&system("git","merge-base","--is-ancestor",h["merge"],admission["candidate"],chdir:ROOT.to_s,out:File::NULL,err:File::NULL)}
    raise "collision final content drift" unless status.success? && writer_status.success? && final_blob==collision["final_blob"] && writer.strip==collision["final_writer"] && valid_history
  end
  actual_collisions.select{|c|c["status"]=="unresolved"}.each do |collision|
    prefix="owned-path-collision-#{Digest::SHA256.hexdigest(collision['path'])[0,12]}"
    raise "ownership collision not classified" unless admission.fetch("findings").any?{|f|f["id"]==prefix && %w[P0 P1].include?(f["severity"])}
  end
  findings=admission.fetch("findings"); raise "finding projection mismatch" unless findings==gap["findings"]
  findings.each do |f|
    raise "invalid/unowned finding" unless %w[P0 P1 P2 P3].include?(f["severity"]) && !f["evidence"].to_a.empty? && !f["owner"].to_s.empty? && !f["disposition"].to_s.empty? && !f["uncertainty"].to_s.empty?
  end
  expected=findings.any?{|f|%w[P0 P1].include?(f["severity"])&&f["disposition"]!="resolved"} ? "blocked" : "admitted"
  raise "decision is not fail closed" unless admission["decision"]==expected && gap["decision"]==expected
  canonical_projection={"counts"=>admission["counts"],"execution_issues"=>gap["execution_issues"],"release_tail_stages"=>tails,"backlog"=>admission["backlog"],"retained_predecessors"=>retained,"ownership_collisions"=>actual_collisions,"findings"=>findings,"decision"=>expected}
  projection_digest=Digest::SHA256.hexdigest(JSON.generate(canonical_projection))
  raise "canonical projection digest mismatch" unless admission["projection_digest"]==projection_digest && gap["projection_digest"]==projection_digest
  raise "Markdown candidate mismatch" unless markdown.include?("Candidate: `#{admission['candidate']}`")
  raise "Markdown digest mismatch" unless markdown.include?("Captured-input digest: `#{digest}`")
  raise "Markdown projection mismatch" unless markdown.include?("Canonical projection digest: `#{projection_digest}`")
  rows.each{|r|raise "Markdown row omitted" unless markdown.include?("| #{r['planned_id']} | ##{r['issue']} |")}
  tails.each{|r|raise "Markdown tail row omitted" unless markdown.include?("| #{r['planned_id']} | ##{r['issue']} | #{r['observed_state']} | #{r['expected_lifecycle']} |")}
  findings.each{|f|raise "Markdown finding omitted" unless markdown.include?(f["id"])}
  raise "admission remains blocked" if require_admitted && expected!="admitted"
  true
end

admission=load_json(OUT.join("release-tail-admission.json")); source=load_json(OUT.join("release-tail-input.#{admission.fetch('candidate')}.json")); gap=load_json(OUT.join("gap_analysis_report.json")); markdown=OUT.join("gap_analysis_report.md").read
if %w[negative all].include?(MODE)
  cases={
    "omitted-root"=>->(s,_a,_g,_m){s["mapping"].delete(s["mapping"].keys.first)},
    "null-revision"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["disposition"]=="satisfied"}["revision"]=nil},
    "fake-ancestry"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["disposition"]=="satisfied"}["merge_ancestry"]="not_proven"},
    "empty-acceptance"=>->(_s,a,_g,_m){a["execution_issues"].first["acceptance_rows"]=[]},
    "do-nothing"=>->(_s,a,_g,_m){a["execution_issues"].first["acceptance_rows"].first["evidence_status"]="placeholder"},
    "missing-production-proof"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["canonical_pr"]}["acceptance_rows"].first["proof"]["production_call_path_or_noncode"]=[]},
    "missing-behavior-proof"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["canonical_pr"]}["acceptance_rows"].first["proof"]["behavioral_validation"]=[]},
    "missing-criterion-content"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["canonical_pr"]}["acceptance_rows"].first["proof"]["criterion_content"]=[]},
    "false-validation-status"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["canonical_pr"]}["acceptance_rows"].first["proof"]["check_status"].first["conclusion"]="FAILURE"},
    "missing-review-proof"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["canonical_pr"]}["acceptance_rows"].first["proof"]["exact_head_review"]=[]},
    "stale-review-basis"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["canonical_pr"]}["acceptance_rows"].first["proof"]["review_basis"]["current"]=false},
    "vacuous-proof"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["canonical_pr"]}["acceptance_rows"].first["proof"]["behavioral_validation"]=["placeholder"]},
    "invented-no-pr-authority"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["closure_disposition"]}["closure_disposition"].delete("comments_sha256")},
    "missing-artifact"=>->(_s,a,_g,_m){a["retained_predecessors"].first["path"]="missing"},
    "stale-review-digest"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["review_evidence"]}["review_evidence"]["sha256"]="0"*64},
    "gap-mismatch"=>->(_s,_a,g,_m){g["execution_issues"]=[]},
    "unowned-finding"=>->(_s,a,g,_m){a["findings"].first["owner"]="";g["findings"]=a["findings"]},
    "collision"=>->(_s,a,_g,_m){a["ownership_collisions"]<<{"path"=>"x"}},
    "false-collision-resolution"=>->(_s,a,_g,_m){c=a["ownership_collisions"].find{|x|x["status"]=="resolved_by_ordered_content"};c["final_blob"]="0"*40 if c},
    "missing-amendment-authority"=>->(s,_a,_g,_m){s["amendment_authority"].delete(s["amendment_authority"].keys.first)},
    "spec-acceptance-drift"=>->(s,_a,_g,_m){s["spec_acceptance"].values.first["acceptance_criteria"]=[]},
    "output-identity-drift"=>->(_s,a,_g,_m){a["output_identity"]="missing.json"},
    "projection-digest-drift"=>->(_s,a,_g,_m){a["projection_digest"]="0"*64},
    "admitted-with-blocker"=>->(_s,a,g,_m){a["decision"]=g["decision"]="admitted"},
    "markdown-omission"=>->(_s,_a,_g,m){m.replace("")}
  }
  cases.each do |name,mutation|
    s,a,g=Marshal.load(Marshal.dump([source,admission,gap])); m=markdown.dup; mutation.call(s,a,g,m)
    begin; validate!(s,a,g,m,require_admitted:false); abort("negative fixture accepted: #{name}"); rescue RuntimeError; end
  end
end
validate!(source,admission,gap,markdown,require_admitted:%w[decision all].include?(MODE)) unless MODE=="negative"
puts JSON.generate(schema:"adl.v0921.release_tail_validation.v2",mode:MODE,status:"pass")
