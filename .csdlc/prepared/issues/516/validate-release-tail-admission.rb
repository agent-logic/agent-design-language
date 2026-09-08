#!/usr/bin/env ruby
# frozen_string_literal: true
require "digest"; require "json"; require "pathname"
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
  rows=admission.fetch("execution_issues"); mapping=source.fetch("mapping")
  raise "execution denominator mismatch" unless rows.to_h{|r|[r["planned_id"],r["issue"]]}==mapping
  raise "duplicate issue mapping" unless rows.map{|r|r["issue"]}.uniq.length==rows.length
  observations=rows.map{|r|r.slice("planned_id","issue","title","acceptance_authority","acceptance_rows","linked_prs","closeout_state","closure_disposition","owned_paths","review_evidence","validation_evidence")}
  raise "captured observation mismatch" unless source["observations"]==observations
  projection=rows.map{|r|r.slice("planned_id","issue","revision","merge_revision","merge_ancestry","disposition","acceptance_rows")}
  raise "gap execution projection mismatch" unless gap["execution_issues"]==projection
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
      raise "explicit no-PR closure authority missing" unless row.dig("closure_disposition","authority") && row.dig("closure_disposition","kind")
    end
    %w[review_evidence validation_evidence].each do |key|
      next unless row[key]
      path=ROOT.join(row[key].fetch("path")); raise "#{key} artifact missing" unless path.file?
      raise "#{key} digest mismatch" unless Digest::SHA256.file(path).hexdigest==row[key]["sha256"]
    end
    row["acceptance_rows"].each do |ac|
      raise "acceptance identity/text missing" if ac["id"].to_s.empty? || ac["text"].to_s.empty?
      linked=ac["evidence_status"]=="evidence_linked" && !ac.fetch("evidence").empty?
      explicitly_blocked=row["disposition"]=="release_blocker" && admission.fetch("findings").any?{|f|f["id"]=="issue-#{row['issue']}-not-terminal" && %w[P0 P1].include?(f["severity"])}
      raise "acceptance has unclassified missing/placeholder/do-nothing evidence" unless linked || explicitly_blocked
    end
  end
  retained=admission.fetch("retained_predecessors")
  raise "retained projection mismatch" unless retained==gap["retained_predecessors"]
  retained.each do |row|
    path=ROOT.join(row.fetch("path")); raise "retained artifact missing" unless path.file?
    raise "retained digest mismatch" unless Digest::SHA256.file(path).hexdigest==row["sha256"]
    raise "retained acceptance empty" if row.fetch("acceptance_rows").empty?
  end
  raise "backlog projection mismatch" unless admission["backlog"]==gap["backlog"]
  admission.fetch("backlog").each{|r|raise "backlog authority missing" if r["disposition_authority"].to_s.empty?}
  claims=Hash.new{|h,k|h[k]=[]}; rows.each{|r|r.fetch("owned_paths").each{|path|claims[path]<<r["issue"]}}
  actual_collisions=admission.fetch("ownership_collisions")
  raise "ownership collision denominator mismatch" unless actual_collisions.map{|c|[c["path"],c["owners"]]}==claims.map{|path,owners|[path,owners.uniq] if owners.uniq.length>1}.compact
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
  raise "Markdown candidate mismatch" unless markdown.include?("Candidate: `#{admission['candidate']}`")
  raise "Markdown digest mismatch" unless markdown.include?("Captured-input digest: `#{digest}`")
  rows.each{|r|raise "Markdown row omitted" unless markdown.include?("| #{r['planned_id']} | ##{r['issue']} |")}
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
    "missing-artifact"=>->(_s,a,_g,_m){a["retained_predecessors"].first["path"]="missing"},
    "stale-review-digest"=>->(_s,a,_g,_m){a["execution_issues"].find{|r|r["review_evidence"]}["review_evidence"]["sha256"]="0"*64},
    "gap-mismatch"=>->(_s,_a,g,_m){g["execution_issues"]=[]},
    "unowned-finding"=>->(_s,a,g,_m){a["findings"].first["owner"]="";g["findings"]=a["findings"]},
    "collision"=>->(_s,a,_g,_m){a["ownership_collisions"]<<{"path"=>"x"}},
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
