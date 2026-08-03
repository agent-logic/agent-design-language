#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCY = 5356
RECEIPT_REF = "csdlc-v2/closeout/5356.json"
HEX40 = /\A[0-9a-f]{40}\z/

def fail_gate(message)
  warn("#5357 WP-18 gate: #{message}")
  exit 1
end

def git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  fail_gate("git #{args.join(' ')} failed: #{out.strip}") unless status.success?
  out.strip
end

begin
  common = Pathname.new(git("rev-parse", "--git-common-dir"))
  common = ROOT.join(common) unless common.absolute?
  receipt_path = common.join(RECEIPT_REF)
  fail_gate("missing retained terminal receipt #{RECEIPT_REF}") unless receipt_path.file?

  receipt = JSON.parse(receipt_path.read)
  record = receipt.fetch("record") { fail_gate("receipt has no typed record") }
  fail_gate("##{DEPENDENCY} is not typed closed_out") unless record["phase"] == "closed_out"
  fail_gate("##{DEPENDENCY} still has an active claim") unless record["claim"].nil?

  terminal = record.fetch("terminal") { fail_gate("receipt has no terminal evidence") }
  unless terminal["disposition"] == "merged" && terminal["observed_state"] == "merged"
    fail_gate("##{DEPENDENCY} terminal disposition is not merged")
  end
  pull_request = terminal["pull_request"]
  fail_gate("##{DEPENDENCY} terminal record has no PR identity") unless pull_request.is_a?(Integer) && pull_request.positive?
  head_sha = terminal["observed_sha"]
  fail_gate("##{DEPENDENCY} reviewed head SHA is invalid") unless head_sha&.match?(HEX40)
  merge_sha = git("log", "--format=%H", "--fixed-strings", "--grep=(##{pull_request})", "-n", "1", "HEAD")
  fail_gate("cannot resolve merged PR ##{pull_request} from target history") unless merge_sha.match?(HEX40)

  head = git("rev-parse", "HEAD")
  _out, ancestry = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", merge_sha, head)
  fail_gate("##{DEPENDENCY} merge commit #{merge_sha} is not ancestral to #{head}") unless ancestry.success?

  puts JSON.generate(
    status: "pass",
    issue: 5357,
    dependency: DEPENDENCY,
    dependency_head_sha: head_sha,
    dependency_merge_sha: merge_sha,
    dependency_generation: record.fetch("generation"),
    receipt_sha256: Digest::SHA256.file(receipt_path).hexdigest,
    revision: head
  )
rescue JSON::ParserError, KeyError => e
  fail_gate("invalid retained receipt: #{e.message}")
end
