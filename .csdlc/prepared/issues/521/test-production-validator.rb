#!/usr/bin/env ruby
require "digest"; require "fileutils"; require "json"; require "tmpdir"
require_relative "validate-external-review"
def wj(p,v); FileUtils.mkdir_p(File.dirname(p)); File.write(p,JSON.generate(v)); end
def sh!(*a); system(*a) or abort("failed #{a.join(' ')}"); end
Dir.mktmpdir("issue-521-production-",File.expand_path("../../../../.adl",__dir__)) do |repo|
 Dir.chdir(repo) do
  sh!("git","init","-q"); sh!("git","config","user.email","f@invalid"); sh!("git","config","user.name","fixture")
  FileUtils.mkdir_p(".csdlc/prepared/issues/520"); validator_path=".csdlc/prepared/issues/520/validate-internal-review.rb"; File.write(validator_path,"puts({status:'passed',candidate_sha:ARGV[0]}.to_json)\n")
  File.write("subject.txt","candidate\n"); sh!("git","add","."); sh!("git","commit","-qm","candidate"); candidate=`git rev-parse HEAD`.strip
  internal_root="docs/internal"; FileUtils.mkdir_p(internal_root)
  denominator_names=%w[repo_inventory.json issue_inventory.json acceptance_coverage.json]
  denominator_names.each_with_index{|n,i| wj(File.join(internal_root,n),{"rows"=>[{"denominator_ref"=>"ref:#{i}"}]})}
  %w[findings.json lane-results.json proof-results.json validation-results.json redaction-report.json quality-report.json].each{|n| wj(File.join(internal_root,n),{"candidate_sha"=>candidate,"outcome"=>"passed","findings"=>[]})}
  validation_path=File.join(internal_root,"semantic.json"); wj(validation_path,{"status"=>"passed","candidate_sha"=>candidate,"validator"=>validator_path})
  stdout=JSON.generate({"status"=>"passed","candidate_sha"=>candidate})
  invocation_path=File.join(internal_root,"invocation.json"); wj(invocation_path,{"validator_path"=>validator_path,"validator_sha256"=>Digest::SHA256.file(validator_path).hexdigest,"argv"=>["ruby",validator_path,"all"],"exit_status"=>0,"candidate_sha"=>candidate,"stdout"=>stdout,"stdout_sha256"=>Digest::SHA256.hexdigest(stdout)})
  names=%w[repo_inventory.json issue_inventory.json acceptance_coverage.json findings.json lane-results.json proof-results.json validation-results.json redaction-report.json quality-report.json semantic.json invocation.json]
  entries=names.map{|n| p=File.join(internal_root,n);{"path"=>p,"sha256"=>Digest::SHA256.file(p).hexdigest}}
  internal_manifest_path=File.join(internal_root,"packet-manifest.json"); wj(internal_manifest_path,{"candidate_sha"=>candidate,"entries"=>entries})
  sh!("git","add","."); sh!("git","commit","-qm","internal review"); internal_merge=`git rev-parse HEAD`.strip; reviewed_head=internal_merge
  root=File.join(repo,"packet"); FileUtils.mkdir_p(root)
  review_receipt=File.join(root,"internal-review.json"); wj(review_receipt,{"reviewed_sha"=>reviewed_head,"outcome"=>"passed","findings"=>[],"reviewer"=>"independent"})
  manifest={"candidate_sha"=>candidate,"internal_review_predecessor"=>{"issue"=>520,"pull_request"=>900,"merge_sha"=>internal_merge,"reviewed_head_sha"=>reviewed_head},"internal_packet_manifest"=>internal_manifest_path,"internal_packet_manifest_sha256"=>Digest::SHA256.hexdigest(`git show #{internal_merge}:#{internal_manifest_path}`),"internal_candidate_sha"=>candidate,"internal_semantic_validation"=>{"evidence_path"=>validation_path,"sha256"=>entries.find{|e|e["path"]==validation_path}["sha256"],"outcome"=>"passed"},"internal_validation_invocation"=>{"receipt_path"=>invocation_path,"sha256"=>entries.find{|e|e["path"]==invocation_path}["sha256"]},"internal_exact_head_review"=>{"receipt_path"=>review_receipt,"sha256"=>Digest::SHA256.file(review_receipt).hexdigest}}
  wj(File.join(root,"run_manifest.json"),manifest)
  ev={"path"=>"subject.txt","sha256"=>Digest::SHA256.hexdigest(`git show #{candidate}:subject.txt`),"source"=>"candidate","revision"=>candidate}
  invocation_id="fixture-1"; provider="vertex"; model="gemini"
  raw={"candidate_sha"=>candidate,"reviewer"=>"external","provider"=>provider,"model"=>model,"invocation_id"=>invocation_id,"scope_rows"=>[],"findings"=>[],"limitations"=>[],"observations"=>[]}
  # Scope includes all denominator references plus the complete internal packet outputs.
  refs=%w[ref:0 ref:1 ref:2]+%w[findings.json lane-results.json proof-results.json validation-results.json redaction-report.json quality-report.json packet-manifest.json].map{|n|"internal-artifact:#{n}"}; refs=refs.uniq
  scope_rows=refs.map{|ref|{"ref"=>ref,"candidate_sha"=>candidate,"evidence"=>ev,"disposition"=>"reviewed"}}
  raw["scope_rows"]=scope_rows; raw["observations"]=refs.map{|ref|{"ref"=>ref,"evidence"=>ev,"conclusion"=>"reviewed substantively"}}
  raw_path=File.join(root,"raw-review-output.json"); wj(raw_path,raw)
  request={"provider"=>provider,"model"=>model,"invocation_id"=>invocation_id,"candidate_sha"=>candidate,"scope_refs"=>refs,"prompt"=>"Review every retained surface"}; request_path=File.join(root,"provider-request.json"); wj(request_path,request)
  native={"provider"=>provider,"model"=>model,"invocation_id"=>invocation_id,"response_id"=>"r1","content"=>"reviewed"}; native_path=File.join(root,"provider-native-response.json"); wj(native_path,native)
  receipt={"provider"=>provider,"model"=>model,"invocation_id"=>invocation_id,"request_sha256"=>Digest::SHA256.file(request_path).hexdigest,"response_sha256"=>Digest::SHA256.file(raw_path).hexdigest,"provider_native_response_sha256"=>Digest::SHA256.file(native_path).hexdigest,"exit_status"=>0,"observed_at"=>"now"}; wj(File.join(root,"provider-invocation-receipt.json"),receipt)
  wj(File.join(root,"standard-runner-receipt.json"),{"runner"=>"docs/tooling/OPUS_REVIEW_RUNBOOK.md","request_sha256"=>receipt["request_sha256"],"response_sha256"=>receipt["response_sha256"],"provider_native_response_sha256"=>receipt["provider_native_response_sha256"],"exit_status"=>0})
  wj(File.join(root,"reviewer-independence.json"),{"reviewer"=>"external","independent"=>true,"evidence"=>ev,"implementation_reviewers"=>[],"internal_reviewers"=>[],"raw_output_sha256"=>receipt["response_sha256"],"provider"=>provider,"model"=>model,"invocation_id"=>invocation_id,"observed_at"=>"now"})
  wj(File.join(root,"scope.json"),{"expected_refs"=>refs,"reviewed_refs"=>refs,"rows"=>scope_rows})
  wj(File.join(root,"findings.json"),{"candidate_sha"=>candidate,"outcome"=>"passed","findings"=>[],"zero_findings_evidence"=>"provider output"})
  wj(File.join(root,"limitations.json"),{"limitations"=>[],"no_limitations_evidence"=>"provider output"})
  required=%w[run_manifest.json reviewer-independence.json provider-request.json provider-invocation-receipt.json standard-runner-receipt.json provider-native-response.json raw-review-output.json scope.json findings.json limitations.json]
  paths=required.map{|n|File.join(root,n)}+[review_receipt]; wj(File.join(root,"packet-manifest.json"),{"entries"=>paths.map{|p|{"path"=>p,"sha256"=>Digest::SHA256.file(p).hexdigest}}})
  bin=File.join(repo,"bin"); FileUtils.mkdir_p(bin); File.write(File.join(bin,"gh"),"#!/bin/sh\ncase \"$1\" in issue) printf '%s' '{\"state\":\"CLOSED\",\"closedByPullRequestsReferences\":[{\"number\":900}]}' ;; pr) printf '%s' '{\"state\":\"MERGED\",\"mergedAt\":\"now\",\"mergeCommit\":{\"oid\":\"#{internal_merge}\"},\"headRefOid\":\"#{reviewed_head}\"}' ;; esac\n"); FileUtils.chmod(0755,File.join(bin,"gh")); ENV["PATH"]="#{bin}:#{ENV["PATH"]}"
  abort("valid production packet failed") unless validate_packet!(root:root)[:status]=="passed"
  raw["observations"]=[]; wj(raw_path,raw); receipt["response_sha256"]=Digest::SHA256.file(raw_path).hexdigest; wj(File.join(root,"provider-invocation-receipt.json"),receipt)
  independence_path=File.join(root,"reviewer-independence.json"); independence=JSON.parse(File.read(independence_path)); independence["raw_output_sha256"]=receipt["response_sha256"]; wj(independence_path,independence)
  packet_manifest_path=File.join(root,"packet-manifest.json"); packet_manifest=JSON.parse(File.read(packet_manifest_path)); packet_manifest.fetch("entries").each{|entry|entry["sha256"]=Digest::SHA256.file(entry.fetch("path")).hexdigest}; wj(packet_manifest_path,packet_manifest)
  begin; validate_packet!(root:root); abort("do-nothing review passed"); rescue SystemExit,KeyError; puts JSON.generate(status:"passed",production_negative:"populated_do_nothing_review"); end
 end
end
