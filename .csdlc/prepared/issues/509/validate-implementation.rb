#!/usr/bin/env ruby
require "json"
require "shellwords"
root = File.expand_path("../../../..", __dir__)
receipt_path = File.join(root, "docs/milestones/v0.92.1/evidence/runtime/drt-d/qualification.json")
unless File.file?(receipt_path)
  puts JSON.pretty_generate({
    schema: "adl.v0921.drt_d.implementation.v1",
    outcome: "blocked",
    reason: "missing_retained_gcp_qualification_receipt",
    required_receipt: "docs/milestones/v0.92.1/evidence/runtime/drt-d/qualification.json"
  })
  exit 1
end
receipt = JSON.parse(File.read(receipt_path))
head = `git -C #{Shellwords.escape(root)} rev-parse HEAD`.strip
abort("source revision mismatch") unless receipt.fetch("source_revision") == head
abort("schema mismatch") unless receipt.fetch("schema") == "adl.v0921.drt_d.gcp_portability_qualification.v1"
abort("issue mismatch") unless receipt.fetch("issue") == 509
abort("status mismatch") unless receipt.fetch("status") == "passed"
deps = receipt.fetch("reviewed_dependencies")
abort("dependency denominator mismatch") unless deps.keys.map(&:to_i).sort == [494, 495, 508]
abort("terminal dependency truth mismatch") unless deps.values.all? { |v| v == "terminal" }
identity = receipt.fetch("gcp_identity")
%w[account project billing_account credential_source].each { |k| abort("missing GCP #{k}") if identity.fetch(k).to_s.empty? }
abort("paid authority absent") unless receipt.fetch("paid_authorization") == true
topology = receipt.fetch("topology")
abort("GCP topology must be exactly two nodes") unless topology.fetch("node_count") == 2
abort("Ollama must not be public") unless topology.fetch("ollama_public") == false
provider = receipt.fetch("provider")
abort("provider mismatch") unless provider.fetch("kind") == "ollama"
abort("runtime surface mismatch") unless provider.fetch("runtime_surface") == "gcp_private_ollama_http"
abort("model source must be GCS object storage") unless provider.fetch("model_source") == "gcs_object_storage"
abort("artifact manifest digest missing") unless provider.fetch("artifact_manifest_sha256").match?(/\A[0-9a-f]{64}\z/)
abort("resident model denominator mismatch") unless provider.fetch("models").sort == ["llama3.1:8b", "phi4-mini:latest", "qwen3:8b"].sort
residents = receipt.fetch("residents")
abort("resident denominator mismatch") unless residents.length == 6 && residents.map { |r| r.fetch("identity") }.uniq.length == 6
abort("workload incomplete") unless residents.all? { |r| r.fetch("workload_completed") == true }
abort("continuity drift") unless receipt.fetch("restored_population_digest") == receipt.fetch("dehydrated_population_digest")
abort("AWS authority changed") unless receipt.fetch("aws_qualification_authority") == "unchanged"
cost = receipt.fetch("cost")
abort("cost currency mismatch") unless cost.fetch("currency") == "USD"
abort("cost availability must not claim exact billing") unless cost.fetch("actual_cost_available") == false
abort("actual cost must be unset when billing is unavailable") unless cost["actual_cost_usd"].nil?
abort("cost budget missing") unless cost.fetch("max_budget_usd").is_a?(Numeric) && cost.fetch("max_budget_usd") > 0
abort("cost method must state bounded budget") unless cost.fetch("method").include?("bounded-budget")
abort("cleanup not proven") unless receipt.fetch("cleanup").values.all? { |v| v == "absent" }
puts '{"schema":"adl.v0921.drt_d.implementation.v1","outcome":"passed"}'
