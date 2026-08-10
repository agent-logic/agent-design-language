#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"
require "uri"

ROOT = Pathname.new(__dir__).join("../../../..").realpath
MANIFEST = ROOT.join(".csdlc/evidence/100/recovery-manifest.json")
SOURCE_ROOT = "docs/milestones/v0.92/publication/articles"
TITLE_CORRECTION_REVISION = "1f6dceeee8deefcce56a56031f884b7074648e32"
REQUIRED_TITLES = [
  "What is ADL?",
  "The ADL Runtime and the Cognitive Spacetime Model",
  "Goedel Agents and the Goedel-Hadamard-Bayes Algorithm",
  "The Freedom Gate",
  "UTS and ACC - Making Agents With Tools Safe",
  "CodeFriend and the Cognitive SDLC",
  "Continuous Adversarial Verification For Continuous Security",
  "Agent Economics",
  "ADL and Social Intelligence",
  "What's Next for ADL?"
].freeze

def fail!(message)
  warn "issue-100 recovery validation: FAIL: #{message}"
  exit 1
end

def git_blob(revision, path)
  stdout, stderr, status = Open3.capture3("git", "-C", ROOT.to_s, "show", "#{revision}:#{path}")
  fail!("cannot read #{revision}:#{path}: #{stderr.strip}") unless status.success?
  stdout
end

fail!("missing recovery manifest") unless MANIFEST.file?
manifest = JSON.parse(MANIFEST.read)
fail!("wrong manifest schema") unless manifest["schema"] == "adl.medium-recovery-manifest.v1"
fail!("canonical revision does not include the title correction") unless manifest["canonical_revision"] == TITLE_CORRECTION_REVISION

articles = manifest["articles"]
fail!("articles must contain exactly ten entries") unless articles.is_a?(Array) && articles.length == 10
titles = articles.map { |article| article["title"] }
fail!("required title set is incomplete or duplicated") unless titles.sort == REQUIRED_TITLES.sort

articles.each do |article|
  title = article.fetch("title")
  fail!("#{title}: recovery is not complete") unless article["recovery_status"] == "recovered"
  fail!("#{title}: canonical source is missing") if article["source"].to_s.strip.empty?
  fail!("#{title}: canonical file is missing") if article["canonical_path"].to_s.strip.empty?
  fail!("#{title}: Drive URL is missing") if article["drive_url"].to_s.strip.empty?

  uri = URI.parse(article["drive_url"])
  fail!("#{title}: Drive URL is not HTTPS") unless uri.is_a?(URI::HTTPS) && uri.host == "drive.google.com"

  path = ROOT.join(article["canonical_path"]).cleanpath
  fail!("#{title}: canonical path escapes repository") unless path.to_s.start_with?(ROOT.to_s + File::SEPARATOR)
  fail!("#{title}: canonical file is absent") unless path.file?
  fail!("#{title}: canonical file is empty") if path.size.zero?

  digest = Digest::SHA256.file(path).hexdigest
  fail!("#{title}: digest mismatch") unless article["sha256"] == digest
  source_match = article["source"].match(/\Agit:([0-9a-f]{7,40}):(.+)\z/)
  fail!("#{title}: canonical source is not a git revision and path") unless source_match
  source_digest = Digest::SHA256.hexdigest(git_blob(source_match[1], source_match[2]))
  fail!("#{title}: source revision digest mismatch") unless source_digest == article["sha256"]
  variants = article["preserved_variants"]
  fail!("#{title}: preserved variants must be listed") unless variants.is_a?(Array)
end

drive = manifest["drive_verification"]
fail!("Drive verification is absent") unless drive.is_a?(Hash)
fail!("Drive folder is wrong") unless drive["folder_id"] == "1hCVwqDLetD9Q8tWEDB8e3nTYzvI1Q-rd"
fail!("Drive inventory is incomplete") unless drive["readable_canonical_count"] == 10
fail!("Drive supporting inventory is incomplete") unless drive["readable_supporting_count"] == 24
fail!("Drive package inventory is incomplete") unless drive["readable_package_count"] == 34
fail!("Drive verification did not use the approved credential") unless drive["credential_scope"] == "approved_company"
fail!("Drive verification used the wrong company account") unless drive["account"] == "daniel@agent-logic.ai"

destination = manifest["destination_reconciliation"]
fail!("destination reconciliation is absent") unless destination.is_a?(Hash)
fail!("original issue folder was not marked inaccessible") unless destination["issue_body_folder_id"] == "1hacu6zwCUlIYXYtvpMW0IFtk506LUb8Q" && destination["issue_body_folder_company_access"] == "not_found"
fail!("approved company folder is not authoritative") unless destination["correct_company_folder_id"] == drive["folder_id"]

canonical_receipt = JSON.parse(ROOT.join(drive.fetch("canonical_receipt")).read)
canonical_results = canonical_receipt.fetch("canonical_results")
fail!("canonical receipt count is wrong") unless canonical_results.length == 10 && canonical_receipt["canonical_count"] == 10
fail!("canonical upload readback failed") unless canonical_receipt["all_exact_readback"] == true
fail!("canonical upload changed remote content") unless canonical_receipt.values_at("remote_deletions", "remote_overwrites", "sharing_changes") == [0, 0, 0]

support_receipt_path = ROOT.join(drive.fetch("supporting_receipt"))
fail!("supporting upload receipt is missing") unless support_receipt_path.file?
support_receipt = JSON.parse(support_receipt_path.read)
fail!("supporting upload used the wrong account") unless support_receipt["account"] == "daniel@agent-logic.ai"
fail!("supporting upload used the wrong folder") unless support_receipt["folder_id"] == drive["folder_id"]
fail!("supporting upload count is wrong") unless support_receipt["supporting_count"] == 24
fail!("supporting upload readback failed") unless support_receipt["all_exact_readback"] == true
fail!("supporting upload changed remote content") unless support_receipt.values_at("remote_deletions", "remote_overwrites", "sharing_changes") == [0, 0, 0]
supporting_results = support_receipt.fetch("supporting_results")
fail!("supporting receipt entry count is wrong") unless supporting_results.length == 24

correction_receipt_path = ROOT.join(drive.fetch("title_correction_receipt"))
fail!("title correction receipt is missing") unless correction_receipt_path.file?
correction_receipt = JSON.parse(correction_receipt_path.read)
fail!("title correction used the wrong account") unless correction_receipt["account"] == "daniel@agent-logic.ai"
fail!("title correction used the wrong folder") unless correction_receipt["folder_id"] == drive["folder_id"]
fail!("title correction is incomplete") unless correction_receipt["requested_title"] == "What is ADL?" && correction_receipt["corrected_count"] == 3
fail!("title correction readback failed") unless correction_receipt["all_exact_readback"] == true
fail!("title correction changed sharing or deleted content") unless correction_receipt.values_at("remote_deletions", "sharing_changes") == [0, 0]
correction_results = correction_receipt.fetch("results")
fail!("title correction result count is wrong") unless correction_results.length == 3
fail!("title corrections were not explicitly title-only") unless correction_results.all? { |result| result["change"] == "title_case_only" && result["exact_readback"] == true }

expected_article_paths = Dir.glob(ROOT.join(SOURCE_ROOT, "[0-9][0-9]-*", "ARTICLE.md")).map { |path| Pathname.new(path).relative_path_from(ROOT).to_s }
expected_review_paths = Dir.glob(ROOT.join(SOURCE_ROOT, "[0-9][0-9]-*", "EDITORIAL_REVIEW.md")).map { |path| Pathname.new(path).relative_path_from(ROOT).to_s }
expected_packet_paths = Dir.glob(ROOT.join(SOURCE_ROOT, "[0-9][0-9]-*", "SOURCE_PACKET.md")).map { |path| Pathname.new(path).relative_path_from(ROOT).to_s }
expected_series_paths = %w[EDITORIAL_PANEL_REVIEW.md PUBLICATION_DISPOSITION.md README.md SERIES_ARC_AND_CLAIM_MATRIX.md].map { |name| "#{SOURCE_ROOT}/#{name}" }
expected_paths = (expected_article_paths + expected_review_paths + expected_packet_paths + expected_series_paths).sort
fail!("source package does not contain 10 articles, 10 reviews, 10 packets, and 4 series records") unless [expected_article_paths.length, expected_review_paths.length, expected_packet_paths.length, expected_series_paths.length] == [10, 10, 10, 4]

receipt_results = canonical_results + supporting_results
receipt_by_path = receipt_results.to_h { |result| [result.fetch("source_path"), result] }
correction_by_path = correction_results.to_h { |result| [result.fetch("source_path"), result] }
fail!("receipt package path set is not exactly 34 files") unless receipt_by_path.keys.sort == expected_paths
expected_paths.each do |source_path|
  result = correction_by_path.fetch(source_path, receipt_by_path.fetch(source_path))
  fail!("#{source_path}: receipt readback failed") unless result["exact_readback"] == true
  fail!("#{source_path}: receipt digest does not match current source") unless result["sha256"] == Digest::SHA256.file(ROOT.join(source_path)).hexdigest
end

fail!("manifest mutation totals are wrong") unless manifest.values_at("remote_deletions", "authorized_title_corrections", "remote_overwrites", "sharing_changes") == [0, 3, 3, 0]
fail!("title correction paths are not the exact authorized three") unless correction_by_path.keys.sort == [
  "#{SOURCE_ROOT}/01-what-is-adl/ARTICLE.md",
  "#{SOURCE_ROOT}/01-what-is-adl/EDITORIAL_REVIEW.md",
  "#{SOURCE_ROOT}/01-what-is-adl/SOURCE_PACKET.md"
].sort

puts "issue-100 recovery validation: PASS (34 files: 10 articles, 10 editorial reviews, 10 source packets, 4 series records; 3 authorized title-only corrections)"
