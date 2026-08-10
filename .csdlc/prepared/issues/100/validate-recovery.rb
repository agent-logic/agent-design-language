#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "net/http"
require "open3"
require "pathname"
require "uri"

ROOT = Pathname.new(__dir__).join("../../../..").realpath
MANIFEST = ROOT.join(".csdlc/evidence/100/recovery-manifest.json")
SOURCE_ROOT = "docs/milestones/v0.92/publication/articles"
TITLE_CORRECTION_REVISION = "1f6dceeee8deefcce56a56031f884b7074648e32"
TITLE_ALIGNMENT_REVISION = "bf66695ced6db9227562a99dbe1615297cb35ed2"
COMPANY_ACCOUNT = "daniel@agent-logic.ai"
COMPANY_FOLDER_ID = "1hCVwqDLetD9Q8tWEDB8e3nTYzvI1Q-rd"
CREDENTIAL_PATH = Pathname.new(
  ENV.fetch(
    "ADL_COMPANY_DRIVE_CREDENTIAL_FILE",
    File.expand_path("~/keys/gcp-adl-drive-mirror-authorized-user.json")
  )
)
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

def https_request(uri, request)
  Net::HTTP.start(uri.hostname, uri.port, use_ssl: true, open_timeout: 15, read_timeout: 30) do |http|
    http.request(request)
  end
rescue StandardError => error
  fail!("company Drive request failed: #{error.class}")
end

def company_drive_token
  fail!("approved company Drive credential is missing") unless CREDENTIAL_PATH.file?
  credential = JSON.parse(CREDENTIAL_PATH.read)
  token_uri = URI(credential.fetch("token_uri", "https://oauth2.googleapis.com/token"))
  request = Net::HTTP::Post.new(token_uri)
  request.set_form_data(
    "client_id" => credential.fetch("client_id"),
    "client_secret" => credential.fetch("client_secret"),
    "refresh_token" => credential.fetch("refresh_token"),
    "grant_type" => "refresh_token"
  )
  response = https_request(token_uri, request)
  fail!("approved company Drive credential refresh failed: HTTP #{response.code}") unless response.is_a?(Net::HTTPSuccess)
  JSON.parse(response.body).fetch("access_token")
rescue JSON::ParserError, KeyError
  fail!("approved company Drive credential is invalid")
end

def drive_get(path, access_token, json: true)
  uri = URI("https://www.googleapis.com#{path}")
  request = Net::HTTP::Get.new(uri)
  request["Authorization"] = "Bearer #{access_token}"
  response = https_request(uri, request)
  fail!("company Drive read failed for #{uri.path}: HTTP #{response.code}") unless response.is_a?(Net::HTTPSuccess)
  json ? JSON.parse(response.body) : response.body
rescue JSON::ParserError
  fail!("company Drive returned invalid metadata for #{uri.path}")
end

fail!("missing recovery manifest") unless MANIFEST.file?
manifest = JSON.parse(MANIFEST.read)
fail!("wrong manifest schema") unless manifest["schema"] == "adl.medium-recovery-manifest.v1"
fail!("canonical revision does not include all title alignments") unless manifest["canonical_revision"] == TITLE_ALIGNMENT_REVISION

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
  variants.each do |variant|
    source = variant.fetch("source")
    expected_digest = variant.fetch("sha256")
    match = source.match(/\Agit:([0-9a-f]{7,40}):(.+)\z/)
    actual_digest = if match
      Digest::SHA256.hexdigest(git_blob(match[1], match[2]))
    else
      variant_path = ROOT.join(source).cleanpath
      fail!("#{title}: variant path escapes repository") unless variant_path.to_s.start_with?(ROOT.to_s + File::SEPARATOR)
      fail!("#{title}: variant path is absent") unless variant_path.file?
      Digest::SHA256.file(variant_path).hexdigest
    end
    fail!("#{title}: preserved variant digest mismatch") unless actual_digest == expected_digest
  end
end

search_inventory_path = ROOT.join(manifest.fetch("search_inventory"))
fail!("recovery search inventory is missing") unless search_inventory_path.file?
search_inventory = JSON.parse(search_inventory_path.read)
fail!("recovery search inventory schema is wrong") unless search_inventory["schema"] == "adl.medium-recovery-search-inventory.v1"
scopes = search_inventory.fetch("scopes")
fail!("recovery search did not cover all required authorities") unless scopes.map { |scope| scope["kind"] }.sort == %w[
  approved_company_drive_destination
  registered_fastwork_worktrees
  repository_demo_history
  repository_history
].sort
fail!("recovery search did not recover all articles") unless search_inventory.dig("result", "all_required_articles_recovered") == true
fail!("recovery search inventory lost variants") unless search_inventory.dig("result", "substantive_variants_retained") == articles.sum { |article| article.fetch("preserved_variants").length }

drive = manifest["drive_verification"]
fail!("Drive verification is absent") unless drive.is_a?(Hash)
fail!("Drive folder is wrong") unless drive["folder_id"] == COMPANY_FOLDER_ID
fail!("Drive inventory is incomplete") unless drive["readable_canonical_count"] == 10
fail!("Drive supporting inventory is incomplete") unless drive["readable_supporting_count"] == 24
fail!("Drive package inventory is incomplete") unless drive["readable_package_count"] == 34
fail!("Drive verification did not use the approved credential") unless drive["credential_scope"] == "approved_company"
fail!("Drive verification used the wrong company account") unless drive["account"] == COMPANY_ACCOUNT

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

alignment_receipt_path = ROOT.join(drive.fetch("title_alignment_receipt"))
fail!("title alignment receipt is missing") unless alignment_receipt_path.file?
alignment_receipt = JSON.parse(alignment_receipt_path.read)
fail!("title alignment used the wrong account") unless alignment_receipt["account"] == COMPANY_ACCOUNT
fail!("title alignment used the wrong folder") unless alignment_receipt["folder_id"] == drive["folder_id"]
alignment_results = alignment_receipt.fetch("results")
fail!("title alignment result count is wrong") unless alignment_receipt["aligned_count"] == 8 && alignment_results.length == 8
fail!("title alignments were not exact title-only readbacks") unless alignment_results.all? { |result| result["change"] == "launch_title_alignment_only" && result["exact_readback"] == true }

expected_article_paths = Dir.glob(ROOT.join(SOURCE_ROOT, "[0-9][0-9]-*", "ARTICLE.md")).map { |path| Pathname.new(path).relative_path_from(ROOT).to_s }
expected_review_paths = Dir.glob(ROOT.join(SOURCE_ROOT, "[0-9][0-9]-*", "EDITORIAL_REVIEW.md")).map { |path| Pathname.new(path).relative_path_from(ROOT).to_s }
expected_packet_paths = Dir.glob(ROOT.join(SOURCE_ROOT, "[0-9][0-9]-*", "SOURCE_PACKET.md")).map { |path| Pathname.new(path).relative_path_from(ROOT).to_s }
expected_series_paths = %w[EDITORIAL_PANEL_REVIEW.md PUBLICATION_DISPOSITION.md README.md SERIES_ARC_AND_CLAIM_MATRIX.md].map { |name| "#{SOURCE_ROOT}/#{name}" }
expected_paths = (expected_article_paths + expected_review_paths + expected_packet_paths + expected_series_paths).sort
fail!("source package does not contain 10 articles, 10 reviews, 10 packets, and 4 series records") unless [expected_article_paths.length, expected_review_paths.length, expected_packet_paths.length, expected_series_paths.length] == [10, 10, 10, 4]

receipt_results = canonical_results + supporting_results
receipt_by_path = receipt_results.to_h { |result| [result.fetch("source_path"), result] }
correction_by_path = correction_results.to_h { |result| [result.fetch("source_path"), result] }
alignment_by_path = alignment_results.to_h { |result| [result.fetch("source_path"), result] }
fail!("receipt package path set is not exactly 34 files") unless receipt_by_path.keys.sort == expected_paths
expected_paths.each do |source_path|
  result = alignment_by_path.fetch(source_path, correction_by_path.fetch(source_path, receipt_by_path.fetch(source_path)))
  fail!("#{source_path}: receipt readback failed") unless result["exact_readback"] == true
  fail!("#{source_path}: receipt digest does not match current source") unless result["sha256"] == Digest::SHA256.file(ROOT.join(source_path)).hexdigest
end

access_token = company_drive_token
profile = drive_get("/drive/v3/about?fields=user(emailAddress)", access_token)
live_account = profile.dig("user", "emailAddress")
fail!("live Drive credential resolved to the wrong account") unless live_account == COMPANY_ACCOUNT

live_results = expected_paths.map do |source_path|
  retained = alignment_by_path.fetch(source_path, correction_by_path.fetch(source_path, receipt_by_path.fetch(source_path)))
  drive_id = retained.fetch("drive_id")
  encoded_id = URI.encode_www_form_component(drive_id)
  metadata = drive_get(
    "/drive/v3/files/#{encoded_id}?fields=id,name,parents,size,trashed,permissions(type,role,allowFileDiscovery)",
    access_token
  )
  fail!("#{source_path}: live Drive file is trashed") if metadata["trashed"] == true
  fail!("#{source_path}: live Drive file is outside the approved folder") unless metadata.fetch("parents", []).include?(COMPANY_FOLDER_ID)
  fail!("#{source_path}: live Drive file is publicly shared") if metadata.fetch("permissions", []).any? { |permission| permission["type"] == "anyone" }

  content = drive_get("/drive/v3/files/#{encoded_id}?alt=media", access_token, json: false)
  local_digest = Digest::SHA256.file(ROOT.join(source_path)).hexdigest
  fail!("#{source_path}: live Drive content is unreadable or empty") if content.empty?
  fail!("#{source_path}: live Drive digest mismatch") unless Digest::SHA256.hexdigest(content) == local_digest
  drive_id
end
fail!("live Drive verification did not cover 34 unique files") unless live_results.uniq.length == 34

fail!("manifest mutation totals are wrong") unless manifest.values_at("remote_deletions", "authorized_title_corrections", "remote_overwrites", "sharing_changes") == [0, 3, 11, 0]
fail!("title correction paths are not the exact authorized three") unless correction_by_path.keys.sort == [
  "#{SOURCE_ROOT}/01-what-is-adl/ARTICLE.md",
  "#{SOURCE_ROOT}/01-what-is-adl/EDITORIAL_REVIEW.md",
  "#{SOURCE_ROOT}/01-what-is-adl/SOURCE_PACKET.md"
].sort

puts "issue-100 recovery validation: PASS (34 live company Drive files, exact SHA-256 readback, no public permissions, 14 retained variants, and launch-plan title alignment)"
