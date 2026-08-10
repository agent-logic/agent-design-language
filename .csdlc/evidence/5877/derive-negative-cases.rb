#!/usr/bin/env ruby
# frozen_string_literal: true
require "digest"; require "fileutils"; require "json"; require "open3"; require "pathname"; require "time"
ISSUE=5877
MARKER="ADL_ISSUE_5877_NEGATIVE_CASE_V1 "
EXPECTED={
  "deterministic_jcs"=>"recovered",
  "raw_evidence_redaction"=>"rejected",
  "openapi_parity"=>"recovered",
  "response_byte_bound"=>"rejected",
  "unauthorized_request"=>"denied",
  "authorization_before_read"=>"denied",
  "unsupported_version"=>"rejected",
  "wrong_authority_domain"=>"rejected",
  "authority_voter_membership_mismatch"=>"rejected",
  "authority_index_behind_membership"=>"rejected",
  "authority_index_ahead_of_membership"=>"rejected",
  "unknown_member_reference"=>"rejected",
  "zero_reference_key"=>"rejected",
  "placement_authority_unavailable"=>"fail_closed",
  "identity_without_transport"=>"rejected",
  "exact_identity_and_transport"=>"recovered",
  "certificate_count_n_plus_one"=>"rejected",
  "identity_rotation_overlap"=>"rejected",
  "keyed_opaque_references"=>"recovered",
  "stale_authority_cut"=>"rejected",
  "nested_openapi_parity"=>"recovered"
}.freeze
EXACT=["ruby",".csdlc/evidence/5877/run-exact-child-tests.rb","cargo","nextest","run","--manifest-path","adl-runtime/Cargo.toml","--test","distributed_projection","--no-tests=fail"].freeze
CLIPPY=["cargo","clippy","--manifest-path","adl-runtime/Cargo.toml","--test","distributed_projection","--","-D","warnings"].freeze
NEGATIVE=["cargo","test","--manifest-path","adl-runtime/Cargo.toml","--test","distributed_projection","--","--nocapture"].freeze
ROOT=Pathname.new(__dir__).join("../../..").cleanpath.expand_path
EVIDENCE=ROOT.join(".csdlc/evidence/5877")
def fail!(message); abort message; end
def sha(path); Digest::SHA256.file(path).hexdigest; end
def relative(path); Pathname.new(path).expand_path.relative_path_from(ROOT).to_s; end
def normalized(text); lines=text.lines.map(&:rstrip); lines.pop while lines.last==""; lines.empty? ? "" : lines.join("\n")+"\n"; end
def run(command); started=Time.now.utc.iso8601(6); out,err,status=Open3.capture3({"CARGO_TERM_COLOR"=>"never"},*command,chdir:ROOT.to_s); [out,err,status,started,Time.now.utc.iso8601(6)]; end

source=ARGV.fetch(0); output=Pathname.new(ARGV.fetch(1)).expand_path
fail!("source revision malformed") unless source.match?(/\A[0-9a-f]{40}\z/)
fail!("producer is not at source revision") unless `git -C #{ROOT} rev-parse HEAD`.strip==source
fail!("output escapes issue evidence") unless output.to_s.start_with?(EVIDENCE.to_s+"/")
fail!("output already exists") if output.exist?
FileUtils.mkdir_p(output)
exact_out,exact_err,exact_status,exact_start,exact_finish=run(EXACT)
File.binwrite(output.join("exact-child-tests.stdout.log"),normalized(exact_out)); File.binwrite(output.join("exact-child-tests.stderr.log"),normalized(exact_err))
fail!("exact tests failed") unless exact_status.success?
summary=(exact_out+exact_err).match(/Summary .*?(\d+) tests run: (\d+) passed, 0 skipped/)
fail!("exact denominator mismatch") unless summary && summary[1].to_i.positive? && summary[1]==summary[2]
clippy_out,clippy_err,clippy_status,clippy_start,clippy_finish=run(CLIPPY)
clippy_out=JSON.generate({"schema"=>"adl.wp04.command_result.v1","command"=>"strict-focused-clippy","exit_code"=>clippy_status.exitstatus})+"\n" if clippy_out.empty?
File.binwrite(output.join("strict-focused-clippy.stdout.log"),normalized(clippy_out)); File.binwrite(output.join("strict-focused-clippy.stderr.log"),normalized(clippy_err)); fail!("Clippy failed") unless clippy_status.success?
negative_out,negative_err,negative_status,negative_start,negative_finish=run(NEGATIVE)
File.binwrite(output.join("negative-cases.stdout.log"),normalized(negative_out)); File.binwrite(output.join("negative-cases.stderr.log"),normalized(negative_err)); fail!("negative run failed") unless negative_status.success?
observed=negative_out.lines.each_with_object([]) do |line, entries|
  next unless line.start_with?(MARKER)
  payload=JSON.parse(line.delete_prefix(MARKER)); entries << {"case"=>payload.fetch("case"),"result"=>payload.fetch("result"),"observed_line_sha256"=>Digest::SHA256.hexdigest(line.chomp)}
end
fail!("negative denominator mismatch") unless observed.length==EXPECTED.length && EXPECTED.all?{|name,result| observed.any?{|entry| entry["case"]==name && entry["result"]==result}}
machine_path=output.join("negative-cases.json")
machine={"schema"=>"adl.wp04.negative_cases.machine.v1","issue"=>ISSUE,"source_revision"=>source,"command"=>{"argv"=>NEGATIVE,"exit_code"=>0,"started_at"=>negative_start,"finished_at"=>negative_finish},"stdout_path"=>relative(output.join("negative-cases.stdout.log")),"stdout_sha256"=>sha(output.join("negative-cases.stdout.log")),"stderr_path"=>relative(output.join("negative-cases.stderr.log")),"stderr_sha256"=>sha(output.join("negative-cases.stderr.log")),"cases"=>observed}
File.write(machine_path,JSON.pretty_generate(machine)+"\n")
runner_path=EVIDENCE.join("runner.txt"); runner={"provider"=>"local-codex","run_id"=>"5877-local-operator-v7","os"=>"macos","arch"=>"aarch64","identity_sha256"=>sha(runner_path)}
artifacts=[Pathname.new(__FILE__),EVIDENCE.join("run-exact-child-tests.rb"),runner_path,output.join("exact-child-tests.stdout.log"),output.join("exact-child-tests.stderr.log"),output.join("strict-focused-clippy.stdout.log"),output.join("strict-focused-clippy.stderr.log"),machine_path,output.join("negative-cases.stdout.log"),output.join("negative-cases.stderr.log")]
paths=["adl-runtime/src/distributed/projection.rs","adl-runtime/tests/distributed_projection.rs","docs/api/runtime-v3/v1/distributed.openapi.json"]
proof={"schema"=>"adl.wp04.execution_proof.v3","issue"=>ISSUE,"wp"=>"WP-04.15","source_revision"=>source,"evidence_revision_strategy"=>"derive_from_receipt_introduction","protected_paths"=>paths,"source_artifacts"=>paths.map{|path|{"path"=>path,"sha256"=>Digest::SHA256.hexdigest(`git -C #{ROOT} show #{source}:#{path}`)}},"commands"=>[
  {"argv"=>EXACT,"exit_code"=>0,"selected_tests"=>summary[1].to_i,"started_at"=>exact_start,"finished_at"=>exact_finish,"runner"=>runner,"stdout_path"=>relative(output.join("exact-child-tests.stdout.log")),"stdout_sha256"=>sha(output.join("exact-child-tests.stdout.log")),"stderr_path"=>relative(output.join("exact-child-tests.stderr.log")),"stderr_sha256"=>sha(output.join("exact-child-tests.stderr.log"))},
  {"argv"=>CLIPPY,"exit_code"=>0,"started_at"=>clippy_start,"finished_at"=>clippy_finish,"runner"=>runner,"stdout_path"=>relative(output.join("strict-focused-clippy.stdout.log")),"stdout_sha256"=>sha(output.join("strict-focused-clippy.stdout.log")),"stderr_path"=>relative(output.join("strict-focused-clippy.stderr.log")),"stderr_sha256"=>sha(output.join("strict-focused-clippy.stderr.log"))}],
  "negative_cases"=>EXPECTED.map{|name,result|{"case"=>name,"result"=>result,"evidence_path"=>relative(machine_path),"evidence_sha256"=>sha(machine_path)}},"artifacts"=>artifacts.map{|path|{"path"=>relative(path),"sha256"=>sha(path)}},"native_receipts"=>[]}
File.write(EVIDENCE.join("execution-proof.json"),JSON.pretty_generate(proof)+"\n")
puts JSON.generate({"schema"=>"adl.wp04.negative_cases.producer_result.v1","issue"=>ISSUE,"source_revision"=>source,"selected_tests"=>summary[1].to_i,"selected_cases"=>observed.length})
