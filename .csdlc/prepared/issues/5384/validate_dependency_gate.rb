#!/usr/bin/env ruby

require "json"
require "digest"
require "open3"
require "time"

root = File.expand_path("../../../..", __dir__)
manifest = JSON.parse(File.read(File.join(__dir__, "dependency-gate.json")))
failures = []

git_dir, git_dir_status = Open3.capture2("git", "-C", root, "rev-parse", "--git-common-dir")
abort "cannot resolve Git common directory" unless git_dir_status.success?
git_dir = File.expand_path(git_dir.strip, root)

expected_base_sha = manifest.fetch("expected_base_sha")
resolved_base_sha, base_status = Open3.capture2("git", "-C", root, "rev-parse", manifest.fetch("base_ref"))
abort "cannot resolve #{manifest.fetch("base_ref")}" unless base_status.success?
resolved_base_sha = resolved_base_sha.strip
failures << "base drift: expected #{expected_base_sha}, observed #{resolved_base_sha}" unless resolved_base_sha == expected_base_sha

live_path = File.join(root, manifest.fetch("live_evidence"))
if File.file?(live_path)
  live = JSON.parse(File.read(live_path))
else
  live = {"predecessors" => []}
  failures << "missing live dependency snapshot: #{manifest.fetch("live_evidence")}"
end

failures << "live snapshot is not marked complete" unless live["complete"] == true
failures << "live snapshot base does not match expected base" unless live["observed_base_sha"] == expected_base_sha
source = live["source"] || {}
failures << "live snapshot connector is not approved" unless source["connector_kind"] == "codex_apps.github_fetch_issue_and_pr"
failures << "live snapshot repository is incorrect" unless source["repository"] == "danielbaustin/agent-design-language"
failures << "live snapshot digest algorithm is unsupported" unless source["observation_digest_algorithm"] == "sha256-canonical-json-predecessors"
actual_observation_digest = Digest::SHA256.hexdigest(JSON.generate(live.fetch("predecessors", [])))
failures << "live snapshot observation digest mismatch" unless source["observation_digest"] == actual_observation_digest
begin
  observed_at = Time.iso8601(live.fetch("observed_at"))
  age = Time.now.utc - observed_at
  max_age = manifest.fetch("max_live_evidence_age_seconds")
  failures << "live snapshot is stale: age #{age.to_i}s exceeds #{max_age}s" if age.negative? || age > max_age
rescue KeyError, ArgumentError
  failures << "live snapshot lacks a valid observed_at timestamp"
end
live_by_issue = live.fetch("predecessors", []).to_h { |entry| [entry["issue"], entry] }

manifest.fetch("predecessors").each do |entry|
  issue = entry.fetch("issue")
  observation = live_by_issue[issue]
  if observation.nil?
    failures << "##{issue}: missing live issue/PR observation"
  else
    failures << "##{issue}: live issue state is #{observation["issue_state"].inspect}" unless observation["issue_state"] == "closed"
    failures << "##{issue}: live implementation PR state is #{observation["pr_state"].inspect}" unless observation["pr_state"] == "merged"
    failures << "##{issue}: merged PR number is absent" unless observation["pr_number"].is_a?(Integer)
    if observation["pr_number"].is_a?(Integer)
      begin
        pr_observed_at = Time.iso8601(observation.fetch("pr_observed_at"))
        pr_age = Time.now.utc - pr_observed_at
        max_age = manifest.fetch("max_live_evidence_age_seconds")
        failures << "##{issue}: PR observation is stale or future-dated" if pr_age.negative? || pr_age > max_age
      rescue KeyError, ArgumentError
        failures << "##{issue}: PR observation timestamp is invalid"
      end
    else
      failures << "##{issue}: PR observation timestamp is absent" if observation["pr_observed_at"].to_s.empty?
    end
  end

  projection_path = File.join(root, ".csdlc", "issues", issue.to_s, "index.json")
  receipt_path = File.join(git_dir, "csdlc-v2", "closeout", "#{issue}.json")
  unless File.file?(projection_path)
    failures << "##{issue}: missing tracked projection"
    next
  end
  unless File.file?(receipt_path)
    failures << "##{issue}: missing shared-Git closeout receipt"
    next
  end

  projection = JSON.parse(File.read(projection_path))
  receipt = JSON.parse(File.read(receipt_path))
  terminal = receipt.dig("record", "terminal") || {}
  observed_sha = terminal["observed_sha"]

  failures << "##{issue}: projection phase is #{projection["phase"].inspect}" unless projection["phase"] == "closed_out"
  failures << "##{issue}: receipt phase is #{receipt.dig("record", "phase").inspect}" unless receipt.dig("record", "phase") == "closed_out"
  failures << "##{issue}: receipt disposition is #{terminal["disposition"].inspect}" unless terminal["disposition"] == "merged"
  failures << "##{issue}: receipt PR disagrees with live observation" if observation && terminal["pull_request"] != observation["pr_number"]
  if observed_sha.to_s.empty?
    failures << "##{issue}: receipt lacks observed SHA"
  else
    ancestral = system("git", "-C", root, "merge-base", "--is-ancestor", observed_sha, expected_base_sha,
                       out: File::NULL, err: File::NULL)
    failures << "##{issue}: #{observed_sha} is not an ancestor of #{expected_base_sha}" unless ancestral
  end
end

result = {
  schema: "adl.csdlc.predecessor_gate.result.v1",
  issue: manifest.fetch("issue"),
  base_sha: expected_base_sha,
  ready: failures.empty?,
  failures: failures
}
puts JSON.pretty_generate(result)
exit(failures.empty? ? 0 : 3)
