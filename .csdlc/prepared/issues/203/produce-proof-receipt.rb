#!/usr/bin/env ruby
# frozen_string_literal: true
require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath.expand_path
OUT = ROOT.join(".csdlc/evidence/203/v3")
PROOF = OUT.join("integration-closeout-proof.json")
CHILDREN = {
  "258" => ["9e206d1ea7ab1be4593fdb6dc435aa5ed1561a9e", "193f77d24a693f955a2fcf3bdfc759ad1db8aff4"],
  "259" => ["4329d38fb870875a0f46969c82d3d0219a638e2b", "119bab39d4eb98cd4013c95633ff070908e4c59c"],
  "260" => ["17eecaa9ba74e870a67a335a79f6394405615e87", "0b5aefd6e75e56ccac59e761a7037902f581c76d"]
}.freeze
COMMANDS = {
  "identity-boundary" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_identity_lease_authority -- --test-threads=1],
  "caller-guard" => %w[cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authority_adapter_callers_260 -- --test-threads=1],
  "strict-clippy" => %w[cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_identity_lease_authority -- -D warnings]
}.freeze
COMMON = Pathname.new(Open3.capture2("git", "rev-parse", "--git-common-dir", chdir: ROOT.to_s).first.strip).expand_path(ROOT)
FINISH = COMMON.parent.join(".adl/bin/csdlc-v2/csdlc-finish").to_s

if PROOF.file?
  ok = system("ruby", ".csdlc/prepared/issues/203/validate-proof-receipt.rb", chdir: ROOT.to_s)
  abort("retained issue #203 integration proof is invalid") unless ok
  puts "PASS: retained issue #203 integration closeout proof is current"
  exit 0
end

def run(*args)
  out, err, status = Open3.capture3(*args, chdir: ROOT.to_s)
  abort("issue 203 proof command failed: #{args.join(' ')}\n#{err}") unless status.success?
  out
end

abort("issue 203 proof requires an exactly clean worktree") unless run("git", "status", "--porcelain=v1", "--untracked-files=all").empty?
source = run("git", "rev-parse", "HEAD").strip
main = run("git", "rev-parse", "origin/main").strip
abort("issue 203 proof requires current main ancestry") unless system("git", "merge-base", "--is-ancestor", main, source, chdir: ROOT.to_s)
product_diff = run("git", "diff", "--name-only", "origin/main...HEAD", "--", "adl-runtime", "adl/Cargo.lock")
abort("issue 203 proof forbids product or lock drift") unless product_diff.empty?
CHILDREN.each do |issue, (head, merge)|
  abort("child ##{issue} merge is not ancestral") unless system("git", "merge-base", "--is-ancestor", merge, main, chdir: ROOT.to_s)
end

terminal_children = {}
CHILDREN.each do |issue, (_head, expected_merge)|
  raw = run(FINISH, "--root", ROOT.to_s, "--validate-cached-issue", issue)
  result = JSON.parse(raw)
  terminal = result.fetch("terminal")
  abort("child ##{issue} cache is not canonical") unless result["canonical_match"] == true
  abort("child ##{issue} terminal disposition mismatch") unless terminal["disposition"] == "merged" && terminal["issue_state"] == "closed_by_merged_pr"
  abort("child ##{issue} cache merge mismatch") unless terminal["merge_sha"] == expected_merge
  terminal_children[issue] = {"canonical_match"=>true,"canonical_generation"=>terminal["canonical_generation"],
    "canonical_digest"=>terminal["canonical_digest"],"head_sha"=>terminal["head_sha"],"merge_sha"=>terminal["merge_sha"],
    "terminal_digest"=>terminal["digest"],"issue_state"=>terminal["issue_state"]}
end

FileUtils.mkdir_p(OUT, mode: 0o700)
commands = {}
COMMANDS.each do |name, argv|
  started = Time.now.utc.iso8601(6)
  stdout, stderr, status = Open3.capture3(*argv, chdir: ROOT.to_s)
  stdout = stdout.rstrip + (stdout.empty? ? "" : "\n")
  stderr = stderr.rstrip + (stderr.empty? ? "" : "\n")
  finished = Time.now.utc.iso8601(6)
  File.binwrite(OUT.join("#{name}.stdout.log"), stdout)
  File.binwrite(OUT.join("#{name}.stderr.log"), stderr)
  abort("issue 203 proof lane failed: #{name}") unless status.success?
  commands[name] = {"argv"=>argv,"exit_code"=>0,"started_at"=>started,"finished_at"=>finished,
    "stdout_sha256"=>Digest::SHA256.hexdigest(stdout),"stderr_sha256"=>Digest::SHA256.hexdigest(stderr)}
end
identity = File.binread(OUT.join("identity-boundary.stdout.log"))
guard = File.binread(OUT.join("caller-guard.stdout.log"))
abort("identity boundary denominator mismatch") unless identity.include?("test result: ok. 4 passed; 0 failed;")
abort("caller guard denominator mismatch") unless guard.include?("test result: ok. 5 passed; 0 failed;")

proof = {"schema"=>"adl.issue203.integration_closeout_proof.v3","issue"=>203,"source_revision"=>source,
  "required_main_ancestor"=>main,"product_diff_paths"=>[],"terminal_children"=>terminal_children,
  "commands"=>commands,"historical_proof_disposition"=>"superseded_nonclaim",
  "nonclaims"=>["No #205 serving eligibility implementation.","No #204 migration/recovery workflow implementation.","No resurrection of the historical synthetic 44-case marker denominator."]}
File.binwrite(PROOF, JSON.generate(proof)+"\n")
puts "PASS: issue #203 integration closeout proof binds terminal children, zero product diff, current focused tests, and strict Clippy"
