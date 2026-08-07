#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
FIXTURE = File.join(ROOT, ".csdlc/evidence/5901/terminal-wave-fixture")
VALIDATOR = File.join(ROOT, ".csdlc/prepared/issues/5862/validate-implementation-wave.rb")
ISSUES = (5863..5878).to_a.freeze

def write(path, content)
  FileUtils.mkdir_p(File.dirname(path))
  File.write(path, content)
end

def run_fixture(failure = nil)
  env = {
    "CSDLC_FINISH_BIN" => File.join(FIXTURE, "fake-finish"),
    "CSDLC_GITHUB_PR_BIN" => File.join(FIXTURE, "fake-pr")
  }
  env["FIXTURE_FAILURE"] = failure if failure
  Open3.capture3(env, "ruby", "validate-implementation-wave.rb", chdir: FIXTURE)
end

FileUtils.rm_rf(FIXTURE)
FileUtils.mkdir_p(FIXTURE)
begin
  %w[docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml .adl/docs/TBD/V092_SPRINT_5862_DISTRIBUTED_GUARDIAN_SESSION_PROMPT.md .csdlc/prepared/issues/5821/design.md].each do |path|
    destination = File.join(FIXTURE, path)
    FileUtils.mkdir_p(File.dirname(destination))
    FileUtils.cp(File.join(ROOT, path), destination)
  end
  FileUtils.cp(VALIDATOR, File.join(FIXTURE, "validate-implementation-wave.rb"))

  system("git", "init", "-q", FIXTURE) or abort "fixture git init failed"
  system("git", "-C", FIXTURE, "config", "user.email", "fixture@example.invalid") or abort "fixture git config failed"
  system("git", "-C", FIXTURE, "config", "user.name", "Fixture") or abort "fixture git config failed"
  write(File.join(FIXTURE, "seed"), "fixture\n")
  system("git", "-C", FIXTURE, "add", "seed") or abort "fixture git add failed"
  system("git", "-C", FIXTURE, "commit", "-qm", "fixture") or abort "fixture git commit failed"
  head, status = Open3.capture2("git", "-C", FIXTURE, "rev-parse", "HEAD")
  abort "fixture git head failed" unless status.success?
  head = head.strip

  entries = []
  ISSUES.each_with_index do |issue, offset|
    source_index = JSON.parse(File.read(File.join(ROOT, ".csdlc/issues/#{issue}/index.json")))
    write(File.join(FIXTURE, ".csdlc/issues/#{issue}/index.json"), JSON.pretty_generate(source_index) + "\n")
    %w[sip stp spp vpp].each do |card|
      source = File.join(ROOT, ".csdlc/issues/#{issue}/cards/#{card}.values.json")
      destination = File.join(FIXTURE, ".csdlc/issues/#{issue}/cards/#{card}.values.json")
      FileUtils.mkdir_p(File.dirname(destination))
      FileUtils.cp(source, destination)
    end
    design = File.join(ROOT, ".csdlc/prepared/issues/#{issue}/design.md")
    destination = File.join(FIXTURE, ".csdlc/prepared/issues/#{issue}/design.md")
    FileUtils.mkdir_p(File.dirname(destination))
    FileUtils.cp(design, destination)
    entries << {
      "issue" => issue,
      "pull_request" => 7000 + offset,
      "head_sha" => head,
      "merge_sha" => head,
      "envelope_digest" => "d" * 64
    }
  end
  umbrella = File.join(FIXTURE, ".csdlc/issues/5862/index.json")
  FileUtils.mkdir_p(File.dirname(umbrella))
  FileUtils.cp(File.join(ROOT, ".csdlc/issues/5862/index.json"), umbrella)

  proof = {
    "schema" => "adl.wp04.execution_proof.v2",
    "source_revision" => head,
    "commands" => [
      {"argv" => ["bash", "adl/tools/validate_v092_distributed_guardian.sh"], "exit_code" => 0},
      {"argv" => ["ruby", "adl/tools/validate_v092_distributed_native_receipts.rb"], "exit_code" => 0}
    ]
  }
  proof_path = File.join(FIXTURE, ".csdlc/evidence/5878/execution-proof.json")
  write(proof_path, JSON.pretty_generate(proof) + "\n")
  manifest = {
    "schema" => "adl.wp04.terminal_child_envelopes.v1",
    "children" => entries,
    "wp04_16_execution_proof_sha256" => Digest::SHA256.file(proof_path).hexdigest
  }
  write(File.join(FIXTURE, ".csdlc/evidence/5862/terminal-child-envelopes.json"), JSON.pretty_generate(manifest) + "\n")

  write(File.join(FIXTURE, "fake-finish"), <<~'RUBY')
    #!/usr/bin/env ruby
    require "json"
    root = ARGV.fetch(ARGV.index("--root") + 1)
    issue = Integer(ARGV.fetch(ARGV.index("--validate-cached-issue") + 1))
    abort "malformed envelope" if ENV["FIXTURE_FAILURE"] == "malformed" && issue == 5863
    index = JSON.parse(File.read(File.join(root, ".csdlc/issues/#{issue}/index.json")))
    manifest = JSON.parse(File.read(File.join(root, ".csdlc/evidence/5862/terminal-child-envelopes.json")))
    entry = manifest.fetch("children").find { |item| item.fetch("issue") == issue }
    digest = ENV["FIXTURE_FAILURE"] == "digest" && issue == 5863 ? "e" * 64 : entry.fetch("envelope_digest")
    terminal = {
      "schema" => "csdlc.derived_terminal.v1", "digest" => digest,
      "repository" => index.fetch("repository"), "issue" => issue,
      "initialization_digest" => index.fetch("initialization_digest"),
      "canonical_generation" => index.fetch("generation"), "canonical_digest" => index.fetch("digest"),
      "disposition" => "merged", "issue_state" => "closed_by_merged_pr", "source" => "live_github",
      "pull_request" => entry.fetch("pull_request"), "head_sha" => entry.fetch("head_sha"),
      "merge_sha" => entry.fetch("merge_sha")
    }
    terminal["head_sha"] = "a" * 40 if ENV["FIXTURE_FAILURE"] == "head" && issue == 5863
    puts JSON.generate("schema" => "csdlc.derived_terminal_validation.v1", "canonical_match" => true, "terminal" => terminal)
  RUBY
  write(File.join(FIXTURE, "fake-pr"), <<~'RUBY')
    #!/usr/bin/env ruby
    require "json"
    request = JSON.parse(File.read(ARGV.fetch(ARGV.index("--request") + 1)))
    manifest = JSON.parse(File.read(".csdlc/evidence/5862/terminal-child-envelopes.json"))
    entry = manifest.fetch("children").find { |item| item.fetch("issue") == request.fetch("linked_issue") }
    issue = entry.fetch("issue")
    linked = ENV["FIXTURE_FAILURE"] == "linkage" && issue == 5863 ? issue + 1 : issue
    merge = ENV["FIXTURE_FAILURE"] == "merge" && issue == 5863 ? "b" * 40 : entry.fetch("merge_sha")
    merge = "c" * 40 if ENV["FIXTURE_FAILURE"] == "ancestry" && issue == 5863
    puts JSON.generate("linked_issue" => linked, "state" => "closed", "merged" => true,
                       "head_sha" => entry.fetch("head_sha"), "merge_commit_sha" => merge)
  RUBY
  FileUtils.chmod(0o755, [File.join(FIXTURE, "fake-finish"), File.join(FIXTURE, "fake-pr")])

  stdout, stderr, status = run_fixture
  abort "valid terminal wave failed: #{stderr} #{stdout}" unless status.success? && stdout.include?("PASS:")
  %w[malformed digest head merge linkage ancestry].each do |failure|
    stdout, stderr, status = run_fixture(failure)
    abort "#{failure} fixture unexpectedly passed: #{stdout}" if status.success?
    abort "#{failure} fixture emitted no diagnostic" if (stdout + stderr).strip.empty?
  end
  puts "PASS: terminal wave success and malformed, digest, head, merge, linkage, and ancestry failures"
ensure
  FileUtils.rm_rf(FIXTURE)
end
