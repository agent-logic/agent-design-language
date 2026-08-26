#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "optparse"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").realpath
EXPECTED_WPS = %w[WP-08 WP-09 WP-10 WP-11 WP-12 WP-13 WP-13A WP-14 WP-15].freeze
EXPECTED_ISSUES = [5825, 5826, 5827, 5828, 5829, 5830, 5831, 209, 5833].freeze
CODE_REPOSITORY = "agent-logic/agent-design-language"
LEGACY_ISSUE_REPOSITORY = "danielbaustin/agent-design-language"
FORBIDDEN_PUBLIC_CLAIMS = /\b(personhood|consciousness|production citizenship|legal citizenship|governance authority|public(?:ation)? (?:is )?(?:approved|authorized|ready)|ready for public release)\b/i
PRIVATE_PATH = %r{(?:\A|/)(?:Users|home|private|tmp|var/folders)(?:/|\z)|(?:\A|/)(?:secrets?|credentials?|tokens?)(?:/|\z)|[A-Za-z]:[\\/]|\\\\|(?:password|api[_-]?key|bearer|gho_|sk-)}i

class ValidationError < StandardError; end

def reject!(message)
  raise ValidationError, message
end

def relative_repo_path!(value, label)
  raw = String(value)
  reject!("#{label} contains private or machine-local material") if raw.match?(PRIVATE_PATH)
  reject!("#{label} contains unsupported characters") unless raw.match?(%r{\A[A-Za-z0-9._/-]+\z})
  path = Pathname.new(raw)
  reject!("#{label} must be repo-relative") if path.absolute? || path.each_filename.any? { |part| ["..", ".", ""].include?(part) }
  resolved = ROOT.join(path).cleanpath
  reject!("#{label} escapes repository") unless resolved.to_s.start_with?(ROOT.to_s + File::SEPARATOR)
  resolved
end

def load_json!(path, label)
  JSON.parse(path.read)
rescue Errno::ENOENT
  reject!("missing #{label}: #{path.relative_path_from(ROOT)}")
rescue JSON::ParserError => error
  reject!("invalid #{label}: #{error.message}")
end

def load_text!(path, label)
  path.read
rescue Errno::ENOENT
  reject!("missing #{label}: #{path.relative_path_from(ROOT)}")
end

def git!(*args)
  stdout, stderr, status = Open3.capture3("git", "-C", ROOT.to_s, *args)
  reject!("git #{args.first} failed: #{stderr.strip}") unless status.success?
  stdout
end

def validate_schema!(schema)
  reject!("unsupported schema id") unless schema["$id"] == "adl.v092.first-birthday-review-packet.schema.v1"
  required = Array(schema["required"])
  expected = %w[schema digest_algorithm packet_sha256 closure_evidence entries public_claims non_claims reviewer_questions]
  reject!("schema required-key mismatch") unless required == expected
  reject!("schema must lock nine entries") unless schema.dig("properties", "entries", "minItems") == 9 && schema.dig("properties", "entries", "maxItems") == 9
  reject!("schema must reject unknown manifest fields") unless schema["additionalProperties"] == false
  entry_required = %w[wp issue_repository code_repository issue pull_request revision merge_commit path digest terminal_state review_state reviewed_revision public_projection]
  reject!("schema entry required-key mismatch") unless schema.dig("$defs", "entry", "required") == entry_required
  reject!("schema must reject unknown entry fields") unless schema.dig("$defs", "entry", "additionalProperties") == false
end

def evidence_revision(entry, source_data)
  source_data["validated_revision"] || source_data["source_revision"] || source_data.dig("independent_validation", "detached_exact_head")
end

def validate_packet_data!(packet, manifest, schema)
  validate_schema!(schema)
  reject!("unsupported manifest schema") unless manifest["schema"] == "adl.v092.first-birthday-review-evidence.v1"
  expected_manifest_keys = %w[schema digest_algorithm packet_sha256 closure_evidence entries public_claims non_claims reviewer_questions]
  reject!("manifest shape mismatch") unless manifest.keys == expected_manifest_keys
  reject!("digest algorithm must be sha256") unless manifest["digest_algorithm"] == "sha256"
  reject!("packet digest mismatch") unless Digest::SHA256.hexdigest(packet) == manifest["packet_sha256"]

  closure_ref = manifest.fetch("closure_evidence") { reject!("missing closure evidence") }
  reject!("closure reference shape mismatch") unless closure_ref.keys == %w[path digest]
  closure_path = relative_repo_path!(closure_ref["path"], "closure evidence path")
  reject!("closure evidence digest mismatch") unless Digest::SHA256.file(closure_path).hexdigest == closure_ref["digest"]
  closure = load_json!(closure_path, "closure evidence")
  reject!("unsupported closure evidence schema") unless closure["schema"] == "adl.v092.birthday-dependency-closure.v1"
  reject!("closure evidence shape mismatch") unless closure.keys == %w[schema observed_at issue_repository code_repository entries]
  closure_entries = Array(closure["entries"])
  reject!("closure issue roster mismatch") unless closure_entries.map { |entry| entry["issue"] } == EXPECTED_ISSUES

  entries = Array(manifest["entries"])
  reject!("WP roster mismatch") unless entries.map { |entry| entry["wp"] } == EXPECTED_WPS
  reject!("issue roster mismatch") unless entries.map { |entry| entry["issue"] } == EXPECTED_ISSUES
  reject!("duplicate evidence digest") unless entries.map { |entry| entry["digest"] }.uniq.length == entries.length

  entries.zip(closure_entries).each do |entry, closed|
    label = entry["wp"]
    expected_entry_keys = %w[wp issue_repository code_repository issue pull_request revision merge_commit path digest terminal_state review_state reviewed_revision public_projection]
    reject!("#{label} entry shape mismatch") unless entry.keys == expected_entry_keys
    expected_closure_keys = %w[issue_repository code_repository issue issue_state pull_request pr_state head_revision merge_commit]
    reject!("#{label} closure entry shape mismatch") unless closed.keys == expected_closure_keys
    expected_issue_repository = entry["issue"] == 209 ? CODE_REPOSITORY : LEGACY_ISSUE_REPOSITORY
    reject!("#{label} issue repository mismatch") unless entry["issue_repository"] == expected_issue_repository && closed["issue_repository"] == expected_issue_repository
    reject!("#{label} code repository mismatch") unless entry["code_repository"] == CODE_REPOSITORY && closed["code_repository"] == CODE_REPOSITORY
    reject!("#{label} closure issue mismatch") unless closed["issue"] == entry["issue"]
    reject!("#{label} issue is not closed") unless closed["issue_state"] == "closed"
    reject!("#{label} pull request is not merged") unless closed["pr_state"] == "merged"
    reject!("#{label} pull request mismatch") unless closed["pull_request"] == entry["pull_request"]
    reject!("#{label} merge mismatch") unless closed["merge_commit"] == entry["merge_commit"]
    reject!("#{label} terminal state is contradictory") unless entry["terminal_state"] == "merged_closed"
    reject!("#{label} lacks approved exact-head review") unless entry["review_state"] == "approved"
    reject!("#{label} reviewed revision malformed") unless String(entry["reviewed_revision"]).match?(/\A(?:git-blake3:[0-9a-f]{40}:[0-9a-f]{64}|[0-9a-f]{40})\z/)
    reject!("#{label} revision malformed") unless String(entry["revision"]).match?(/\A[0-9a-f]{40}\z/)
    reject!("#{label} merge commit malformed") unless String(entry["merge_commit"]).match?(/\A[0-9a-f]{40}\z/)
    reject!("#{label} projection unsafe or unbounded") unless entry["public_projection"].is_a?(String) && entry["public_projection"].bytesize.between?(1, 240) && !entry["public_projection"].match?(PRIVATE_PATH) && !entry["public_projection"].match?(FORBIDDEN_PUBLIC_CLAIMS)

    source = relative_repo_path!(entry["path"], "#{label} evidence path")
    reject!("missing #{label} evidence path") unless source.file?
    actual_digest = Digest::SHA256.file(source).hexdigest
    reject!("#{label} digest mismatch") unless actual_digest == entry["digest"]
    source_data = load_json!(source, "#{label} evidence")
    reject!("#{label} evidence revision mismatch") unless evidence_revision(entry, source_data) == entry["revision"]

    git!("merge-base", "--is-ancestor", entry["merge_commit"], "HEAD")
    committed = git!("show", "#{entry['merge_commit']}:#{entry['path']}")
    reject!("#{label} merge-tree evidence digest mismatch") unless Digest::SHA256.hexdigest(committed) == entry["digest"]
    issue_index = JSON.parse(git!("show", "#{entry['merge_commit']}:.csdlc/issues/#{entry['issue']}/index.json"))
    retained_review = issue_index["review"]
    reject!("#{label} lacks retained typed review authority") unless retained_review.is_a?(Hash) && retained_review["completed"] == true
    reject!("#{label} reviewed revision contradicts retained authority") unless retained_review["reviewed_revision"] == entry["reviewed_revision"]
    reject!("packet omits #{label} evidence path") unless packet.include?(entry["path"])
    reject!("packet omits qualified #{label} issue") unless packet.include?("#{entry['issue_repository']}##{entry['issue']}")
    reject!("packet omits qualified #{label} PR") unless packet.include?("#{entry['code_repository']}##{entry['pull_request']}")
  end

  public_claims = Array(manifest["public_claims"])
  reject!("public claims must be unique bounded strings") unless public_claims.uniq == public_claims && public_claims.all? { |claim| claim.is_a?(String) && claim.bytesize.between?(1, 240) }
  forbidden = public_claims.find { |claim| claim.match?(FORBIDDEN_PUBLIC_CLAIMS) || claim.match?(PRIVATE_PATH) }
  reject!("forbidden public claim") if forbidden
  non_claims = Array(manifest["non_claims"])
  reject!("non-claim boundary is incomplete") unless non_claims.length >= 6 && non_claims.uniq == non_claims
  questions = Array(manifest["reviewer_questions"])
  reject!("reviewer questions are incomplete") unless questions.length >= 5 && questions.uniq == questions

  packet.scan(/`([^`]+)`/).flatten.each do |candidate|
    next unless candidate.start_with?(".csdlc/", "docs/")

    relative_repo_path!(candidate, "packet reference")
  end
end

def mutate!(manifest, mutation)
  case mutation
  when "stale_digest"
    manifest["entries"][0]["digest"] = "0" * 64
  when "missing_roster"
    manifest["entries"].pop
  when "private_path"
    manifest["entries"][0]["path"] = "/Users/operator/private/evidence.json"
  when "contradictory_status"
    manifest["entries"][0]["terminal_state"] = "open"
  when "forbidden_public_claim"
    manifest["public_claims"] << "This proves legal personhood."
  when "publication_ready_overclaim"
    manifest["public_claims"] << "This packet is ready for public release."
  when "unreviewed_entry"
    manifest["entries"][0]["review_state"] = "pending"
  when "unknown_manifest_field"
    manifest["undeclared_field"] = true
  when "forbidden_projection"
    manifest["entries"][0]["public_projection"] = "This proves legal personhood and publication readiness."
  when "wrong_issue_repository"
    manifest["entries"].find { |entry| entry["wp"] == "WP-14" }["issue_repository"] = LEGACY_ISSUE_REPOSITORY
  else
    reject!("unknown negative mutation: #{mutation}")
  end
end

options = {}
OptionParser.new do |parser|
  parser.on("--packet PATH") { |value| options[:packet] = value }
  parser.on("--manifest PATH") { |value| options[:manifest] = value }
  parser.on("--schema PATH") { |value| options[:schema] = value }
  parser.on("--negative-fixtures PATH") { |value| options[:negative_fixtures] = value }
end.parse!

begin
  if options[:negative_fixtures]
    root = relative_repo_path!(options[:negative_fixtures], "negative fixture root")
    config = load_json!(root.join("cases.json"), "negative fixtures")
    reject!("unsupported negative fixture schema") unless config["schema"] == "adl.v092.birthday-review-negative-fixtures.v1"
    packet = load_text!(relative_repo_path!(config["packet"], "fixture packet"), "fixture packet")
    baseline = load_json!(relative_repo_path!(config["manifest"], "fixture manifest"), "fixture manifest")
    schema = load_json!(relative_repo_path!(config["schema_path"], "fixture schema"), "fixture schema")
    cases = Array(config["cases"])
    expected_names = %w[stale-digest missing-roster private-path contradictory-status forbidden-public-claim publication-ready-overclaim unreviewed-entry unknown-manifest-field forbidden-projection wrong-issue-repository]
    reject!("negative case roster mismatch") unless cases.map { |item| item["name"] } == expected_names
    cases.each do |item|
      candidate = Marshal.load(Marshal.dump(baseline))
      mutate!(candidate, item["mutation"])
      begin
        validate_packet_data!(packet, candidate, schema)
      rescue ValidationError
        next
      end
      reject!("negative fixture unexpectedly passed: #{item['name']}")
    end
    puts JSON.generate(schema: "adl.v092.birthday-review-negative-proof.v1", cases: expected_names, outcome: "passed")
    exit 0
  end

  %i[packet manifest schema].each { |key| reject!("--#{key} is required") unless options[key] }
  packet = load_text!(relative_repo_path!(options[:packet], "packet"), "review packet")
  manifest = load_json!(relative_repo_path!(options[:manifest], "manifest"), "evidence manifest")
  schema = load_json!(relative_repo_path!(options[:schema], "schema"), "packet schema")
  validate_packet_data!(packet, manifest, schema)
  puts JSON.generate(schema: "adl.v092.birthday-review-packet-validation.v1", entries: EXPECTED_WPS.length, outcome: "passed")
rescue ValidationError => error
  warn "validate-review-packet: #{error.message}"
  exit 1
end
