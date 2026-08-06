#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"
require "time"

ROOT = Pathname.new(__dir__).join("../../../..").cleanpath
EVIDENCE = ROOT.join(".csdlc/evidence/5819")
REPORT = EVIDENCE.join("copy-report.json")
REPOSITORIES = [
  ["cognitive-sdlc-paper", "private"],
  ["godel-hadamard-bayes-paper", "private"],
  ["general-intelligence-paper-private", "private"],
  ["universal-tool-schema", "private"],
  ["agent-design-language", "public"]
].freeze
CONTROLS = {"asksifu" => "private", "Horust" => "public"}.freeze
PLATFORM_SURFACES = %w[
  issues pull_requests milestones projects discussions wiki assignees
  collaborators outside_collaborators teams rulesets branch_protections
  required_checks approvals codeowners releases actions workflows schedules runner_labels
  environments variables secrets oidc webhooks deploy_keys apps
  oauth_integrations callbacks pages custom_domains packages security forks
  submodules downstream_consumers operational_urls
].freeze
DISPOSITIONS = %w[
  recreated source_authoritative not_applicable operator_verified
  cold_mirror_disabled deferred_to_5888
].freeze
SHA256 = /\A[0-9a-f]{64}\z/
GIT_SHA = /\A[0-9a-f]{40,64}\z/
SECRET_PATTERN = %r{
  ghp_|github_pat_|AKIA[0-9A-Z]{16}|AIza[0-9A-Za-z_-]{30,}|
  sk-(?:proj-|ant-)[A-Za-z0-9_-]{20,}|sk-[A-Za-z0-9]{40,}|
  -----BEGIN\ [A-Z\ ]*PRIVATE\ KEY-----|
  Bearer\s+[A-Za-z0-9._~+/=-]{20,}|
  (?:password|passwd|api[_-]?key|secret|token|authorization)\s*[:=]\s*["']?[^\s"',;]{8,}
}ix
SENSITIVE_KEY = /(token|password|credential|private[_-]?key|secret[_-]?value|variable[_-]?value|authorization)/i

def file_artifact(relative, expected_digest, label, allow_empty: false)
  path = ROOT.join(relative.to_s).cleanpath
  abort "#{label} path escapes repository" unless path.to_s.start_with?(ROOT.to_s + File::SEPARATOR)
  abort "missing #{label}: #{relative}" unless path.file? && (allow_empty || !path.zero?)
  abort "invalid #{label} digest" unless expected_digest.to_s.match?(SHA256)
  abort "#{label} digest mismatch" unless Digest::SHA256.file(path).hexdigest == expected_digest
  bytes = path.binread
  abort "secret-like value retained in #{label}" if bytes.match?(SECRET_PATTERN)
  bytes
end

def artifact(relative, expected_digest, label)
  JSON.parse(file_artifact(relative, expected_digest, label))
end

def reject_sensitive_json_values(value, label, path = [])
  case value
  when Hash
    value.each do |key, child|
      if key.match?(SENSITIVE_KEY) && ![false, nil, [], {}].include?(child)
        abort "sensitive value-bearing field retained in #{label}: #{(path + [key]).join('.')}"
      end
      reject_sensitive_json_values(child, label, path + [key])
    end
  when Array
    value.each_with_index { |child, index| reject_sensitive_json_values(child, label, path + [index.to_s]) }
  end
end

def timestamp(value, label)
  Time.iso8601(value.to_s)
rescue ArgumentError
  abort "invalid #{label} timestamp"
end

def required_snapshot(snapshot, repository, label)
  abort "#{label} repository mismatch" unless snapshot["repository"] == repository
  abort "#{label} lacks repository id" if snapshot["repository_id"].to_s.empty?
  abort "#{label} lacks visibility" unless %w[private public].include?(snapshot["visibility"])
  abort "#{label} lacks default branch" if snapshot["default_branch"].to_s.empty?
  abort "#{label} lacks exact HEAD" unless snapshot["exact_head"].to_s.match?(GIT_SHA)
  %w[refs_sha256 api_surface_sha256].each do |field|
    abort "#{label} lacks #{field}" unless snapshot[field].to_s.match?(SHA256)
  end
  timestamp(snapshot["observed_at"], "#{label} observation")
end

EVIDENCE.find.each do |path|
  next unless path.file?
  bytes = path.binread
  abort "secret-like value retained in #{path.relative_path_from(ROOT)}" if bytes.match?(SECRET_PATTERN)
  next unless path.extname == ".json"
  reject_sensitive_json_values(JSON.parse(bytes), path.relative_path_from(ROOT).to_s)
end

abort "missing copy report" unless REPORT.file? && !REPORT.zero?
report_bytes = REPORT.binread
abort "secret-like value retained in copy report" if report_bytes.match?(SECRET_PATTERN)
report = JSON.parse(report_bytes)
abort "wrong copy report schema" unless report["schema"] == "adl.wp02.copy-report.v1"
abort "copy-only authority missing" unless report["copy_only"] == true
abort "source mutation was authorized" unless report["source_mutation_authorized"] == false
abort "secret values retained" unless report["secret_values_retained"] == false
org = report.fetch("organization_readiness")
abort "organization readiness confirmation missing" unless org["confirmation_comment_id"].to_i.positive?

rows = Array(report["repositories"])
abort "copy repository order mismatch" unless rows.map { |row| row["name"] } == REPOSITORIES.map(&:first)
abort "copy sequence mismatch" unless rows.map { |row| row["copy_sequence"] } == (1..REPOSITORIES.length).to_a

previous_completed = nil
first_started = nil
rows.each_with_index do |row, index|
  name, visibility = REPOSITORIES.fetch(index)
  source = "danielbaustin/#{name}"
  destination = "agent-logic/#{name}"
  abort "#{name} source mismatch" unless row["source"] == source
  abort "#{name} destination mismatch" unless row["destination"] == destination
  abort "#{name} visibility mismatch" unless row["expected_visibility"] == visibility

  started = timestamp(row["copy_started_at"], "#{name} copy start")
  completed = timestamp(row["copy_completed_at"], "#{name} copy completion")
  first_started ||= started
  abort "#{name} copy completed before start" if completed < started
  abort "#{name} copy overlapped prior repository" if previous_completed && started <= previous_completed
  previous_completed = completed

  source_before = artifact(row["source_before_path"], row["source_before_sha256"], "#{name} source-before")
  destination_after = artifact(row["destination_after_path"], row["destination_after_sha256"], "#{name} destination-after")
  source_after = artifact(row["source_after_path"], row["source_after_sha256"], "#{name} source-after")
  required_snapshot(source_before, source, "#{name} source-before")
  required_snapshot(source_after, source, "#{name} source-after")
  required_snapshot(destination_after, destination, "#{name} destination-after")
  source_before_at = timestamp(source_before["observed_at"], "#{name} source-before observation")
  source_after_at = timestamp(source_after["observed_at"], "#{name} source-after observation")
  destination_after_at = timestamp(destination_after["observed_at"], "#{name} destination-after observation")
  abort "#{name} source-before snapshot is not before the copy window" if source_before_at > started
  abort "#{name} source-after snapshot precedes the copy window" unless source_after_at >= started
  abort "#{name} destination-after snapshot precedes the copy window" unless destination_after_at >= started

  abort "#{name} source visibility changed" unless source_before["visibility"] == source_after["visibility"]
  abort "#{name} source repository identity changed" unless source_before["repository_id"] == source_after["repository_id"]
  abort "#{name} source default branch changed" unless source_before["default_branch"] == source_after["default_branch"]
  abort "#{name} source HEAD changed" unless source_before["exact_head"] == source_after["exact_head"]
  abort "#{name} source refs changed" unless source_before["refs_sha256"] == source_after["refs_sha256"]
  abort "#{name} source API-visible settings changed" unless source_before["api_surface_sha256"] == source_after["api_surface_sha256"]
  abort "#{name} source immutability not asserted" unless row["source_unchanged"] == true

  abort "#{name} destination visibility incorrect" unless destination_after["visibility"] == visibility
  abort "#{name} destination reused the source repository identity" unless destination_after["repository_id"] != source_before["repository_id"]
  abort "#{name} destination default branch differs" unless destination_after["default_branch"] == source_before["default_branch"]
  abort "#{name} destination HEAD differs" unless destination_after["exact_head"] == source_before["exact_head"]
  abort "#{name} destination refs differ" unless destination_after["refs_sha256"] == source_before["refs_sha256"]
  abort "#{name} destination not verified" unless row["destination_verified"] == true
  abort "#{name} operator confirmation missing" unless row["operator_confirmation_comment_id"].to_i.positive?
  abort "#{name} source visibility field differs from snapshot" unless row["source_live_visibility"] == source_before["visibility"]
  abort "#{name} source default-branch field differs from snapshot" unless row["source_default_branch"] == source_before["default_branch"]
  abort "#{name} exact HEAD field differs from snapshot" unless row["exact_head"] == source_before["exact_head"]
  abort "#{name} expected Actions state is not boolean" unless [true, false].include?(row["expected_actions_enabled"])
  abort "#{name} destination Actions snapshot differs from expected state" unless destination_after["actions_enabled"] == row["expected_actions_enabled"]

  receipt = artifact(row["actions_disabled_receipt_path"], row["actions_disabled_receipt_sha256"], "#{name} Actions-disabled receipt")
  abort "#{name} Actions was not disabled before push" unless receipt["repository"] == destination && receipt["actions_enabled"] == false
  actions_response = artifact(receipt["api_response_path"], receipt["api_response_sha256"], "#{name} Actions-disabled API response")
  abort "#{name} Actions API response is not disabled" unless actions_response["enabled"] == false
  disabled_at = timestamp(receipt["observed_at"], "#{name} Actions-disabled observation")
  abort "#{name} Actions disablement is outside the copy window" unless started <= disabled_at && disabled_at <= completed

  push = artifact(row["first_push_receipt_path"], row["first_push_receipt_sha256"], "#{name} first-push receipt")
  abort "#{name} first-push destination mismatch" unless push["destination"] == destination
  abort "#{name} first push did not succeed" unless push["exit_status"] == 0
  abort "#{name} first-push source refs mismatch" unless push["source_refs_sha256"] == source_before["refs_sha256"]
  abort "#{name} first-push destination refs mismatch" unless push["destination_refs_sha256"] == source_before["refs_sha256"]
  file_artifact(push["transcript_path"], push["transcript_sha256"], "#{name} first-push transcript")
  pushed_at = timestamp(push["pushed_at"], "#{name} first push")
  abort "#{name} first push is outside the copy window" unless started <= pushed_at && pushed_at <= completed
  abort "#{name} Actions disablement was not before the first push" unless disabled_at < pushed_at

  first_ref_event = artifact(
    row["first_ref_event_path"],
    row["first_ref_event_sha256"],
    "#{name} first-ref GitHub event"
  )
  abort "#{name} first-ref event id is missing" if first_ref_event["id"].to_s.empty?
  abort "#{name} first-ref event type mismatch" unless first_ref_event["type"] == "CreateEvent"
  abort "#{name} first-ref event repository mismatch" unless first_ref_event.dig("repo", "name") == destination
  abort "#{name} first-ref event actor mismatch" unless first_ref_event.dig("actor", "login") == "danielbaustin"
  abort "#{name} first-ref event is not a branch" unless first_ref_event.dig("payload", "ref_type") == "branch"
  abort "#{name} first-ref event lacks a ref" if first_ref_event.dig("payload", "ref").to_s.empty?
  first_ref_at = timestamp(first_ref_event["created_at"], "#{name} first-ref GitHub event")
  abort "#{name} GitHub recorded ref arrival before Actions disablement" unless disabled_at < first_ref_at
  abort "#{name} first-ref event is outside the copy window" unless started <= first_ref_at && first_ref_at <= completed
  abort "#{name} local push completion precedes GitHub ref arrival" unless first_ref_at <= pushed_at

  serial_gate_at = timestamp(row["serial_gate_confirmed_at"], "#{name} serial-gate confirmation")
  abort "#{name} serial-gate confirmation id missing" unless row["serial_gate_confirmation_comment_id"].to_i.positive?
  abort "#{name} serial gate preceded its first push" unless serial_gate_at >= pushed_at
  if index + 1 < rows.length
    next_started = timestamp(rows.fetch(index + 1)["copy_started_at"], "#{name} next-copy start")
    abort "#{name} was not confirmed before the next copy started" unless serial_gate_at < next_started
  else
    abort "#{name} final serial gate is outside the copy window" unless serial_gate_at <= completed
  end
  abort "#{name} source-after snapshot was not captured after the first push" unless source_after_at >= pushed_at
  abort "#{name} destination-after snapshot was not captured after the first push" unless destination_after_at >= pushed_at

  lfs = row.fetch("lfs")
  abort "#{name} invalid LFS disposition" unless %w[verified no_lfs].include?(lfs["status"])
  lfs_receipt = artifact(lfs["receipt_path"], lfs["receipt_sha256"], "#{name} LFS receipt")
  abort "#{name} LFS source mismatch" unless lfs_receipt["source"] == source
  abort "#{name} LFS destination mismatch" unless lfs_receipt["destination"] == destination
  abort "#{name} LFS status mismatch" unless lfs_receipt["status"] == lfs["status"]
  abort "#{name} source LFS fsck failed" unless lfs_receipt["source_fsck_exit"] == 0
  abort "#{name} destination LFS fsck failed" unless lfs_receipt["destination_fsck_exit"] == 0
  source_lfs = file_artifact(lfs_receipt["source_inventory_path"], lfs_receipt["source_inventory_sha256"], "#{name} source LFS inventory", allow_empty: true)
  destination_lfs = file_artifact(lfs_receipt["destination_inventory_path"], lfs_receipt["destination_inventory_sha256"], "#{name} destination LFS inventory", allow_empty: true)
  abort "#{name} LFS inventories differ" unless source_lfs == destination_lfs
  abort "#{name} LFS object counts differ" unless lfs_receipt["source_object_count"] == lfs_receipt["destination_object_count"]
  if lfs["status"] == "verified"
    abort "#{name} verified LFS receipt has no objects" unless lfs_receipt["source_object_count"].to_i.positive?
    file_artifact(lfs_receipt["push_transcript_path"], lfs_receipt["push_transcript_sha256"], "#{name} LFS push transcript")
  else
    abort "#{name} no-LFS receipt reports objects" unless lfs_receipt["source_object_count"] == 0
    abort "#{name} no-LFS disposition lacks evidence" if lfs["reason"].to_s.empty?
  end

  packet = artifact(
    row["platform_disposition_packet_path"],
    row["platform_disposition_packet_sha256"],
    "#{name} platform disposition packet"
  )
  abort "#{name} platform packet source mismatch" unless packet["source"] == source
  abort "#{name} platform packet destination mismatch" unless packet["destination"] == destination
  abort "#{name} platform packet retained secret values" unless packet["secret_values_retained"] == false
  timestamp(packet["observed_at"], "#{name} platform packet observation")
  dispositions = row.fetch("platform_dispositions")
  abort "#{name} platform disposition denominator mismatch" unless dispositions.keys.sort == PLATFORM_SURFACES.sort
  abort "#{name} platform packet denominator mismatch" unless packet.fetch("surfaces").keys.sort == PLATFORM_SURFACES.sort
  dispositions.each do |surface, disposition|
    abort "#{name} #{surface} disposition invalid" unless DISPOSITIONS.include?(disposition["status"])
    abort "#{name} #{surface} disposition lacks rationale" if disposition["reason"].to_s.empty?
    entry = packet.fetch("surfaces").fetch(surface)
    abort "#{name} #{surface} evidence status mismatch" unless entry["status"] == disposition["status"]
    observed_at = timestamp(entry["observed_at"], "#{name} #{surface} evidence")
    abort "#{name} #{surface} evidence precedes the copy window" unless observed_at >= started
    proof = entry.fetch("proof")
    case proof["kind"]
    when "live_api"
      %w[source_sha256 destination_sha256].each do |field|
        abort "#{name} #{surface} live API proof lacks #{field}" unless proof[field].to_s.match?(SHA256)
      end
    when "operator_confirmation"
      manual = artifact(proof["evidence_path"], proof["evidence_sha256"], "#{name} #{surface} manual evidence")
      abort "#{name} #{surface} manual evidence source mismatch" unless manual["source"] == source
      abort "#{name} #{surface} manual evidence destination mismatch" unless manual["destination"] == destination
      abort "#{name} #{surface} manual evidence identity mismatch" unless manual["surface"] == surface
      abort "#{name} #{surface} manual evidence did not pass" unless manual["result"] == "pass"
      abort "#{name} #{surface} manual evidence retained secret values" unless manual["secret_values_retained"] == false
    when "not_applicable"
      abort "#{name} #{surface} proof is not applicable but disposition is not" unless disposition["status"] == "not_applicable"
    else
      abort "#{name} #{surface} proof kind is invalid"
    end
    if %w[secrets variables].include?(surface)
      abort "#{name} #{surface} evidence is not names-only" unless entry["names_only"] == true
      abort "#{name} #{surface} values were retained" unless entry["values_retained"] == false
    end
  end
end

controls = report.fetch("negative_controls")
abort "negative-control denominator mismatch" unless controls.keys.sort == CONTROLS.keys.sort
CONTROLS.each do |name, visibility|
  row = controls.fetch(name)
  repository = "danielbaustin/#{name}"
  before = artifact(row["source_before_path"], row["source_before_sha256"], "#{name} control-before")
  after = artifact(row["source_after_path"], row["source_after_sha256"], "#{name} control-after")
  required_snapshot(before, repository, "#{name} control-before")
  required_snapshot(after, repository, "#{name} control-after")
  before_at = timestamp(before["observed_at"], "#{name} control-before observation")
  after_at = timestamp(after["observed_at"], "#{name} control-after observation")
  abort "#{name} control-before snapshot is not before the copy wave" if before_at > first_started
  abort "#{name} control-after snapshot is not after the copy wave" if after_at < previous_completed
  abort "#{name} control visibility mismatch" unless before["visibility"] == visibility && after["visibility"] == visibility
  abort "#{name} control changed" unless before.slice("repository_id", "visibility", "default_branch", "exact_head", "refs_sha256", "api_surface_sha256") == after.slice("repository_id", "visibility", "default_branch", "exact_head", "refs_sha256", "api_surface_sha256")
  abort "#{name} control repository-id field differs from snapshot" unless row["repository_id"].to_s == before["repository_id"].to_s
  abort "#{name} control HEAD field differs from snapshot" unless row["exact_head"] == before["exact_head"]
  abort "#{name} destination unexpectedly exists" unless row["destination_absent"] == true
end

website = report.fetch("website_handoff")
abort "website handoff must target #5888" unless website["issue"] == 5888
abort "website handoff preceded ADL verification" unless website["blocked_until_adl_verified"] == true

puts "WP-02 copy evidence valid: five destinations, seven immutable sources, #{PLATFORM_SURFACES.length} disposition surfaces"
