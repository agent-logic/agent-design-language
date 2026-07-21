#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
DEPENDENCIES = %w[5499 5498].freeze

def fail_closed(message)
  warn(message)
  exit 2
end

common_dir, status = Open3.capture2("git", "-C", ROOT.to_s, "rev-parse", "--git-common-dir")
fail_closed("cannot resolve shared Git directory") unless status.success?
common = Pathname.new(common_dir.strip)
common = ROOT.join(common).cleanpath unless common.absolute?

DEPENDENCIES.each do |issue|
  receipt_path = common.join("csdlc-v2/closeout/#{issue}.json")
  fail_closed("##{issue} retained closeout receipt is absent") unless receipt_path.file?

  receipt = JSON.parse(receipt_path.read)
  record = receipt["record"] || receipt
  terminal = record["terminal"] || {}
  merge_sha = terminal["observed_sha"] || record["observed_sha"] || record["merge_sha"]

  fail_closed("##{issue} receipt is not closed_out") unless record["phase"] == "closed_out"
  fail_closed("##{issue} receipt retained an active claim") unless record["claim"].nil?
  fail_closed("##{issue} receipt is not merged") unless terminal["disposition"] == "merged"
  fail_closed("##{issue} receipt omits observed merge SHA") unless merge_sha.to_s.match?(/\A[0-9a-f]{40}\z/)

  ancestral = system("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", merge_sha, "HEAD",
                     out: File::NULL, err: File::NULL)
  fail_closed("##{issue} merge SHA is not ancestral to #5502") unless ancestral

  index_path = ROOT.join(".csdlc/issues/#{issue}/index.json")
  fail_closed("##{issue} typed projection is absent") unless index_path.file?
  index = JSON.parse(index_path.read)
  fail_closed("##{issue} typed projection is not closed_out") unless index["phase"] == "closed_out"
  fail_closed("##{issue} claim remains active") unless index["claim"].nil?
rescue JSON::ParserError => e
  fail_closed("##{issue} terminal evidence is malformed: #{e.message}")
end

puts JSON.pretty_generate(status: "ready", dependencies: DEPENDENCIES.map(&:to_i))
