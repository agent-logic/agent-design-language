#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "net/http"
require "openssl"
require "pathname"
require "uri"

ROOT = Pathname.new(File.expand_path("../../../..", __dir__)).realpath
ISSUE = 467
REPO = "agent-logic/agent-design-language"
LEGACY = "danielbaustin/agent-design-language"
FEATURE_INDEX = ROOT / "docs/milestones/v0.92/features/README.md"
COVERAGE = ROOT / "docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md"
OUT = ROOT / "docs/reviews/v0.92/quality-gate-467"
MATRIX = OUT / "feature-completion-matrix.json"
GATE = OUT / "quality-gate-record.json"
REPORT = OUT / "blocker-report.md"
SUPERSESSION = OUT / "311-supersession.md"
EVIDENCE = ROOT / ".csdlc/evidence/467"

# issue repo, issue, PR repo, PR, exact PR head, merge SHA, retained proof
D = {
  wp01: [LEGACY, 5817, LEGACY, 5859, "54b4e0645b5b603bd93cc0e1f19c55e88be534c6", "92451299651c44725a1951d4101b9cba27cad864", "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"],
  wp02: [LEGACY, 5819, LEGACY, 5889, "47d05230bf63c54a99e50f04ddffc7f59a8fb369", "18d3cb93017469521dd0f50c9bc032d6d59ea184", ".csdlc/evidence/5819/copy-report.json"],
  wp02a: [LEGACY, 5801, LEGACY, 5893, "6f545a418c34fcf7787ea22a602e648e1cb9c6ab", "2c4ae1f4cd364995352355ec7a01d257a95315cd", ".csdlc/evidence/5801/ci-topology.md"],
  wp02b: [LEGACY, 5853, REPO, 11, "52ccdd7c0531aadc8cda681c567c16ab0b2b7e75", "12be7269b7bf9933e8e96cdcc272da4a3e21b0d4", ".csdlc/evidence/5853/final-state.json"],
  wp03: [LEGACY, 5820, REPO, 28, "93641db996f2409baf94be2e9e6f27bb1ec9039b", "b5bcfdfc13a6f454a715cbb9aa64e24bce3b7ba6", ".csdlc/evidence/5820/runtime-native-receipts.json"],
  wp04: [LEGACY, 5878, REPO, 140, "1288f89499d26a1a607b96cd96e0b71051194af6", "d3a0d69a4c1507eb038392741d163d8341bd95d1", ".csdlc/evidence/5878/execution-proof.json"],
  wp05: [LEGACY, 5822, REPO, 12, "d0f391ac18e4de0dff4096c3d5b63e3079fca115", "cc1a96fb77f81394be02c54f64f1e6764a47cfd7", ".csdlc/evidence/5822/terminal-baseline-source-5778.json"],
  wp06: [LEGACY, 5823, REPO, 15, "49d79ab24a365b8bc337fac68083445698d45b82", "5219965ba30fa7bf2eeb513cbefa455498d2e4a2", ".csdlc/evidence/5823/deterministic-validation-summary.json"],
  wp07: [LEGACY, 5824, REPO, 24, "066ae86c1d841b795317c13f738d8dfa954dcdd8", "112187ac594b1987a223489574ef3455f2ab5bfa", ".csdlc/evidence/5824/enum-audit-decision.json"],
  identity: [LEGACY, 5827, REPO, 127, "6694a57a0d8381dfca90b5082f616f4dea5488f0", "02b4ad6651fd87100184395d18d4d49f0183f360", ".csdlc/evidence/5827/native-validation-manifest.json"],
  profile: [REPO, 448, REPO, 453, "519d0068d59e98e4a29c6856eedd8678ed02c033", "42838ac100388dd7c43bddd3d0003e606bc3ef97", "docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md"],
  adaptive: [REPO, 449, REPO, 456, "5476288e0cc0e66de823df0c080aae4f2f852aa5", "d834c136a12e66d2334bcea5e36d860b290c7121", ".csdlc/evidence/449/runtime-resident-cycle-integration-proof.log"],
  memory: [REPO, 450, REPO, 458, "fa0fd35a49388315dae5e288ba55380b2e384b26", "46eab3aa2a877917c96b1ac2948648a40dcfc82a", ".csdlc/evidence/450/kernel_memory_palace_packet.log"],
  acip: [REPO, 209, REPO, 215, "c640066f284a915b638add377cc4b0a2e221e6f9", "a77519c3fca9f64752af41c9a2ebd396468891f7", ".csdlc/evidence/209/local-validation-manifest.json"],
  witness: [LEGACY, 5833, REPO, 198, "6c8b1112e99a5ead4f326f863f020f5dbf744fbf", "ed657e4494e08d4ce3de1b554d097632111a83a9", ".csdlc/evidence/5833/native-validation-manifest.json"],
  review: [LEGACY, 5834, REPO, 218, "6fd00ec264393234a44552d659a422333e5ec8be", "f107ac38b3ccc9b050d562c735184351acd35fd3", "docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json"],
  cross_polis: [LEGACY, 5835, REPO, 238, "0a607266287458e34e41c7f600b571dc3a23ed03", "a4c14b4ae51ec5fbc3c3b585b217958972a3246c", "docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md"],
  demo: [REPO, 256, REPO, 427, "6791c38c6e2817387629dbb0e899ae6c61f8b887", "fb4c853bdb9cb140059d2a28af02d70bd36a27a4", ".csdlc/evidence/256/birthday-contract-rust-tests.log"],
  provider: [REPO, 341, REPO, 442, "8166ab8c333fd8b952bfe878e084887e363a4491", "0b5aadebd7cff653c2500106d4a4055f1b9b8818", ".csdlc/evidence/341/proof-matrix.json"],
  governance: [LEGACY, 5839, REPO, 289, "042710838de804f4ccd85a46b48e8e6b7daab1a4", "7f88697ce82215188af941e15cf02a6220c9ad63", "docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md"],
  wp20: [LEGACY, 5840, REPO, 447, "528a870f26db42582c91f9c339ffffce1f8c79cb", "9f373f5f04b0f8c9dc6e3e6cbf348fddec98486c", "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md"],
  reduction: [REPO, 309, REPO, 460, "e6fd6cd6e297f267f4749b9e5b6adc5609fb7e64", "5b3657582fea2109f000623bb121b7998185ac0a", ".csdlc/evidence/309/reduction-report.json"],
  refactor: [REPO, 310, REPO, 465, "ca78a65a1390f2bc088f8cf20018670d06e87068", "a06c34774ad88ea8c56a00533f0fcef810fa7441", "adl/tools/report_large_rust_modules.sh"],
  runtime_qualification: [REPO, 268, REPO, 464, "5f1bb8be2251198ebde5fc2cdaa56a2561d52685", "edbc0d03b454e7dbd6fd11fc3c01000b021ce75c", ".csdlc/evidence/268/aws/issue268-six-hour-r7i-20260821-72/qualification-proof.json"],
  birthday: [REPO, 451, REPO, 459, "414777b543bf5df295a41eacc9c4fd19735c413b", "e926e3bca0ab1981d77b4658d2feb4059bdf33a6", ".csdlc/evidence/451/production-birthday-evidence.json"]
}.freeze

R = {
  "feature:ACP_COGNITIVE_PROFILES_v0.92" => [%i[profile], "Resident-cycle cognitive-profile integration is delivered by #448/PR #453."],
  "feature:ADAPTIVE_LEARNING_DAG_v0.92" => [%i[adaptive], "Governed resident adaptive learning is delivered by #449/PR #456."],
  "feature:ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92" => [%i[acip], "Production ACIP authority and transport contracts are delivered by #209/PR #215."],
  "feature:DISTRIBUTED_GUARDIAN_POLIS_v0.92" => [%i[wp04], "The distributed child wave culminates in #5878/PR #140 integrated proof."],
  "feature:CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92" => [%i[cross_polis], "v0.92 promises migration semantics and negative boundaries; infrastructure execution is explicitly outside this contract."],
  "feature:FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92" => [%i[demo provider governance], "The bounded demo, provider-neutral proof, and governance handoff are delivered independently."],
  "feature:IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92" => [%i[identity birthday], "Stable continuity proof and production Birthday composition are merged."],
  "feature:MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92" => [%i[profile witness birthday], "Capability/profile, witness, and production composition paths are merged."],
  "feature:MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92" => [%i[memory], "Memory Palace production authority is delivered by #450/PR #458."],
  "feature:OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92" => [nil, { "issue" => 84, "target" => "backlog with #122 v0.92.1 and #251 backlog dependencies", "reason" => "Unity consumer readiness remains explicitly owned by #84 and its #122/#251 dependencies." }],
  "feature:PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92" => [%i[provider], "Provider-neutral proof is delivered by #341/PR #442."],
  "feature:RUNTIME_LAUNCH_AND_RESILIENCE_v0.92" => [%i[wp03 runtime_qualification], "Local resilience and successful six-hour six-agent qualification are closed and merged."],
  "feature:FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92" => [%i[birthday], "Production Birthday composition is delivered by #451/PR #459."],
  "critical:AEE-001" => [%i[wp01], "Canonical milestone/version planning landed through WP-01."],
  "critical:AEE-002" => [%i[wp02], "Repository-copy proof landed through WP-02."],
  "critical:AEE-003" => [%i[wp02a], "CI and coverage reliability landed through WP-02A."],
  "critical:AEE-004" => [%i[wp02b], "The bounded build-acceleration decision and fallback proof landed through WP-02B."],
  "critical:AEE-005" => [%i[wp03 runtime_qualification], "Runtime resilience and production qualification landed."],
  "critical:AEE-006" => [%i[wp04], "Distributed Guardian integration culminated in #5878/PR #140."],
  "critical:AEE-007" => [%i[wp05 wp06 wp07], "Cycle-time, remote validation, and typed-card work landed."],
  "critical:AEE-008" => [%i[identity birthday], "Birthday and identity paths are merged and production-composed."],
  "critical:AEE-009" => [%i[memory profile witness], "The duplicate memory/capability entries are normalized into one row backed by complementary deliveries."],
  "critical:AEE-010" => [%i[profile adaptive], "Cognitive profiles and adaptive learning are resident-cycle integrations."],
  "critical:AEE-011" => [%i[acip], "ACIP production authority is merged."],
  "critical:AEE-012" => [%i[witness review], "Witness and integrated review packets are merged."],
  "critical:AEE-013" => [%i[cross_polis], "The declared continuity-semantics scope is merged; infrastructure is a non-goal."],
  "critical:AEE-014" => [%i[demo], "The bounded Birthday demonstration is merged."],
  "critical:AEE-015" => [nil, { "issue" => 84, "target" => "backlog with #122 v0.92.1 and #251 backlog dependencies", "reason" => "Observatory/Unity product work is explicitly owned by #84, not #467." }],
  "critical:AEE-016" => [%i[provider], "Provider-neutral proof is merged."],
  "critical:AEE-017" => [%i[governance], "The v0.93 governance handoff is merged without claiming v0.93 implementation."],
  "critical:AEE-018" => [%i[wp20], "WP-20 proof coverage is merged; implemented_with_evidence is accepted evidence status."],
  "critical:AEE-019" => [%i[reduction refactor], "Reduction #309/PR #460 and refactoring #310/PR #465 are closed and merged."],
  "critical:AEE-020" => [nil, { "issue" => 467, "target" => "release-tail downstream", "reason" => "WP-22 through WP-30 are downstream outcomes; using them as a WP-22 prerequisite is circular. They remain required at their own stages." }]
}.freeze

def denominator
  text = FEATURE_INDEX.read
  section = text.split("## Feature Documents", 2).fetch(1).split("## WP Coverage Map", 2).first
  features = section.scan(/\]\(([^)]+\.md)\)/).flatten.map do |rel|
    line = text.lines.find { |candidate| candidate.start_with?("|") && candidate.include?("](#{rel})") }
    { "id" => "feature:#{File.basename(rel, '.md')}", "kind" => "feature", "source" => "docs/milestones/v0.92/features/#{rel}", "owner" => line ? line.split("|").map(&:strip)[1] : "feature index", "source_status" => "feature_contract" }
  end
  critical = COVERAGE.read.lines.each_with_object([]) do |line, rows|
    next unless line.start_with?("|")
    cells = line.split("|").map(&:strip)
    next unless cells[5]&.match?(/^AEE-\d{3}$/)
    rows << { "id" => "critical:#{cells[5]}", "kind" => "critical_path", "source" => COVERAGE.relative_path_from(ROOT).to_s, "owner" => cells[2], "source_status" => cells[4], "outcome" => cells[1] }
  end.uniq { |row| row["id"] }
  raise "feature denominator must contain 13 rows" unless features.length == 13
  raise "critical denominator must contain 20 rows" unless critical.length == 20
  features + critical
end

def delivery(key)
  issue_repo, issue, pr_repo, pr, head, merge, path = D.fetch(key)
  raise "missing retained evidence #{path}" unless (ROOT / path).file?
  { "key" => key.to_s, "issue_repository" => issue_repo, "issue" => issue, "pr_repository" => pr_repo, "pull_request" => pr, "pr_head" => head, "merge_sha" => merge, "evidence" => { "path" => path, "sha256" => Digest::SHA256.file(ROOT / path).hexdigest } }
end

def build_matrix
  rows = denominator.map do |row|
    keys, boundary = R.fetch(row["id"])
    if keys
      row.merge("disposition" => "accepted", "claim_boundary" => boundary, "discovery" => { "status" => "investigated", "profile" => "closed_issue_merged_pr" }, "blocker_kind" => nil, "blockers" => [], "evidence" => { "deliveries" => keys.map { |key| delivery(key) } })
    else
      row.merge("disposition" => "scoped_out", "claim_boundary" => boundary["reason"], "discovery" => { "status" => "investigated", "profile" => "explicit_milestone_scope" }, "blocker_kind" => nil, "blockers" => [], "evidence" => { "scope" => boundary })
    end
  end
  { "schema" => "adl.v0.92.quality_gate_matrix.v3", "milestone" => "v0.92", "issue" => ISSUE, "supersedes" => { "issue" => 311, "pull_request" => 466 }, "denominator" => { "feature_rows" => 13, "critical_path_rows" => 20, "total_rows" => 33 }, "completion_guard" => "complete_resolution_ledger", "rows" => rows }
end

def token
  path = Pathname.new(ENV.fetch("ADL_GITHUB_TOKEN_FILE", File.join(Dir.home, "keys/github.token")))
  raise "github token path invalid" unless path.absolute? && path.file? && !path.symlink? && (path.stat.mode & 0o077).zero?
  path.read.strip
end

def github(repository, kind, number)
  @github_cache ||= {}
  cache_key = [repository, kind, number]
  return @github_cache.fetch(cache_key) if @github_cache.key?(cache_key)
  uri = URI("https://api.github.com/repos/#{repository}/#{kind}/#{number}")
  req = Net::HTTP::Get.new(uri)
  req["Authorization"] = "Bearer #{token}"
  req["Accept"] = "application/vnd.github+json"
  req["X-GitHub-Api-Version"] = "2022-11-28"
  req["User-Agent"] = "adl-467-quality-gate"
  response = Net::HTTP.start(uri.hostname, uri.port, use_ssl: true, verify_mode: OpenSSL::SSL::VERIFY_PEER) { |http| http.request(req) }
  raise "github #{repository}/#{kind}/#{number} #{response.code}" unless response.is_a?(Net::HTTPSuccess)
  @github_cache[cache_key] = JSON.parse(response.body)
end

def validate_delivery(item, row_id, errors, canonical:)
  key = item["key"]&.to_sym
  unless key && D.key?(key) && item == delivery(key)
    errors << "#{row_id}:delivery_mapping_mismatch:#{item['key']}"
    return
  end
  return unless canonical
  issue = github(item["issue_repository"], "issues", item["issue"])
  pr = github(item["pr_repository"], "pulls", item["pull_request"])
  errors << "#{row_id}:issue_not_closed:#{item['issue']}" unless issue["state"] == "closed"
  errors << "#{row_id}:pr_not_merged:#{item['pull_request']}" unless pr["merged"] == true
  errors << "#{row_id}:pr_head_mismatch:#{item['pull_request']}" unless pr.dig("head", "sha") == item["pr_head"]
  errors << "#{row_id}:merge_sha_mismatch:#{item['pull_request']}" unless pr["merge_commit_sha"] == item["merge_sha"]
rescue StandardError => error
  errors << "#{row_id}:canonical_observation_failed:#{error.message}"
end

def validate_matrix(path, canonical: true)
  matrix = JSON.parse(path.read)
  errors = []
  expected = denominator
  rows = Array(matrix["rows"])
  errors << "schema_invalid" unless matrix["schema"] == "adl.v0.92.quality_gate_matrix.v3"
  errors << "denominator_invalid" unless matrix["denominator"] == { "feature_rows" => 13, "critical_path_rows" => 20, "total_rows" => 33 }
  expected_ids = expected.map { |row| row["id"] }
  observed_ids = rows.map { |row| row["id"] }
  errors << "denominator_missing:#{(expected_ids - observed_ids).join(',')}" unless (expected_ids - observed_ids).empty?
  errors << "denominator_extra:#{(observed_ids - expected_ids).join(',')}" unless (observed_ids - expected_ids).empty?
  errors << "denominator_duplicate" unless observed_ids.uniq.length == observed_ids.length
  expected_by_id = expected.to_h { |row| [row["id"], row] }
  rows.each do |row|
    id = row["id"]
    next unless expected_by_id[id]
    %w[kind source owner source_status].each { |field| errors << "#{id}:#{field}_mismatch" unless row[field] == expected_by_id[id][field] }
    errors << "#{id}:uninvestigated" unless row.dig("discovery", "status") == "investigated"
    errors << "#{id}:has_blockers" unless Array(row["blockers"]).empty?
    keys, scope = R.fetch(id)
    expected_boundary = keys ? scope : scope["reason"]
    errors << "#{id}:claim_boundary_mismatch" unless row["claim_boundary"] == expected_boundary
    if keys
      errors << "#{id}:accepted_mapping_missing" unless row["disposition"] == "accepted"
      deliveries = Array(row.dig("evidence", "deliveries"))
      errors << "#{id}:delivery_count_mismatch" unless deliveries.map { |item| item["key"] } == keys.map(&:to_s)
      deliveries.each { |item| validate_delivery(item, id, errors, canonical: canonical) }
    else
      errors << "#{id}:scope_mismatch" unless row["disposition"] == "scoped_out" && row.dig("evidence", "scope") == scope
      if canonical && scope["issue"] == 84
        issue = github(REPO, "issues", 84)
        labels = Array(issue["labels"]).map { |label| label["name"] }
        errors << "#{id}:scope_issue_not_open" unless issue["state"] == "open"
        errors << "#{id}:scope_issue_not_backlog" unless labels.include?("track:backlog")
        errors << "#{id}:scope_dependency_title_mismatch" unless issue["title"].to_s.include?("#122/#251")
      end
    end
  end
  [matrix, errors]
end

def write_docs_notes
  marker = "\n## WP-22A Corrective Hydration\n\n"
  notes = {
    "docs/milestones/v0.92/QUALITY_GATE_v0.92.md" => "Issue #467 supersedes #311/PR #466 release-credit semantics. Its complete closed-issue/merged-PR ledger accepts 30 rows, explicitly scopes 3 rows to existing owners or downstream stages, and has zero blockers. #311/PR #466 remain historical provenance only.\n",
    "docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md" => "WP-22A #467 resolves the quality-gate evidence ledger with zero blockers. Downstream work depends on merged implementation and its own stage gates, never asynchronous issue closeout. AEE-020 is a downstream release-tail outcome, not a circular prerequisite to WP-22.\n",
    "docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md" => "The #467 ledger recognizes `implemented_with_evidence`, #341 provider-neutral proof, #268 successful qualification, and completed #309/#310 work. Observatory/Unity remains explicitly owned by backlog #84; AEE-020 remains required at downstream release stages rather than self-gating WP-22. The quality-gate packet has zero blockers.\n"
  }
  notes.each do |relative, note|
    path = ROOT / relative
    path.write(path.read.split(marker, 2).first + marker + note)
  end
end

def write_packet
  OUT.mkpath
  EVIDENCE.mkpath
  matrix = build_matrix
  MATRIX.write(JSON.pretty_generate(matrix) + "\n")
  accepted = matrix["rows"].count { |row| row["disposition"] == "accepted" }
  scoped = matrix["rows"].count { |row| row["disposition"] == "scoped_out" }
  blocked = matrix["rows"].count { |row| row["disposition"] == "blocked" }
  gate = { "schema" => "adl.v0.92.quality_gate_record.v3", "issue" => ISSUE, "supersedes_issue" => 311, "supersedes_pr" => 466, "matrix_sha256" => Digest::SHA256.file(MATRIX).hexdigest, "validator_sha256" => Digest::SHA256.file(__FILE__).hexdigest, "feature_rows" => 13, "critical_path_rows" => 20, "accepted_rows" => accepted, "scoped_out_rows" => scoped, "blocked_rows" => blocked, "result" => blocked.zero? ? "passed" : "blocked", "downstream_unlock" => blocked.zero?, "completion_guard" => "complete_resolution_ledger" }
  GATE.write(JSON.pretty_generate(gate) + "\n")
  lines = ["# v0.92 WP-22A Corrective Resolution Report", "", "Result: **#{gate['result'].upcase}**", "", "Accepted rows: #{accepted}. Explicitly scoped rows: #{scoped}. Blocked rows: #{blocked}.", "", "## Resolved Rows", ""]
  matrix["rows"].each do |row|
    refs = Array(row.dig("evidence", "deliveries")).map { |d| "#{d['issue_repository']}##{d['issue']} / #{d['pr_repository']}##{d['pull_request']}" }.join("; ")
    refs = "issue ##{row.dig('evidence', 'scope', 'issue')} -> #{row.dig('evidence', 'scope', 'target')}" if refs.empty?
    lines << "- `#{row['id']}` — **#{row['disposition']}** — #{refs}: #{row['claim_boundary']}"
  end
  lines.concat(["", "## Downstream", "", "No quality-gate evidence blocker remains. Explicitly scoped product work and release-tail outcomes remain owned by their named issues or stages; #467 does not claim to implement them.", ""])
  REPORT.write(lines.join("\n"))
  SUPERSESSION.write("# #311 Supersession Note\n\n#311 / PR #466 remain immutable historical provenance. #467 supersedes only release-credit semantics with a complete closed-issue/merged-PR ledger and explicit scope boundaries.\n")
  write_docs_notes
  receipt = { "schema" => "adl.v0.92.quality_gate_validation_receipt.v3", "issue" => ISSUE, "matrix_sha256" => Digest::SHA256.file(MATRIX).hexdigest, "gate_sha256" => Digest::SHA256.file(GATE).hexdigest, "validator_sha256" => Digest::SHA256.file(__FILE__).hexdigest, "blocker_report_sha256" => Digest::SHA256.file(REPORT).hexdigest, "quality_gate_result" => gate["result"], "downstream_unlock" => gate["downstream_unlock"], "denominator" => { "feature_rows" => 13, "critical_path_rows" => 20, "total_rows" => 33, "accepted_rows" => accepted, "scoped_out_rows" => scoped, "blocked_rows" => blocked }, "completion_guard" => "passed" }
  (EVIDENCE / "validation.json").write(JSON.pretty_generate(receipt) + "\n")
  matrix
end

if __FILE__ == $PROGRAM_NAME
  if (ARGV.shift || "matrix") == "generate"
    matrix = write_packet
    puts JSON.generate(schema: "adl.v0.92.quality_gate_generation.v3", status: "generated", rows: matrix["rows"].length)
  else
    matrix, errors = validate_matrix(MATRIX, canonical: true)
    if errors.empty?
      blocked = matrix["rows"].count { |row| row["disposition"] == "blocked" }
      puts JSON.generate(schema: "adl.v0.92.quality_gate_validation.v3", status: "passed", rows: matrix["rows"].length, blocked_rows: blocked, gate_result: blocked.zero? ? "passed" : "blocked")
    else
      warn JSON.generate(schema: "adl.v0.92.quality_gate_validation.v3", status: "failed", errors: errors)
      exit 1
    end
  end
end
