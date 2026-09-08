#!/usr/bin/env ruby
# frozen_string_literal: true
require "digest"; require "json"; require "open3"; require "pathname"; require "time"; require "yaml"
ROOT=Pathname.new(__dir__).join("../../../..").realpath
OUT=ROOT.join("docs/milestones/v0.92.1/evidence/integration")
MODE=ARGV.fetch(0,"all")
STARTED_AT=Time.now.utc.iso8601(6)
abort("usage: #{$PROGRAM_NAME} [denominator|gaps|decision|admitted|negative|all]") unless %w[denominator gaps decision admitted negative all].include?(MODE)

def load_json(path)
  JSON.parse(path.read)
end
def committed_content!(entry)
  revision=entry.fetch("revision"); path=entry.fetch("path")
  raise "generated evidence revision invalid" unless revision.match?(/\A[0-9a-f]{40}\z/) && system("git","merge-base","--is-ancestor",revision,"HEAD",chdir:ROOT.to_s,out:File::NULL,err:File::NULL)
  content,_err,status=Open3.capture3("git","show","#{revision}:#{path}",chdir:ROOT.to_s)
  raise "generated evidence missing at bound revision" unless status.success?
  raise "generated evidence digest/size drift" unless Digest::SHA256.hexdigest(content)==entry["sha256"] && content.bytesize==entry["bytes"]
  blob,_blob_err,blob_status=Open3.capture3("git","rev-parse","#{revision}:#{path}",chdir:ROOT.to_s)
  raise "generated evidence blob drift" unless blob_status.success? && blob.strip==entry["candidate_blob"]
  content
end
def validate_receipts!(source, admission)
  expected={
    "release-tail-denominator.log"=>"denominator",
    "implementation-gap-analysis.log"=>"gaps",
    "admission-consistency.log"=>"decision"
  }
  expected.each do |name,mode|
    receipt=load_json(ROOT.join(".csdlc/evidence/516",name))
    raise "validation receipt schema mismatch" unless receipt["schema"]=="adl.v0921.release_tail_validation_receipt.v1"
    raise "validation receipt candidate/source mismatch" unless receipt["candidate"]==source["candidate"] && receipt["source_digest"]==admission["source_digest"]
    raise "validation receipt argv mismatch" unless receipt["argv"]==["ruby",".csdlc/prepared/issues/516/validate-release-tail-admission.rb",mode]
    raise "validation receipt failed" unless receipt["exit_code"]==0 && receipt.dig("stdout","mode")==mode && receipt.dig("stdout","status")=="pass"
    raise "validation receipt stdout digest mismatch" unless Digest::SHA256.hexdigest(JSON.generate(receipt["stdout"]))==receipt["stdout_sha256"]
    Time.iso8601(receipt.fetch("started_at")); Time.iso8601(receipt.fetch("ended_at"))
    raise "validation receipt head invalid" unless receipt["exact_head"]&.match?(/\A[0-9a-f]{40}\z/) && system("git","merge-base","--is-ancestor",receipt["exact_head"],"HEAD",chdir:ROOT.to_s,out:File::NULL,err:File::NULL)
    post,_post_err,post_status=Open3.capture3("git","diff","--name-only","#{receipt['exact_head']}..HEAD",chdir:ROOT.to_s)
    raise "validation receipt post-head inspection failed" unless post_status.success?
    raise "validation receipt followed by substantive changes" unless post.lines.map(&:strip).reject(&:empty?).all?{|path|expected.key?(File.basename(path)) && path.start_with?(".csdlc/evidence/516/")}
  end
end
def validate!(source, admission, gap, markdown, require_admitted:)
  raise "wrong source schema" unless source["schema"]=="adl.v0921.release_tail_input.v1"
  raise "wrong admission schema" unless admission["schema"]=="adl.v0921.release_tail_admission.v2"
  raise "wrong gap schema" unless gap["schema"]=="adl.gap_analysis_report.v2"
  digest=Digest::SHA256.hexdigest(JSON.generate(source))
  raise "source digest mismatch" unless admission["source_digest"]==digest && gap["source_digest"]==digest
  raise "candidate mismatch" unless [source["candidate"],admission["candidate"],gap["candidate"]].uniq.one?
  remote_main,_remote_err,remote_status=Open3.capture3("git","rev-parse","origin/main",chdir:ROOT.to_s)
  raise "admission candidate is stale" unless remote_status.success? && source["candidate"]==remote_main.strip
  validate_receipts!(source,admission) unless ENV["ADL_RECORD_VALIDATION_RECEIPT"] == "1"
  source.fetch("planning").each do |entry|
    content,_err,status=Open3.capture3("git","show","#{source.fetch('candidate')}:#{entry.fetch('path')}",chdir:ROOT.to_s)
    raise "planning source missing at candidate" unless status.success?
    raise "planning source digest drift" unless Digest::SHA256.hexdigest(content)==entry["sha256"] && content.bytesize==entry["bytes"]
    blob,_blob_err,blob_status=Open3.capture3("git","rev-parse","#{source.fetch('candidate')}:#{entry.fetch('path')}",chdir:ROOT.to_s)
    raise "planning source blob drift" unless blob_status.success? && blob.strip==entry["candidate_blob"]
  end
  generated=source.fetch("generated_evidence"); expected_generated=[".csdlc/evidence/516/semantic-criterion-evidence.json",".csdlc/evidence/516/no-v2-canary-#{source.fetch('candidate')[0,8]}.json",".csdlc/evidence/516/no-v2-canary-#{source.fetch('candidate')[0,8]}.stderr.log"]
  raise "generated evidence denominator mismatch" unless generated.map{|e|e["path"]}.sort==expected_generated.sort
  generated_content=generated.to_h{|entry|[entry.fetch("path"),committed_content!(entry)]}
  canary=JSON.parse(generated_content.fetch(expected_generated[1])); stderr_entry=canary.fetch("sanitized_stderr")
  raise "no-v2 canary identity/status invalid" unless canary["schema"]=="adl.v0921.no_v2_canary.v2" && canary["candidate"]==source["candidate"] && canary["gap_owner_issue"]==725 && canary["exit_status"]==0 && canary["csdlc_v2_present_during_command"]==false
  raise "no-v2 canary output missing/drifted" unless Digest::SHA256.hexdigest(generated_content.fetch(expected_generated[2]))==stderr_entry["sha256"]
  expected_sources=["csdlc-v3/Cargo.toml","csdlc-v3/src/authority.rs","csdlc-v3/src/commands/remote/mod.rs"]; raise "no-v2 canary source census mismatch" unless canary["source_dependencies"]==expected_sources
  source_contents=expected_sources.to_h do |path|
    content,_err,status=Open3.capture3("git","show","#{source.fetch('candidate')}:#{path}",chdir:ROOT.to_s)
    raise "no-v2 source path missing" unless status.success?
    [path,content]
  end
  raise "manifest still depends on v2" if source_contents.fetch(expected_sources[0]).include?('csdlc-v2 = { path = "../csdlc-v2" }')
  raise "authority still depends on v2 selector" if source_contents.fetch(expected_sources[1]).include?("csdlc-v2/operator/generation-selector.json")
  raise "remote route still depends on v2 selector" if source_contents.fetch(expected_sources[2]).include?("csdlc-v2/operator/generation-selector.json")
  semantic=JSON.parse(generated_content.fetch(expected_generated[0])); raise "semantic manifest candidate mismatch" unless semantic["candidate"]==source["candidate"]
  semantic_source=source.fetch("semantic_evidence"); raise "semantic manifest source identity mismatch" unless semantic_source==generated.find{|e|e["path"]==expected_generated[0]}
  semantic_entries=semantic.fetch("entries"); raise "duplicate semantic criterion evidence" unless semantic_entries.map{|e|e["criterion_id"]}.uniq.length==semantic_entries.length
  semantic_entries.each do |entry|
    raise "invalid semantic classification" unless %w[proven accepted_recordless accepted_with_explicit_amendment implementation_gap proof_gap product_gap].include?(entry["classification"])
    %w[implementation_evidence validation_evidence review_evidence docs_evidence closeout_evidence].each do |key|
      entry.fetch(key,[]).each do |ref|
        next if ref.start_with?("github:","https://")
        _content,_err,status=Open3.capture3("git","show","#{source.fetch('candidate')}:#{ref}",chdir:ROOT.to_s)
        raise "semantic evidence path missing: #{ref}" unless status.success?
      end
    end
    entry.fetch("semantic_mapping",[]).each{|m|raise "semantic mapping digest mismatch" unless Digest::SHA256.hexdigest(m.fetch("live_text"))==m["live_digest"]}
  end
  spec_text,_spec_err,spec_status=Open3.capture3("git","show","#{source.fetch('candidate')}:docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml",chdir:ROOT.to_s)
  plan_text,_plan_err,plan_status=Open3.capture3("git","show","#{source.fetch('candidate')}:docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml",chdir:ROOT.to_s)
  catalog,_catalog_err,catalog_status=Open3.capture3("git","show","#{source.fetch('candidate')}:docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md",chdir:ROOT.to_s)
  raise "candidate planning denominator missing" unless spec_status.success? && plan_status.success? && catalog_status.success?
  specs=YAML.safe_load(spec_text).fetch("issue_specifications").to_h{|s|[s.fetch("id"),s]}
  declared=[]; walk=lambda{|x|x.is_a?(Hash) ? (declared<<x["id"] if x["id"];x.each_value{|v|walk.call(v)}) : (x.each{|v|walk.call(v)} if x.is_a?(Array))};walk.call(YAML.safe_load(plan_text).fetch("work_packages"))
  catalog_ids=catalog.scan(/^\| ([A-Z][A-Z0-9-]+) \|/).flatten.select{|id|specs.key?(id)}
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
  expected_semantic=rows.flat_map{|row|row.fetch("acceptance_rows")}.to_h{|ac|[ac.fetch("id"),ac.fetch("text_digest")]}
  actual_semantic=semantic_entries.to_h{|entry|[entry.fetch("criterion_id"),entry.fetch("criterion_digest")]}
  raise "semantic criterion denominator/digest mismatch" unless actual_semantic==expected_semantic
  observations=rows.map{|r|r.slice("planned_id","issue","title","acceptance_authority","issue_body_sha256","acceptance_rows","linked_prs","closeout_state","closure_disposition","owned_paths","review_evidence","review_truth","validation_evidence")}
  raise "captured observation mismatch" unless source["observations"]==observations
  projection=rows.map{|r|r.slice("planned_id","issue","revision","merge_revision","merge_ancestry","disposition","acceptance_rows")}
  raise "gap execution projection mismatch" unless gap["execution_issues"]==projection
  tails=admission.fetch("release_tail_stages"); raise "tail denominator mismatch" unless tails.to_h{|r|[r["planned_id"],r["issue"]]}==source.fetch("tail_mapping") && tails==source["tail_observations"] && tails==gap["release_tail_stages"]
  raise "tail stage became circular gate" unless tails.all?{|r|r["gate_role"]=="denominator_only_not_execution_root" && %w[active_admission_work future_serial_stage].include?(r["expected_lifecycle"])}
  rows.each do |row|
    raise "empty acceptance denominator #{row['issue']}" if row.fetch("acceptance_rows").empty?
    truth=row.fetch("review_truth")
    if row["revision"]
      valid_tail=truth["post_review_paths"].to_a.empty? || truth["non_substantive_tail"]==true
      raise "review truth contradicts its evidence" if truth["current"] && !(truth["result"]=="pass" && truth["reviewed_revision"]&.match?(/\A[0-9a-f]{40}\z/) && valid_tail)
      if truth["current"]
        raise "reviewed revision is not ancestral to implementation head" unless system("git","merge-base","--is-ancestor",truth["reviewed_revision"],row["revision"],chdir:ROOT.to_s,out:File::NULL,err:File::NULL)
        post,_post_err,post_status=Open3.capture3("git","diff","--name-only","#{truth['reviewed_revision']}..#{row['revision']}",chdir:ROOT.to_s)
        raise "review-tail inspection failed" unless post_status.success?
        actual_post=post.lines.map(&:strip).reject(&:empty?)
        metadata_tail=actual_post.all? do |path|
          path.match?(%r{\A\.csdlc/issues/#{row['issue']}/(?:audit\.jsonl|index\.json|cards/(?:srp|sor)(?:\.values)?\.(?:md|json))\z}) ||
            path.match?(%r{\A\.csdlc/prepared/issues/#{row['issue']}/(?:publish|review|final-review)[^/]*\.json\z})
        end
        raise "review-tail projection drift" unless actual_post==truth["post_review_paths"] && truth["non_substantive_tail"]==metadata_tail
      end
    end
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
      content,_err,status=Open3.capture3("git","show","#{source.fetch('candidate')}:#{row[key].fetch('path')}",chdir:ROOT.to_s)
      raise "#{key} artifact missing at candidate" unless status.success?
      raise "#{key} digest mismatch" unless Digest::SHA256.hexdigest(content)==row[key]["sha256"] && content.bytesize==row[key]["bytes"]
      blob,_blob_err,blob_status=Open3.capture3("git","rev-parse","#{source.fetch('candidate')}:#{row[key].fetch('path')}",chdir:ROOT.to_s)
      raise "#{key} blob mismatch" unless blob_status.success? && blob.strip==row[key]["candidate_blob"]
      if key=="review_evidence" && row["disposition"]=="observed_execution_evidence"
        raise "review evidence is not successful/current" unless row[key]["result"]=="pass" && row[key]["reviewed_revision"]&.match?(/\A[0-9a-f]{40}\z/) && (row[key]["post_review_paths"].empty? || row[key]["non_substantive_tail"]==true)
      end
    end
    row["acceptance_rows"].each do |ac|
      raise "acceptance identity/text missing" if ac["id"].to_s.empty? || ac["text"].to_s.empty?
      allowed=%w[proven accepted_recordless accepted_with_explicit_amendment implementation_gap proof_gap product_gap]
      raise "criterion classification invalid" unless allowed.include?(ac["evidence_status"])
      proof=ac.fetch("proof")
      if ac["evidence_status"]=="proven"
        raise "proven criterion lacks curated implementation/validation/review" if proof["implementation_evidence"].to_a.empty?||proof["validation_evidence"].to_a.empty?||proof["review_evidence"].to_a.empty?
        refs=proof.values_at("implementation_evidence","validation_evidence","review_evidence","docs_evidence").flatten.compact.reject{|ref|ref.start_with?("github:","https://")}.uniq
        digests=proof.fetch("evidence_digests")
        raise "proven evidence digest denominator mismatch" unless digests.map{|d|d["path"]}.sort==refs.sort
        digests.each do |entry|
          content,_content_stderr,content_status=Open3.capture3("git","show","#{admission['candidate']}:#{entry.fetch('path')}",chdir:ROOT.to_s)
          raise "proven evidence missing" unless content_status.success?
          raise "proven evidence is empty" unless entry.fetch("bytes")>0 && content.bytesize==entry["bytes"]
          raise "proven evidence digest drift" unless Digest::SHA256.hexdigest(content)==entry["sha256"]
          blob,_stderr,status=Open3.capture3("git","rev-parse","#{admission['candidate']}:#{entry['path']}",chdir:ROOT.to_s)
          raise "proven evidence candidate blob mismatch" unless status.success? && entry["candidate_blob"] && entry["candidate_blob"]==blob.strip
          normalized=content.strip.downcase.gsub(/[^a-z0-9]+/," ").strip
          raise "proven evidence is vacuous: #{entry['path']}" if normalized.match?(/\A(?:stub|placeholder|do nothing|todo|tbd)(?: evidence| only)?\z/)
        end
      elsif %w[accepted_recordless accepted_with_explicit_amendment].include?(ac["evidence_status"])
        raise "amended criterion lacks explicit closeout/rationale" if proof["closeout_evidence"].to_a.empty?||proof["rationale"].to_s.empty?
        if ac["evidence_status"]=="accepted_with_explicit_amendment"
          raise "amended criterion lacks explicit authority mapping" unless source.fetch("amendment_authority")[ac.fetch("id")]
        end
      end
      classified=admission.fetch("findings").any?{|f|f["affected_rows"].to_a.include?(ac["id"])} || admission.fetch("findings").any?{|f|f["id"]=="issue-#{row['issue']}-execution-gap"}
      raise "acceptance has unclassified missing/placeholder/do-nothing evidence" unless %w[proven accepted_recordless accepted_with_explicit_amendment].include?(ac["evidence_status"]) || classified
    end
  end
  review_gaps=rows.select{|row|row["revision"]&&!row.dig("review_truth","current")}
  unless review_gaps.empty?
    finding=admission.fetch("findings").find{|f|f["id"]=="execution-exact-head-review-gaps"&&f["severity"]=="P1"}
    expected_ids=review_gaps.flat_map{|r|r.fetch("acceptance_rows").map{|a|a["id"]}}.sort
    raise "missing P1 exact-head review-gap blocker" unless finding && finding.fetch("affected_rows").sort==expected_ids
  end
  raise "admitted despite stale exact-head review" if require_admitted && !review_gaps.empty?
  retained=admission.fetch("retained_predecessors")
  raise "retained projection mismatch" unless retained==gap["retained_predecessors"]
  retained_ids=retained.flat_map{|row|row.fetch("acceptance_rows").map{|ac|ac.fetch("id")}}
  raise "duplicate retained acceptance identity" unless retained_ids.uniq.length==retained_ids.length
  retained.each do |row|
    content,_err,status=Open3.capture3("git","show","#{source.fetch('candidate')}:#{row.fetch('path')}",chdir:ROOT.to_s)
    raise "retained artifact missing at candidate" unless status.success?
    raise "retained digest mismatch" unless Digest::SHA256.hexdigest(content)==row["sha256"] && content.bytesize==row["bytes"]
    blob,_blob_err,blob_status=Open3.capture3("git","rev-parse","#{source.fetch('candidate')}:#{row.fetch('path')}",chdir:ROOT.to_s)
    raise "retained blob mismatch" unless blob_status.success? && blob.strip==row["candidate_blob"]
    if row.fetch("acceptance_rows").empty?
      raise "empty retained projection lacks duplicate authority" unless row["acceptance_projection"]=="reference_only_duplicate_predecessor" && retained.any?{|other|other["issue"]==row["issue"] && other["planned_id"]==row["canonical_projection_planned_id"] && other["acceptance_projection"]=="canonical" && !other["acceptance_rows"].empty?}
    else
      raise "retained canonical projection missing" unless row["acceptance_projection"]=="canonical" && row["canonical_projection_planned_id"]==row["planned_id"]
    end
    raise "retained observed status missing" unless %w[criterion_mapped_to_candidate_evidence proof_gap].include?(row["observed_status"])
    row.fetch("acceptance_rows").each do |criterion|
      raise "retained criterion cannot be observed without evidence" if criterion["observed_status"]=="criterion_mapped_to_candidate_evidence" && criterion["observed_evidence"].to_a.empty?
      raise "retained criterion status invalid" unless %w[criterion_mapped_to_candidate_evidence proof_gap].include?(criterion["observed_status"])
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
  actual_collisions.select{|c|c["status"]=="unresolved"}.each{|c|raise "ownership collision not classified" unless admission.fetch("findings").any?{|f|f["id"]=="consolidated-owned-path-resolution-proof-debt"&&f["affected_rows"].to_a.include?(c["path"])}}
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
  rows.each{|r|r["acceptance_rows"].each{|a|raise "Markdown acceptance omitted" unless markdown.include?("| #{r['planned_id']} | ##{r['issue']} | #{a['id']} |")}}
  retained.each{|r|r["acceptance_rows"].each{|a|raise "Markdown retained acceptance omitted" unless markdown.include?("| #{r['planned_id']} | ##{r['issue']} | #{a['id']} |")}}
  actual_collisions.each{|c|raise "Markdown collision omitted" unless markdown.include?("| #{c['path']} | #{c['owners'].join(',')} |")}
  admission.fetch("backlog").each{|r|raise "Markdown backlog omitted" unless markdown.include?("| ##{r['issue']} |")}
  tails.each{|r|raise "Markdown tail row omitted" unless markdown.include?("| #{r['planned_id']} | ##{r['issue']} | #{r['observed_state']} | #{r['expected_lifecycle']} |")}
  findings.each{|f|raise "Markdown finding omitted" unless markdown.include?(f["id"])}
  expected_rows=[]
  expected_rows.concat(rows.map{|r|"| #{r['planned_id']} | ##{r['issue']} | #{r['revision']||'none'} | #{r['merge_revision']||'none'} | #{r['merge_ancestry']} | #{r['disposition']} |"})
  expected_rows.concat(tails.map{|r|"| #{r['planned_id']} | ##{r['issue']} | #{r['observed_state']} | #{r['expected_lifecycle']} | #{r['gate_role']} |"})
  expected_rows.concat(rows.flat_map{|r|r["acceptance_rows"].map{|a|"| #{r['planned_id']} | ##{r['issue']} | #{a['id']} | #{a['evidence_status']} | #{a['text'].gsub('|','/')} |"}})
  expected_rows.concat(retained.flat_map{|r|r["acceptance_rows"].map{|a|"| #{r['planned_id']} | ##{r['issue']} | #{a['id']} | #{a['observed_status']} | #{a['text'].gsub('|','/')} |"}})
  expected_rows.concat(actual_collisions.map{|c|"| #{c['path'].gsub('|','/')} | #{c['owners'].join(',')} | #{c['status']} |"})
  expected_rows.concat(admission["backlog"].map{|r|"| ##{r['issue']} | #{r['disposition']} | #{Digest::SHA256.hexdigest(JSON.generate(r['disposition_authority']))} |"})
  actual_data=markdown.lines.map(&:chomp).select{|l|l.start_with?("| ")&&!l.match?(/^\| (?:Planned ID|Successor|Path|Issue) \|/)}
  raise "Markdown/JSON table projection differs" unless actual_data==expected_rows
  expected_findings=findings.map{|f|"- **#{f['severity']} #{f['id']}** — #{f['summary']} Evidence: #{f['evidence'].join(', ')}. Owner: #{f['owner']}. Disposition: #{f['disposition']}."}
  raise "Markdown/JSON finding projection differs" unless markdown.lines.map(&:chomp).select{|l|l.start_with?("- **")}==expected_findings
  raise "Markdown decision differs" unless markdown.include?("**#{expected.upcase}**")
  raise "Markdown counts differ" unless markdown.include?("Execution roots: #{rows.length}; release-tail stages: #{tails.length}; retained predecessors: #{retained.length}; backlog dispositions: #{admission['backlog'].length}; acceptance rows: #{admission.dig('counts','acceptance_rows')}.")
  raise "admission remains blocked" if require_admitted && expected!="admitted"
  true
end

admission=load_json(OUT.join("release-tail-admission.json")); source_path=OUT.join("release-tail-input.#{admission.fetch('candidate')}.#{admission.fetch('source_digest')}.json"); source=load_json(source_path); gap=load_json(OUT.join("gap_analysis_report.json")); markdown=OUT.join("gap_analysis_report.md").read
if %w[negative all].include?(MODE)
  cases={
    "omitted-root"=>->(s,_a,_g,_m){s["mapping"].delete(s["mapping"].keys.first)},
    "empty-acceptance"=>->(_s,a,_g,_m){a["execution_issues"].first["acceptance_rows"]=[]},
    "do-nothing"=>->(_s,a,_g,_m){a["execution_issues"].first["acceptance_rows"].first["evidence_status"]="placeholder"},
    "invented-criterion-link"=>->(_s,a,_g,_m){a["execution_issues"].first["acceptance_rows"].first["evidence_status"]="evidence_linked"},
    "invented-no-pr-authority"=>->(_s,a,_g,_m){a["execution_issues"].first["closure_disposition"]={"kind"=>"absorbed","authority"=>"invented"}},
    "missing-artifact"=>->(_s,a,_g,_m){a["retained_predecessors"].first["path"]="missing"},
    "invented-retained-mapping"=>->(_s,a,_g,_m){a["retained_predecessors"].first["acceptance_rows"].first["observed_status"]="observed_in_successor"},
    "gap-mismatch"=>->(_s,_a,g,_m){g["execution_issues"]=[]},
    "unowned-finding"=>->(_s,a,g,_m){a["findings"].first["owner"]="";g["findings"]=a["findings"]},
    "collision"=>->(_s,a,_g,_m){a["ownership_collisions"]<<{"path"=>"x"}},
    "false-collision-resolution"=>->(_s,a,_g,_m){a["ownership_collisions"].first["status"]="resolved_by_ordered_content"},
    "spec-acceptance-drift"=>->(s,_a,_g,_m){s["spec_acceptance"].values.first["acceptance_criteria"]=[]},
    "output-identity-drift"=>->(_s,a,_g,_m){a["output_identity"]="missing.json"},
    "projection-digest-drift"=>->(_s,a,_g,_m){a["projection_digest"]="0"*64},
    "admitted-with-blocker"=>->(_s,a,g,_m){a["decision"]=g["decision"]="admitted"},
    "stale-candidate"=>->(s,a,g,_m){s["candidate"]=a["candidate"]=g["candidate"]="0"*40;digest=Digest::SHA256.hexdigest(JSON.generate(s));a["source_digest"]=g["source_digest"]=digest},
    "stale-review-result"=>->(_s,a,_g,_m){row=a["execution_issues"].find{|r|r["revision"]};row["review_truth"].merge!("current"=>true,"result"=>"not_pass","reviewed_revision"=>row["revision"],"post_review_paths"=>[],"non_substantive_tail"=>true)},
    "forged-reviewed-sha"=>->(_s,a,_g,_m){row=a["execution_issues"].find{|r|r["revision"]};row["review_truth"].merge!("current"=>true,"result"=>"pass","reviewed_revision"=>"f"*40,"post_review_paths"=>[],"non_substantive_tail"=>true)},
    "substantive-review-tail"=>->(_s,a,_g,_m){row=a["execution_issues"].find{|r|r["revision"]};row["review_truth"].merge!("current"=>true,"result"=>"pass","reviewed_revision"=>row["revision"],"post_review_paths"=>["adl/src/lib.rs"],"non_substantive_tail"=>false)},
    "generated-evidence-blob-drift"=>->(s,_a,_g,_m){s["generated_evidence"].first["candidate_blob"]="0"*40},
    "generated-evidence-digest-drift"=>->(s,_a,_g,_m){s["generated_evidence"].first["sha256"]="0"*64},
    "markdown-omission"=>->(_s,_a,_g,m){m.replace("")}
  }
  cases.each do |name,mutation|
    s,a,g=Marshal.load(Marshal.dump([source,admission,gap])); m=markdown.dup; mutation.call(s,a,g,m)
    begin; validate!(s,a,g,m,require_admitted:false); abort("negative fixture accepted: #{name}"); rescue RuntimeError; end
  end
end
validate!(source,admission,gap,markdown,require_admitted:MODE=="admitted") unless MODE=="negative"
result={schema:"adl.v0921.release_tail_validation.v2",mode:MODE,status:"pass"}
head,_head_err,head_status=Open3.capture3("git","rev-parse","HEAD",chdir:ROOT.to_s)
abort("cannot bind validation receipt to HEAD") unless head_status.success?
puts JSON.generate(
  schema:"adl.v0921.release_tail_validation_receipt.v1",
  exact_head:head.strip,
  candidate:source["candidate"],
  source_digest:admission["source_digest"],
  argv:["ruby",".csdlc/prepared/issues/516/validate-release-tail-admission.rb",MODE],
  started_at:STARTED_AT,
  ended_at:Time.now.utc.iso8601(6),
  exit_code:0,
  stdout:result,
  stdout_sha256:Digest::SHA256.hexdigest(JSON.generate(result))
)
